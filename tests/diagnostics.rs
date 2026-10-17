//! What the receiver of a message that deviates answers it with: the cause and the
//! Criticality Diagnostics of TS 36.413 clause 10, from what `check` reports of the
//! message, and from the error of the macro that extracts its IEs.
//!
//! Clause 10.2 has an IE whose value does not decode as a transfer syntax error, which
//! an error indication tells. Clause 10.3.4 has the receiver act on an IE that it does
//! not comprehend by the criticality that the IE was received with, and clause 10.3.5 on
//! a missing IE by the criticality that the specification assigns it. The Criticality
//! Diagnostics name each IE of the criticality `reject` or `notify` with that
//! criticality, its identifier and the type of the error.

use oxirush_s1ap::aper::{self, ErrorIe};
use oxirush_s1ap::helpers::{EnbKind, eutran_cgi, global_enb_id, plmn, tai};
use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{Deviation, Error, ErrorKind, build_s1ap, extract_s1ap_ies};

/// The identifiers of two IEs of an S1 SETUP REQUEST: SupportedTAs, which is mandatory
/// with the criticality `reject`, and DefaultPagingDRX, which is mandatory with the
/// criticality `ignore`.
const SUPPORTED_TAS: u16 = 64;
const DEFAULT_PAGING_DRX: u16 = 137;

fn areas() -> Result<SupportedTAs, Error> {
    let networks = BPLMNs(vec![plmn("208", "93")?]);
    let area = SupportedTAsItem::new(TAC([0x00, 0x01]), networks, None);
    Ok(SupportedTAs(vec![area]))
}

/// An S1 SETUP REQUEST as an eNB sends it, whose last IE is the DefaultPagingDRX.
fn request() -> Result<Vec<u8>, Error> {
    build_s1ap!(InitiatingMessage, S1Setup, REJECT, S1SetupRequest,
        REJECT Global_ENB_ID(global_enb_id(plmn("208", "93")?, EnbKind::Macro, 1)?),
        IGNORE eNBname(ENBname("enb-1".into())),
        REJECT SupportedTAs(areas()?),
        IGNORE DefaultPagingDRX(PagingDRX::v128),
    )?
    .encode()
}

/// The same request without its SupportedTAs.
fn without_areas() -> Result<Vec<u8>, Error> {
    build_s1ap!(InitiatingMessage, S1Setup, REJECT, S1SetupRequest,
        REJECT Global_ENB_ID(global_enb_id(plmn("208", "93")?, EnbKind::Macro, 1)?),
        IGNORE eNBname(ENBname("enb-1".into())),
        IGNORE DefaultPagingDRX(PagingDRX::v128),
    )?
    .encode()
}

/// The request with a DefaultPagingDRX that is no value of its type: its one octet
/// says that the value is an extension, and there is none.
fn spoiled() -> Result<Vec<u8>, Error> {
    let mut octets = request()?;
    *octets.last_mut().unwrap() = 0xff;
    Ok(octets)
}

/// What an MME reads of the request: the first IE that is missing or does not decode
/// is its error.
fn accept(request: &S1SetupRequest) -> Result<usize, Error> {
    extract_s1ap_ies!(request, S1SetupRequest,
        req node: GlobalENBID = Global_ENB_ID(node) => node,
        req areas: SupportedTAs = SupportedTAs(areas) => areas,
        req drx: PagingDRX = DefaultPagingDRX(drx) => drx,
        opt name: ENBname = eNBname(name) => name,
    );
    let _ = (node, drx, name);
    Ok(areas.0.len())
}

/// The Criticality Diagnostics that answer an S1 SETUP REQUEST, with these items. In the
/// S1 SETUP FAILURE, the response of the procedure, they have the IEs alone; in an
/// ERROR INDICATION also the procedure, the message and its criticality (TS 36.413
/// clause 9.2.1.21). They have no list without an item, as the list has one at least.
fn diagnostics(
    indication: bool,
    items: Vec<CriticalityDiagnosticsIEItem>,
) -> CriticalityDiagnostics {
    let items = (!items.is_empty()).then_some(CriticalityDiagnosticsIEList(items));
    match indication {
        true => CriticalityDiagnostics::new(
            Some(ProcedureCode(17)),
            Some(TriggeringMessage::initiating_message),
            Some(Criticality::reject),
            items,
            None,
        ),
        false => CriticalityDiagnostics::new(None, None, None, items, None),
    }
}

/// The cause of the first deviation of a request that has one, and the items of the
/// Criticality Diagnostics of its IEs.
fn reported(pdu: &S1AP_PDU) -> Result<(Cause, Vec<CriticalityDiagnosticsIEItem>), Error> {
    let deviations = pdu.check()?;
    let cause = deviations.iter().find_map(Deviation::cause);
    let items = deviations.iter().filter_map(Deviation::diagnostics_item);
    let cause = Cause::protocol(cause.unwrap_or(CauseProtocol::unspecified));
    Ok((cause, items.collect()))
}

/// The S1 SETUP FAILURE that rejects a request which deviates, as clauses 10.3.4.2,
/// 10.3.5 and 10.3.6 have the procedure rejected.
fn failure(pdu: &S1AP_PDU) -> Result<S1AP_PDU, Error> {
    let (cause, items) = reported(pdu)?;
    build_s1ap!(UnsuccessfulOutcome, S1Setup, REJECT, S1SetupFailure,
        IGNORE Cause(cause),
        IGNORE CriticalityDiagnostics(diagnostics(false, items)),
    )
}

/// The ERROR INDICATION that answers a request with a transfer syntax error, as clause
/// 10.2 has it.
fn indication(pdu: &S1AP_PDU) -> Result<S1AP_PDU, Error> {
    let (cause, items) = reported(pdu)?;
    build_s1ap!(InitiatingMessage, ErrorIndication, IGNORE, ErrorIndication,
        IGNORE Cause(cause),
        IGNORE CriticalityDiagnostics(diagnostics(true, items)),
    )
}

/// The cause and the Criticality Diagnostics of a failure, as its receiver reads them.
fn answered(failure: &S1AP_PDU) -> Result<(Cause, CriticalityDiagnostics), Error> {
    let received = S1AP_PDU::decode(&failure.encode()?)?;
    let message: S1SetupFailure = received.decode_value()?;
    extract_s1ap_ies!(message, S1SetupFailure,
        req cause: Cause = Cause(cause) => cause,
        req diagnostics: CriticalityDiagnostics = CriticalityDiagnostics(diagnostics) => diagnostics,
    );
    Ok((cause, diagnostics))
}

/// The cause and the Criticality Diagnostics of an error indication, as its receiver
/// reads them.
fn indicated(indication: &S1AP_PDU) -> Result<(Cause, CriticalityDiagnostics), Error> {
    let received = S1AP_PDU::decode(&indication.encode()?)?;
    let message: ErrorIndication = received.decode_value()?;
    extract_s1ap_ies!(message, ErrorIndication,
        req cause: Cause = Cause(cause) => cause,
        req diagnostics: CriticalityDiagnostics = CriticalityDiagnostics(diagnostics) => diagnostics,
    );
    Ok((cause, diagnostics))
}

#[test]
fn a_request_as_the_specification_has_it_deviates_in_nothing() -> Result<(), Error> {
    let pdu = S1AP_PDU::decode(&request()?)?;
    assert_eq!(pdu.check()?, []);
    assert_eq!(accept(&pdu.decode_value()?)?, 1);
    Ok(())
}

#[test]
fn the_error_of_a_missing_ie_has_the_criticality_that_is_assigned() -> Result<(), Error> {
    let pdu = S1AP_PDU::decode(&without_areas()?)?;
    let error = accept(&pdu.decode_value()?).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::MissingIe);
    let ie = ErrorIe::new(SUPPORTED_TAS, aper::Criticality::Reject, None);
    assert_eq!(error.ie(), Some(ie));

    // The deviation of the error is the one that `check` reports.
    let missing = Deviation::MissingIe {
        id: SUPPORTED_TAS,
        name: "SupportedTAs",
        criticality: Criticality::reject,
    };
    assert_eq!(Deviation::of(&error), Some(missing.clone()));
    assert_eq!(pdu.check()?, std::slice::from_ref(&missing));

    // Clause 10.3.5: rejected, and reported as missing with the assigned criticality.
    assert_eq!(
        missing.cause(),
        Some(CauseProtocol::abstract_syntax_error_reject)
    );
    assert_eq!(missing.type_of_error(), Some(TypeOfError::missing));
    let item = CriticalityDiagnosticsIEItem::new(
        Criticality::reject,
        ProtocolIEID(SUPPORTED_TAS),
        TypeOfError::missing,
        None,
    );
    assert_eq!(missing.diagnostics_item(), Some(item.clone()));
    let cause = Cause::protocol(CauseProtocol::abstract_syntax_error_reject);
    assert_eq!(
        answered(&failure(&pdu)?)?,
        (cause, diagnostics(false, vec![item]))
    );
    Ok(())
}

#[test]
fn the_error_of_an_ie_that_does_not_decode_has_the_criticality_that_was_received()
-> Result<(), Error> {
    let pdu = S1AP_PDU::decode(&spoiled()?)?;
    let error = accept(&pdu.decode_value()?).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::UndecodableIe);
    assert_eq!(
        error.to_string(),
        "the DefaultPagingDRX of this S1SetupRequest does not decode: the octets end before \
         the value does"
    );
    // The fourth IE of the message, which was sent with the criticality `ignore`.
    let ie = ErrorIe::new(DEFAULT_PAGING_DRX, aper::Criticality::Ignore, Some(3));
    assert_eq!(error.ie(), Some(ie));

    // The deviation of the error is the one that `check` reports.
    let undecodable = Deviation::of(&error).unwrap();
    assert!(matches!(
        undecodable,
        Deviation::UndecodableIe {
            id: DEFAULT_PAGING_DRX,
            name: "DefaultPagingDRX",
            position: 3,
            criticality: Criticality::ignore,
            ..
        }
    ));
    assert_eq!(pdu.check()?, std::slice::from_ref(&undecodable));
    assert_eq!(
        undecodable.to_string(),
        "DefaultPagingDRX (IE 137), at position 3, does not decode: the octets end before \
         the value does"
    );

    // Clause 10.2: a transfer syntax error, which an error indication tells. The IE was
    // received as `ignore`, which no item of the diagnostics has.
    assert_eq!(
        undecodable.cause(),
        Some(CauseProtocol::transfer_syntax_error)
    );
    assert_eq!(
        undecodable.type_of_error(),
        Some(TypeOfError::not_understood)
    );
    assert_eq!(undecodable.diagnostics_item(), None);
    let cause = Cause::protocol(CauseProtocol::transfer_syntax_error);
    assert_eq!(
        indicated(&indication(&pdu)?)?,
        (cause, diagnostics(true, vec![]))
    );
    Ok(())
}

/// TS 36.413 clause 9.2.1.21: the criticality of an item of the Criticality Diagnostics
/// is never `ignore` ("The value 'ignore' shall not be used"), as clauses 10.3.4.2 and
/// 10.3.5 have a receiver report the IEs of the criticalities `reject` and `notify`.
#[test]
fn an_ie_of_the_criticality_ignore_is_in_no_item_of_the_diagnostics() -> Result<(), Error> {
    // Received as `ignore`, and not decoded.
    let deviations = S1AP_PDU::decode(&spoiled()?)?.check()?;
    assert!(matches!(
        deviations[..],
        [Deviation::UndecodableIe {
            criticality: Criticality::ignore,
            ..
        }]
    ));
    assert_eq!(deviations[0].diagnostics_item(), None);
    // Missing, where the object set assigns `ignore`.
    let missing = Deviation::MissingIe {
        id: DEFAULT_PAGING_DRX,
        name: "DefaultPagingDRX",
        criticality: Criticality::ignore,
    };
    assert_eq!(missing.diagnostics_item(), None);
    // The same IE missing as `reject` has its item.
    let reject = Deviation::MissingIe {
        id: DEFAULT_PAGING_DRX,
        name: "DefaultPagingDRX",
        criticality: Criticality::reject,
    };
    let item = CriticalityDiagnosticsIEItem::new(
        Criticality::reject,
        ProtocolIEID(DEFAULT_PAGING_DRX),
        TypeOfError::missing,
        None,
    );
    assert_eq!(reject.diagnostics_item(), Some(item));
    Ok(())
}

#[test]
fn check_reports_every_ie_that_is_missing_or_does_not_decode() -> Result<(), Error> {
    // Without its SupportedTAs, and with a DefaultPagingDRX that does not decode: the
    // macro stops at the first, and `check` has both.
    let mut octets = without_areas()?;
    *octets.last_mut().unwrap() = 0xff;
    let pdu = S1AP_PDU::decode(&octets)?;
    // The first of its fields that has no value: the areas, which are missing.
    let first = accept(&pdu.decode_value()?).unwrap_err();
    assert_eq!(first.kind(), ErrorKind::MissingIe);

    let deviations = pdu.check()?;
    let ies: Vec<_> = deviations.iter().map(Deviation::ie).collect();
    assert_eq!(
        ies,
        [
            Some((DEFAULT_PAGING_DRX, Criticality::ignore)),
            Some((SUPPORTED_TAS, Criticality::reject)),
        ]
    );
    let types: Vec<_> = deviations.iter().map(Deviation::type_of_error).collect();
    assert_eq!(
        types,
        [
            Some(TypeOfError::not_understood),
            Some(TypeOfError::missing)
        ]
    );
    // The items: the missing IE, which is `reject`, and not the one that does not
    // decode, which was received as `ignore`.
    let items: Vec<_> = (deviations.iter())
        .filter_map(Deviation::diagnostics_item)
        .collect();
    let missing = CriticalityDiagnosticsIEItem::new(
        Criticality::reject,
        ProtocolIEID(SUPPORTED_TAS),
        TypeOfError::missing,
        None,
    );
    assert_eq!(items, [missing]);
    Ok(())
}

/// Clause 10.3.4.2: an IE that the receiver does not comprehend is acted on by the
/// criticality that it was received with, and clause 10.3.6 rejects a message with an
/// IE that is there twice or in a wrong order.
#[test]
fn each_deviation_has_the_cause_of_its_clause() -> Result<(), Error> {
    // An UPLINK NAS TRANSPORT with an IE of no message, received as `notify`, another
    // as `ignore`, and its identifiers in the other order.
    let ie =
        |id: u16, criticality| ProtocolIEField::new(ProtocolIEID(id), criticality, vec![0].into());
    let network = plmn("208", "93")?;
    let mut pdu = build_s1ap!(InitiatingMessage, uplinkNASTransport, IGNORE, UplinkNASTransport,
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT NAS_PDU(vec![0x07, 0x5d]),
        IGNORE EUTRAN_CGI(eutran_cgi(network, EnbKind::Macro, 1, 1)?),
        IGNORE TAI(tai(network, [0x00, 0x01])),
    )?;
    let mut ies = pdu.protocol_ies()?.into_owned();
    ies.0.push(ie(60_000, Criticality::notify));
    ies.0.push(ie(60_001, Criticality::ignore));
    pdu.set_protocol_ies(ies)?;

    let deviations = pdu.check()?;
    let causes: Vec<_> = deviations.iter().map(Deviation::cause).collect();
    assert_eq!(
        causes,
        [
            Some(CauseProtocol::abstract_syntax_error_falsely_constructed_message),
            Some(CauseProtocol::abstract_syntax_error_ignore_and_notify),
            None,
        ],
        "{deviations:?}"
    );
    let items: Vec<_> = (deviations.iter())
        .filter_map(Deviation::diagnostics_item)
        .collect();
    let item = |id: u16, criticality| {
        CriticalityDiagnosticsIEItem::new(
            criticality,
            ProtocolIEID(id),
            TypeOfError::not_understood,
            None,
        )
    };
    // The IE received as `ignore` has none.
    assert_eq!(items, [item(60_000, Criticality::notify)]);
    Ok(())
}

/// TS 36.413 clause 9.1.8.5: the MME name of an S1 SETUP RESPONSE is a PrintableString
/// (SIZE (1..150, ...)). A name of no character encodes, as a value of a later version
/// of the type, which a sender of this version does not send. Clause 10.3.1 has a
/// receiver handle a value that its version does not define as an IE that it does not
/// comprehend.
#[test]
fn a_value_outside_the_root_of_an_extensible_constraint_is_reported() -> Result<(), Error> {
    // It encodes, and an encoding that is strict refuses it.
    let none = MMEname("".into());
    assert_eq!(aper::encode(&none)?, [0x80, 0x00]);
    let refused = aper::encode_strict(&none).unwrap_err();
    assert_eq!(refused.kind(), ErrorKind::Encode);
    assert_eq!(
        refused.to_string(),
        "a value of MMEname does not encode: a size of 0 is outside the root of its \
         constraint (1..150, ...)"
    );

    // An S1 SETUP RESPONSE with that name is built as any other, and `check` says it.
    let response = |name: &'static str| {
        let served = ServedGUMMEIsItem::new(
            ServedPLMNs(vec![plmn("208", "93")?]),
            ServedGroupIDs(vec![MMEGroupID([0x00, 0x01])]),
            ServedMMECs(vec![MMECode([0x01])]),
            None,
        );
        build_s1ap!(SuccessfulOutcome, S1Setup, REJECT, S1SetupResponse,
            IGNORE MMEname(MMEname(name.into())),
            REJECT ServedGUMMEIs(ServedGUMMEIs(vec![served])),
            IGNORE RelativeMMECapacity(RelativeMMECapacity(255)),
        )
    };
    assert_eq!(response("mme-1")?.check()?, []);
    let deviations = response("")?.check()?;
    assert!(matches!(
        deviations[..],
        [Deviation::OutsideRoot {
            id: 61,
            name: "MMEname",
            position: 0,
            criticality: Criticality::ignore,
            ..
        }]
    ));
    assert_eq!(
        deviations[0].to_string(),
        "MMEname (IE 61), at position 0, has a value of a later version of its type: a size \
         of 0 is outside the root of its constraint (1..150, ...)"
    );
    assert_eq!(deviations[0].ie(), Some((61, Criticality::ignore)));
    assert_eq!(
        deviations[0].type_of_error(),
        Some(TypeOfError::not_understood)
    );
    // The receiver ignores an IE of the criticality `ignore` that it does not comprehend.
    assert_eq!(deviations[0].cause(), None);
    Ok(())
}
