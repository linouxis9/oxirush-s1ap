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
    assert_eq!(tree.pointer(path), Some(&serde_json::json!(1)));
    *tree.pointer_mut(path).unwrap() = serde_json::json!(3);
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
fn the_octets_of_an_ie_are_replaced_or_added_as_its_raw_value_without_value() {
    let other = inspect::inspect_pdu(&release_request(9)).unwrap();
    let mut tree = inspect::inspect_pdu(&release_request(7)).unwrap();
    let ie = tree.pointer_mut("/message/protocolIEs/1").unwrap();
    ie.as_object_mut().unwrap().remove("value");
    ie["_raw_value"] = other["message"]["protocolIEs"][1]["_raw_value"].clone();
    let ies = tree.pointer_mut("/message/protocolIEs").unwrap();
    ies.as_array_mut()
        .unwrap()
        .push(serde_json::json!({"id": 60000, "criticality": "ignore", "_raw_value": "C0FFEE"}));
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
fn a_new_ie_takes_a_typed_value_of_the_type_of_its_identifier() {
    let with_cause = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    );
    let added = |id: u16, value: serde_json::Value| {
        let mut tree = inspect::inspect_pdu(&release_request(7)).unwrap();
        let ies = tree.pointer_mut("/message/protocolIEs").unwrap();
        ies.as_array_mut()
            .unwrap()
            .push(serde_json::json!({"id": id, "criticality": "ignore", "value": value}));
        inspect::encode_pdu(&tree)
    };
    let cause = serde_json::json!({"radioNetwork": "user-inactivity"});
    assert_eq!(
        added(2, cause.clone()).unwrap().encode().unwrap(),
        with_cause.encode().unwrap()
    );
    // A value that the type does not have, and an identifier without a type.
    assert!(added(2, serde_json::json!({"radioNetwork": "no-such-cause"})).is_err());
    assert!(added(60000, cause).unwrap_err().contains("_raw_value"));
}

#[test]
fn a_new_ie_whose_value_is_a_string_is_typed_too() {
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
    );
    let typed = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        IGNORE RRC_Establishment_Cause(RRCEstablishmentCause::mo_Signalling),
        REJECT NAS_PDU(vec![0x07, 0x6a]),
    );
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    let ies = tree.pointer_mut("/message/protocolIEs").unwrap();
    let ies = ies.as_array_mut().unwrap();
    // An ENUMERATED by its name and an OCTET STRING by its octets, not the
    // octets of their open types.
    ies.push(serde_json::json!({"id": 134, "criticality": "ignore", "value": "mo-Signalling"}));
    ies.push(serde_json::json!({"id": 26, "criticality": "reject", "value": "076A"}));
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        typed.encode().unwrap()
    );
    // Hexadecimal in lower case is the same octets.
    *tree.pointer_mut("/message/protocolIEs/2/value").unwrap() = serde_json::json!("076a");
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        typed.encode().unwrap()
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

/// The octets of the message `name` of the fixtures.
fn fixture(name: &str) -> Vec<u8> {
    let mut lines = include_str!("fixtures/messages.tsv").lines();
    let line = lines.find(|line| line.split('\t').next() == Some(name));
    hex::decode(line.unwrap().split('\t').nth(1).unwrap()).unwrap()
}

/// The entry of the IE `id` of a tree.
fn ie(tree: &mut serde_json::Value, id: u16) -> &mut serde_json::Value {
    let ies = tree["message"]["protocolIEs"].as_array_mut().unwrap();
    ies.iter_mut().find(|ie| ie["id"] == id).unwrap()
}

/// The tree of a message that has `entries` for IEs, encoded and decoded again.
fn sent(entries: &[serde_json::Value]) -> Result<serde_json::Value, String> {
    let pdu = build_s1ap!(
        InitiatingMessage,
        UEContextReleaseRequest,
        IGNORE,
        UEContextReleaseRequest,
    );
    let mut tree = inspect::inspect_pdu(&pdu)?;
    *tree["message"]["protocolIEs"].as_array_mut().unwrap() = entries.to_vec();
    inspect::inspect_pdu(&inspect::encode_pdu(&tree)?)
}

#[test]
fn well_known_values_are_shown_and_taken_as_they_are_usually_written() {
    use serde_json::json;
    // S1 SETUP REQUEST: Global-ENB-ID is IE 59 and SupportedTAs IE 64.
    let wire = fixture("S1SetupRequest");
    let mut tree = inspect::inspect_pdu(&S1AP_PDU::decode(&wire).unwrap()).unwrap();
    let area = &ie(&mut tree, 64)["value"][0];
    assert_eq!(area["tAC"], json!(0x9302));
    assert_eq!(area["broadcastPLMNs"], json!(["208-93"]));
    assert_eq!(ie(&mut tree, 59)["value"]["pLMNidentity"], json!("208-93"));
    // What is not edited keeps its octets.
    assert_eq!(inspect::encode_pdu(&tree).unwrap().encode().unwrap(), wire);

    // Edited as they are shown, a number in hexadecimal too.
    let area = &mut ie(&mut tree, 64)["value"][0];
    area["tAC"] = json!("0x7");
    area["broadcastPLMNs"] = json!(["001-01", "310-410"]);
    let edited = inspect::encode_pdu(&tree).unwrap();
    let mut shown = inspect::inspect_pdu(&edited).unwrap();
    let area = &ie(&mut shown, 64)["value"][0];
    assert_eq!(area["tAC"], json!(7));
    assert_eq!(area["broadcastPLMNs"], json!(["001-01", "310-410"]));
    assert_eq!(
        ie(&mut shown, 59)["_raw_value"],
        ie(&mut tree, 59)["_raw_value"]
    );

    // As JER writes them, which is what a tree had before.
    let area = &mut ie(&mut tree, 64)["value"][0];
    area["tAC"] = json!("0007");
    area["broadcastPLMNs"] = json!(["00F110", "130014"]);
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        edited.encode().unwrap()
    );

    // Neither form, and a number that the type has no room for.
    for refused in [json!("7"), json!(1 << 16), json!("0x10000")] {
        ie(&mut tree, 64)["value"][0]["tAC"] = refused.clone();
        assert!(inspect::encode_pdu(&tree).is_err(), "{refused}");
    }
    ie(&mut tree, 64)["value"][0]["tAC"] = json!(7);
    ie(&mut tree, 64)["value"][0]["broadcastPLMNs"] = json!(["208-9"]);
    assert!(inspect::encode_pdu(&tree).is_err());
}

#[test]
fn the_parts_of_a_gummei_are_numbers() {
    use serde_json::json;
    // S1 SETUP RESPONSE: ServedGUMMEIs is IE 105.
    let wire = fixture("S1SetupResponse");
    let mut tree = inspect::inspect_pdu(&S1AP_PDU::decode(&wire).unwrap()).unwrap();
    let served = &mut ie(&mut tree, 105)["value"][0];
    assert_eq!(served["servedPLMNs"], json!(["208-93"]));
    assert_eq!(served["servedGroupIDs"], json!([0xB8A3]));
    assert_eq!(served["servedMMECs"], json!([0x89]));
    assert_eq!(inspect::encode_pdu(&tree).unwrap().encode().unwrap(), wire);
    let served = &mut ie(&mut tree, 105)["value"][0];
    served["servedGroupIDs"] = json!([2, "0x8000"]);
    served["servedMMECs"] = json!([1]);
    let edited = inspect::encode_pdu(&tree).unwrap();
    let mut shown = inspect::inspect_pdu(&edited).unwrap();
    let served = &ie(&mut shown, 105)["value"][0];
    assert_eq!(served["servedGroupIDs"], json!([2, 0x8000]));
    assert_eq!(served["servedMMECs"], json!([1]));
    // An MME code has one octet.
    ie(&mut tree, 105)["value"][0]["servedMMECs"] = json!([256]);
    assert!(inspect::encode_pdu(&tree).is_err());
}

#[test]
fn a_tunnel_endpoint_and_the_identities_of_a_ue_are_readable() {
    use oxirush_s1ap::helpers::bytes_to_bitvec;
    use serde_json::json;
    let added = |id: u16, value: serde_json::Value| {
        sent(&[json!({"id": id, "criticality": "ignore", "value": value})])
    };
    let octets = |value: &[u8]| json!(hex::encode_upper(value));
    // E-RABSetupListCtxtSURes is IE 51, a list of IEs 50.
    let bearer = json!({"e-RAB-ID": 5, "transportLayerAddress": "10.0.0.2", "gTP-TEID": 2});
    let bearers = json!([{"id": 50, "criticality": "ignore", "value": bearer}]);
    let mut shown = added(51, bearers).unwrap();
    let item = &mut ie(&mut shown, 51)["value"][0];
    assert_eq!(item["value"], bearer);
    let expected = ERABSetupItemCtxtSURes::new(
        ERABID(5u8.into()),
        TransportLayerAddress(bytes_to_bitvec(&[10, 0, 0, 2])),
        GTPTEID::from([0, 0, 0, 2]),
        None,
    );
    assert_eq!(
        item["_raw_value"],
        octets(encode_open_type(&expected).unwrap().as_bytes())
    );

    // UEPagingID is IE 43: an IMSI by its digits, or an S-TMSI by its numbers.
    let mut shown = added(43, json!({"iMSI": "208930000000001"})).unwrap();
    let imsi = IMSI(vec![0x02, 0x98, 0x03, 0x00, 0x00, 0x00, 0x00, 0xF1].into());
    let expected = encode_open_type(&UEPagingID::iMSI(imsi)).unwrap();
    assert_eq!(
        ie(&mut shown, 43)["value"]["iMSI"],
        json!("208930000000001")
    );
    assert_eq!(
        ie(&mut shown, 43)["_raw_value"],
        octets(expected.as_bytes())
    );
    // As JER writes it, with the filler that says it is not digits.
    let mut same = added(43, json!({"iMSI": "02980300000000F1"})).unwrap();
    assert_eq!(
        ie(&mut same, 43)["_raw_value"],
        ie(&mut shown, 43)["_raw_value"]
    );
    let temporary = json!({"s-TMSI": {"mMEC": 1, "m-TMSI": "0xC0FFEE01"}});
    let mut shown = added(43, temporary).unwrap();
    let identity = &ie(&mut shown, 43)["value"]["s-TMSI"];
    assert_eq!(identity, &json!({"mMEC": 1, "m-TMSI": 0xC0FFEE01u32}));
}

#[test]
fn an_enumerated_value_is_named_whatever_its_case_and_separators() {
    use serde_json::json;
    let causes = |cause: &str, establishment: &str| {
        sent(&[
            json!({"id": 2, "criticality": "ignore", "value": {"radioNetwork": cause}}),
            json!({"id": 134, "criticality": "ignore", "value": establishment}),
        ])
    };
    let mut exact = causes("user-inactivity", "mo-Signalling").unwrap();
    let mut loose = causes("User_Inactivity", "MO SIGNALLING").unwrap();
    for id in [2, 134] {
        assert_eq!(ie(&mut loose, id)["value"], ie(&mut exact, id)["value"]);
        assert_eq!(
            ie(&mut loose, id)["_raw_value"],
            ie(&mut exact, id)["_raw_value"]
        );
    }
    // A name of another type, and no name at all.
    assert!(causes("mo-Signalling", "mo-Signalling").is_err());
    assert!(causes("user-inactivit", "mo-Signalling").is_err());
}
