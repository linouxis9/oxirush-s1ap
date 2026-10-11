use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, macros::MissingIeError};

#[test]
fn public_macros_round_trip_typed_open_message() -> Result<(), MissingIeError> {
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    );

    let S1AP_PDU::initiatingMessage(message) = &pdu else {
        panic!("expected initiating message");
    };
    // Extension bit, octet padding, then the aligned 16-bit ProtocolIE count.
    assert_eq!(&message.value.as_bytes()[..3], &[0x00, 0x00, 0x03]);

    let wire = pdu.encode().expect("encode PDU");
    let decoded = S1AP_PDU::decode(&wire).expect("decode PDU");
    assert_eq!(decoded, pdu);

    let request: UEContextReleaseRequest = decoded.decode_value().expect("decode message");
    extract_s1ap_ies!(&request, UEContextReleaseRequest,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        req cause: Cause = Cause(value) => value,
    );
    assert_eq!(mme_id, 42);
    assert_eq!(enb_id, 7);
    assert_eq!(
        cause,
        Cause::radioNetwork(CauseRadioNetwork::user_inactivity)
    );
    Ok(())
}

/// Constrained lists whose `rasn` attribute rustfmt splits over several
/// lines use the corrected APER codec too: an octet-aligned length for
/// SIZE(1..256) (X.691 (02/2021) §11.9.3.3), as Wireshark decodes it.
#[test]
fn every_constrained_list_encodes_its_length_per_x691() {
    let diagnostics = CriticalityDiagnostics::new(
        Some(ProcedureCode(9)),
        Some(TriggeringMessage::initiating_message),
        Some(Criticality::reject),
        Some(CriticalityDiagnosticsIEList(vec![
            CriticalityDiagnosticsIEItem::new(
                Criticality::reject,
                ProtocolIEID(26),
                TypeOfError::missing,
                None,
            ),
        ])),
        None,
    );
    let error = build_s1ap!(InitiatingMessage, ErrorIndication,
        IGNORE, ErrorIndication,
        IGNORE Cause(Cause::protocol(CauseProtocol::abstract_syntax_error_reject)),
        IGNORE CriticalityDiagnostics(diagnostics),
    );
    let wire = error.encode().expect("encode ErrorIndication");
    assert_eq!(
        hex::encode(&wire),
        "000f40140000020002400131003a40087809000000001a40"
    );
    assert_eq!(S1AP_PDU::decode(&wire).expect("decode"), error);
}

/// Lists declared through the parameterized E-RAB-IE-ContainerList keep the
/// SIZE(1..maxnoofE-RABs) that ProtocolIE-ContainerList takes as parameters:
/// an octet-aligned count for SIZE(1..256) (X.691 (02/2021) §11.9.3.3), as
/// Wireshark decodes E-RABAdmittedList.
#[test]
fn e_rab_container_lists_encode_their_size_constraint() {
    let item = ERABAdmittedItem::new(
        ERABID(5u8.into()),
        TransportLayerAddress(oxirush_s1ap::helpers::bytes_to_bitvec(&[10, 0, 0, 3])),
        GTPTEID::from([0, 0, 0, 3]),
        None,
        None,
        None,
        None,
        None,
    );
    let admitted = ERABAdmittedList(vec![ProtocolIEField::new(
        ID_E_RABADMITTED_ITEM.0,
        Criticality::ignore,
        encode_open_type(&item).expect("encode E-RAB admitted item"),
    )]);
    let acknowledge = build_s1ap!(SuccessfulOutcome, HandoverResourceAllocation,
        REJECT, HandoverRequestAcknowledge,
        IGNORE MME_UE_S1AP_ID(1u32),
        IGNORE eNB_UE_S1AP_ID(2u32),
        IGNORE E_RABAdmittedList(admitted.clone()),
        REJECT Target_ToSource_TransparentContainer(vec![0x55]),
    );
    let wire = acknowledge
        .encode()
        .expect("encode HandoverRequestAcknowledge");
    assert_eq!(
        hex::encode(&wire),
        "2001002900000400004002000100084002000200124010000014400b00a1f00a00000300000003007b00020155"
    );

    let decoded: HandoverRequestAcknowledge = S1AP_PDU::decode(&wire)
        .expect("decode PDU")
        .decode_value()
        .expect("decode HandoverRequestAcknowledge");
    let ie = decoded
        .protocol_ies
        .0
        .iter()
        .find(|ie| ie.id.0 == ID_E_RABADMITTED_LIST.0)
        .expect("E-RABAdmittedList IE");
    assert_eq!(
        decode_open_type::<ERABAdmittedList>(&ie.value).expect("decode E-RABAdmittedList"),
        admitted
    );
}

/// TS 36.413 declares both ECGIList, SIZE(1..maxnoofCellID) with maxnoofCellID
/// 65535, for the cells of WarningAreaList, and ECGI-List,
/// SIZE(1..maxnoofCellsineNB) with maxnoofCellsineNB 256, for the aggressor
/// cells of SynchronisationInformation: a two-octet and a one-octet count
/// (X.691 (02/2021) §11.9.3.3, §11.5.7), as Wireshark decodes them.
#[test]
fn each_ecgi_list_encodes_its_own_size_constraint() {
    use oxirush_s1ap::helpers::{eutran_cgi, plmn};

    let cell = eutran_cgi(plmn("208", "93"), 0x12345, 1);
    let mut identifier = rasn::types::FixedBitString::<16>::ZERO;
    identifier.set(0, true);
    let request = build_s1ap!(InitiatingMessage, WriteReplaceWarning,
        REJECT, WriteReplaceWarningRequest,
        REJECT MessageIdentifier(MessageIdentifier(identifier)),
        REJECT SerialNumber(SerialNumber(identifier)),
        IGNORE WarningAreaList(WarningAreaList::cellIDList(ECGIList(vec![cell.clone()]))),
        REJECT RepetitionPeriod(RepetitionPeriod(10)),
        REJECT NumberofBroadcastRequest(NumberofBroadcastRequest(1)),
    );
    let wire = request.encode().expect("encode WriteReplaceWarningRequest");
    assert_eq!(
        hex::encode(&wire),
        "0024002a000005006f000280000070000280000071400b0000000002f8391234501000720002000a007300020001"
    );
    assert_eq!(S1AP_PDU::decode(&wire).expect("decode"), request);

    let synchronisation =
        SynchronisationInformation::new(None, None, Some(ECGI_List(vec![cell])), None);
    let wire = rasn::aper::encode(&synchronisation).expect("encode SynchronisationInformation");
    assert_eq!(hex::encode(&wire), "10000002f83912345010");
    assert_eq!(
        rasn::aper::decode::<SynchronisationInformation>(&wire).expect("decode"),
        synchronisation
    );
}

/// An open type holds a complete encoding, and the empty encoding of a
/// single-value ENUMERATED becomes one zero octet (X.691 (02/2021) §11.1.4,
/// §11.2.1): pycrate rejects an empty KillAllWarningMessages, and Wireshark
/// decodes it only from that octet.
#[test]
fn empty_encoding_in_an_open_type_is_one_zero_octet() -> Result<(), MissingIeError> {
    let mut identifier = rasn::types::FixedBitString::<16>::ZERO;
    identifier.set(0, true);
    let kill = build_s1ap!(InitiatingMessage, Kill, REJECT, KillRequest,
        REJECT MessageIdentifier(MessageIdentifier(identifier)),
        REJECT SerialNumber(SerialNumber(identifier)),
        REJECT KillAllWarningMessages(KillAllWarningMessages::R_true),
    );
    let wire = kill.encode().expect("encode KillRequest");
    assert_eq!(
        hex::encode(&wire),
        "002b0014000003006f0002800000700002800000bf000100"
    );

    let request: KillRequest = S1AP_PDU::decode(&wire)
        .expect("decode PDU")
        .decode_value()
        .expect("decode KillRequest");
    extract_s1ap_ies!(&request, KillRequest,
        req kill_all: KillAllWarningMessages = KillAllWarningMessages(value) => value,
    );
    assert_eq!(kill_all, KillAllWarningMessages::R_true);
    Ok(())
}

/// id-S1-Message, a mandatory IE of RerouteNASRequest, has the inline type
/// OCTET STRING rather than a named one; the macros address it too.
#[test]
fn reroute_nas_request_carries_the_s1_message_ie() -> Result<(), MissingIeError> {
    let message = hex::decode("000f40140000020002400131003a40087809000000001a40").unwrap();
    let reroute = build_s1ap!(InitiatingMessage, RerouteNASRequest,
        REJECT, RerouteNASRequest,
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT S1_Message(message.clone()),
        REJECT MME_Group_ID([0x80, 0x01]),
    );
    let wire = reroute.encode().expect("encode RerouteNASRequest");
    assert_eq!(
        hex::encode(&wire),
        "0034002c00000300080002000700e1001918000f40140000020002400131003a40087809000000001a4000df00028001"
    );

    let request: RerouteNASRequest = S1AP_PDU::decode(&wire)
        .expect("decode PDU")
        .decode_value()
        .expect("decode RerouteNASRequest");
    extract_s1ap_ies!(&request, RerouteNASRequest,
        req s1_message: Vec<u8> = S1_Message(value) => value.to_vec(),
    );
    assert_eq!(s1_message, message);
    Ok(())
}

/// An extracted field and its binding may share a name.
#[test]
fn extraction_binding_may_share_the_field_name() -> Result<(), MissingIeError> {
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    );
    let request: UEContextReleaseRequest = pdu.decode_value().expect("decode message");
    extract_s1ap_ies!(&request, UEContextReleaseRequest,
        req mme_ue_s1ap_id: u32 = MME_UE_S1AP_ID(mme_ue_s1ap_id),
        opt cause: Cause = Cause(cause) => cause,
    );
    assert_eq!(mme_ue_s1ap_id, 42);
    assert_eq!(
        cause,
        Some(Cause::radioNetwork(CauseRadioNetwork::user_inactivity))
    );
    Ok(())
}

/// The crate names the type of id-eNB-UE-S1AP-ID `ENB_UE_S1AP_ID`, and the macros
/// take that alias wherever they take `eNB_UE_S1AP_ID`, the name of the IE.
#[test]
fn the_alias_of_the_enb_ue_s1ap_id_names_its_ie_in_each_macro() -> Result<(), MissingIeError> {
    use oxirush_s1ap::{build_s1ap_ie, with_s1ap_ie_mut};

    // The alias is the type, and the two names are one entry.
    let id: ENB_UE_S1AP_ID = ENBUES1APID(7);
    let by_alias = build_s1ap_ie!(UEContextReleaseRequest, REJECT ENB_UE_S1AP_ID(id));
    let by_name = build_s1ap_ie!(UEContextReleaseRequest, REJECT eNB_UE_S1AP_ID(7u32));
    assert_eq!(by_alias, by_name);
    assert_eq!(by_alias.id, ID_E_NB_UE_S1_AP_ID);

    // A PDU that is built by the alias is the octets of one built by the name.
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT ENB_UE_S1AP_ID(7u32),
    );
    let named = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
    );
    assert_eq!(pdu.encode().unwrap(), named.encode().unwrap());

    // The alias reads the IE, and changes it.
    let mut request: UEContextReleaseRequest = pdu.decode_value().expect("decode message");
    extract_s1ap_ies!(&request, UEContextReleaseRequest,
        req read: u32 = ENB_UE_S1AP_ID(id),
        opt absent: Cause = Cause(cause) => cause,
    );
    assert_eq!((read, absent), (7, None));
    assert!(with_s1ap_ie_mut!(
        request,
        UEContextReleaseRequest,
        ENB_UE_S1AP_ID(id) = 9u32
    ));
    extract_s1ap_ies!(&request, UEContextReleaseRequest,
        req changed: u32 = eNB_UE_S1AP_ID(id),
    );
    assert_eq!(changed, 9);
    Ok(())
}

/// A message with the identifier that the functions below require, or without it.
fn release_request(id: Option<u32>) -> UEContextReleaseRequest {
    let ies = id
        .map(|id| oxirush_s1ap::build_s1ap_ie!(UEContextReleaseRequest, REJECT MME_UE_S1AP_ID(id)));
    UEContextReleaseRequest::new(ProtocolIEContainer(ies.into_iter().collect()))
}

/// A `req` field that is not there is the error of the enclosing function: the
/// macro converts its `MissingIeError` as `?` converts one, here into an error
/// type of the caller that has a `From<MissingIeError>`.
#[test]
fn a_missing_required_ie_is_an_error_of_the_callers_own_type() {
    #[derive(Debug, PartialEq)]
    enum Refused {
        Missing(&'static str),
    }
    impl From<MissingIeError> for Refused {
        fn from(error: MissingIeError) -> Self {
            Self::Missing(error.ie_name)
        }
    }
    fn id(request: &UEContextReleaseRequest) -> Result<u32, Refused> {
        extract_s1ap_ies!(request, UEContextReleaseRequest,
            req mme_id: u32 = MME_UE_S1AP_ID(id),
        );
        Ok(mme_id)
    }
    assert_eq!(id(&release_request(None)), Err(Refused::Missing("mme_id")));
    assert_eq!(id(&release_request(Some(42))), Ok(42));
}

/// The same in a function whose error is a `Box<dyn Error>`, which holds the
/// `MissingIeError`.
#[test]
fn a_missing_required_ie_is_a_boxed_error() {
    fn id(request: &UEContextReleaseRequest) -> Result<u32, Box<dyn std::error::Error>> {
        extract_s1ap_ies!(request, UEContextReleaseRequest,
            req mme_id: u32 = MME_UE_S1AP_ID(id),
        );
        Ok(mme_id)
    }
    let error = id(&release_request(None)).unwrap_err();
    let missing = error.downcast_ref::<MissingIeError>().expect("the error");
    assert_eq!(missing.ie_name, "mme_id");
    assert_eq!(
        error.to_string(),
        "missing or invalid required S1AP IE: mme_id"
    );
    assert_eq!(id(&release_request(Some(42))).unwrap(), 42);
}

/// And in a function whose error is `MissingIeError` itself, which needs no
/// conversion.
#[test]
fn a_missing_required_ie_is_a_missing_ie_error() {
    fn id(request: &UEContextReleaseRequest) -> Result<u32, MissingIeError> {
        extract_s1ap_ies!(request, UEContextReleaseRequest,
            req mme_id: u32 = MME_UE_S1AP_ID(id),
        );
        Ok(mme_id)
    }
    assert_eq!(id(&release_request(None)).unwrap_err().ie_name, "mme_id");
    assert_eq!(id(&release_request(Some(42))).unwrap(), 42);
}
