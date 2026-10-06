#![cfg(feature = "inspect")]

use oxirush_s1ap::{build_s1ap, inspect, s1ap::*};

#[test]
fn inspect_every_canonical_procedure_and_round_trip() {
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let wire = hex::decode(fields[1]).unwrap();
        let pdu = S1AP_PDU::decode(&wire).unwrap();
        let tree = inspect::inspect_pdu(&pdu).unwrap();
        assert!(tree["message"].is_object(), "{}", fields[0]);
        assert_eq!(
            inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
            wire,
            "{}",
            fields[0]
        );
    }
}

#[test]
fn editing_a_known_ie_cannot_drop_unknown_sequence_additions() {
    // Independent complete warning request from unknown_extensions.rs. The
    // first TAI has a future SEQUENCE addition; the second must stay intact.
    let wire = hex::decode(
        "00240031000005006f00028000007000028000007140122000018002f83900010101000002f839000200720002000a007300020001",
    )
    .unwrap();
    let pdu = S1AP_PDU::decode(&wire).unwrap();
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    assert_eq!(inspect::encode_pdu(&tree).unwrap().encode().unwrap(), wire);
    let path = "/message/protocolIEs/2/value/trackingAreaListforWarning/0/tAC";
    assert_eq!(tree.pointer(path), Some(&serde_json::json!("0001")));
    *tree.pointer_mut(path).unwrap() = serde_json::json!("0003");
    assert!(
        inspect::encode_pdu(&tree).is_err(),
        "a typed edit must not silently discard a future IE extension"
    );
}

fn release_request(enb_ue_id: u32) -> S1AP_PDU {
    build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(enb_ue_id),
    )
}

#[test]
fn a_typed_edit_is_encoded_and_the_other_ies_keep_their_octets() {
    let mut tree = inspect::inspect_pdu(&release_request(7)).unwrap();
    let path = "/message/protocolIEs/1/value";
    assert_eq!(tree.pointer(path), Some(&serde_json::json!(7)));
    *tree.pointer_mut(path).unwrap() = serde_json::json!(9);
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        release_request(9).encode().unwrap()
    );
    tree["typo"] = serde_json::json!(1);
    assert!(inspect::encode_pdu(&tree).is_err());
}

#[test]
fn the_octets_of_an_ie_are_replaced_or_added_as_its_value_without_raw_value() {
    let other = inspect::inspect_pdu(&release_request(9)).unwrap();
    let mut tree = inspect::inspect_pdu(&release_request(7)).unwrap();
    let ie = tree.pointer_mut("/message/protocolIEs/1").unwrap();
    ie.as_object_mut().unwrap().remove("_raw_value");
    ie["value"] = other["message"]["protocolIEs"][1]["_raw_value"].clone();
    let ies = tree.pointer_mut("/message/protocolIEs").unwrap();
    ies.as_array_mut()
        .unwrap()
        .push(serde_json::json!({"id": 60000, "criticality": "ignore", "value": "C0FFEE"}));
    let edited = inspect::encode_pdu(&tree).unwrap();
    let tree = inspect::inspect_pdu(&edited).unwrap();
    assert_eq!(
        tree.pointer("/message/protocolIEs/1/value"),
        Some(&serde_json::json!(9))
    );
    assert_eq!(
        tree.pointer("/message/protocolIEs/2/value"),
        Some(&serde_json::json!("C0FFEE"))
    );
}

#[test]
fn an_ie_that_does_not_decode_is_edited_as_its_octets() {
    let request = UEContextReleaseRequest::new(ProtocolIEContainer(vec![ProtocolIEField::new(
        ProtocolIEID(60000),
        Criticality::ignore,
        vec![0xC0, 0xFF, 0xEE].into(),
    )]));
    let pdu = S1AP_PDU::initiatingMessage(InitiatingMessage::new(
        ProcedureCode(18),
        Criticality::ignore,
        encode_open_type(&request).unwrap(),
    ));
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    let path = "/message/protocolIEs/0/value";
    assert_eq!(tree.pointer(path), Some(&serde_json::json!("C0FFEE")));
    assert!(tree["message"]["protocolIEs"][0]["_decode_error"].is_string());
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
    *tree.pointer_mut(path).unwrap() = serde_json::json!("DECAFBAD");
    let edited = inspect::encode_pdu(&tree).unwrap();
    assert_eq!(
        inspect::inspect_pdu(&edited).unwrap().pointer(path),
        Some(&serde_json::json!("DECAFBAD"))
    );
    // Its type is unknown, so a typed value has no encoding.
    *tree.pointer_mut(path).unwrap() = serde_json::json!(5);
    assert!(inspect::encode_pdu(&tree).is_err());
}

#[test]
fn a_changed_raw_value_alone_sends_nothing_else() {
    let other = inspect::inspect_pdu(&release_request(9)).unwrap();
    let pdu = release_request(7);
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    tree["message"]["protocolIEs"][1]["_raw_value"] =
        other["message"]["protocolIEs"][1]["_raw_value"].clone();
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
}
