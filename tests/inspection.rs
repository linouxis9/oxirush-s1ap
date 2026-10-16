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
        let view = pdu.to_view().unwrap();
        assert!(view["message"].is_object(), "{}", fields[0]);
        assert_eq!(
            S1AP_PDU::from_view(&view).unwrap().encode().unwrap(),
            wire,
            "{}",
            fields[0]
        );
        // Its view, as it is, is the PDU itself.
        assert_eq!(pdu.with_view(&view).unwrap(), pdu, "{}", fields[0]);
    }
}

#[test]
fn the_view_of_a_pdu_that_was_edited_is_that_pdu_edited() {
    use serde_json::json;

    // UE CONTEXT RELEASE REQUEST
    let wire = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode(&wire).unwrap();
    let mut view = pdu.to_view().unwrap();
    inspect::set(&mut view, "/s1ap/Cause/radioNetwork", json!("unspecified")).unwrap();
    inspect::remove(&mut view, "/s1ap/MME-UE-S1AP-ID").unwrap();

    // The edits, and the octets received for what was not edited.
    let edited = pdu.with_view(&view).unwrap();
    assert_eq!(edited, S1AP_PDU::from_view(&view).unwrap());
    assert_eq!(
        hex::encode(edited.encode().unwrap()),
        "0012400f000002000800020007000240020000"
    );
    assert_eq!(
        edited.to_tree().unwrap(),
        json!({"message": "UEContextReleaseRequest", "ies": [
            {"eNB-UE-S1AP-ID": 7},
            {"Cause": {"radioNetwork": "unspecified"}},
        ]})
    );
    // The PDU is as it was.
    assert_eq!(pdu.encode().unwrap(), wire);
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
    let mut view = pdu.to_view().unwrap();
    assert_eq!(S1AP_PDU::from_view(&view).unwrap().encode().unwrap(), wire);
    let path = "/message/protocolIEs/2/value/trackingAreaListforWarning/0/tAC";
    assert_eq!(view.pointer(path), Some(&serde_json::json!(1)));
    *view.pointer_mut(path).unwrap() = serde_json::json!(3);
    assert!(
        S1AP_PDU::from_view(&view).is_err(),
        "a typed edit must not silently discard a future IE extension"
    );
}

fn release_request(enb_ue_id: u32) -> S1AP_PDU {
    build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(enb_ue_id),
    )
    .unwrap()
}

#[test]
fn a_typed_edit_is_encoded_and_the_other_ies_keep_their_octets() {
    let mut view = release_request(7).to_view().unwrap();
    let path = "/message/protocolIEs/1/value";
    assert_eq!(view.pointer(path), Some(&serde_json::json!(7)));
    *view.pointer_mut(path).unwrap() = serde_json::json!(9);
    assert_eq!(
        S1AP_PDU::from_view(&view).unwrap().encode().unwrap(),
        release_request(9).encode().unwrap()
    );
    view["typo"] = serde_json::json!(1);
    assert!(S1AP_PDU::from_view(&view).is_err());
}

#[test]
fn the_octets_of_an_ie_are_replaced_or_added_as_its_raw_value_without_value() {
    let other = release_request(9).to_view().unwrap();
    let mut view = release_request(7).to_view().unwrap();
    let ie = view.pointer_mut("/message/protocolIEs/1").unwrap();
    ie.as_object_mut().unwrap().remove("value");
    ie["_raw_value"] = other["message"]["protocolIEs"][1]["_raw_value"].clone();
    let ies = view.pointer_mut("/message/protocolIEs").unwrap();
    ies.as_array_mut()
        .unwrap()
        .push(serde_json::json!({"id": 60000, "criticality": "ignore", "_raw_value": "C0FFEE"}));
    let edited = S1AP_PDU::from_view(&view).unwrap();
    let view = edited.to_view().unwrap();
    assert_eq!(
        view.pointer("/message/protocolIEs/1/value"),
        Some(&serde_json::json!(9))
    );
    assert_eq!(
        view.pointer("/message/protocolIEs/2/value"),
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
    )
    .unwrap();
    let added = |id: u16, value: serde_json::Value| {
        let mut view = release_request(7).to_view().unwrap();
        let ies = view.pointer_mut("/message/protocolIEs").unwrap();
        ies.as_array_mut()
            .unwrap()
            .push(serde_json::json!({"id": id, "criticality": "ignore", "value": value}));
        S1AP_PDU::from_view(&view)
    };
    let cause = serde_json::json!({"radioNetwork": "user-inactivity"});
    assert_eq!(
        added(2, cause.clone()).unwrap().encode().unwrap(),
        with_cause.encode().unwrap()
    );
    // A value that the type does not have, and an identifier without a type.
    assert!(added(2, serde_json::json!({"radioNetwork": "no-such-cause"})).is_err());
    assert!(
        added(60000, cause)
            .unwrap_err()
            .to_string()
            .contains("_raw_value")
    );
}

#[test]
fn a_new_ie_whose_value_is_a_string_is_typed_too() {
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
    )
    .unwrap();
    let typed = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        IGNORE RRC_Establishment_Cause(RRCEstablishmentCause::mo_Signalling),
        REJECT NAS_PDU(vec![0x07, 0x6a]),
    )
    .unwrap();
    let mut view = pdu.to_view().unwrap();
    let ies = view.pointer_mut("/message/protocolIEs").unwrap();
    let ies = ies.as_array_mut().unwrap();
    // An ENUMERATED by its name and an OCTET STRING by its octets, not the
    // octets of their open types.
    ies.push(serde_json::json!({"id": 134, "criticality": "ignore", "value": "mo-Signalling"}));
    ies.push(serde_json::json!({"id": 26, "criticality": "reject", "value": "076A"}));
    assert_eq!(
        S1AP_PDU::from_view(&view).unwrap().encode().unwrap(),
        typed.encode().unwrap()
    );
    // Hexadecimal in lower case is the same octets.
    *view.pointer_mut("/message/protocolIEs/2/value").unwrap() = serde_json::json!("076a");
    assert_eq!(
        S1AP_PDU::from_view(&view).unwrap().encode().unwrap(),
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
    let mut view = pdu.to_view().unwrap();
    let path = "/message/protocolIEs/0/value";
    assert_eq!(view.pointer(path), Some(&serde_json::json!("C0FFEE")));
    assert!(view["message"]["protocolIEs"][0]["_decode_error"].is_string());
    assert_eq!(
        S1AP_PDU::from_view(&view).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
    *view.pointer_mut(path).unwrap() = serde_json::json!("DECAFBAD");
    let edited = S1AP_PDU::from_view(&view).unwrap();
    assert_eq!(
        edited.to_view().unwrap().pointer(path),
        Some(&serde_json::json!("DECAFBAD"))
    );
    // Its type is unknown, so a typed value has no encoding.
    *view.pointer_mut(path).unwrap() = serde_json::json!(5);
    assert!(S1AP_PDU::from_view(&view).is_err());
}

#[test]
fn a_changed_raw_value_alone_sends_nothing_else() {
    let other = release_request(9).to_view().unwrap();
    let pdu = release_request(7);
    let mut view = pdu.to_view().unwrap();
    view["message"]["protocolIEs"][1]["_raw_value"] =
        other["message"]["protocolIEs"][1]["_raw_value"].clone();
    assert_eq!(
        S1AP_PDU::from_view(&view).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
}

#[test]
fn a_message_is_shown_and_written_as_its_name_and_its_ies_by_name() {
    // Every message of the fixtures is the same octets once it was shown and written.
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let wire = hex::decode(fields[1]).unwrap();
        let shown = match S1AP_PDU::decode(&wire).unwrap().to_tree() {
            Ok(shown) => shown,
            // A private message has no IEs of the protocol to name.
            Err(reason) => {
                assert_eq!(reason.kind(), oxirush_s1ap::ErrorKind::Limit);
                assert!(
                    reason
                        .to_string()
                        .contains("PrivateMessage has no list of IEs"),
                    "{reason}"
                );
                continue;
            }
        };
        assert!(
            shown["message"].is_string() && shown["ies"].is_array(),
            "{}",
            fields[0]
        );
        let written = S1AP_PDU::from_tree(&shown);
        assert_eq!(
            written.unwrap().encode().unwrap(),
            wire,
            "{}: {shown}",
            fields[0]
        );
    }
    // The name gives the procedure and the criticalities; an IE that the message does
    // not have says its own, and octets are sent as they are.
    let request = serde_json::json!({
        "message": "ue context release request",
        "ies": [
            {"mme-ue-s1ap-id": 1},
            {"eNB-UE-S1AP-ID": 2},
            {"Cause": {"radioNetwork": "user-inactivity"}},
            {"65000": {"octets": "00"}, "criticality": "reject"},
        ],
    });
    let pdu = S1AP_PDU::from_tree(&request).unwrap();
    assert_eq!(inspect::message_name(&pdu), Some("UEContextReleaseRequest"));
    let view = pdu.to_view().unwrap();
    assert_eq!(view["criticality"], "ignore");
    let criticalities = view["message"]["protocolIEs"].as_array().unwrap().iter();
    let criticalities: Vec<_> = criticalities
        .map(|ie| ie["criticality"].as_str().unwrap())
        .collect();
    assert_eq!(criticalities, ["reject", "reject", "ignore", "reject"]);
    let shown = pdu.to_tree().unwrap();
    assert_eq!(shown["message"], "UEContextReleaseRequest");
    assert_eq!(shown["ies"][0], serde_json::json!({"MME-UE-S1AP-ID": 1}));
    assert_eq!(
        shown["ies"][3],
        serde_json::json!({"65000": {"octets": "00"}, "criticality": "reject"})
    );
    // What is not a message, not an IE, or an IE of another message without its
    // criticality, is refused with what is wrong.
    let refused = |tree: serde_json::Value| S1AP_PDU::from_tree(&tree).unwrap_err().to_string();
    let named = |message: &str, ie: &str| serde_json::json!({"message": message, "ies": [{ie: 1}]});
    assert!(
        refused(named("UEContextReleaseRequst", "MME-UE-S1AP-ID")).contains("is not a message")
    );
    assert!(refused(named("UEContextReleaseRequest", "MME-UE-ID")).contains("is not an IE"));
    assert!(
        refused(named("UEContextReleaseRequest", "CSG-Id")).contains("says its \"criticality\"")
    );
    assert!(refused(serde_json::json!({"message": "Paging", "ies": [{}]})).contains("one IE"));
    assert!(refused(serde_json::json!({"message": "Paging", "ie": []})).contains("is no member"));
}

/// The members of `value`, a tree or a part of one, that only repeat what its form says:
/// those of an IE in a view, and a `criticality` that the tables have.
fn redundant(value: &serde_json::Value, found: &mut Vec<String>) {
    use serde_json::Value;
    match value {
        Value::Object(members) => {
            for (name, member) in members {
                let wraps = match name.as_str() {
                    "decoded" | "id" | "extensionValue" | "criticality" => true,
                    // A bit string whose size varies has a `value` and a `length`.
                    "value" => !members.contains_key("length"),
                    name => name.starts_with('_'),
                };
                if wraps {
                    found.push(name.clone());
                }
                redundant(member, found);
            }
        }
        Value::Array(entries) => entries.iter().for_each(|entry| redundant(entry, found)),
        _ => {}
    }
}

#[test]
fn a_message_is_shown_and_written_by_name_to_any_depth() {
    use serde_json::json;
    let shown = |wire: &[u8]| S1AP_PDU::decode(wire).unwrap().to_tree();
    let written =
        |tree: &serde_json::Value| S1AP_PDU::from_tree(tree).map(|pdu| pdu.encode().unwrap());
    // The items of a list of E-RABs go by name, without the criticality that their
    // object set assigns.
    let wire = fixture("E-RABSetupRequest");
    let bearer = json!({
        "e-RAB-ID": 0,
        "e-RABlevelQoSParameters": {"qCI": 0, "allocationRetentionPriority": {
            "priorityLevel": 0,
            "pre-emptionCapability": "shall-not-trigger-pre-emption",
            "pre-emptionVulnerability": "not-pre-emptable",
        }},
        "transportLayerAddress": {"value": "00", "length": 1},
        "gTP-TEID": 2778534635u32,
        "nAS-PDU": "076002",
    });
    let request = |bearer: &serde_json::Value| {
        json!({
            "message": "E-RABSetupRequest",
            "ies": [
                {"MME-UE-S1AP-ID": 0},
                {"eNB-UE-S1AP-ID": 0},
                {"E-RABToBeSetupListBearerSUReq": [bearer]},
            ],
        })
    };
    let by_name = request(&json!({"E-RABToBeSetupItemBearerSUReq": bearer}));
    assert_eq!(shown(&wire).unwrap(), by_name);
    assert_eq!(written(&by_name).unwrap(), wire);
    // No message of the fixtures is shown with a member that only wraps a value, nor
    // with a criticality: the tables have each of them.
    let mut found = Vec::new();
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        if let Ok(shown) = shown(&hex::decode(fields[1]).unwrap()) {
            redundant(&shown["ies"], &mut found);
        }
    }
    assert_eq!(found, Vec::<String>::new());
    // The same message as a view has it, with an entry for the item.
    let entry = json!({"id": 17, "criticality": "reject", "value": bearer});
    assert_eq!(written(&request(&entry)).unwrap(), wire);
    // An IE of an extension container says its criticality, which `to_tree`
    // shows: the tables have none for it.
    let mut extended = bearer.clone();
    extended["iE-Extensions"] = json!([{"BearerType": "non-IP", "criticality": "reject"}]);
    let extended = request(&json!({"E-RABToBeSetupItemBearerSUReq": extended}));
    let pdu = S1AP_PDU::from_tree(&extended).unwrap();
    let view = pdu.to_view().unwrap();
    let extension = "/s1ap/E-RABToBeSetupListBearerSUReq/0/iE-Extensions/0";
    let value = inspect::select(&view, &format!("{extension}/extensionValue")).unwrap();
    assert_eq!(value, [&json!("non-IP")]);
    assert_eq!(pdu.to_tree().unwrap(), extended);
    let mut unassigned = bearer.clone();
    unassigned["iE-Extensions"] = json!([{"BearerType": "non-IP"}]);
    let unassigned = request(&json!({"E-RABToBeSetupItemBearerSUReq": unassigned}));
    let error = written(&unassigned).unwrap_err().to_string();
    assert!(
        error.contains("BearerType says its \"criticality\""),
        "{error}"
    );
    // An item that two object sets give two criticalities says its own: that of a
    // connection is `reject` in a RESET and `ignore` in its acknowledgement.
    let reset = |connection: &serde_json::Value| {
        json!({"message": "Reset", "ies": [
            {"Cause": {"misc": "unspecified"}},
            {"ResetType": {"partOfS1-Interface": [connection]}},
        ]})
    };
    let mut connection = json!({"UE-associatedLogicalS1-ConnectionItem": {"mME-UE-S1AP-ID": 1}});
    let error = written(&reset(&connection)).unwrap_err().to_string();
    assert!(error.contains("Item says its \"criticality\""), "{error}");
    connection["criticality"] = json!("reject");
    let pdu = S1AP_PDU::from_tree(&reset(&connection)).unwrap();
    assert_eq!(pdu.to_tree().unwrap(), reset(&connection));
    // An IE that a value holds alone, as the alternative that extends a CHOICE, has
    // the criticality of its object set.
    let node = json!({
        "global-ENB-ID": {"pLMNidentity": "208-93", "eNB-ID": {"macroENB-ID": "000010"}},
        "selected-TAI": {"pLMNidentity": "208-93", "tAC": 1},
    });
    let report = json!({"rLFReportInformation": {"uE-RLF-Report-Container": "00"}});
    let transfer = json!({"message": "ENBConfigurationTransfer", "ies": [
        {"SONConfigurationTransferECT": {
            "targeteNB-ID": node,
            "sourceeNB-ID": node,
            "sONInformation": {"sONInformation-Extension": {"SON-Information-Report": report}},
        }},
    ]});
    let pdu = S1AP_PDU::from_tree(&transfer).unwrap();
    let view = pdu.to_view().unwrap();
    let held = "/s1ap/SONConfigurationTransferECT/sONInformation/sONInformation-Extension";
    let criticality = inspect::select(&view, &format!("{held}/criticality")).unwrap();
    assert_eq!(criticality, [&json!("ignore")]);
    let container = format!("{held}/rLFReportInformation/uE-RLF-Report-Container");
    assert_eq!(inspect::select(&view, &container).unwrap(), [&json!("00")]);
    assert_eq!(pdu.to_tree().unwrap(), transfer);
    // What is in the place of an IE and is none is refused.
    let error = written(&request(&json!({"a": 1, "b": 2})))
        .unwrap_err()
        .to_string();
    assert!(error.contains("one IE"), "{error}");
}

#[test]
fn the_value_of_an_ie_may_be_left_out_of_a_path() {
    use serde_json::{Value, json};
    let name = "E-RABSetupRequest";
    let view = S1AP_PDU::decode(&fixture(name)).unwrap().to_view().unwrap();
    let long = "/s1ap/E-RABToBeSetupListBearerSUReq/value/0/value";
    let list = "/s1ap/E-RABToBeSetupListBearerSUReq";
    let short = format!("{list}/0");
    // Each of the two is left out alone, and both together; the item goes by its
    // position, its name or its identifier.
    let tunnel = inspect::select(&view, &format!("{long}/gTP-TEID")).unwrap();
    assert_eq!(tunnel, [&json!(2778534635u32)]);
    for path in [
        format!("{list}/0/value"),
        format!("{list}/value/0"),
        short.clone(),
        short.to_lowercase(),
        format!("{list}/E-RABToBeSetupItemBearerSUReq"),
        format!("{list}/@id=17"),
    ] {
        let selected = inspect::select(&view, &format!("{path}/gTP-TEID"));
        assert_eq!(selected.as_ref(), Ok(&tunnel), "{path}");
        let checked = inspect::check_path(name, &format!("{path}/gTP-TEID"));
        assert_eq!(checked, Ok(()), "{path}");
    }
    // What an IE has itself comes first: `*` is each of its members, and the entries
    // of its value are after `value`.
    let select = |path: &str| inspect::select(&view, path).unwrap();
    assert_eq!(select(&format!("{list}/criticality")), [&json!("reject")]);
    assert_eq!(select(&format!("{short}/criticality")), [&json!("reject")]);
    assert_eq!(select(&format!("{list}/*")).len(), 3);
    assert_eq!(select(&format!("{list}/value/*/id")), [&json!(17)]);
    assert!(select(&format!("{short}/octets"))[0].is_string());
    // The names of an ENUMERATED are found on the same path.
    let capability = "e-RABlevelQoSParameters/allocationRetentionPriority/pre-emptionCapability";
    let item = format!("{list}/E-RABToBeSetupItemBearerSUReq");
    let names = inspect::enumerated_at(name, &format!("{item}/{capability}")).unwrap();
    assert!(names.is_some_and(|names| names.contains(&"may-trigger-pre-emption")));
    assert_eq!(
        inspect::enumerated_at(name, &format!("{list}/value/@id=17/value/{capability}")),
        Ok(names)
    );
    // Edits take the same paths: a member is set, an item is added and one taken out,
    // and the octets of the IE are no longer those of its value.
    let mut edited = view.clone();
    let second = inspect::select(&view, &short).unwrap()[0].clone();
    inspect::set(&mut edited, &format!("{short}/e-RAB-ID"), json!(5)).unwrap();
    let error = inspect::select(&edited, &format!("{list}/octets"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("the value was edited"), "{error}");
    inspect::insert(&mut edited, &format!("{list}/-"), second.clone()).unwrap();
    inspect::insert(&mut edited, &format!("{list}/0"), second).unwrap();
    inspect::set(&mut edited, &format!("{list}/0/nAS-PDU"), json!("0760")).unwrap();
    inspect::remove(&mut edited, &format!("{list}/2")).unwrap();
    let after = S1AP_PDU::from_view(&edited).unwrap().to_view().unwrap();
    let bearers = inspect::select(&after, &format!("{list}/value/*/e-RAB-ID")).unwrap();
    assert_eq!(bearers, [&json!(0), &json!(5)]);
    let messages = inspect::select(&after, &format!("{list}/value/*/nAS-PDU")).unwrap();
    assert_eq!(messages, [&json!("0760"), &json!("076002")]);
    let error = inspect::set(&mut edited, &format!("{list}/0/nAS-PDU"), Value::Null);
    assert!(
        error
            .unwrap_err()
            .to_string()
            .contains("its type always has it")
    );
    // A name that neither has is refused with the members of both, and the view is as
    // it was.
    let before = edited.clone();
    for (path, value) in [
        (
            format!("{list}/0/misspelled"),
            "is not a member of ERABToBeSetupItemBearerSUReq, which has e-RAB-ID",
        ),
        (format!("{list}/misspelled"), "is not a position in a list"),
        (
            "/s1ap/eNB-UE-S1AP-ID/valeu".to_string(),
            "\"valeu\" is not a member of this value, which has none",
        ),
    ] {
        for error in [
            inspect::select(&edited, &path)
                .map(|_| ())
                .unwrap_err()
                .to_string(),
            inspect::set(&mut edited, &path, json!(1))
                .unwrap_err()
                .to_string(),
        ] {
            assert!(error.contains(value), "{path}: {error}");
            let own = "; an IE itself has id, criticality, value, octets";
            assert!(error.ends_with(own), "{path}: {error}");
        }
    }
    assert_eq!(edited, before);
    // A member of a value that is named as one of its IE is that of the IE: the `value`
    // of a bit string whose size varies is reached after `value`.
    let address = json!({"value": "0A0000", "length": 24});
    let entry =
        json!({"id": "TraceCollectionEntityIPAddress", "criticality": "ignore", "value": address});
    let view = sent(&[entry]).unwrap();
    let ie = "/s1ap/TraceCollectionEntityIPAddress";
    let select = |path: &str| inspect::select(&view, &format!("{ie}/{path}")).unwrap();
    assert_eq!(select("value"), [&address]);
    assert_eq!(select("value/value"), [&json!("0A0000")]);
    assert_eq!(select("length"), [&json!(24)]);
    assert_eq!(select("value/length"), [&json!(24)]);
}

#[test]
fn the_container_of_a_handover_is_written_and_selected_by_the_type_that_it_carries() {
    use serde_json::{Value, json};
    // Between eNBs, the Source to Target Transparent Container of a HANDOVER REQUIRED
    // carries a Source eNB to Target eNB Transparent Container: TS 36.413 clause
    // 9.2.1.56 says so, and the ASN.1 has an OCTET STRING.
    let name = "SourceeNB-ToTargeteNB-TransparentContainer";
    let container = json!({
        "rRC-Container": "0102",
        "targetCell-ID": {"pLMNidentity": "208-93", "cell-ID": 4660},
        "uE-HistoryInformation": [{"uTRAN-Cell": "00"}],
    });
    let required = |container: Value| {
        json!({"message": "HandoverRequired", "ies": [
            {"MME-UE-S1AP-ID": 1},
            {"eNB-UE-S1AP-ID": 2},
            {"Source-ToTarget-TransparentContainer": container},
        ]})
    };
    let pdu = S1AP_PDU::from_tree(&required(json!({name: container}))).unwrap();
    // The octets are the APER encoding of the type, which the bindings read.
    let message: HandoverRequired = pdu.decode_value().unwrap();
    let octets: SourceToTargetTransparentContainer =
        decode_open_type(&message.protocol_ies.0[2].value).unwrap();
    let carried: SourceeNBToTargeteNBTransparentContainer =
        oxirush_s1ap::aper::decode(&octets.0).unwrap();
    assert_eq!(carried.rrc_container.0, [1, 2]);
    assert_eq!(oxirush_s1ap::aper::encode(&carried).unwrap(), octets.0);
    // Nothing says what the octets carry: a view and a tree have them as they are.
    let mut view = pdu.to_view().unwrap();
    let ie = "/s1ap/Source-ToTarget-TransparentContainer";
    let hexadecimal = json!(hex::encode_upper(&octets.0));
    let value = inspect::select(&view, &format!("{ie}/value")).unwrap();
    assert_eq!(value, [&hexadecimal]);
    let shown = pdu.to_tree().unwrap();
    assert_eq!(shown, required(hexadecimal.clone()));
    // A path names the type. The types say whether it can select, and the octets are
    // opened before it does.
    let cell = format!("{ie}/{name}/targetCell-ID/cell-ID");
    assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));
    let error = inspect::select(&view, &cell).unwrap_err().to_string();
    assert!(error.contains("open decodes them"), "{error}");
    inspect::open(&mut view, &cell).unwrap();
    assert_eq!(inspect::select(&view, &cell).unwrap(), [&json!(4660)]);
    // What was written by the type is selected by the type, in any case of its name,
    // and its paths are listed.
    let decoded = inspect::select(&view, &format!("{ie}/{name}/decoded")).unwrap();
    assert_eq!(decoded, [&container]);
    let loose = format!("{ie}/value/{}/rrc_container", name.to_lowercase());
    assert_eq!(inspect::select(&view, &loose).unwrap(), [&json!("0102")]);
    let listed = format!("{ie}/value/{name}/decoded/rRC-Container");
    let paths = inspect::paths(&view);
    assert!(paths.iter().any(|(path, _)| *path == listed), "{paths:?}");
    for (path, value) in &paths {
        assert_eq!(inspect::select(&view, path).unwrap(), [value], "{path}");
    }
    // The octets that were received are kept, and are those of the transfer.
    let received = inspect::select(&view, &format!("{ie}/{name}/octets")).unwrap();
    assert_eq!(received, [&hexadecimal]);
    assert_eq!(S1AP_PDU::from_view(&view).unwrap(), pdu);
    // An edit changes what the view has: octets are opened before it goes into them.
    let mut view = pdu.to_view().unwrap();
    for path in [cell.clone(), cell.replace(name, &format!("value/{name}"))] {
        let error = inspect::set(&mut view, &path, json!(7))
            .unwrap_err()
            .to_string();
        assert!(error.contains("open decodes them"), "{error}");
    }
    inspect::open(&mut view, &cell).unwrap();
    inspect::set(&mut view, &cell, json!(7)).unwrap();
    inspect::remove(&mut view, &format!("{ie}/{name}/uE-HistoryInformation/0")).unwrap();
    let cells = json!({"uTRAN-Cell": "01"});
    inspect::insert(
        &mut view,
        &format!("{ie}/{name}/uE-HistoryInformation/-"),
        cells,
    )
    .unwrap();
    let mut after = S1AP_PDU::from_view(&view).unwrap().to_view().unwrap();
    assert!(inspect::select(&after, &format!("{ie}/value")).unwrap()[0].is_string());
    inspect::open(&mut after, &cell).unwrap();
    assert_eq!(inspect::select(&after, &cell).unwrap(), [&json!(7)]);
    let history = format!("{ie}/{name}/uE-HistoryInformation/0");
    let visited = inspect::select(&after, &history).unwrap();
    assert_eq!(visited, [&json!({"uTRAN-Cell": "01"})]);
    // The value of the IE is set by the type, and an IE is added by it: the container
    // of the other direction.
    let back = json!({"targetenb_tosourceenb_transparentcontainer": {"rRC-Container": "03"}});
    inspect::set(&mut after, &format!("{ie}/value"), json!({name: container})).unwrap();
    let id = "Target-ToSource-TransparentContainer";
    let entry = json!({"id": id, "criticality": "reject", "value": back});
    inspect::insert(&mut after, "/s1ap/-", entry).unwrap();
    let sent = S1AP_PDU::from_view(&after).unwrap().to_view().unwrap();
    let octets = |ie: &str| inspect::select(&sent, &format!("/s1ap/{ie}/value")).unwrap();
    assert_eq!(
        octets("Source-ToTarget-TransparentContainer"),
        [&hexadecimal]
    );
    assert_eq!(
        octets("Target-ToSource-TransparentContainer"),
        [&json!("000103")]
    );
    // Octets that are not of the type say so where they are selected, and are sent
    // as they came.
    let other = S1AP_PDU::from_tree(&required(json!("FF"))).unwrap();
    let mut view = other.to_view().unwrap();
    inspect::open(&mut view, &cell).unwrap();
    let error = inspect::select(&view, &cell).unwrap_err().to_string();
    assert!(error.contains("the transfer did not decode"), "{error}");
    assert_eq!(S1AP_PDU::from_view(&view).unwrap(), other);
    // A path that goes through no such name opens nothing.
    let before = view.clone();
    inspect::open(&mut view, "/s1ap/eNB-UE-S1AP-ID/value").unwrap();
    assert_eq!(view, before);
    // The name is one of a type that the IE may carry, and the secondary container,
    // which a handover to GERAN has, carries none of this specification.
    let written = S1AP_PDU::from_tree(&required(back));
    let error = written.unwrap_err().to_string();
    assert!(
        error.contains(&format!("for a type that they carry: {name}")),
        "{error}"
    );
    for path in [
        format!("{ie}/NoSuchType/rRC-Container"),
        format!("{ie}/rRC-Container"),
    ] {
        let error = inspect::check_path("HandoverRequired", &path).unwrap_err();
        let text = error.to_string();
        assert!(text.contains(&format!("which has {name}, at")), "{error}");
        assert_eq!(inspect::select(&view, &path), Err(error.clone()), "{path}");
        assert_eq!(inspect::open(&mut view, &path), Err(error), "{path}");
    }
    let secondary = format!("/s1ap/Source-ToTarget-TransparentContainer-Secondary/{name}");
    let error = inspect::check_path("HandoverRequired", &secondary)
        .unwrap_err()
        .to_string();
    assert!(error.contains("which has none"), "{error}");
}

/// Each value of `shown`, a tree or a part of one, with the path that joins the names
/// and the positions on the way to it. A list of plain values is one value.
fn joined(shown: &serde_json::Value, path: &str, found: &mut Vec<(String, serde_json::Value)>) {
    use serde_json::Value;
    match shown {
        Value::Object(members) if !members.is_empty() => {
            for (name, member) in members {
                joined(member, &format!("{path}/{name}"), found);
            }
        }
        Value::Array(entries) if entries.iter().any(|e| e.is_object() || e.is_array()) => {
            for (index, entry) in entries.iter().enumerate() {
                joined(entry, &format!("{path}/{index}"), found);
            }
        }
        value => found.push((path.to_string(), value.clone())),
    }
}

#[test]
fn the_names_that_a_message_shows_join_into_the_path_of_each_value() {
    use serde_json::json;
    let (mut values, mut whole, mut bits) = (0, 0, 0);
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = S1AP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        // A private message has no IEs of the protocol to name.
        let Ok(shown) = pdu.to_tree() else {
            continue;
        };
        let view = pdu.to_view().unwrap();
        let ies = shown["ies"].as_array().unwrap();
        for (index, ie) in ies.iter().enumerate() {
            // An IE of the message by its position, then by its name, and by its name
            // alone, which selects it when the message has it once.
            let mut found = Vec::new();
            joined(ie, &format!("/s1ap/{index}"), &mut found);
            for (path, value) in found {
                let at = format!("{} {path}", fields[0]);
                let selected = inspect::select(&view, &path).expect(&at);
                assert_eq!(inspect::check_path(fields[0], &path), Ok(()), "{at}");
                let named = path.replacen(&format!("/{index}/"), "/", 1);
                if !named.ends_with("/criticality") || named.matches('/').count() > 3 {
                    assert_eq!(inspect::select(&view, &named), Ok(selected.clone()), "{at}");
                }
                values += 1;
                // A path that ends with the name of an IE selects the IE, as the name
                // alone does in a list: a value that is the whole value of its IE is
                // after `value`.
                if selected[0].get("_raw_value").is_some() {
                    let of_the_ie = inspect::select(&view, &format!("{path}/value")).expect(&at);
                    assert_eq!(of_the_ie, [&value], "{at}");
                    whole += 1;
                    continue;
                }
                // A bit string whose size varies that is the value of an IE has a
                // member that is named as one of the IE: its `value` is after `value`.
                if path.ends_with("/value") && selected[0].get("length").is_some() {
                    let after = inspect::select(&view, &format!("{path}/value")).expect(&at);
                    assert_eq!(after, [&value], "{at}");
                    bits += 1;
                    continue;
                }
                assert_eq!(selected, [&value], "{at}");
            }
        }
    }
    assert!(
        values > 300 && whole > 100 && bits > 0,
        "{values} {whole} {bits}"
    );

    // An item of a list of E-RABs, by its position then its name.
    let name = "E-RABSetupRequest";
    let mut view = S1AP_PDU::decode(&fixture(name)).unwrap().to_view().unwrap();
    let list = "/s1ap/E-RABToBeSetupListBearerSUReq";
    let tunnel = format!("{list}/0/E-RABToBeSetupItemBearerSUReq/gTP-TEID");
    assert_eq!(
        inspect::select(&view, &tunnel).unwrap(),
        [&json!(2778534635u32)]
    );
    // The name of another IE says which IE the entry is, where a view says it.
    let other = format!("{list}/0/E-RABItem/e-RAB-ID");
    let error = inspect::select(&view, &other).unwrap_err();
    let which = "this IE is E-RABToBeSetupItemBearerSUReq, not E-RABItem: ";
    let text = error.to_string();
    assert!(text.starts_with(which), "{error}");
    assert!(
        text.contains("is not a member of ERABToBeSetupItemBearerSUReq"),
        "{error}"
    );
    assert_eq!(inspect::set(&mut view, &other, json!(5)), Err(error));
    // Without a view, the entry at a position is the IE that the segment names, and
    // the path is read against its type.
    assert_eq!(inspect::check_path(name, &tunnel), Ok(()));
    assert_eq!(inspect::check_path(name, &other), Ok(()));
    let wrong = format!("{list}/0/E-RABToBeSetupItemBearerSUReq/gTP-TEIDs");
    let error = inspect::check_path(name, &wrong).unwrap_err().to_string();
    assert!(
        error.contains("is not a member of ERABToBeSetupItemBearerSUReq"),
        "{error}"
    );
    let capability = "e-RABlevelQoSParameters/allocationRetentionPriority/pre-emptionCapability";
    let item = format!("{list}/0/E-RABToBeSetupItemBearerSUReq");
    let names = inspect::enumerated_at(name, &format!("{item}/{capability}")).unwrap();
    assert!(names.is_some_and(|names| names.contains(&"may-trigger-pre-emption")));
    // An edit takes the same path, and the entry itself where the path ends with its
    // name.
    inspect::set(&mut view, &tunnel, json!(7)).unwrap();
    let second = inspect::select(&view, &item).unwrap()[0].clone();
    inspect::insert(&mut view, &item, second).unwrap();
    inspect::remove(
        &mut view,
        &format!("{list}/1/E-RABToBeSetupItemBearerSUReq"),
    )
    .unwrap();
    let sent = S1AP_PDU::from_view(&view).unwrap().to_view().unwrap();
    let tunnels = inspect::select(&sent, &format!("{list}/value/*/gTP-TEID")).unwrap();
    assert_eq!(tunnels, [&json!(7)]);
    // A member of a value that is named as another IE is that member: an E-RAB has a
    // `nAS-PDU`, and `NAS-PDU` is an IE.
    let message = inspect::select(&sent, &format!("{item}/nAS-PDU")).unwrap();
    assert_eq!(message, [&json!("076002")]);
    let checked = inspect::check_path(name, &format!("{item}/nAS-PDU"));
    assert_eq!(checked, Ok(()));
    // The value of one IE has a member that is named as the IE itself. A segment is the
    // IE when it spells the name as ASN.1 spells that of the IE, which is how a message
    // shows it, and the member otherwise, as it was before an IE could be named there.
    let limit = json!({"uESidelinkAggregateMaximumBitRate": 5});
    let limit = json!({"UESidelinkAggregateMaximumBitrate": limit, "criticality": "ignore"});
    let request = json!({"message": "InitialContextSetupRequest", "ies": [limit]});
    let view = S1AP_PDU::from_tree(&request).unwrap().to_view().unwrap();
    for path in [
        "/s1ap/0/UESidelinkAggregateMaximumBitrate/uESidelinkAggregateMaximumBitRate",
        "/s1ap/UESidelinkAggregateMaximumBitrate/uESidelinkAggregateMaximumBitRate",
        "/s1ap/0/uESidelinkAggregateMaximumBitRate",
        "/s1ap/0/uesidelinkaggregatemaximumbitrate",
        "/s1ap/0/UESidelinkAggregateMaximumBitrate/value/UESidelinkAggregateMaximumBitrate",
    ] {
        assert_eq!(inspect::select(&view, path).unwrap(), [&json!(5)], "{path}");
        let checked = inspect::check_path("InitialContextSetupRequest", path);
        assert_eq!(checked, Ok(()), "{path}");
    }
    let ie = inspect::select(&view, "/s1ap/0/UESidelinkAggregateMaximumBitrate").unwrap();
    assert_eq!(ie[0]["id"], json!(248));
}

/// The octets of the message `name` of the fixtures.
fn fixture(name: &str) -> Vec<u8> {
    let mut lines = include_str!("fixtures/messages.tsv").lines();
    let line = lines.find(|line| line.split('\t').next() == Some(name));
    hex::decode(line.unwrap().split('\t').nth(1).unwrap()).unwrap()
}

/// The entry of the IE `id` of a view.
fn ie(view: &mut serde_json::Value, id: u16) -> &mut serde_json::Value {
    let ies = view["message"]["protocolIEs"].as_array_mut().unwrap();
    ies.iter_mut().find(|ie| ie["id"] == id).unwrap()
}

/// The view of a message that has `entries` for IEs, encoded and decoded again.
fn sent(entries: &[serde_json::Value]) -> Result<serde_json::Value, oxirush_s1ap::Error> {
    let pdu = build_s1ap!(
        InitiatingMessage,
        UEContextReleaseRequest,
        IGNORE,
        UEContextReleaseRequest,
    )
    .unwrap();
    let mut view = pdu.to_view()?;
    *view["message"]["protocolIEs"].as_array_mut().unwrap() = entries.to_vec();
    S1AP_PDU::from_view(&view)?.to_view()
}

#[test]
fn well_known_values_are_shown_and_taken_as_they_are_usually_written() {
    use serde_json::json;
    // S1 SETUP REQUEST: Global-ENB-ID is IE 59 and SupportedTAs IE 64.
    let wire = fixture("S1SetupRequest");
    let mut view = S1AP_PDU::decode(&wire).unwrap().to_view().unwrap();
    let area = &ie(&mut view, 64)["value"][0];
    assert_eq!(area["tAC"], json!(0x9302));
    assert_eq!(area["broadcastPLMNs"], json!(["208-93"]));
    assert_eq!(ie(&mut view, 59)["value"]["pLMNidentity"], json!("208-93"));
    // What is not edited keeps its octets.
    assert_eq!(S1AP_PDU::from_view(&view).unwrap().encode().unwrap(), wire);

    // Edited as they are shown, a number in hexadecimal too.
    let area = &mut ie(&mut view, 64)["value"][0];
    area["tAC"] = json!("0x7");
    area["broadcastPLMNs"] = json!(["001-01", "310-410"]);
    let edited = S1AP_PDU::from_view(&view).unwrap();
    let mut shown = edited.to_view().unwrap();
    let area = &ie(&mut shown, 64)["value"][0];
    assert_eq!(area["tAC"], json!(7));
    assert_eq!(area["broadcastPLMNs"], json!(["001-01", "310-410"]));
    assert_eq!(
        ie(&mut shown, 59)["_raw_value"],
        ie(&mut view, 59)["_raw_value"]
    );

    // As JER writes them, which is what a view had before: the octets that TS 36.413
    // clause 9.2.3.8 has for 001-01 and for 310-410.
    let area = &mut ie(&mut view, 64)["value"][0];
    area["tAC"] = json!("0007");
    area["broadcastPLMNs"] = json!(["00F110", "134001"]);
    assert_eq!(
        S1AP_PDU::from_view(&view).unwrap().encode().unwrap(),
        edited.encode().unwrap()
    );

    // Neither form, and a number that the type has no room for.
    for refused in [json!("7"), json!(1 << 16), json!("0x10000")] {
        ie(&mut view, 64)["value"][0]["tAC"] = refused.clone();
        assert!(S1AP_PDU::from_view(&view).is_err(), "{refused}");
    }
    ie(&mut view, 64)["value"][0]["tAC"] = json!(7);
    ie(&mut view, 64)["value"][0]["broadcastPLMNs"] = json!(["208-9"]);
    assert!(S1AP_PDU::from_view(&view).is_err());
}

#[test]
fn the_parts_of_a_gummei_are_numbers() {
    use serde_json::json;
    // S1 SETUP RESPONSE: ServedGUMMEIs is IE 105.
    let wire = fixture("S1SetupResponse");
    let mut view = S1AP_PDU::decode(&wire).unwrap().to_view().unwrap();
    let served = &mut ie(&mut view, 105)["value"][0];
    assert_eq!(served["servedPLMNs"], json!(["208-93"]));
    assert_eq!(served["servedGroupIDs"], json!([0xB8A3]));
    assert_eq!(served["servedMMECs"], json!([0x89]));
    assert_eq!(S1AP_PDU::from_view(&view).unwrap().encode().unwrap(), wire);
    let served = &mut ie(&mut view, 105)["value"][0];
    served["servedGroupIDs"] = json!([2, "0x8000"]);
    served["servedMMECs"] = json!([1]);
    let edited = S1AP_PDU::from_view(&view).unwrap();
    let mut shown = edited.to_view().unwrap();
    let served = &ie(&mut shown, 105)["value"][0];
    assert_eq!(served["servedGroupIDs"], json!([2, 0x8000]));
    assert_eq!(served["servedMMECs"], json!([1]));
    // An MME code has one octet.
    ie(&mut view, 105)["value"][0]["servedMMECs"] = json!([256]);
    assert!(S1AP_PDU::from_view(&view).is_err());
}

#[test]
fn a_tunnel_endpoint_and_the_identities_of_a_ue_are_readable() {
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
        ERABID(5),
        TransportLayerAddress(oxirush_s1ap::BitString::from([10, 0, 0, 2])),
        GTPTEID::from([0, 0, 0, 2]),
        None,
    );
    assert_eq!(
        item["_raw_value"],
        octets(&encode_open_type(&expected).unwrap())
    );

    // UEPagingID is IE 43: an IMSI by its digits, or an S-TMSI by its numbers.
    let mut shown = added(43, json!({"iMSI": "208930000000001"})).unwrap();
    let imsi = IMSI(vec![0x02, 0x98, 0x03, 0x00, 0x00, 0x00, 0x00, 0xF1].into());
    let expected = encode_open_type(&UEPagingID::iMSI(imsi)).unwrap();
    assert_eq!(
        ie(&mut shown, 43)["value"]["iMSI"],
        json!("208930000000001")
    );
    assert_eq!(ie(&mut shown, 43)["_raw_value"], octets(&expected));
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
    .unwrap()
}

#[test]
fn the_ies_of_a_message_have_paths_by_their_names() {
    use serde_json::json;
    let view = release_with_cause().to_view().unwrap();
    let paths = inspect::paths(&view);
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
        assert_eq!(inspect::select(&view, path).unwrap(), [&json!(7)], "{path}");
    }
    // The octets that an IE was received as.
    let octets = inspect::select(&view, "/s1ap/eNB-UE-S1AP-ID/octets").unwrap();
    assert_eq!(octets, [&json!("0007")]);
    assert_eq!(inspect::select(&view, "/s1ap/*/id").unwrap().len(), 3);
}

#[test]
fn every_path_of_every_canonical_procedure_selects_its_value() {
    let mut shorter = 0;
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = S1AP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        let view = pdu.to_view().unwrap();
        for (path, value) in inspect::paths(&view) {
            // The list that holds the IEs of a message is never named.
            assert!(!path.contains("protocolIEs"), "{} {path}", fields[0]);
            let selected = inspect::select(&view, &path);
            assert_eq!(selected, Ok(vec![&value]), "{} {path}", fields[0]);
            // The path that leaves out what it may selects the same value.
            let short = short(&view, &path);
            let selected = inspect::select(&view, &short);
            assert_eq!(selected, Ok(vec![&value]), "{} {short}", fields[0]);
            shorter += usize::from(short != path);
        }
    }
    assert!(shorter > 100, "{shorter}");
}

/// `path` of `view` without the `value` of each IE and the `decoded` of each transfer on
/// its way, which a path may leave out. One that the path ends with stays, and so does
/// one before a member of the value that is named as a member of the IE or of the
/// transfer.
fn short(view: &serde_json::Value, path: &str) -> String {
    let segments: Vec<&str> = path.split('/').skip(1).collect();
    let mut short = String::new();
    for (at, segment) in segments.iter().enumerate() {
        // What has the segment, by the path so far.
        let holder = match at {
            0 => None,
            at => inspect::select(view, &format!("/{}", segments[..at].join("/")))
                .unwrap()
                .pop(),
        };
        let has = |member: &str| holder.is_some_and(|holder| holder.get(member).is_some());
        let own: &[&str] = match *segment {
            "value" if has("_raw_value") => &["id", "criticality", "value", "octets"],
            "extensionValue" if has("_raw_value") => {
                &["id", "criticality", "extensionValue", "octets"]
            }
            "decoded" if has("_raw_transfer") => &["decoded", "octets"],
            _ => &[],
        };
        let left_out = !own.is_empty()
            && (segments.get(at + 1))
                .is_some_and(|next| !own.contains(next) && !next.starts_with('_'));
        if !left_out {
            short.push('/');
            short.push_str(segment);
        }
    }
    short
}

#[test]
fn an_ie_that_is_there_twice_goes_by_its_position() {
    use serde_json::json;
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT MME_UE_S1AP_ID(2u32),
    )
    .unwrap();
    let mut view = pdu.to_view().unwrap();
    let paths = inspect::paths(&view);
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
    let both = inspect::select(&view, "/s1ap/MME-UE-S1AP-ID/value").unwrap();
    assert_eq!(both, [&json!(1), &json!(2)]);
    inspect::set(&mut view, "/s1ap/2/value", json!(3)).unwrap();
    let edited = S1AP_PDU::from_view(&view).unwrap();
    let view = edited.to_view().unwrap();
    let both = inspect::select(&view, "/s1ap/MME-UE-S1AP-ID/value").unwrap();
    assert_eq!(both, [&json!(1), &json!(3)]);
}

#[test]
fn a_value_set_at_a_path_changes_the_octets_of_its_ie_alone() {
    use serde_json::json;
    let mut view = release_with_cause().to_view().unwrap();
    let octets = |view: &serde_json::Value| -> Vec<serde_json::Value> {
        let octets = inspect::select(view, "/s1ap/*/octets").unwrap();
        octets.into_iter().cloned().collect()
    };
    let before = octets(&view);
    inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", json!(9)).unwrap();
    let cause = json!("Release Due To EUTRAN Generated Reason");
    inspect::set(&mut view, "/s1ap/Cause/value/radioNetwork", cause).unwrap();
    let view = S1AP_PDU::from_view(&view).unwrap().to_view().unwrap();
    let after = octets(&view);
    assert_eq!(after[0], before[0]);
    assert_eq!(after[1], json!("0009"));
    assert_ne!(after[2], before[2]);
    let cause = inspect::select(&view, "/s1ap/Cause/value/radioNetwork").unwrap();
    assert_eq!(cause, [&json!("release-due-to-eutran-generated-reason")]);
}

#[test]
fn ies_are_added_taken_out_and_given_their_octets_at_a_path() {
    use serde_json::json;
    let mut view = release_with_cause().to_view().unwrap();
    // An IE by the name of its identifier, with its typed value, at the end.
    let name = json!({"id": "eNBname", "criticality": "ignore", "value": "enb-1"});
    inspect::insert(&mut view, "/s1ap/-", name).unwrap();
    // Given octets as an IE, before the cause.
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "C0FFEE"});
    inspect::insert(&mut view, "/s1ap/Cause", unknown).unwrap();
    inspect::remove(&mut view, "/s1ap/MME-UE-S1AP-ID").unwrap();
    inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/octets", json!("400102")).unwrap();
    let view = S1AP_PDU::from_view(&view).unwrap().to_view().unwrap();
    let ids: Vec<_> = (inspect::select(&view, "/s1ap/*/id").unwrap().into_iter())
        .map(|id| id.as_u64().unwrap())
        .collect();
    assert_eq!(ids, [8, 60000, 2, 60]);
    let select = |path: &str| inspect::select(&view, path).unwrap();
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
    let mut view = pdu.to_view().unwrap();
    let list = "/s1ap/E-RABToBeSetupListBearerSUReq/value";
    let by_name = format!("{list}/E-RABToBeSetupItemBearerSUReq/value/e-RAB-ID");
    let by_position = format!("{list}/0/value/e-RAB-ID");
    assert_eq!(inspect::select(&view, &by_name).unwrap(), [&json!(0)]);
    inspect::set(&mut view, &by_name, json!(5)).unwrap();
    let view = S1AP_PDU::from_view(&view).unwrap().to_view().unwrap();
    assert_eq!(inspect::select(&view, &by_position).unwrap(), [&json!(5)]);
    // The IEs of a message that a view contains are selected in its place: no S1AP
    // type has one, so this is a view of that shape.
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
    let mut view = release_with_cause().to_view().unwrap();
    let before = view.clone();
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
            "\"misspelled\" is not a member of a view, which has procedure_code, direction, \
             criticality, message",
        ),
        ("/s1ap/+0/id", "\"+0\" is not an IE of S1AP"),
        ("/s1ap/00/id", "\"00\" is not an IE of S1AP"),
        ("s1ap/Cause", "a path starts with \"/\""),
        ("/s1ap/@id=70000", "is not the identifier of an IE"),
    ] {
        let error = inspect::select(&view, path).unwrap_err().to_string();
        assert!(error.contains(reason), "{path}: {error}");
    }
    for (path, reason) in [
        ("/s1ap/NoSuchIE/value", "is not an IE of S1AP"),
        ("/s1ap/GUMMEI-ID/value", "an edit changes what is there"),
        ("/s1ap/7/value", "an edit changes what is there"),
    ] {
        let error = inspect::set(&mut view, path, json!(1))
            .unwrap_err()
            .to_string();
        assert!(error.contains(reason), "{path}: {error}");
    }
    assert!(inspect::remove(&mut view, "/s1ap/GUMMEI-ID").is_err());
    let both = json!({"id": "Cause", "criticality": "ignore", "value": 1, "octets": "00"});
    assert!(inspect::insert(&mut view, "/s1ap/-", both).is_err());
    let unnamed = json!({"id": "NoSuchIE", "criticality": "ignore", "octets": "00"});
    assert!(inspect::insert(&mut view, "/s1ap/-", unnamed).is_err());
    assert_eq!(view, before);
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
    // The view of a message that is written from its name.
    let (direction, procedure_code) = inspect::message_named("ue context release request").unwrap();
    let mut view = json!({
        "direction": direction,
        "procedure_code": procedure_code,
        "criticality": "ignore",
        "message": {"protocolIEs": []},
    });
    inspect::insert(
        &mut view,
        "/s1ap/-",
        json!({"id": "eNB-UE-S1AP-ID", "criticality": "reject", "value": 7}),
    )
    .unwrap();
    let pdu = S1AP_PDU::from_view(&view).unwrap();
    assert_eq!(inspect::message_name(&pdu), Some("UEContextReleaseRequest"));
    assert!(pdu.is_initiating());
}

#[test]
fn an_edit_changes_what_is_sent_or_is_refused() {
    use serde_json::{Value, json};
    let mut view = release_request(7).to_view().unwrap();
    let before = view.clone();
    // The value of an IE is not taken out: its octets would go out as they came.
    for edit in [
        inspect::remove(&mut view, "/s1ap/eNB-UE-S1AP-ID/value"),
        inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", Value::Null),
    ] {
        let error = edit.unwrap_err().to_string();
        assert!(error.contains("value is not taken out"), "{error}");
    }
    assert_eq!(view, before);
    // `*` is each member of a value, without what the view keeps of what came.
    let members = inspect::select(&view, "/s1ap/eNB-UE-S1AP-ID/*").unwrap();
    assert_eq!(members.len(), 3, "{members:?}");
    // The octets of an IE whose value was edited are no longer its octets.
    let octets = "/s1ap/eNB-UE-S1AP-ID/octets";
    let received = inspect::select(&view, octets).unwrap()[0].clone();
    inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", json!(9)).unwrap();
    let error = inspect::select(&view, octets).unwrap_err().to_string();
    assert!(error.contains("the value was edited"), "{error}");
    assert_eq!(
        inspect::select(&view, "/s1ap/MME-UE-S1AP-ID/octets")
            .unwrap()
            .len(),
        1
    );
    assert!(
        inspect::paths(&view)
            .iter()
            .all(|(path, _)| !path.contains("_edited"))
    );
    let sent = S1AP_PDU::from_view(&view).unwrap().to_view().unwrap();
    assert_ne!(inspect::select(&sent, octets).unwrap(), [&received]);
    assert_eq!(
        inspect::select(&sent, "/s1ap/eNB-UE-S1AP-ID/value").unwrap(),
        [&json!(9)]
    );
    // Octets that are set are those of the IE again.
    inspect::set(&mut view, octets, received.clone()).unwrap();
    assert_eq!(inspect::select(&view, octets).unwrap(), [&received]);
    assert_eq!(S1AP_PDU::from_view(&view).unwrap(), release_request(7));
    // The name of an ENUMERATED value of the PDU itself is taken as any other.
    inspect::set(&mut view, "/criticality", json!("Reject")).unwrap();
    let pdu = S1AP_PDU::from_view(&view).unwrap();
    assert_eq!(pdu.to_view().unwrap()["criticality"], "reject");
}

#[test]
fn null_takes_an_optional_member_out() {
    use serde_json::{Value, json};
    let pdu = S1AP_PDU::decode(&fixture("InitialContextSetupRequest")).unwrap();
    let view = pdu.to_view().unwrap();
    let sent = |view: &Value| S1AP_PDU::from_view(view)?.to_view();
    // An optional member that is there is taken out, and one that is not is not
    // selected.
    let nas = "/s1ap/E-RABToBeSetupListCtxtSUReq/value/0/value/nAS-PDU";
    let mut edited = view.clone();
    inspect::set(&mut edited, nas, json!("0102")).unwrap();
    let mut edited = sent(&edited).unwrap();
    assert_eq!(inspect::select(&edited, nas).unwrap(), [&json!("0102")]);
    inspect::set(&mut edited, nas, Value::Null).unwrap();
    let none = Vec::<&Value>::new();
    assert_eq!(inspect::select(&sent(&edited).unwrap(), nas).unwrap(), none);
    let error = inspect::set(&mut edited, nas, Value::Null)
        .unwrap_err()
        .to_string();
    assert!(error.contains("an edit changes what is there"), "{error}");
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
fn a_message_has_the_criticality_that_asn1_assigns() {
    let id = |name: &str| {
        let mut names = inspect::ie_names().iter();
        names.find(|(_, known)| *known == name).unwrap().0
    };
    let request = inspect::message_ie_criticalities("InitialContextSetupRequest").unwrap();
    assert_eq!(request[0], (0, "reject"));
    assert_eq!(request[0].0, id("MME-UE-S1AP-ID"));
    assert!(request.contains(&(id("TraceActivation"), "ignore")));
    // An IE has the criticality that the set of each message gives it.
    let response = inspect::message_ie_criticalities("InitialContextSetupResponse").unwrap();
    assert_eq!(response[0], (id("MME-UE-S1AP-ID"), "ignore"));
    // A name is taken as that of a message is, and a message without IEs has none.
    assert_eq!(
        inspect::message_ie_criticalities("initial-context-setup-request"),
        Some(request)
    );
    assert_eq!(
        inspect::message_ie_criticalities("PrivateMessage"),
        Some(&[][..])
    );
    assert_eq!(inspect::message_ie_criticalities("NoSuchMessage"), None);
    // The procedure has its own, which is that of each of its messages.
    let procedure = inspect::message_criticality;
    assert_eq!(procedure("InitialContextSetupRequest"), Some("reject"));
    assert_eq!(procedure("initial-context-setup-response"), Some("reject"));
    assert_eq!(procedure("Paging"), Some("ignore"));
    assert_eq!(procedure("ErrorIndication"), Some("ignore"));
    assert_eq!(procedure("NoSuchMessage"), None);
    // Every message has the IEs of its set in their order, each with a criticality.
    let known = ["reject", "ignore", "notify"];
    for (.., name) in inspect::message_names() {
        let ies = inspect::message_ies(name).unwrap().iter().map(|(ie, _)| ie);
        let assigned = inspect::message_ie_criticalities(name).unwrap();
        assert!(ies.eq(assigned.iter().map(|(ie, _)| ie)), "{name}");
        for (ie, criticality) in assigned {
            assert!(known.contains(criticality), "{name} {ie}");
        }
        assert!(known.contains(&procedure(name).unwrap()), "{name}");
    }
}

#[test]
fn the_messages_of_the_fixtures_have_the_criticality_that_asn1_assigns() {
    let mut ies = 0;
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = S1AP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        let view = pdu.to_view().unwrap();
        let procedure = inspect::message_criticality(fields[0]);
        assert_eq!(view["criticality"].as_str(), procedure, "{}", fields[0]);
        let assigned = inspect::message_ie_criticalities(fields[0]).unwrap();
        // A private message has no IEs of an object set.
        let entries = view["message"]["protocolIEs"].as_array();
        for ie in entries.into_iter().flatten() {
            let id = ie["id"].as_u64().unwrap();
            let mut assigned = assigned.iter().filter(|(known, _)| u64::from(*known) == id);
            let assigned = assigned.next().map(|(_, criticality)| *criticality);
            assert_eq!(ie["criticality"].as_str(), assigned, "{}", fields[0]);
            ies += 1;
        }
    }
    assert!(ies > 200, "{ies}");
}

/// The IEs that `value` holds, each as its identifier and its criticality: the entries
/// that have a `value`, which an extension has not.
fn held(value: &serde_json::Value, ies: &mut Vec<(u64, String)>) {
    use serde_json::Value;
    let entry = (value["id"].as_u64(), value["criticality"].as_str());
    if let (Some(id), Some(criticality)) = entry
        && !value["value"].is_null()
    {
        ies.push((id, criticality.into()));
    }
    match value {
        Value::Object(members) => members.values().for_each(|member| held(member, ies)),
        Value::Array(entries) => entries.iter().for_each(|entry| held(entry, ies)),
        _ => {}
    }
}

#[test]
fn an_ie_that_a_value_holds_alone_has_the_criticality_of_its_object_sets() {
    let id = |name: &str| {
        let mut names = inspect::ie_names().iter();
        names.find(|(_, known)| *known == name).unwrap().0
    };
    // The items of a list of E-RABs, in a request and in its response.
    let item = |name: &str| inspect::item_criticality(id(name));
    assert_eq!(inspect::item_criticality(52), Some("reject"));
    assert_eq!(item("E-RABToBeSetupItemCtxtSUReq"), Some("reject"));
    assert_eq!(item("E-RABSetupItemCtxtSURes"), Some("ignore"));
    // A list that a parameterized type makes of single containers, and the alternative
    // that extends a CHOICE.
    assert_eq!(item("E-RABToBeSetupItemHOReq"), Some("reject"));
    assert_eq!(item("LoggedMBSFNMDT"), Some("ignore"));
    // RESET has this item with `reject` and RESET ACKNOWLEDGE with `ignore`.
    assert_eq!(item("UE-associatedLogicalS1-ConnectionItem"), None);
    // The IEs of a message are in no single container.
    assert_eq!(item("MME-UE-S1AP-ID"), None);
    assert_eq!(inspect::item_criticality(u16::MAX), None);
    // The items of the fixtures have the criticality of their sets.
    let mut items = Vec::new();
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = S1AP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        let view = pdu.to_view().unwrap();
        let entries = view["message"]["protocolIEs"].as_array();
        for ie in entries.into_iter().flatten() {
            held(&ie["value"], &mut items);
        }
    }
    assert!(items.len() > 10, "{}", items.len());
    for (id, criticality) in items {
        let assigned = inspect::item_criticality(id.try_into().unwrap());
        assert_eq!(assigned, Some(criticality.as_str()), "{id}");
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
    // The procedures of the list: `<code> <Name> <criticality> {`.
    let mut procedures = 0;
    for line in include_str!("../src/registry.rs").lines() {
        let words: Vec<_> = line.split_whitespace().take(4).collect();
        let [code, name, _, "{"] = words[..] else {
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
        oxirush_s1ap::Octets::from(vec![0]),
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
    let view = release_with_cause().to_view().unwrap();
    let none = Vec::<&Value>::new();
    let select = |path: &str| inspect::select(&view, path);
    // The alternative of a CHOICE that is there, and another one.
    let cause = json!("user-inactivity");
    assert_eq!(select("/s1ap/Cause/value/radioNetwork").unwrap(), [&cause]);
    assert_eq!(select("/s1ap/Cause/value/nas").unwrap(), none);
    // What follows an IE or a member that is not there is still a path of its type.
    assert_eq!(select("/s1ap/GUMMEI-ID/value/mME-Code").unwrap(), none);
    let error = select("/s1ap/GUMMEI-ID/value/misspelled")
        .unwrap_err()
        .to_string();
    let members = "is not a member of GUMMEI, which has pLMN-Identity, mME-Group-ID, mME-Code, \
                   iE-Extensions";
    assert!(error.contains(members), "{error}");
    let error = select("/s1ap/Cause/value/nas/deeper")
        .unwrap_err()
        .to_string();
    assert!(error.contains("which has none"), "{error}");
    // After `*`, a member is an error when no value can have it.
    assert_eq!(select("/s1ap/*/value/radioNetwork").unwrap(), [&cause]);
    assert_eq!(select("/s1ap/*/value/*").unwrap(), [&cause]);
    let error = select("/s1ap/*/value/misspelled").unwrap_err().to_string();
    assert!(error.contains("is not a member of"), "{error}");
    // An optional member that is not there, and a list of values that has no IEs to name.
    let pdu = S1AP_PDU::decode(&fixture("InitialContextSetupRequest")).unwrap();
    let setup = pdu.to_view().unwrap();
    let item = "/s1ap/E-RABToBeSetupListCtxtSUReq/value/E-RABToBeSetupItemCtxtSUReq/value";
    let nas = inspect::select(&setup, &format!("{item}/nAS-PDU")).unwrap();
    assert_eq!(nas, none);
    let error = inspect::select(&setup, &format!("{item}/nas-pdu-typo"))
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("is not a member of ERABToBeSetupItemCtxtSUReq, which has e-RAB-ID"),
        "{error}"
    );
    let pdu = S1AP_PDU::decode(&fixture("S1SetupRequest")).unwrap();
    let setup = pdu.to_view().unwrap();
    let error = inspect::select(&setup, "/s1ap/SupportedTAs/value/Cause/value")
        .unwrap_err()
        .to_string();
    assert!(error.contains("go by position"), "{error}");
}

#[test]
fn a_procedure_that_the_specification_does_not_have_has_a_view_of_its_ies() {
    use oxirush_s1ap::ErrorKind;
    use serde_json::json;
    // The message of a UE CONTEXT RELEASE REQUEST, under a procedure code that the
    // specification does not have: the second octet of a PDU.
    let mut octets = release_request(7).encode().unwrap();
    octets[1] = 200;
    let pdu = S1AP_PDU::decode(&octets).unwrap();
    assert_eq!(pdu.procedure_code(), 200);
    let mut view = pdu.to_view().unwrap();
    assert_eq!(view["procedure_code"], 200);
    assert_eq!(view["direction"], "InitiatingMessage");
    // Each IE is read with the type of its identifier, and has its name.
    let id = inspect::select(&view, "/s1ap/eNB-UE-S1AP-ID/value").unwrap();
    assert_eq!(id, [&json!(7)]);
    let paths = inspect::paths(&view);
    assert!(
        paths.contains(&("/s1ap/MME-UE-S1AP-ID/value".to_string(), json!(1))),
        "{paths:?}"
    );
    // The view is the PDU, and an edit of it is sent under the same procedure code.
    assert_eq!(pdu.with_view(&view).unwrap(), pdu);
    assert_eq!(
        S1AP_PDU::from_view(&view).unwrap().encode().unwrap(),
        octets
    );
    inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", json!(9)).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    assert_eq!(edited.procedure_code(), 200);
    let again = edited.to_view().unwrap();
    let id = inspect::select(&again, "/s1ap/eNB-UE-S1AP-ID/value").unwrap();
    assert_eq!(id, [&json!(9)]);
    // A view that is written by hand, without what a view keeps of what was received.
    let written = json!({
        "procedure_code": 200, "direction": "InitiatingMessage", "criticality": "ignore",
        "message": {"protocolIEs": [{"id": 8, "criticality": "reject", "value": 9}]},
    });
    let written = S1AP_PDU::from_view(&written).unwrap();
    assert_eq!(written.protocol_ies().unwrap().0.len(), 1);
    // It has no tree, which names its message.
    assert_eq!(pdu.to_tree().unwrap_err().kind(), ErrorKind::UnknownMessage);
    // A message that is no container of protocol IEs does not decode, and has no view:
    // one IE is claimed, and none is there.
    let pdu = S1AP_PDU::decode(&[0x00, 200, 0x00, 0x03, 0x00, 0x00, 0x01]).unwrap();
    assert!(pdu.to_view().is_err());
}

#[test]
fn an_edit_that_fails_after_it_changed_an_ie_leaves_the_view_as_it_was() {
    use serde_json::json;
    // UE-S1AP-IDs is IE 99, a CHOICE that has no `radioNetwork`, and Cause IE 2, which
    // has it.
    let mut view = sent(&[
        json!({"id": 99, "criticality": "ignore", "value": {"mME-UE-S1AP-ID": 1}}),
        json!({"id": 2, "criticality": "ignore", "value": {"radioNetwork": "user-inactivity"}}),
    ])
    .unwrap();
    let before = view.clone();
    // The entries that `*` selects are edited from the last: the cause takes the edit,
    // and the identifiers, before it, refuse the path.
    let everywhere = "/s1ap/*/value/radioNetwork";
    let error = inspect::set(&mut view, everywhere, json!("unspecified"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("is not a member of UES1APIDs"), "{error}");
    assert_eq!(view, before);
    assert!(inspect::remove(&mut view, everywhere).is_err());
    assert_eq!(view, before);
    // The cause alone takes it.
    inspect::set(
        &mut view,
        "/s1ap/Cause/value/radioNetwork",
        json!("unspecified"),
    )
    .unwrap();
    let cause = inspect::select(&view, "/s1ap/Cause/value/radioNetwork").unwrap();
    assert_eq!(cause, [&json!("unspecified")]);
    // An entry that is taken out before another refuses the path is put back where it
    // was: `0` then names the identifiers again.
    let mut view = before.clone();
    let both = view["message"]["protocolIEs"].as_array().unwrap().clone();
    let entries = view["message"]["protocolIEs"].as_array_mut().unwrap();
    entries.extend(both);
    let before = view.clone();
    assert!(inspect::remove(&mut view, "/s1ap/*/Cause").is_err());
    assert_eq!(view, before);
}

#[test]
fn an_edit_names_a_member_that_the_type_has_and_keeps_what_it_always_has() {
    use serde_json::{Value, json};
    let mut view = release_with_cause().to_view().unwrap();
    let before = view.clone();
    let error = inspect::set(&mut view, "/s1ap/Cause/value/misspelled", json!(1))
        .unwrap_err()
        .to_string();
    assert!(error.contains("is not a member of Cause"), "{error}");
    let error = inspect::remove(&mut view, "/s1ap/Cause/value/radioNetwork")
        .unwrap_err()
        .to_string();
    assert!(error.contains("a CHOICE has one alternative"), "{error}");
    let error = inspect::remove(&mut view, "/s1ap/Cause/criticality")
        .unwrap_err()
        .to_string();
    assert!(error.contains("its type always has it"), "{error}");
    assert_eq!(view, before);
    // An alternative takes the place of the one that is there.
    inspect::set(&mut view, "/s1ap/Cause/value/nas", json!("detach")).unwrap();
    let sent = S1AP_PDU::from_view(&view).unwrap().to_view().unwrap();
    let cause = inspect::select(&sent, "/s1ap/Cause/value").unwrap();
    assert_eq!(cause, [&json!({"nas": "detach"})]);
    // A member that the type always has stays, and an optional one is added by its name.
    let pdu = S1AP_PDU::decode(&fixture("InitialContextSetupRequest")).unwrap();
    let mut setup = pdu.to_view().unwrap();
    let item = "/s1ap/E-RABToBeSetupListCtxtSUReq/value/0/value";
    let error = inspect::remove(&mut setup, &format!("{item}/e-RAB-ID"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("its type always has it"), "{error}");
    let error = inspect::set(&mut setup, &format!("{item}/e-RAB-ID"), Value::Null)
        .unwrap_err()
        .to_string();
    assert!(error.contains("its type always has it"), "{error}");
    inspect::set(&mut setup, &format!("{item}/nas_pdu"), json!("0102")).unwrap();
    let sent = S1AP_PDU::from_view(&setup).unwrap().to_view().unwrap();
    let nas = inspect::select(&sent, &format!("{item}/nAS-PDU")).unwrap();
    assert_eq!(nas, [&json!("0102")]);
}

#[test]
fn a_name_is_its_letters_and_its_digits() {
    use serde_json::json;
    let view = release_with_cause().to_view().unwrap();
    for path in [
        "/s1ap/eNB-UE-S1AP-ID/value",
        "/S1AP/enb_ue_s1ap_id/Value",
        "/S1ap/enbues1apid/value",
        "/s1ap/ENB UE S1AP ID/value",
        "/message/protocol-ies/1/value",
        "/Message/ProtocolIEs/1/VALUE",
    ] {
        assert_eq!(inspect::select(&view, path).unwrap(), [&json!(7)], "{path}");
    }
    for path in [
        "/s1ap/cause/value/radionetwork",
        "/s1ap/Cause/value/Radio-Network",
    ] {
        let cause = inspect::select(&view, path).unwrap();
        assert_eq!(cause, [&json!("user-inactivity")], "{path}");
    }
    assert_eq!(
        inspect::select(&view, "/Procedure-Code").unwrap(),
        inspect::select(&view, "/procedure_code").unwrap()
    );
    // A member that a view keeps of what was received is named as it is.
    assert!(inspect::select(&view, "/s1ap/Cause/_raw_value").is_ok());
    assert!(inspect::select(&view, "/s1ap/Cause/rawvalue").is_err());
    // An edit writes the member by the name that ASN.1 gives it.
    let mut edited = view.clone();
    inspect::set(&mut edited, "/s1ap/cause/value/NAS", json!("detach")).unwrap();
    let cause = inspect::select(&edited, "/s1ap/Cause/value").unwrap();
    assert_eq!(cause, [&json!({"nas": "detach"})]);
    assert!(S1AP_PDU::from_view(&edited).is_ok());
    assert_eq!(
        inspect::message_named("ue context-release_REQUEST"),
        Some(("InitiatingMessage", 18))
    );
}

#[test]
fn a_path_is_checked_against_a_message_without_a_view() {
    // Every path of every message is one that its message can have.
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let wire = hex::decode(line.split('\t').nth(1).unwrap()).unwrap();
        let pdu = S1AP_PDU::decode(&wire).unwrap();
        let name = inspect::message_name(&pdu).unwrap();
        let view = pdu.to_view().unwrap();
        for (path, _) in inspect::paths(&view) {
            let checked = inspect::check_path(name, &path);
            assert!(checked.is_ok(), "{name} {path}: {checked:?}");
            // And so is the path that leaves out what it may.
            let short = short(&view, &path);
            let checked = inspect::check_path(name, &short);
            assert!(checked.is_ok(), "{name} {short}: {checked:?}");
        }
    }
    let item = "/s1ap/E-RABToBeSetupListCtxtSUReq/value/E-RABToBeSetupItemCtxtSUReq";
    let short_item = "/s1ap/E-RABToBeSetupListCtxtSUReq/E-RABToBeSetupItemCtxtSUReq";
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
        // The value of an IE may be left out.
        ("UEContextReleaseRequest", "/s1ap/Cause/nas"),
        ("UEContextReleaseRequest", "/s1ap/*/radioNetwork"),
        ("InitialContextSetupRequest", "/s1ap/GUMMEI-ID/mME-Code"),
        (
            "InitialContextSetupRequest",
            &format!("{short_item}/e-RABlevelQoSParameters/qCI"),
        ),
        (
            "InitialContextSetupRequest",
            "/s1ap/E-RABToBeSetupListCtxtSUReq/0/e-RAB-ID",
        ),
        (
            "InitialContextSetupRequest",
            "/s1ap/E-RABToBeSetupListCtxtSUReq/-",
        ),
        // A segment under an IE whose type is not known may be a member of its value.
        ("UEContextReleaseRequest", "/s1ap/@id=60000/anything"),
        (
            "InitialContextSetupRequest",
            "/s1ap/E-RABToBeSetupListCtxtSUReq/0/anything",
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
        // A name that neither an IE nor its value has: the members of both.
        (
            "UEContextReleaseRequest",
            "/s1ap/Cause/misspelled",
            "is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, \
             at /s1ap/Cause/misspelled; an IE itself has id, criticality, value, octets",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/eNB-UE-S1AP-ID/valeu",
            "is not a member of this value, which has none, at /s1ap/eNB-UE-S1AP-ID/valeu; an \
             IE itself has id, criticality, value, octets",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/Cause/radioNetwork/deeper",
            "which has none",
        ),
        (
            "InitialContextSetupRequest",
            &format!("{short_item}/misspelled"),
            "is not a member of ERABToBeSetupItemCtxtSUReq, which has e-RAB-ID",
        ),
        (
            "InitialContextSetupRequest",
            "/s1ap/E-RABToBeSetupListCtxtSUReq/misspelled",
            "is not a position in a list",
        ),
        (
            "InitialContextSetupRequest",
            "/s1ap/E-RABToBeSetupListCtxtSUReq/Cause/misspelled",
            "is not a member of Cause",
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
        let error = inspect::check_path(message, path).unwrap_err().to_string();
        assert!(error.contains(reason), "{message} {path}: {error}");
    }
}

#[test]
fn an_ie_is_written_by_its_name_and_its_octets_wherever_a_view_has_ies() {
    use serde_json::json;
    let pdu = S1AP_PDU::decode(&fixture("InitialContextSetupRequest")).unwrap();
    let view = pdu.to_view().unwrap();
    let list = "/s1ap/E-RABToBeSetupListCtxtSUReq/value";
    let sent = |view: &serde_json::Value| S1AP_PDU::from_view(view)?.to_view();
    let item = inspect::select(&view, &format!("{list}/0/value")).unwrap()[0].clone();
    let named = json!({
        "id": "E-RABToBeSetupItemCtxtSUReq", "criticality": "reject", "value": item,
    });
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "00"});
    // The items of a list of IEs: in a value that is set whole, and at the end of the
    // list, on a path that has the root and on one that has not.
    let mut edited = view.clone();
    inspect::set(&mut edited, list, json!([named, unknown])).unwrap();
    let after = sent(&edited).unwrap();
    let id = format!("{list}/E-RABToBeSetupItemCtxtSUReq/value/e-RAB-ID");
    assert_eq!(
        inspect::select(&after, &id).unwrap(),
        inspect::select(&view, &id).unwrap()
    );
    let octets = format!("{list}/@id=60000/octets");
    assert_eq!(inspect::select(&after, &octets).unwrap(), [&json!("00")]);
    let mut edited = view.clone();
    let position = (view["message"]["protocolIEs"].as_array().unwrap().iter())
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
    // In a view written by hand.
    let mut edited = view.clone();
    let ies = edited["message"]["protocolIEs"].as_array_mut().unwrap();
    ies[0]["id"] = json!("mme-ue-s1ap-id");
    ies.push(json!({"id": "eNBname", "criticality": "ignore", "octets": "0461"}));
    let after = sent(&edited).unwrap();
    assert_eq!(after["message"]["protocolIEs"][0]["id"], json!(0));
    let name = inspect::select(&after, "/s1ap/eNBname/octets").unwrap();
    assert_eq!(name, [&json!("0461")]);
    // The octets of an IE that was added are set at its path too.
    let mut edited = view.clone();
    let name = json!({"id": "eNBname", "criticality": "ignore", "value": "a"});
    inspect::insert(&mut edited, "/s1ap/-", name).unwrap();
    inspect::set(&mut edited, "/s1ap/eNBname/octets", json!("0461")).unwrap();
    let after = sent(&edited).unwrap();
    let name = inspect::select(&after, "/s1ap/eNBname/octets").unwrap();
    assert_eq!(name, [&json!("0461")]);
    // A name that no IE has is refused, and so are a value and octets together.
    let mut edited = view.clone();
    edited["message"]["protocolIEs"][0]["id"] = json!("NoSuchIE");
    let error = S1AP_PDU::from_view(&edited).unwrap_err().to_string();
    assert!(error.contains("is not an IE of S1AP"), "{error}");
    let mut edited = view.clone();
    edited["message"]["protocolIEs"][0]["octets"] = json!("00");
    let error = S1AP_PDU::from_view(&edited).unwrap_err().to_string();
    assert!(
        error.contains("its value or its octets, not both"),
        "{error}"
    );
}

#[test]
fn the_octets_of_an_ie_without_a_value_have_a_path() {
    use serde_json::json;
    let mut view = release_with_cause().to_view().unwrap();
    // The octets repeat a value that is there.
    let listed = |view: &serde_json::Value, path: &str| {
        let paths = inspect::paths(view);
        paths.iter().any(|(listed, _)| listed == path)
    };
    assert!(!listed(&view, "/s1ap/Cause/octets"));
    inspect::set(&mut view, "/s1ap/Cause/octets", json!("0240")).unwrap();
    assert!(listed(&view, "/s1ap/Cause/octets"));
    assert!(!listed(&view, "/s1ap/Cause/value"));
    for (path, value) in inspect::paths(&view) {
        assert_eq!(inspect::select(&view, &path).unwrap(), [&value], "{path}");
    }
}

#[test]
fn a_private_message_has_no_ies_under_the_root() {
    let pdu = S1AP_PDU::decode(&fixture("PrivateMessage")).unwrap();
    let view = pdu.to_view().unwrap();
    let error = inspect::select(&view, "/s1ap/Cause")
        .unwrap_err()
        .to_string();
    let reason = "a message PrivateMessage has no IEs under /s1ap: it has privateIEs";
    assert!(error.contains(reason), "{error}");
    assert!(!error.contains("protocolIEs"), "{error}");
    let error = inspect::check_path("PrivateMessage", "/s1ap/Cause")
        .unwrap_err()
        .to_string();
    assert!(error.contains(reason), "{error}");
    assert!(inspect::select(&view, "/message/privateIEs/0/id/local").is_ok());
    assert!(inspect::check_path("PrivateMessage", "/message/privateIEs/0/id/global").is_ok());
}

#[test]
fn the_identifier_that_the_octets_were_decoded_as_is_a_number() {
    use serde_json::json;
    let mut view = release_with_cause().to_view().unwrap();
    inspect::set(&mut view, "/s1ap/Cause/value/nas", json!("detach")).unwrap();
    ie(&mut view, 2)["_original_id"] = json!("two");
    let error = S1AP_PDU::from_view(&view).unwrap_err().to_string();
    assert!(error.contains("_original_id is the identifier"), "{error}");
}

/// A report of the data that a secondary RAT carried, with this count for the uplink.
fn usage_report(count: u64) -> serde_json::Value {
    serde_json::json!({"message": "SecondaryRATDataUsageReport", "ies": [
        {"MME-UE-S1AP-ID": 1},
        {"eNB-UE-S1AP-ID": 2},
        {"SecondaryRATDataUsageReportList": [{"SecondaryRATDataUsageReportItem": {
            "e-RAB-ID": 5,
            "secondaryRATType": "nR",
            "e-RABUsageReportList": [{"E-RABUsageReportItem": {
                "startTimestamp": "00000000",
                "endTimestamp": "00000001",
                "usageCountUL": count,
                "usageCountDL": 1,
            }}],
        }}]},
    ]})
}

/// An INTEGER is a number of any size (ITU-T X.697 (02/2021) §21). One of 2^63 or more
/// is written, read, edited and encoded as any other, and the bindings read it in the
/// octets.
#[test]
fn an_integer_of_two_to_the_63_or_more_is_a_number() {
    use serde_json::json;
    let report = "/s1ap/SecondaryRATDataUsageReportList/0/e-RABUsageReportList/0";
    let count = format!("{report}/usageCountUL");

    // The octets of the largest count of 63 bits, which has eight octets as these have.
    let below = S1AP_PDU::from_tree(&usage_report((1 << 63) - 1)).unwrap();
    let below = hex::encode(below.encode().unwrap());
    assert_eq!(below.matches("7fffffffffffffff").count(), 1);

    for (number, octets) in [
        (1u64 << 63, "8000000000000000"),
        (u64::MAX, "ffffffffffffffff"),
    ] {
        // Written, it is the eight octets of the count in the same message.
        let pdu = S1AP_PDU::from_tree(&usage_report(number)).unwrap();
        let wire = pdu.encode().unwrap();
        assert_eq!(
            hex::encode(&wire),
            below.replace("7fffffffffffffff", octets)
        );

        // Received, it is the number that it is, and the bindings read the same.
        let mut view = S1AP_PDU::decode(&wire).unwrap().to_view().unwrap();
        let list = &view["message"]["protocolIEs"][2]["value"][0]["value"]["e-RABUsageReportList"];
        let entry = list[0].clone();
        assert!(entry.get("_decode_error").is_none(), "{entry}");
        assert_eq!(inspect::select(&view, &count).unwrap(), [&json!(number)]);
        let item = hex::decode(entry["_raw_value"].as_str().unwrap()).unwrap();
        let item: ERABUsageReportItem = decode_open_type(&item).unwrap();
        assert_eq!(item.usage_count_ul, number);

        // A tree has it, and both forms are the same octets.
        assert_eq!(pdu.to_tree().unwrap(), usage_report(number));
        assert_eq!(S1AP_PDU::from_view(&view).unwrap().encode().unwrap(), wire);

        // Edited, it is the tree of the other number.
        inspect::set(&mut view, &count, json!(number - 1)).unwrap();
        let edited = S1AP_PDU::from_view(&view).unwrap();
        assert_eq!(edited.to_tree().unwrap(), usage_report(number - 1));
    }

    // One more than the type has is no number that JSON has, and its decimal digits
    // are no number: it is refused, where it is.
    let mut beyond = usage_report(1);
    let item = &mut beyond["ies"][2]["SecondaryRATDataUsageReportList"][0];
    let entry = &mut item["SecondaryRATDataUsageReportItem"]["e-RABUsageReportList"][0];
    entry["E-RABUsageReportItem"]["usageCountUL"] = json!("18446744073709551616");
    let error = S1AP_PDU::from_tree(&beyond).unwrap_err();
    assert_eq!(error.kind(), oxirush_s1ap::ErrorKind::Value);
    assert_eq!(
        error.to_string(),
        "this is not a value of E-RABUsageReportItem, at usageCountUL: a number is expected, \
         and \"18446744073709551616\" is written"
    );
}

/// The bindings hold an INTEGER in 64 bits. An IE with a number that a later release
/// makes larger than that is not read with its type: a view and a tree have the octets
/// that were received, with why they do not decode, and write them back as they are.
#[test]
fn an_ie_with_an_integer_beyond_64_bits_keeps_the_octets_received() {
    use serde_json::json;
    // The count of the paging attempts has an extension marker: INTEGER (1..16, ...).
    let attempts = PagingAttemptInformation::new(
        PagingAttemptCount(u64::MAX),
        IntendedNumberOfPagingAttempts(1),
        None,
        None,
    );
    let largest = AssistanceDataForPaging::new(None, None, Some(attempts), None);
    let largest = encode_open_type(&largest).unwrap();
    // The octets of the largest count that the bindings hold, with the nine octets of
    // 2^64 in the place of its own nine.
    let nine = [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let at = largest
        .windows(9)
        .position(|octets| octets == nine)
        .unwrap();
    assert_eq!(largest[at - 1], 9);
    let mut beyond = largest.to_vec();
    beyond[at..at + 9].copy_from_slice(&[0x01, 0, 0, 0, 0, 0, 0, 0, 0]);
    let error = decode_open_type::<AssistanceDataForPaging>(&beyond).unwrap_err();
    assert_eq!(
        error.to_string(),
        "these octets do not decode as AssistanceDataForPaging: the number does not fit in 64 \
         bits, in its field `paging_attempt_information.paging_attempt_count`"
    );

    // A PAGING with these octets as its assistance data.
    let mut pdu = S1AP_PDU::from_tree(&json!({"message": "Paging", "ies": [
        {"UEIdentityIndexValue": "0A40"},
        {"UEPagingID": {"s-TMSI": {"mMEC": 1, "m-TMSI": 1}}},
        {"CNDomain": "ps"},
        {"TAIList": [{"TAIItem": {"tAI": {"pLMNidentity": "208-93", "tAC": 1}}}]},
        {"AssistanceDataForPaging": {"pagingAttemptInformation": {
            "pagingAttemptCount": 1,
            "intendedNumberOfPagingAttempts": 1,
        }}},
    ]}))
    .unwrap();
    let mut ies = pdu.protocol_ies().unwrap().into_owned();
    ies.0[4].value = beyond.clone().into();
    pdu.set_protocol_ies(ies).unwrap();
    let wire = pdu.encode().unwrap();

    // The IE is the octets that were received, and the others are read.
    let view = S1AP_PDU::decode(&wire).unwrap().to_view().unwrap();
    let entry = &view["message"]["protocolIEs"][4];
    assert_eq!(entry["_raw_value"], json!(hex::encode_upper(&beyond)));
    assert_eq!(entry["value"], entry["_raw_value"]);
    let why = entry["_decode_error"].as_str().unwrap();
    assert!(why.contains("the number does not fit in 64 bits"), "{why}");
    assert_eq!(view["message"]["protocolIEs"][2]["value"], json!("ps"));
    // The view and the tree give the octets back.
    assert_eq!(S1AP_PDU::from_view(&view).unwrap().encode().unwrap(), wire);
    let tree = pdu.to_tree().unwrap();
    assert_eq!(S1AP_PDU::from_tree(&tree).unwrap().encode().unwrap(), wire);
}

/// A request to modify a UE context, with UE security capabilities whose encryption
/// algorithms are this many bits.
fn modification(bits: usize) -> S1AP_PDU {
    // The first two bits of a string of this length.
    let mask = |length: usize| {
        let mut octets = vec![0; length.div_ceil(8)];
        octets[0] = 0xc0;
        oxirush_s1ap::BitString::new(octets, length).unwrap()
    };
    let capabilities = UESecurityCapabilities::new(
        EncryptionAlgorithms(mask(bits)),
        IntegrityProtectionAlgorithms(mask(16)),
        None,
    );
    build_s1ap!(InitiatingMessage, UEContextModification,
        REJECT, UEContextModificationRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(1u32),
        REJECT UESecurityCapabilities(capabilities),
    )
    .unwrap()
}

/// A BIT STRING whose type gives one size and an extension marker has its length when
/// it is of another size (ITU-T X.697 (02/2021) §24.3), and is its octets alone when it
/// is of that size, as it always was. Both are read, edited and written: the octets
/// are those that the bindings encode.
#[test]
fn a_bit_string_of_another_size_than_its_type_gives_has_its_length() {
    use serde_json::json;
    let capabilities = "/s1ap/UESecurityCapabilities";
    let algorithms = format!("{capabilities}/encryptionAlgorithms");
    let message = "UEContextModificationRequest";

    // The size that the type gives: hexadecimal, read and edited.
    let mut view = modification(16).to_view().unwrap();
    assert_eq!(
        inspect::select(&view, &algorithms).unwrap(),
        [&json!("C000")]
    );
    inspect::set(&mut view, &algorithms, json!("8000")).unwrap();
    let edited = S1AP_PDU::from_view(&view).unwrap().to_view().unwrap();
    assert_eq!(
        inspect::select(&edited, &algorithms).unwrap(),
        [&json!("8000")]
    );

    // 20 bits and 24 bits are the same three octets, and not the same string.
    for bits in [20, 24] {
        let pdu = modification(bits);
        let wire = pdu.encode().unwrap();
        let mut view = S1AP_PDU::decode(&wire).unwrap().to_view().unwrap();
        let entry = &view["message"]["protocolIEs"][2];
        assert!(entry.get("_decode_error").is_none(), "{entry}");
        let shown = json!({"value": "C00000", "length": bits});
        assert_eq!(inspect::select(&view, &algorithms).unwrap(), [&shown]);

        // Its two members have paths, which a message is checked against.
        let length = format!("{algorithms}/length");
        assert_eq!(inspect::select(&view, &length).unwrap(), [&json!(bits)]);
        let listed = format!("{capabilities}/value/encryptionAlgorithms/length");
        let paths = inspect::paths(&view);
        assert!(paths.iter().any(|(path, _)| *path == listed), "{paths:?}");
        assert_eq!(inspect::check_path(message, &length), Ok(()));

        // A tree has it, and both forms are the octets that the bindings encoded.
        let tree = pdu.to_tree().unwrap();
        let written = &tree["ies"][2]["UESecurityCapabilities"];
        assert_eq!(written["encryptionAlgorithms"], shown);
        assert_eq!(S1AP_PDU::from_tree(&tree).unwrap().encode().unwrap(), wire);
        assert_eq!(S1AP_PDU::from_view(&view).unwrap().encode().unwrap(), wire);

        // Edited to the size of the type, it is what the bindings encode for that.
        inspect::set(&mut view, &algorithms, json!("C000")).unwrap();
        assert_eq!(S1AP_PDU::from_view(&view).unwrap(), modification(16));
    }

    // From the size of the type to another one, by the whole value and by its length.
    let mut view = modification(16).to_view().unwrap();
    let longer = json!({"value": "C00000", "length": 24});
    inspect::set(&mut view, &algorithms, longer).unwrap();
    assert_eq!(S1AP_PDU::from_view(&view).unwrap(), modification(24));
    inspect::set(&mut view, &format!("{algorithms}/length"), json!(20)).unwrap();
    assert_eq!(S1AP_PDU::from_view(&view).unwrap(), modification(20));

    // What is not the string that it says is refused: three octets without a length
    // are not the 16 bits of the type, and 20 bits are not two octets.
    let written = |algorithms: serde_json::Value| {
        S1AP_PDU::from_tree(&json!({"message": message, "ies": [
            {"MME-UE-S1AP-ID": 1},
            {"eNB-UE-S1AP-ID": 1},
            {"UESecurityCapabilities": {
                "encryptionAlgorithms": algorithms,
                "integrityProtectionAlgorithms": "C000",
            }},
        ]}))
    };
    assert!(written(json!("C000")).is_ok());
    let error = written(json!("C00000")).unwrap_err();
    assert_eq!(error.kind(), oxirush_s1ap::ErrorKind::Value);
    assert_eq!(
        error.to_string(),
        "this is not a value of UESecurityCapabilities, at encryptionAlgorithms: 3 octets are \
         written for a bit string of 16 bits, which has 2"
    );
    assert!(written(json!({"value": "C000", "length": 20})).is_err());
}
