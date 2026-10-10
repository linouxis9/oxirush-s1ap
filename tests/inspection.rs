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

/// A UE CONTEXT RELEASE REQUEST: two UE S1AP IDs and a cause.
fn release_with_cause() -> S1AP_PDU {
    build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )
}

#[test]
fn the_ies_of_a_message_have_paths_by_their_names() {
    use serde_json::json;
    let tree = inspect::inspect_pdu(&release_with_cause()).unwrap();
    let paths = inspect::paths(&tree);
    let printed: Vec<_> = (paths.iter())
        .map(|(path, value)| format!("{path} = {value}"))
        .collect();
    assert_eq!(
        printed,
        [
            "/criticality = \"ignore\"",
            "/direction = \"InitiatingMessage\"",
            "/s1ap/MME-UE-S1AP-ID/criticality = \"reject\"",
            "/s1ap/MME-UE-S1AP-ID/id = 0",
            "/s1ap/MME-UE-S1AP-ID/value = 1",
            "/s1ap/eNB-UE-S1AP-ID/criticality = \"reject\"",
            "/s1ap/eNB-UE-S1AP-ID/id = 8",
            "/s1ap/eNB-UE-S1AP-ID/value = 7",
            "/s1ap/Cause/criticality = \"ignore\"",
            "/s1ap/Cause/id = 2",
            "/s1ap/Cause/value/radioNetwork = \"user-inactivity\"",
            "/procedure_code = 18",
        ]
    );
    // A name is taken in any case, and an IE also by its identifier or its position.
    for path in [
        "/s1ap/eNB-UE-S1AP-ID/value",
        "/s1ap/enb_ue_s1ap_id/value",
        "/s1ap/@id=8/value",
        "/s1ap/1/value",
    ] {
        assert_eq!(inspect::select(&tree, path).unwrap(), [&json!(7)], "{path}");
    }
    // The octets that an IE was received as.
    let octets = inspect::select(&tree, "/s1ap/eNB-UE-S1AP-ID/octets").unwrap();
    assert_eq!(octets, [&json!("0007")]);
    assert_eq!(inspect::select(&tree, "/s1ap/*/id").unwrap().len(), 3);
}

#[test]
fn every_path_of_every_canonical_procedure_selects_its_value() {
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = S1AP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        let tree = inspect::inspect_pdu(&pdu).unwrap();
        for (path, value) in inspect::paths(&tree) {
            // The list that holds the IEs of a message is never named.
            assert!(!path.contains("protocolIEs"), "{} {path}", fields[0]);
            let selected = inspect::select(&tree, &path);
            assert_eq!(selected, Ok(vec![&value]), "{} {path}", fields[0]);
        }
    }
}

#[test]
fn an_ie_that_is_there_twice_goes_by_its_position() {
    use serde_json::json;
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT MME_UE_S1AP_ID(2u32),
    );
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    let paths = inspect::paths(&tree);
    let values: Vec<_> = (paths.iter())
        .filter(|(path, _)| path.ends_with("/value"))
        .map(|(path, value)| (path.as_str(), value))
        .collect();
    assert_eq!(
        values,
        [
            ("/s1ap/0/value", &json!(1)),
            ("/s1ap/eNB-UE-S1AP-ID/value", &json!(7)),
            ("/s1ap/2/value", &json!(2)),
        ]
    );
    // The name selects each of them, in the order of the message.
    let both = inspect::select(&tree, "/s1ap/MME-UE-S1AP-ID/value").unwrap();
    assert_eq!(both, [&json!(1), &json!(2)]);
    inspect::set(&mut tree, "/s1ap/2/value", json!(3)).unwrap();
    let edited = inspect::encode_pdu(&tree).unwrap();
    let tree = inspect::inspect_pdu(&edited).unwrap();
    let both = inspect::select(&tree, "/s1ap/MME-UE-S1AP-ID/value").unwrap();
    assert_eq!(both, [&json!(1), &json!(3)]);
}

#[test]
fn a_value_set_at_a_path_changes_the_octets_of_its_ie_alone() {
    use serde_json::json;
    let mut tree = inspect::inspect_pdu(&release_with_cause()).unwrap();
    let octets = |tree: &serde_json::Value| -> Vec<serde_json::Value> {
        let octets = inspect::select(tree, "/s1ap/*/octets").unwrap();
        octets.into_iter().cloned().collect()
    };
    let before = octets(&tree);
    inspect::set(&mut tree, "/s1ap/eNB-UE-S1AP-ID/value", json!(9)).unwrap();
    let cause = json!("Release Due To EUTRAN Generated Reason");
    inspect::set(&mut tree, "/s1ap/Cause/value/radioNetwork", cause).unwrap();
    let tree = inspect::inspect_pdu(&inspect::encode_pdu(&tree).unwrap()).unwrap();
    let after = octets(&tree);
    assert_eq!(after[0], before[0]);
    assert_eq!(after[1], json!("0009"));
    assert_ne!(after[2], before[2]);
    let cause = inspect::select(&tree, "/s1ap/Cause/value/radioNetwork").unwrap();
    assert_eq!(cause, [&json!("release-due-to-eutran-generated-reason")]);
}

#[test]
fn ies_are_added_taken_out_and_given_their_octets_at_a_path() {
    use serde_json::json;
    let mut tree = inspect::inspect_pdu(&release_with_cause()).unwrap();
    // An IE by the name of its identifier, with its typed value, at the end.
    let name = json!({"id": "eNBname", "criticality": "ignore", "value": "enb-1"});
    inspect::insert(&mut tree, "/s1ap/-", name).unwrap();
    // Given octets as an IE, before the cause.
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "C0FFEE"});
    inspect::insert(&mut tree, "/s1ap/Cause", unknown).unwrap();
    inspect::remove(&mut tree, "/s1ap/MME-UE-S1AP-ID").unwrap();
    inspect::set(&mut tree, "/s1ap/eNB-UE-S1AP-ID/octets", json!("400102")).unwrap();
    let tree = inspect::inspect_pdu(&inspect::encode_pdu(&tree).unwrap()).unwrap();
    let ids: Vec<_> = (inspect::select(&tree, "/s1ap/*/id").unwrap().into_iter())
        .map(|id| id.as_u64().unwrap())
        .collect();
    assert_eq!(ids, [8, 60000, 2, 60]);
    let select = |path: &str| inspect::select(&tree, path).unwrap();
    assert_eq!(select("/s1ap/eNB-UE-S1AP-ID/value"), [&json!(258)]);
    assert_eq!(select("/s1ap/eNBname/value"), [&json!("enb-1")]);
    assert_eq!(select("/s1ap/@id=60000/octets"), [&json!("C0FFEE")]);
    // An IE that the message does not have is nothing, not an error.
    assert!(select("/s1ap/MME-UE-S1AP-ID/value").is_empty());
}

#[test]
fn the_entries_of_a_list_of_ies_go_by_name_too() {
    use serde_json::json;
    let pdu = S1AP_PDU::decode(&fixture("E-RABSetupRequest")).unwrap();
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    let list = "/s1ap/E-RABToBeSetupListBearerSUReq/value";
    let by_name = format!("{list}/E-RABToBeSetupItemBearerSUReq/value/e-RAB-ID");
    let by_position = format!("{list}/0/value/e-RAB-ID");
    assert_eq!(inspect::select(&tree, &by_name).unwrap(), [&json!(0)]);
    inspect::set(&mut tree, &by_name, json!(5)).unwrap();
    let tree = inspect::inspect_pdu(&inspect::encode_pdu(&tree).unwrap()).unwrap();
    assert_eq!(inspect::select(&tree, &by_position).unwrap(), [&json!(5)]);
    // The IEs of a message that a tree contains are selected in its place: no S1AP
    // type has one, so this is a tree of that shape.
    let mut contained = json!({"message": {"protocolIEs": [{"id": 2, "value": {"decoded":
        {"protocolIEs": [{"id": 8, "value": 7}, {"id": 60000, "value": "00"}]}}}]}});
    let paths: Vec<_> = (inspect::paths(&contained).into_iter())
        .map(|(path, _)| path)
        .collect();
    assert_eq!(
        paths,
        [
            "/s1ap/Cause/id",
            "/s1ap/Cause/value/decoded/eNB-UE-S1AP-ID/id",
            "/s1ap/Cause/value/decoded/eNB-UE-S1AP-ID/value",
            "/s1ap/Cause/value/decoded/1/id",
            "/s1ap/Cause/value/decoded/1/value",
        ]
    );
    let inner = "/s1ap/Cause/value/decoded";
    for path in [
        "/eNB-UE-S1AP-ID",
        "/0",
        "/@id=8",
        "/protocolIEs/eNB-UE-S1AP-ID",
    ] {
        let value = inspect::select(&contained, &format!("{inner}{path}/value"));
        assert_eq!(value, Ok(vec![&json!(7)]), "{path}");
    }
    let all = inspect::select(&contained, &format!("{inner}/*/id")).unwrap();
    assert_eq!(all, [&json!(8), &json!(60000)]);
    inspect::remove(&mut contained, &format!("{inner}/eNB-UE-S1AP-ID")).unwrap();
    let all = inspect::select(&contained, &format!("{inner}/*/id")).unwrap();
    assert_eq!(all, [&json!(60000)]);
}

#[test]
fn a_path_that_names_nothing_is_refused_and_changes_nothing() {
    use serde_json::json;
    let mut tree = inspect::inspect_pdu(&release_with_cause()).unwrap();
    let before = tree.clone();
    for (path, reason) in [
        ("/s1ap/NoSuchIE/value", "\"NoSuchIE\" is not an IE of S1AP"),
        (
            "/s1ap/Cause/value/misspelled",
            "\"misspelled\" is not a member of Cause, which has radioNetwork, transport, nas, \
             protocol, misc",
        ),
        (
            "/s1ap/Cause/value/radioNetwork/deeper",
            "\"deeper\" is not a member of this value, which has none",
        ),
        (
            "/misspelled",
            "\"misspelled\" is not a member of a tree, which has procedure_code, direction, \
             criticality, message",
        ),
        ("/s1ap/+0/id", "\"+0\" is not an IE of S1AP"),
        ("/s1ap/00/id", "\"00\" is not an IE of S1AP"),
        ("s1ap/Cause", "must start with /"),
        ("/s1ap/@id=70000", "IE id must be u16"),
    ] {
        let error = inspect::select(&tree, path).unwrap_err();
        assert!(error.contains(reason), "{path}: {error}");
    }
    for (path, reason) in [
        ("/s1ap/NoSuchIE/value", "is not an IE of S1AP"),
        ("/s1ap/GUMMEI-ID/value", "selected no field"),
        ("/s1ap/7/value", "selected no field"),
    ] {
        let error = inspect::set(&mut tree, path, json!(1)).unwrap_err();
        assert!(error.contains(reason), "{path}: {error}");
    }
    assert!(inspect::remove(&mut tree, "/s1ap/GUMMEI-ID").is_err());
    let both = json!({"id": "Cause", "criticality": "ignore", "value": 1, "octets": "00"});
    assert!(inspect::insert(&mut tree, "/s1ap/-", both).is_err());
    let unnamed = json!({"id": "NoSuchIE", "criticality": "ignore", "octets": "00"});
    assert!(inspect::insert(&mut tree, "/s1ap/-", unnamed).is_err());
    assert_eq!(tree, before);
}

#[test]
fn a_message_goes_by_the_name_that_asn1_gives_it() {
    use serde_json::json;
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = S1AP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        assert_eq!(inspect::message_name(&pdu), Some(fields[0]));
    }
    // A name finds its message again, however it is written, and no other does.
    for (direction, code, name) in inspect::message_names() {
        let found = Some((direction, code));
        assert_eq!(inspect::message_named(name), found, "{name}");
        assert_eq!(inspect::message_named(&name.to_lowercase()), found);
        assert_eq!(inspect::message_named(&name.replace('-', "")), found);
    }
    assert_eq!(inspect::message_named("NoSuchMessage"), None);
    // The tree of a message that is written from its name.
    let (direction, procedure_code) = inspect::message_named("ue context release request").unwrap();
    let mut tree = json!({
        "direction": direction,
        "procedure_code": procedure_code,
        "criticality": "ignore",
        "message": {"protocolIEs": []},
    });
    inspect::insert(
        &mut tree,
        "/s1ap/-",
        json!({"id": "eNB-UE-S1AP-ID", "criticality": "reject", "value": 7}),
    )
    .unwrap();
    let pdu = inspect::encode_pdu(&tree).unwrap();
    assert_eq!(inspect::message_name(&pdu), Some("UEContextReleaseRequest"));
    assert!(pdu.is_initiating());
}

#[test]
fn an_edit_changes_what_is_sent_or_is_refused() {
    use serde_json::{Value, json};
    let mut tree = inspect::inspect_pdu(&release_request(7)).unwrap();
    let before = tree.clone();
    // The value of an IE is not taken out: its octets would go out as they came.
    for edit in [
        inspect::remove(&mut tree, "/s1ap/eNB-UE-S1AP-ID/value"),
        inspect::set(&mut tree, "/s1ap/eNB-UE-S1AP-ID/value", Value::Null),
    ] {
        let error = edit.unwrap_err();
        assert!(error.contains("value is not taken out"), "{error}");
    }
    assert_eq!(tree, before);
    // `*` is each member of a value, without what the tree keeps of what came.
    let members = inspect::select(&tree, "/s1ap/eNB-UE-S1AP-ID/*").unwrap();
    assert_eq!(members.len(), 3, "{members:?}");
    // The octets of an IE whose value was edited are no longer its octets.
    let octets = "/s1ap/eNB-UE-S1AP-ID/octets";
    let received = inspect::select(&tree, octets).unwrap()[0].clone();
    inspect::set(&mut tree, "/s1ap/eNB-UE-S1AP-ID/value", json!(9)).unwrap();
    let error = inspect::select(&tree, octets).unwrap_err();
    assert!(error.contains("the value was edited"), "{error}");
    assert_eq!(
        inspect::select(&tree, "/s1ap/MME-UE-S1AP-ID/octets")
            .unwrap()
            .len(),
        1
    );
    assert!(
        inspect::paths(&tree)
            .iter()
            .all(|(path, _)| !path.contains("_edited"))
    );
    let sent = inspect::inspect_pdu(&inspect::encode_pdu(&tree).unwrap()).unwrap();
    assert_ne!(inspect::select(&sent, octets).unwrap(), [&received]);
    assert_eq!(
        inspect::select(&sent, "/s1ap/eNB-UE-S1AP-ID/value").unwrap(),
        [&json!(9)]
    );
    // Octets that are set are those of the IE again.
    inspect::set(&mut tree, octets, received.clone()).unwrap();
    assert_eq!(inspect::select(&tree, octets).unwrap(), [&received]);
    assert_eq!(inspect::encode_pdu(&tree).unwrap(), release_request(7));
    // The name of an ENUMERATED value of the PDU itself is taken as any other.
    inspect::set(&mut tree, "/criticality", json!("Reject")).unwrap();
    let pdu = inspect::encode_pdu(&tree).unwrap();
    assert_eq!(inspect::inspect_pdu(&pdu).unwrap()["criticality"], "reject");
}

#[test]
fn null_takes_an_optional_member_out() {
    use serde_json::{Value, json};
    let pdu = S1AP_PDU::decode(&fixture("InitialContextSetupRequest")).unwrap();
    let tree = inspect::inspect_pdu(&pdu).unwrap();
    let sent = |tree: &Value| inspect::inspect_pdu(&inspect::encode_pdu(tree)?);
    // An optional member that is there is taken out, and one that is not is not
    // selected.
    let nas = "/s1ap/E-RABToBeSetupListCtxtSUReq/value/0/value/nAS-PDU";
    let mut edited = tree.clone();
    inspect::set(&mut edited, nas, json!("0102")).unwrap();
    let mut edited = sent(&edited).unwrap();
    assert_eq!(inspect::select(&edited, nas).unwrap(), [&json!("0102")]);
    inspect::set(&mut edited, nas, Value::Null).unwrap();
    let none = Vec::<&Value>::new();
    assert_eq!(inspect::select(&sent(&edited).unwrap(), nas).unwrap(), none);
    let error = inspect::set(&mut edited, nas, Value::Null).unwrap_err();
    assert!(error.contains("selected no field"), "{error}");
}

#[test]
fn a_message_has_the_ies_of_its_object_set() {
    let id = |name: &str| {
        let mut names = inspect::ie_names().iter();
        names.find(|(_, known)| *known == name).unwrap().0
    };
    let ies = inspect::message_ies("InitialContextSetupRequest").unwrap();
    assert_eq!(ies[0], (id("MME-UE-S1AP-ID"), true));
    assert!(ies.contains(&(id("E-RABToBeSetupListCtxtSUReq"), true)));
    assert!(ies.contains(&(id("TraceActivation"), false)));
    let listed = |name: &str| ies.iter().any(|(ie, _)| *ie == id(name));
    assert!(!listed("E-RABToBeSetupListBearerSUReq"));
    // A name is taken as that of a message is, and a message without IEs has none.
    assert_eq!(
        inspect::message_ies("initial-context-setup-request"),
        Some(ies)
    );
    assert_eq!(inspect::message_ies("PrivateMessage"), Some(&[][..]));
    assert_eq!(inspect::message_ies("NoSuchMessage"), None);
    // Every message has its IEs in the order of its set, each of them known.
    for (.., name) in inspect::message_names() {
        let ies = inspect::message_ies(name).unwrap();
        assert!(name == "PrivateMessage" || !ies.is_empty(), "{name}");
        for (ie, _) in ies {
            assert!(
                inspect::ie_names().iter().any(|(known, _)| known == ie),
                "{name} {ie}"
            );
        }
    }
}

#[test]
fn the_identifiers_of_the_registry_are_the_constants_of_the_bindings() {
    use std::collections::BTreeMap;
    let letters = |name: &str| -> String {
        let letters = name.chars().filter(char::is_ascii_alphanumeric);
        letters.flat_map(char::to_lowercase).collect()
    };
    // `pub const ID_<NAME>: <Type> = <Type>(<number>);`, as the compiler writes them.
    let mut constants: BTreeMap<(String, String), u64> = BTreeMap::new();
    for constant in include_str!("../src/s1ap.rs")
        .split("pub const ID_")
        .skip(1)
    {
        let constant: String = constant
            .split(';')
            .next()
            .unwrap()
            .split_whitespace()
            .collect();
        let (name, value) = constant.split_once(':').unwrap();
        let (kind, number) = value.split_once('=').unwrap();
        let number = number.trim_start_matches(kind).trim_matches(['(', ')']);
        constants.insert((kind.into(), letters(name)), number.parse().unwrap());
    }
    for (id, name) in inspect::ie_names() {
        let constant = constants.get(&("ProtocolIEID".into(), letters(name)));
        assert_eq!(constant, Some(&u64::from(*id)), "{name}");
    }
    // The procedures of the list: `<code> <Name> {`.
    let mut procedures = 0;
    for line in include_str!("../src/registry.rs").lines() {
        let mut words = line.split_whitespace();
        let (Some(code), Some(name), Some("{")) = (words.next(), words.next(), words.next()) else {
            continue;
        };
        let constant = constants.get(&("ProcedureCode".into(), letters(name)));
        assert_eq!(constant, Some(&code.parse().unwrap()), "{name}");
        procedures += 1;
    }
    assert!(procedures > 50, "{procedures}");
}

#[test]
fn a_kind_that_the_specification_does_not_have_says_the_direction_of_its_pdu() {
    let pdu = S1AP_PDU::unsuccessfulOutcome(UnsuccessfulOutcome::new(
        ProcedureCode(13),
        Criticality::ignore,
        rasn::types::Any::new(vec![0]),
    ));
    let oxirush_s1ap::S1apPduKind::Other {
        direction,
        procedure_code,
    } = pdu.kind()
    else {
        panic!("{:?}", pdu.kind());
    };
    assert_eq!((direction, procedure_code), (pdu.direction(), 13));
}

#[test]
fn a_member_that_is_absent_selects_nothing_and_one_that_its_type_has_not_is_an_error() {
    use serde_json::{Value, json};
    let tree = inspect::inspect_pdu(&release_with_cause()).unwrap();
    let none = Vec::<&Value>::new();
    let select = |path: &str| inspect::select(&tree, path);
    // The alternative of a CHOICE that is there, and another one.
    let cause = json!("user-inactivity");
    assert_eq!(select("/s1ap/Cause/value/radioNetwork").unwrap(), [&cause]);
    assert_eq!(select("/s1ap/Cause/value/nas").unwrap(), none);
    // What follows an IE or a member that is not there is still a path of its type.
    assert_eq!(select("/s1ap/GUMMEI-ID/value/mME-Code").unwrap(), none);
    let error = select("/s1ap/GUMMEI-ID/value/misspelled").unwrap_err();
    let members = "is not a member of GUMMEI, which has pLMN-Identity, mME-Group-ID, mME-Code, \
                   iE-Extensions";
    assert!(error.contains(members), "{error}");
    let error = select("/s1ap/Cause/value/nas/deeper").unwrap_err();
    assert!(error.contains("which has none"), "{error}");
    // After `*`, a member is an error when no value can have it.
    assert_eq!(select("/s1ap/*/value/radioNetwork").unwrap(), [&cause]);
    assert_eq!(select("/s1ap/*/value/*").unwrap(), [&cause]);
    let error = select("/s1ap/*/value/misspelled").unwrap_err();
    assert!(error.contains("is not a member of"), "{error}");
    // An optional member that is not there, and a list of values that has no IEs to name.
    let pdu = S1AP_PDU::decode(&fixture("InitialContextSetupRequest")).unwrap();
    let setup = inspect::inspect_pdu(&pdu).unwrap();
    let item = "/s1ap/E-RABToBeSetupListCtxtSUReq/value/E-RABToBeSetupItemCtxtSUReq/value";
    let nas = inspect::select(&setup, &format!("{item}/nAS-PDU")).unwrap();
    assert_eq!(nas, none);
    let error = inspect::select(&setup, &format!("{item}/nas-pdu-typo")).unwrap_err();
    assert!(
        error.contains("is not a member of ERABToBeSetupItemCtxtSUReq, which has e-RAB-ID"),
        "{error}"
    );
    let pdu = S1AP_PDU::decode(&fixture("S1SetupRequest")).unwrap();
    let setup = inspect::inspect_pdu(&pdu).unwrap();
    let error = inspect::select(&setup, "/s1ap/SupportedTAs/value/Cause/value").unwrap_err();
    assert!(error.contains("go by position"), "{error}");
}

#[test]
fn an_edit_names_a_member_that_the_type_has_and_keeps_what_it_always_has() {
    use serde_json::{Value, json};
    let mut tree = inspect::inspect_pdu(&release_with_cause()).unwrap();
    let before = tree.clone();
    let error = inspect::set(&mut tree, "/s1ap/Cause/value/misspelled", json!(1)).unwrap_err();
    assert!(error.contains("is not a member of Cause"), "{error}");
    let error = inspect::remove(&mut tree, "/s1ap/Cause/value/radioNetwork").unwrap_err();
    assert!(error.contains("a CHOICE has one alternative"), "{error}");
    let error = inspect::remove(&mut tree, "/s1ap/Cause/criticality").unwrap_err();
    assert!(error.contains("its type always has it"), "{error}");
    assert_eq!(tree, before);
    // An alternative takes the place of the one that is there.
    inspect::set(&mut tree, "/s1ap/Cause/value/nas", json!("detach")).unwrap();
    let sent = inspect::inspect_pdu(&inspect::encode_pdu(&tree).unwrap()).unwrap();
    let cause = inspect::select(&sent, "/s1ap/Cause/value").unwrap();
    assert_eq!(cause, [&json!({"nas": "detach"})]);
    // A member that the type always has stays, and an optional one is added by its name.
    let pdu = S1AP_PDU::decode(&fixture("InitialContextSetupRequest")).unwrap();
    let mut setup = inspect::inspect_pdu(&pdu).unwrap();
    let item = "/s1ap/E-RABToBeSetupListCtxtSUReq/value/0/value";
    let error = inspect::remove(&mut setup, &format!("{item}/e-RAB-ID")).unwrap_err();
    assert!(error.contains("its type always has it"), "{error}");
    let error = inspect::set(&mut setup, &format!("{item}/e-RAB-ID"), Value::Null).unwrap_err();
    assert!(error.contains("its type always has it"), "{error}");
    inspect::set(&mut setup, &format!("{item}/nas_pdu"), json!("0102")).unwrap();
    let sent = inspect::inspect_pdu(&inspect::encode_pdu(&setup).unwrap()).unwrap();
    let nas = inspect::select(&sent, &format!("{item}/nAS-PDU")).unwrap();
    assert_eq!(nas, [&json!("0102")]);
}

#[test]
fn a_name_is_its_letters_and_its_digits() {
    use serde_json::json;
    let tree = inspect::inspect_pdu(&release_with_cause()).unwrap();
    for path in [
        "/s1ap/eNB-UE-S1AP-ID/value",
        "/S1AP/enb_ue_s1ap_id/Value",
        "/S1ap/enbues1apid/value",
        "/s1ap/ENB UE S1AP ID/value",
        "/message/protocol-ies/1/value",
        "/Message/ProtocolIEs/1/VALUE",
    ] {
        assert_eq!(inspect::select(&tree, path).unwrap(), [&json!(7)], "{path}");
    }
    for path in [
        "/s1ap/cause/value/radionetwork",
        "/s1ap/Cause/value/Radio-Network",
    ] {
        let cause = inspect::select(&tree, path).unwrap();
        assert_eq!(cause, [&json!("user-inactivity")], "{path}");
    }
    assert_eq!(
        inspect::select(&tree, "/Procedure-Code").unwrap(),
        inspect::select(&tree, "/procedure_code").unwrap()
    );
    // A member that a tree keeps of what was received is named as it is.
    assert!(inspect::select(&tree, "/s1ap/Cause/_raw_value").is_ok());
    assert!(inspect::select(&tree, "/s1ap/Cause/rawvalue").is_err());
    // An edit writes the member by the name that ASN.1 gives it.
    let mut edited = tree.clone();
    inspect::set(&mut edited, "/s1ap/cause/value/NAS", json!("detach")).unwrap();
    let cause = inspect::select(&edited, "/s1ap/Cause/value").unwrap();
    assert_eq!(cause, [&json!({"nas": "detach"})]);
    assert!(inspect::encode_pdu(&edited).is_ok());
    assert_eq!(
        inspect::message_named("ue context-release_REQUEST"),
        Some(("InitiatingMessage", 18))
    );
}

#[test]
fn a_path_is_checked_against_a_message_without_a_tree() {
    // Every path of every message is one that its message can have.
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let wire = hex::decode(line.split('\t').nth(1).unwrap()).unwrap();
        let pdu = S1AP_PDU::decode(&wire).unwrap();
        let name = inspect::message_name(&pdu).unwrap();
        let tree = inspect::inspect_pdu(&pdu).unwrap();
        for (path, _) in inspect::paths(&tree) {
            let checked = inspect::check_path(name, &path);
            assert!(checked.is_ok(), "{name} {path}: {checked:?}");
        }
    }
    let item = "/s1ap/E-RABToBeSetupListCtxtSUReq/value/E-RABToBeSetupItemCtxtSUReq";
    for (message, path) in [
        ("UEContextReleaseRequest", "/s1ap/Cause/value/nas"),
        (
            "ue-context-release-request",
            "/s1ap/cause/value/radio_network",
        ),
        ("UEContextReleaseRequest", "/s1ap/Cause/octets"),
        ("UEContextReleaseRequest", "/s1ap/*/value/radioNetwork"),
        ("UEContextReleaseRequest", "/s1ap/0/criticality"),
        ("UEContextReleaseRequest", "/s1ap/-"),
        ("UEContextReleaseRequest", "/s1ap"),
        ("UEContextReleaseRequest", "/procedure_code"),
        ("UEContextReleaseRequest", "/message/protocolIEs/0/value"),
        // An identifier that has no type may be anything.
        ("UEContextReleaseRequest", "/s1ap/@id=60000/value/anything"),
        (
            "InitialContextSetupRequest",
            "/s1ap/GUMMEI-ID/value/mME-Code",
        ),
        (
            "InitialContextSetupRequest",
            &format!("{item}/value/nAS-PDU"),
        ),
        (
            "InitialContextSetupRequest",
            "/s1ap/E-RABToBeSetupListCtxtSUReq/value/0/value/e-RABlevelQoSParameters/qCI",
        ),
        (
            "InitialContextSetupRequest",
            &format!("{item}/value/transportLayerAddress/length"),
        ),
    ] {
        let checked = inspect::check_path(message, path);
        assert!(checked.is_ok(), "{message} {path}: {checked:?}");
    }
    for (message, path, reason) in [
        ("NoSuchMessage", "/s1ap/Cause", "is not a message of S1AP"),
        (
            "InitialContextSetupRequest",
            "/s1ap/E-RABToBeSetupListBearerSUReq",
            "is not an IE of InitialContextSetupRequest, which has MME-UE-S1AP-ID, eNB-UE-S1AP-ID",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/Cause/value/misspelled",
            "is not a member of Cause",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/*/value/misspelled",
            "is not a member of",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/eNB-UE-S1AP-ID/value/deeper",
            "which has none",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/eNB-UE-S1AP-ID/value/*/deeper",
            "selects nothing in a message UEContextReleaseRequest",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/Cause/misspelled",
            "is not a member of an IE, which has id, criticality, value, octets",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/NoSuchIE",
            "is not an IE of S1AP",
        ),
        (
            "InitialContextSetupRequest",
            &format!("{item}/value/misspelled"),
            "is not a member of ERABToBeSetupItemCtxtSUReq",
        ),
        (
            "S1SetupRequest",
            "/s1ap/SupportedTAs/value/Cause",
            "go by position",
        ),
    ] {
        let error = inspect::check_path(message, path).unwrap_err();
        assert!(error.contains(reason), "{message} {path}: {error}");
    }
}

#[test]
fn an_ie_is_written_by_its_name_and_its_octets_wherever_a_tree_has_ies() {
    use serde_json::json;
    let pdu = S1AP_PDU::decode(&fixture("InitialContextSetupRequest")).unwrap();
    let tree = inspect::inspect_pdu(&pdu).unwrap();
    let list = "/s1ap/E-RABToBeSetupListCtxtSUReq/value";
    let sent = |tree: &serde_json::Value| inspect::inspect_pdu(&inspect::encode_pdu(tree)?);
    let item = inspect::select(&tree, &format!("{list}/0/value")).unwrap()[0].clone();
    let named = json!({
        "id": "E-RABToBeSetupItemCtxtSUReq", "criticality": "reject", "value": item,
    });
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "00"});
    // The items of a list of IEs: in a value that is set whole, and at the end of the
    // list, on a path that has the root and on one that has not.
    let mut edited = tree.clone();
    inspect::set(&mut edited, list, json!([named, unknown])).unwrap();
    let after = sent(&edited).unwrap();
    let id = format!("{list}/E-RABToBeSetupItemCtxtSUReq/value/e-RAB-ID");
    assert_eq!(
        inspect::select(&after, &id).unwrap(),
        inspect::select(&tree, &id).unwrap()
    );
    let octets = format!("{list}/@id=60000/octets");
    assert_eq!(inspect::select(&after, &octets).unwrap(), [&json!("00")]);
    let mut edited = tree.clone();
    let position = (tree["message"]["protocolIEs"].as_array().unwrap().iter())
        .position(|ie| ie["id"] == 24)
        .unwrap();
    let raw = format!("/message/protocolIEs/{position}/value/-");
    inspect::insert(&mut edited, &raw, named.clone()).unwrap();
    inspect::insert(&mut edited, &format!("{list}/-"), unknown).unwrap();
    let after = sent(&edited).unwrap();
    assert_eq!(
        inspect::select(&after, &format!("{list}/*/id"))
            .unwrap()
            .len(),
        3
    );
    assert_eq!(inspect::select(&after, &octets).unwrap(), [&json!("00")]);
    // In a tree written by hand.
    let mut edited = tree.clone();
    let ies = edited["message"]["protocolIEs"].as_array_mut().unwrap();
    ies[0]["id"] = json!("mme-ue-s1ap-id");
    ies.push(json!({"id": "eNBname", "criticality": "ignore", "octets": "0461"}));
    let after = sent(&edited).unwrap();
    assert_eq!(after["message"]["protocolIEs"][0]["id"], json!(0));
    let name = inspect::select(&after, "/s1ap/eNBname/octets").unwrap();
    assert_eq!(name, [&json!("0461")]);
    // The octets of an IE that was added are set at its path too.
    let mut edited = tree.clone();
    let name = json!({"id": "eNBname", "criticality": "ignore", "value": "a"});
    inspect::insert(&mut edited, "/s1ap/-", name).unwrap();
    inspect::set(&mut edited, "/s1ap/eNBname/octets", json!("0461")).unwrap();
    let after = sent(&edited).unwrap();
    let name = inspect::select(&after, "/s1ap/eNBname/octets").unwrap();
    assert_eq!(name, [&json!("0461")]);
    // A name that no IE has is refused, and so are a value and octets together.
    let mut edited = tree.clone();
    edited["message"]["protocolIEs"][0]["id"] = json!("NoSuchIE");
    let error = inspect::encode_pdu(&edited).unwrap_err();
    assert!(error.contains("is not an IE of S1AP"), "{error}");
    let mut edited = tree.clone();
    edited["message"]["protocolIEs"][0]["octets"] = json!("00");
    let error = inspect::encode_pdu(&edited).unwrap_err();
    assert!(
        error.contains("its value or its octets, not both"),
        "{error}"
    );
}

#[test]
fn the_octets_of_an_ie_without_a_value_have_a_path() {
    use serde_json::json;
    let mut tree = inspect::inspect_pdu(&release_with_cause()).unwrap();
    // The octets repeat a value that is there.
    let listed = |tree: &serde_json::Value, path: &str| {
        let paths = inspect::paths(tree);
        paths.iter().any(|(listed, _)| listed == path)
    };
    assert!(!listed(&tree, "/s1ap/Cause/octets"));
    inspect::set(&mut tree, "/s1ap/Cause/octets", json!("0240")).unwrap();
    assert!(listed(&tree, "/s1ap/Cause/octets"));
    assert!(!listed(&tree, "/s1ap/Cause/value"));
    for (path, value) in inspect::paths(&tree) {
        assert_eq!(inspect::select(&tree, &path).unwrap(), [&value], "{path}");
    }
}

#[test]
fn a_private_message_has_no_ies_under_the_root() {
    let pdu = S1AP_PDU::decode(&fixture("PrivateMessage")).unwrap();
    let tree = inspect::inspect_pdu(&pdu).unwrap();
    let error = inspect::select(&tree, "/s1ap/Cause").unwrap_err();
    let reason = "a message PrivateMessage has no IEs under /s1ap: it has privateIEs";
    assert!(error.contains(reason), "{error}");
    assert!(!error.contains("protocolIEs"), "{error}");
    let error = inspect::check_path("PrivateMessage", "/s1ap/Cause").unwrap_err();
    assert!(error.contains(reason), "{error}");
    assert!(inspect::select(&tree, "/message/privateIEs/0/id/local").is_ok());
    assert!(inspect::check_path("PrivateMessage", "/message/privateIEs/0/id/global").is_ok());
}

#[test]
fn the_identifier_that_the_octets_were_decoded_as_is_a_number() {
    use serde_json::json;
    let mut tree = inspect::inspect_pdu(&release_with_cause()).unwrap();
    inspect::set(&mut tree, "/s1ap/Cause/value/nas", json!("detach")).unwrap();
    ie(&mut tree, 2)["_original_id"] = json!("two");
    let error = inspect::encode_pdu(&tree).unwrap_err();
    assert!(error.contains("_original_id is the identifier"), "{error}");
}
