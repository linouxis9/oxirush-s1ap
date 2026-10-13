//! What `check()` reports of a PDU against the object set of its message, and that it
//! refuses nothing: a message that deviates is built by the macros, written from a
//! tree, encoded and decoded as any other.
//!
//! TS 36.413 clause 9.3.0 has what a sender constructs: the IEs in the order of the
//! object set, a mandatory IE once, and an optional or a conditional IE at most once.
//! Clause 10.3 has what a receiver does with a message that is not so: with an IE that
//! it does not comprehend (10.3.4), with a missing one (10.3.5), and with IEs in a
//! wrong order or with too many occurrences (10.3.6). These tests are for a tool that
//! sends such a message to see what its peer does.

use oxirush_s1ap::Octets;
use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{Deviation, Error, ErrorKind, build_s1ap};

/// What `check()` reports of `pdu`, which is the same for the PDU that its octets
/// decode to. With the `inspect` feature, `tree` is the same message written as a
/// tree: it has the same octets.
fn checked(pdu: &S1AP_PDU, tree: &str) -> Vec<Deviation> {
    let deviations = pdu.check().unwrap();
    let octets = pdu.encode().unwrap();
    let received = S1AP_PDU::decode(&octets).unwrap();
    assert_eq!(received, *pdu);
    assert_eq!(received.check().unwrap(), deviations);
    #[cfg(feature = "inspect")]
    {
        let tree = serde_json::from_str(tree).unwrap();
        let written = S1AP_PDU::from_tree(&tree).unwrap();
        assert_eq!(written.encode().unwrap(), octets, "{tree}");
        assert_eq!(written.check().unwrap(), deviations);
    }
    let _ = tree;
    deviations
}

fn cause() -> Cause {
    Cause::radioNetwork(CauseRadioNetwork::user_inactivity)
}

/// The messages of the fixtures: the name of each, and the octets of its PDU.
fn fixtures() -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    vectors(include_str!("fixtures/messages.tsv"))
}

/// The vectors of a file of the fixtures: the name of each, and the octets of its PDU.
fn vectors(file: &'static str) -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let lines = file.lines();
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let mut fields = line.split('\t');
        let name = fields.next().unwrap();
        (name, hex::decode(fields.next().unwrap()).unwrap())
    })
}

#[test]
fn a_message_as_its_object_set_has_it_deviates_in_nothing() -> Result<(), Error> {
    // UE CONTEXT RELEASE REQUEST: its three mandatory IEs, in the order of the set.
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}}
    ]}"#;
    assert_eq!(checked(&pdu, tree), []);
    Ok(())
}

#[test]
fn a_mandatory_ie_that_is_missing_is_reported() -> Result<(), Error> {
    // Without its Cause, whose presence is mandatory.
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7}
    ]}"#;
    let missing = Deviation::MissingIe {
        id: 2,
        name: "Cause",
        criticality: Criticality::ignore,
    };
    assert_eq!(checked(&pdu, tree), [missing]);
    Ok(())
}

#[test]
fn an_ie_that_is_there_twice_is_reported() -> Result<(), Error> {
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(cause()),
        REJECT MME_UE_S1AP_ID(43u32),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
        {"MME-UE-S1AP-ID": 43}
    ]}"#;
    let repeated = Deviation::RepeatedIe {
        id: 0,
        name: "MME-UE-S1AP-ID",
        position: 3,
        first: 0,
    };
    assert_eq!(checked(&pdu, tree), [repeated]);
    Ok(())
}

#[test]
fn an_ie_of_another_message_is_reported() -> Result<(), Error> {
    // TimeToWait is an IE of the failures of the setup procedures.
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE TimeToWait(TimeToWait::v5s),
        IGNORE Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"TimeToWait": "v5s", "criticality": "ignore"},
        {"Cause": {"radioNetwork": "user-inactivity"}}
    ]}"#;
    let unexpected = Deviation::UnexpectedIe {
        id: 65,
        name: Some("TimeToWait"),
        position: 2,
        criticality: Criticality::ignore,
    };
    assert_eq!(checked(&pdu, tree), [unexpected]);
    Ok(())
}

#[test]
fn ies_that_are_out_of_the_order_of_the_object_set_are_reported() -> Result<(), Error> {
    // The two identifiers in the other order.
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT MME_UE_S1AP_ID(42u32),
        IGNORE Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"eNB-UE-S1AP-ID": 7},
        {"MME-UE-S1AP-ID": 42},
        {"Cause": {"radioNetwork": "user-inactivity"}}
    ]}"#;
    let out_of_order = Deviation::OutOfOrder {
        id: 0,
        name: "MME-UE-S1AP-ID",
        position: 1,
        after: 8,
        after_name: "eNB-UE-S1AP-ID",
    };
    assert_eq!(checked(&pdu, tree), [out_of_order]);
    Ok(())
}

#[test]
fn a_criticality_that_is_not_the_assigned_one_is_reported() -> Result<(), Error> {
    // The Cause with `reject`, where the object set of the message has `ignore`.
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}, "criticality": "reject"}
    ]}"#;
    let criticality = Deviation::IeCriticality {
        id: 2,
        name: "Cause",
        position: 2,
        criticality: Criticality::reject,
        assigned: Criticality::ignore,
    };
    assert_eq!(checked(&pdu, tree), [criticality]);

    // The PDU with `reject`, where the procedure has `ignore`.
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        REJECT, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(cause()),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "criticality": "reject", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}}
    ]}"#;
    let of_the_pdu = Deviation::PduCriticality {
        criticality: Criticality::reject,
        assigned: Criticality::ignore,
    };
    assert_eq!(checked(&pdu, tree), [of_the_pdu]);
    Ok(())
}

#[test]
fn every_deviation_of_a_message_is_reported_in_the_order_of_the_message() -> Result<(), Error> {
    // The message of the README: each sentence is what `Display` says.
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        REJECT, UEContextReleaseRequest,
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE MME_UE_S1AP_ID(42u32),
        IGNORE TimeToWait(TimeToWait::v5s),
        REJECT eNB_UE_S1AP_ID(8u32),
    )?;
    let tree = r#"{"message": "UEContextReleaseRequest", "criticality": "reject", "ies": [
        {"eNB-UE-S1AP-ID": 7},
        {"MME-UE-S1AP-ID": 42, "criticality": "ignore"},
        {"TimeToWait": "v5s", "criticality": "ignore"},
        {"eNB-UE-S1AP-ID": 8}
    ]}"#;
    let said: Vec<_> = checked(&pdu, tree)
        .iter()
        .map(|deviation| deviation.to_string())
        .collect();
    assert_eq!(
        said,
        [
            "the criticality of the PDU is reject, and that of its procedure is ignore",
            "MME-UE-S1AP-ID (IE 0), at position 1, comes after eNB-UE-S1AP-ID (IE 8), which \
             the object set of the message has after it",
            "the criticality of MME-UE-S1AP-ID (IE 0), at position 1, is ignore, and the \
             message assigns it reject",
            "TimeToWait (IE 65), at position 2, is not an IE of the message",
            "eNB-UE-S1AP-ID (IE 8), at position 3, is in the message already, at position 0",
            "Cause (IE 2) is mandatory, and the message does not have it",
        ]
    );

    // An IE that the protocol does not have is told by its identifier.
    let mut unknown = pdu.clone();
    let mut ies = unknown.protocol_ies()?.into_owned();
    ies.0.truncate(1);
    let octets = Octets::from(vec![0xc0, 0xff]);
    ies.0
        .push(ProtocolIEField::new(60000u16, Criticality::ignore, octets));
    unknown.set_protocol_ies(ies)?;
    let said = unknown.check()?[1].to_string();
    assert_eq!(
        said,
        "IE 60000, at position 1, is not an IE of the message, nor of the protocol"
    );
    Ok(())
}

#[test]
fn a_conditional_ie_is_not_reported_missing() -> Result<(), Error> {
    // HANDOVER COMMAND, with its four mandatory IEs. Its NASSecurityParametersfromE-UTRAN
    // is conditional: TS 36.413 clause 9.1.5.2 has it present when the Handover Type is
    // LTEtoUTRAN or LTEtoGERAN, which is prose that the object set does not have.
    let pdu = build_s1ap!(SuccessfulOutcome, HandoverPreparation,
        REJECT, HandoverCommand,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT HandoverType(HandoverType::intralte),
        REJECT Target_ToSource_TransparentContainer(vec![0x00]),
    )?;
    assert_eq!(pdu.check()?, []);
    Ok(())
}

#[test]
fn what_has_no_object_set_is_not_checked_against_one() -> Result<(), Error> {
    // The private message has private IEs: the criticality of its PDU is all that the
    // specification assigns it.
    let (_, wire) = fixtures()
        .find(|(name, _)| *name == "PrivateMessage")
        .expect("the private message");
    let private = S1AP_PDU::decode(&wire)?;
    assert_eq!(private.check()?, []);
    let S1AP_PDU::initiatingMessage(mut held) = private else {
        panic!("an initiating message");
    };
    held.criticality = Criticality::reject;
    let of_the_pdu = Deviation::PduCriticality {
        criticality: Criticality::reject,
        assigned: Criticality::ignore,
    };
    let private = S1AP_PDU::initiatingMessage(held.clone());
    assert_eq!(private.check()?, [of_the_pdu]);

    // A procedure that the specification does not have in that direction has none.
    held.procedure_code = ProcedureCode(200);
    let error = S1AP_PDU::initiatingMessage(held).check().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::UnknownMessage);
    assert_eq!(
        error.to_string(),
        "the specification has no InitiatingMessage of procedure 200, and so no object set \
         for its IEs"
    );

    // A message that does not decode is an error, and no deviation.
    let request = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
    )?;
    let S1AP_PDU::initiatingMessage(mut held) = request else {
        panic!("an initiating message");
    };
    held.value = Octets::from(vec![0x00, 0x00, 0x03, 0x00]);
    let error = S1AP_PDU::initiatingMessage(held).check().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    Ok(())
}

#[test]
fn the_messages_of_the_fixtures_are_checked_against_their_object_sets() {
    let mut deviating = Vec::new();
    let mut checked = 0;
    for (name, wire) in fixtures() {
        let pdu = S1AP_PDU::decode(&wire).unwrap();
        let found = pdu
            .check()
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        checked += 1;
        for deviation in found {
            deviating.push(format!("{name}: {deviation}"));
        }
    }
    assert_eq!(checked, 101);
    assert_eq!(deviating, Vec::<String>::new(), "{deviating:#?}");
}

/// The vectors with content have the IEs of the object sets as pycrate has them from
/// the ASN.1: each IE that a set has, in the order of the set and with its criticality.
#[test]
fn the_vectors_with_content_are_checked_against_their_object_sets() {
    let mut checked = 0;
    for (name, wire) in vectors(include_str!("fixtures/vectors.tsv")) {
        let pdu = S1AP_PDU::decode(&wire).unwrap();
        let found = pdu
            .check()
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(found, [], "{name}");
        checked += 1;
    }
    assert!(checked > 101, "{checked} vectors");
}
