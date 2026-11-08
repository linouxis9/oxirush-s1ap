//! The protocol IEs of a PDU, whatever its message: read, and put back.

use oxirush_s1ap::Octets;
use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{Error, ErrorKind, Ie, build_s1ap, ie};

/// The messages of the fixtures, then the vectors with content: the name of each, and
/// the octets of its PDU.
fn fixtures() -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let messages = include_str!("fixtures/messages.tsv").lines();
    let lines = messages.chain(include_str!("fixtures/vectors.tsv").lines());
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let mut fields = line.split('\t');
        let name = fields.next().unwrap();
        (name, hex::decode(fields.next().unwrap()).unwrap())
    })
}

/// A UE CONTEXT RELEASE REQUEST with its three IEs, the message of the README.
const RELEASE_REQUEST: &str = "0012401500000300000002002a000800020007000240020280";

#[test]
fn the_ies_of_every_message_are_read_and_put_back_as_the_same_octets() {
    let mut read = 0;
    for (name, wire) in fixtures() {
        let mut pdu = S1APPDU::decode(&wire).unwrap();
        // The private message has private IEs: see the test of the errors.
        if name.split('/').next() == Some("PrivateMessage") {
            continue;
        }
        // The IEs borrow from the PDU: they are owned before the PDU is changed.
        let ies = pdu.protocol_ies();
        let ies = ies.unwrap_or_else(|error| panic!("{name}: {error}"));
        let ies = ies.into_owned();
        pdu.set_protocol_ies(ies).unwrap();
        assert_eq!(pdu.encode().unwrap(), wire, "{name}");
        read += 1;
    }
    // Each message of the fixtures but the private one, and each vector.
    assert!(read > 100, "{read} messages");
}

#[test]
fn a_message_that_the_macros_built_has_the_ies_that_it_was_built_with() -> Result<(), Error> {
    let cause = || Cause::radioNetwork(CauseRadioNetwork::user_inactivity);
    let pdu = build_s1ap!(IGNORE UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(cause()),
    )?;
    let built = vec![
        ie::MME_UE_S1AP_ID::field(Criticality::Reject, 42u32)?,
        ie::eNB_UE_S1AP_ID::field(Criticality::Reject, 7u32)?,
        ie::Cause::field(Criticality::Ignore, cause())?,
    ];
    assert_eq!(pdu.protocol_ies()?.0, built);

    // They are those of the typed message, and of the octets of the README.
    let request: UEContextReleaseRequest = pdu.decode_value()?;
    assert_eq!(pdu.protocol_ies()?, request.protocol_ies);
    let received = S1APPDU::decode(&hex::decode(RELEASE_REQUEST).unwrap())?.into_owned();
    assert_eq!(received.protocol_ies()?.0, built);
    Ok(())
}

#[test]
fn an_unknown_ie_and_a_repeated_one_are_kept_with_their_criticality() -> Result<(), Error> {
    // An IE of the message, an IE that the protocol does not have, the first again with
    // another value and another criticality, and an IE of another message.
    let unknown = ProtocolIEField {
        id: ProtocolIEID(60000),
        criticality: Criticality::Notify,
        value: Octets::from(vec![0xc0, 0xff]),
    };
    let mut again = ie::MME_UE_S1AP_ID::field(Criticality::Reject, 2u32)?;
    again.criticality = Criticality::Ignore;
    let ies = vec![
        ie::MME_UE_S1AP_ID::field(Criticality::Reject, 1u32)?,
        unknown,
        again,
        ie::TimeToWait::field(Criticality::Notify, TimeToWait::v5s)?,
    ];

    // Put into a message that had three other IEs: the PDU is the same but for them.
    let mut pdu = S1APPDU::decode(&hex::decode(RELEASE_REQUEST).unwrap())?.into_owned();
    pdu.set_protocol_ies(ProtocolIEContainer(ies.clone()))?;
    assert_eq!(pdu.kind(), S1apPduKind::UEContextReleaseRequest);
    let S1APPDU::initiatingMessage(held) = &pdu else {
        panic!("{pdu}");
    };
    assert_eq!(held.criticality, Criticality::Ignore);

    // They are read back as they were written, from the PDU and from its octets.
    assert_eq!(pdu.protocol_ies()?.0, ies);
    let received = S1APPDU::decode(&pdu.encode()?)?.into_owned();
    assert_eq!(received, pdu);
    let read = received.protocol_ies()?.0;
    assert_eq!(read, ies);
    let told: Vec<_> = read.iter().map(|ie| (ie.id.0, ie.criticality)).collect();
    assert_eq!(
        told,
        [
            (0, Criticality::Reject),
            (60000, Criticality::Notify),
            (0, Criticality::Ignore),
            (65, Criticality::Notify),
        ]
    );
    Ok(())
}

#[test]
fn the_message_of_an_unknown_procedure_is_read_as_a_container_of_protocol_ies() -> Result<(), Error>
{
    // The IEs of the release request, under a procedure code that TS 36.413 does not have.
    let known = S1APPDU::decode(&hex::decode(RELEASE_REQUEST).unwrap())?.into_owned();
    let S1APPDU::initiatingMessage(mut held) = known.clone() else {
        panic!("{known}");
    };
    held.procedure_code = ProcedureCode(200);
    let mut pdu = S1APPDU::initiatingMessage(held);
    assert!(matches!(pdu.kind(), S1apPduKind::Other { .. }));
    let mut ies = pdu.protocol_ies()?.into_owned();
    assert_eq!(ies, known.protocol_ies()?);
    ies.0.pop();
    pdu.set_protocol_ies(ies)?;
    assert_eq!(pdu.protocol_ies()?.0.len(), 2);
    assert_eq!(pdu.procedure_code(), 200);
    Ok(())
}

#[test]
fn a_message_without_protocol_ies_and_octets_that_are_none_are_errors() -> Result<(), Error> {
    // The private message has private IEs, and no container of protocol IEs.
    let (_, wire) = fixtures()
        .find(|(name, _)| *name == "PrivateMessage")
        .expect("the private message");
    let mut private = S1APPDU::decode(&wire)?;
    let before = private.clone();
    let read = private.protocol_ies().unwrap_err();
    assert_eq!(read.kind(), ErrorKind::WrongMessage);
    assert_eq!(
        read.to_string(),
        "the message of this PDU is PrivateMessage, which has no container of protocol IEs"
    );
    let written = private.set_protocol_ies(ProtocolIEContainer(vec![]));
    assert_eq!(written, Err(read));
    assert_eq!(private, before);

    // Octets that are no message: the message is named.
    let request = S1APPDU::decode(&hex::decode(RELEASE_REQUEST).unwrap())?.into_owned();
    let S1APPDU::initiatingMessage(mut held) = request.clone() else {
        panic!("{request}");
    };
    held.value = Octets::from(vec![0x00, 0x00, 0x03, 0x00]);
    let short = S1APPDU::initiatingMessage(held.clone());
    let error = short.protocol_ies().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert_eq!(
        error.to_string(),
        "the UEContextReleaseRequest of this PDU does not decode: the octets end before the \
         value does, in its field `protocol_ies.id`"
    );
    // The same octets under a procedure that the specification does not have.
    held.procedure_code = ProcedureCode(200);
    let error = S1APPDU::initiatingMessage(held).protocol_ies().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert_eq!(
        error.to_string(),
        "the message of this PDU does not decode as a container of protocol IEs: the octets \
         end before the value does, in its field `protocol_ies.id`"
    );

    // More IEs than a container has: the PDU is as it was.
    let ie = ie::eNB_UE_S1AP_ID::field(Criticality::Reject, 7u32)?;
    let mut pdu = request.clone();
    let error = pdu
        .set_protocol_ies(ProtocolIEContainer(vec![ie; 65536]))
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Encode);
    assert!(
        error
            .to_string()
            .starts_with("these 65536 protocol IEs do not encode as a message: "),
        "{error}"
    );
    assert_eq!(pdu, request);
    Ok(())
}

/// A field of the macro that extracts is the value of the IE when it has the type of
/// the IE, and the one value that the type is a name for when it has the type of that
/// value: an MME-UE-S1AP-ID is a name for a number, and a Cause is a name for nothing.
#[test]
fn a_field_takes_the_value_of_an_ie_as_its_type_or_as_the_value_that_it_names() -> Result<(), Error>
{
    let pdu = build_s1ap!(IGNORE UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;
    let request: UEContextReleaseRequest = pdu.decode_value()?;
    oxirush_s1ap::extract_s1ap_ies!(request, UEContextReleaseRequest,
        req cause: Cause = Cause(cause),
        req id: MMEUES1APID = MME_UE_S1AP_ID(id),
        req number: u32 = MME_UE_S1AP_ID(id),
        opt enb: ENBUES1APID = eNB_UE_S1AP_ID(id),
        lax enb_number: u32 = eNB_UE_S1AP_ID(id),
        opt twice: u32 = MME_UE_S1AP_ID(id) => id.0 * 2,
    );
    assert_eq!(
        cause,
        Cause::radioNetwork(CauseRadioNetwork::user_inactivity)
    );
    assert_eq!((id, number), (MMEUES1APID(42), 42));
    assert_eq!((enb, enb_number), (Some(ENBUES1APID(7)), Some(7)));
    assert_eq!(twice, Some(84));
    Ok(())
}

/// TS 36.413 clause 9.1.4.1: the E-RABs of an INITIAL CONTEXT SETUP REQUEST are a list
/// of single containers, each of which holds one IE, the item of an E-RAB. An item is
/// in no message, and is built without the name of one.
#[test]
fn an_item_of_a_list_of_single_containers_is_built_without_the_name_of_a_message()
-> Result<(), Error> {
    let bearer = |id: u64| -> Result<ProtocolIEField<'static>, Error> {
        let priority = AllocationAndRetentionPriority {
            priority_level: PriorityLevel(8),
            pre_emption_capability: PreEmptionCapability::shall_not_trigger_pre_emption,
            pre_emption_vulnerability: PreEmptionVulnerability::not_pre_emptable,
            ie_extensions: None,
        };
        let item = ERABToBeSetupItemCtxtSUReq {
            e_rab_id: ERABID(id),
            e_rab_level_qos_parameters: ERABLevelQoSParameters {
                qci: QCI(9),
                allocation_retention_priority: priority,
                gbr_qos_information: None,
                ie_extensions: None,
            },
            transport_layer_address: TransportLayerAddress(oxirush_s1ap::BitString::from([
                10, 0, 0, 1,
            ])),
            gtp_teid: GTPTEID([0, 0, 0, id as u8]),
            nas_pdu: None,
            ie_extensions: None,
        };
        ie::E_RABToBeSetupItemCtxtSUReq::field(Criticality::Reject, item)
    };
    let bearers = ERABToBeSetupListCtxtSUReq(vec![bearer(5)?, bearer(6)?]);
    let rates = UEAggregateMaximumBitrate {
        ue_aggregate_maximum_bit_rate_dl: BitRate(100_000_000),
        ue_aggregate_maximum_bit_rate_ul: BitRate(50_000_000),
        ie_extensions: None,
    };
    let pdu = build_s1ap!(REJECT InitialContextSetupRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT uEaggregateMaximumBitrate(rates),
        REJECT E_RABToBeSetupListCtxtSUReq(bearers),
        REJECT UESecurityCapabilities(oxirush_s1ap::helpers::ue_security_capabilities(0xe0, 0xe0)),
        REJECT SecurityKey(SecurityKey::from([0x5a; 32])),
    )?;
    // The message is as the specification has a sender construct it.
    assert_eq!(pdu.check()?, []);

    // Its receiver reads each item out of its container.
    let received = S1APPDU::decode(&pdu.encode()?)?.into_owned();
    let message: InitialContextSetupRequest = received.decode_value()?;
    oxirush_s1ap::extract_s1ap_ies!(message, InitialContextSetupRequest,
        req list: ERABToBeSetupListCtxtSUReq = E_RABToBeSetupListCtxtSUReq(list),
    );
    let mut ids = Vec::new();
    for container in &list.0 {
        assert_eq!(
            (container.id.0, container.criticality),
            (52, Criticality::Reject)
        );
        let item: ERABToBeSetupItemCtxtSUReq = oxirush_s1ap::decode(&container.value)?;
        ids.push(item.e_rab_id.0);
    }
    assert_eq!(ids, [5, 6]);

    // With the name of a message before it, an IE is the same IE.
    let named = ie::MME_UE_S1AP_ID::field(Criticality::Reject, 42u32)?;
    assert_eq!(
        named,
        ie::MME_UE_S1AP_ID::field(Criticality::Reject, 42u32)?
    );
    Ok(())
}
