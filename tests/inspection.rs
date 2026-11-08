#![cfg(feature = "inspect")]
//! The trees of the PDUs of S1AP: those of the fixtures and the paths of each of their
//! values, the forms that are S1AP's, and its registry. TOY, the protocol of the tests of
//! oxirush-aper, has the tests of what `to_tree`, `from_tree` and the paths do with the
//! tree of any PDU.

use oxirush_s1ap::tree;
use oxirush_s1ap::tree::Value;
use oxirush_s1ap::{S1ap, inspect, s1ap::*};

/// The name that ASN.1 gives the message of a PDU.
fn name_of(pdu: &S1APPDU<'_>) -> Option<&'static str> {
    S1ap::object_set(pdu.direction(), pdu.procedure_code()).map(|set| set.name)
}

/// The octets of the message `name` of the fixtures.
fn fixture(name: &str) -> Vec<u8> {
    let mut lines = include_str!("fixtures/messages.tsv").lines();
    let line = lines.find(|line| line.split('\t').next() == Some(name));
    hex::decode(line.unwrap().split('\t').nth(1).unwrap()).unwrap()
}

/// The messages of the fixtures: the name of each, and its octets.
fn fixtures() -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let lines = include_str!("fixtures/messages.tsv").lines();
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let fields: Vec<_> = line.split('\t').collect();
        (fields[0], hex::decode(fields[1]).unwrap())
    })
}

/// The tree of the PDU that `tree` writes, which is written and decoded again.
fn sent(tree: &Value) -> Result<Value, oxirush_s1ap::Error> {
    Ok(S1APPDU::from_tree(tree)?.to_tree())
}

/// The octets of each IE of the message of a PDU, in their order.
fn octets_of(pdu: &S1APPDU<'_>) -> Vec<Vec<u8>> {
    let ies = pdu.protocol_ies().unwrap();
    ies.0.iter().map(|ie| ie.value.to_vec()).collect()
}

/// The values at `path` in `tree`.
fn select(tree: &Value, path: &str) -> Vec<Value> {
    let selected = inspect::select(tree, path).unwrap();
    selected.into_iter().cloned().collect()
}

/// The value of the IE `name` of a tree, the first one of that name.
fn ie<'t>(tree: &'t mut Value, name: &str) -> &'t mut Value {
    let ies = tree["ies"].as_array_mut().unwrap();
    let entry = ies.iter_mut().find(|ie| ie.get(name).is_some()).unwrap();
    &mut entry[name]
}

#[test]
fn every_message_of_the_fixtures_is_its_octets_through_its_tree() {
    for (name, wire) in fixtures() {
        let pdu = S1APPDU::decode(&wire).unwrap();
        let shown = pdu.to_tree();
        assert_eq!(shown["message"], tree!(name));
        // A private message has no IEs of the protocol to name: its octets.
        if name == "PrivateMessage" {
            assert!(shown["octets"].is_string() && shown.get("ies").is_none());
        } else {
            assert!(shown["ies"].is_array(), "{name}");
        }
        let written = S1APPDU::from_tree(&shown).unwrap();
        assert_eq!(written.encode().unwrap(), wire, "{name}: {shown}");
        assert_eq!(written, pdu, "{name}");
    }
}

/// The members of `value`, a tree or a part of one, that only repeat what its form says:
/// those of an IE of another form, and a `criticality` that the tables have.
fn redundant(value: &Value, found: &mut Vec<String>) {
    match value {
        Value::Object(members) => {
            for (name, member) in members {
                let wraps = match name.as_str() {
                    "id" | "extensionValue" | "decoded" | "criticality" => true,
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
fn the_messages_of_the_fixtures_are_shown_and_written_by_name_to_any_depth() {
    let shown = |wire: &[u8]| S1APPDU::decode(wire).unwrap().to_tree();
    let written = |tree: &Value| S1APPDU::from_tree(tree).map(|pdu| pdu.encode().unwrap());
    let wire = fixture("E-RABSetupRequest");
    let bearer = tree!({
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
    let request = |bearer: &Value| {
        tree!({
            "message": "E-RABSetupRequest",
            "ies": [
                {"MME-UE-S1AP-ID": 0},
                {"eNB-UE-S1AP-ID": 0},
                {"E-RABToBeSetupListBearerSUReq": [bearer]},
            ],
        })
    };
    let by_name = request(&tree!({"E-RABToBeSetupItemBearerSUReq": &bearer}));
    assert_eq!(shown(&wire), by_name);
    assert_eq!(written(&by_name).unwrap(), wire);
    let mut found = Vec::new();
    for (_, wire) in fixtures() {
        let shown = shown(&wire);
        if shown.get("ies").is_some() {
            redundant(&shown["ies"], &mut found);
        }
    }
    assert_eq!(found, Vec::<String>::new());
    // An item whose criticality two messages assign otherwise says its own.
    let reset = |connection: &Value| {
        tree!({"message": "Reset", "ies": [
            {"Cause": {"misc": "unspecified"}},
            {"ResetType": {"partOfS1-Interface": [connection]}},
        ]})
    };
    let mut connection = tree!({"UE-associatedLogicalS1-ConnectionItem": {"mME-UE-S1AP-ID": 1}});
    let error = written(&reset(&connection)).unwrap_err().to_string();
    assert!(error.contains("Item says its \"criticality\""), "{error}");
    connection["criticality"] = tree!("reject");
    let pdu = S1APPDU::from_tree(&reset(&connection)).unwrap();
    assert_eq!(pdu.to_tree(), reset(&connection));
}

/// Each value of `shown`, a tree or a part of one, with the JSON pointer of the tree that
/// joins the names and the positions on the way to it. A list of plain values is one
/// value.
fn joined(shown: &Value, path: &str, found: &mut Vec<(String, Value)>) {
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
fn the_names_and_the_positions_of_a_tree_are_the_path_of_each_value() {
    let mut values = 0;
    for (name, wire) in fixtures() {
        let tree = S1APPDU::decode(&wire).unwrap().to_tree();
        let Some(Value::Array(ies)) = tree.get("ies") else {
            continue;
        };
        for (index, ie) in ies.iter().enumerate() {
            let mut found = Vec::new();
            joined(ie, &format!("/s1ap/{index}"), &mut found);
            for (path, value) in found {
                let at = format!("{name} {path}");
                assert_eq!(select(&tree, &path), std::slice::from_ref(&value), "{at}");
                assert_eq!(inspect::check_path(name, &path), Ok(()), "{at}");
                let named = path.replacen(&format!("/{index}/"), "/", 1);
                let selected = inspect::select(&tree, &named).expect(&at);
                assert!(selected.contains(&&value), "{at}");
                values += 1;
            }
        }
    }
    assert!(values > 300, "{values}");
}

#[test]
fn well_known_values_are_shown_and_taken_as_they_are_usually_written() {
    let wire = fixture("S1SetupRequest");
    let mut tree = S1APPDU::decode(&wire).unwrap().to_tree();
    let area = "/s1ap/SupportedTAs/0";
    assert_eq!(select(&tree, &format!("{area}/tAC")), [tree!(0x9302)]);
    assert_eq!(
        select(&tree, &format!("{area}/broadcastPLMNs")),
        [tree!(["208-93"])]
    );
    assert_eq!(
        select(&tree, "/s1ap/Global-ENB-ID/pLMNidentity"),
        [tree!("208-93")]
    );
    assert_eq!(S1APPDU::from_tree(&tree).unwrap().encode().unwrap(), wire);
    // Edited as they are shown, a number in hexadecimal too.
    let before = S1APPDU::decode(&wire).unwrap();
    inspect::set(&mut tree, &format!("{area}/tAC"), tree!("0x7")).unwrap();
    let networks = tree!(["001-01", "310-410"]);
    inspect::set(
        &mut tree,
        &format!("{area}/broadcastPLMNs"),
        networks.clone(),
    )
    .unwrap();
    let edited = S1APPDU::from_tree(&tree).unwrap();
    let shown = edited.to_tree();
    assert_eq!(select(&shown, &format!("{area}/tAC")), [tree!(7)]);
    assert_eq!(
        select(&shown, &format!("{area}/broadcastPLMNs")),
        [networks]
    );
    assert_eq!(octets_of(&edited)[0], octets_of(&before)[0]);
    // As JER writes them, after 0x where they are digits alone: the octets that TS 36.413
    // clause 9.2.3.8 has for 001-01 and for 310-410.
    fn areas(tree: &mut Value) -> &mut Value {
        &mut ie(tree, "SupportedTAs")[0]
    }
    areas(&mut tree)["tAC"] = tree!("0x0007");
    areas(&mut tree)["broadcastPLMNs"] = tree!(["00f110", "0x134001"]);
    assert_eq!(
        S1APPDU::from_tree(&tree).unwrap().encode().unwrap(),
        edited.encode().unwrap()
    );
    // Digits alone are refused: read in two ways, with both.
    areas(&mut tree)["tAC"] = tree!("0010");
    let error = S1APPDU::from_tree(&tree).unwrap_err().to_string();
    assert!(
        error.contains("the number 10 ") && error.contains("the number 16"),
        "{error}"
    );
    areas(&mut tree)["tAC"] = tree!("0007");
    let error = S1APPDU::from_tree(&tree).unwrap_err().to_string();
    assert!(
        error.contains("\"0007\" is a number in quotes: write 7 without quotes"),
        "{error}"
    );
    areas(&mut tree)["tAC"] = tree!(7);
    areas(&mut tree)["broadcastPLMNs"] = tree!(["00f110", "134001"]);
    let error = S1APPDU::from_tree(&tree).unwrap_err().to_string();
    assert!(
        error.contains("134-001") && error.contains("310-410"),
        "{error}"
    );
    // Neither form, and a number that the type has no room for.
    for refused in [tree!("7"), tree!(1 << 16), tree!("0x10000")] {
        areas(&mut tree)["broadcastPLMNs"] = tree!(["208-93"]);
        areas(&mut tree)["tAC"] = refused.clone();
        assert!(S1APPDU::from_tree(&tree).is_err(), "{refused}");
    }
    areas(&mut tree)["tAC"] = tree!(7);
    areas(&mut tree)["broadcastPLMNs"] = tree!(["208-9"]);
    assert!(S1APPDU::from_tree(&tree).is_err());
}

#[test]
fn the_parts_of_a_gummei_are_numbers() {
    let wire = fixture("S1SetupResponse");
    let mut tree = S1APPDU::decode(&wire).unwrap().to_tree();
    let served = &mut ie(&mut tree, "ServedGUMMEIs")[0];
    assert_eq!(served["servedPLMNs"], tree!(["208-93"]));
    assert_eq!(served["servedGroupIDs"], tree!([0xB8A3]));
    assert_eq!(served["servedMMECs"], tree!([0x89]));
    assert_eq!(S1APPDU::from_tree(&tree).unwrap().encode().unwrap(), wire);
    let served = &mut ie(&mut tree, "ServedGUMMEIs")[0];
    served["servedGroupIDs"] = tree!([2, "0x8000"]);
    served["servedMMECs"] = tree!([1]);
    let mut shown = sent(&tree).unwrap();
    let served = &ie(&mut shown, "ServedGUMMEIs")[0];
    assert_eq!(served["servedGroupIDs"], tree!([2, 0x8000]));
    assert_eq!(served["servedMMECs"], tree!([1]));
    // An MME code has one octet.
    ie(&mut tree, "ServedGUMMEIs")[0]["servedMMECs"] = tree!([256]);
    assert!(S1APPDU::from_tree(&tree).is_err());
}

#[test]
fn a_tunnel_endpoint_and_the_identities_of_a_ue_are_readable() {
    let added = |ie: &str, value: Value| {
        S1APPDU::from_tree(&tree!({"message": "UEContextReleaseRequest", "ies": [
            {(ie): value, "criticality": "ignore"},
        ]}))
    };
    let octets = |pdu: &S1APPDU<'_>| octets_of(pdu).remove(0);
    // An item of a list of E-RABs.
    let bearer = tree!({"e-RAB-ID": 5, "transportLayerAddress": "10.0.0.2", "gTP-TEID": 2});
    let pdu = added(
        "E-RABSetupListCtxtSURes",
        tree!([{"E-RABSetupItemCtxtSURes": &bearer}]),
    )
    .unwrap();
    let expected = ERABSetupItemCtxtSURes {
        e_rab_id: ERABID(5),
        transport_layer_address: TransportLayerAddress(oxirush_s1ap::BitString::from([
            10, 0, 0, 2,
        ])),
        gtp_teid: GTPTEID::from([0, 0, 0, 2]),
        ie_extensions: None,
    };
    let held = octets(&pdu);
    let list: ERABSetupListCtxtSURes = oxirush_s1ap::decode(&held).unwrap();
    assert_eq!(
        list.0[0].value.to_vec(),
        oxirush_s1ap::encode(&expected).unwrap()
    );
    let mut shown = pdu.to_tree();
    assert_eq!(
        ie(&mut shown, "E-RABSetupListCtxtSURes")[0]["E-RABSetupItemCtxtSURes"],
        bearer
    );
    // A paging identity: an IMSI by its digits, or an S-TMSI by its numbers.
    let pdu = added("UEPagingID", tree!({"iMSI": "208930000000001"})).unwrap();
    let imsi = IMSI(vec![0x02, 0x98, 0x03, 0x00, 0x00, 0x00, 0x00, 0xF1].into());
    let expected = oxirush_s1ap::encode(&UEPagingID::iMSI(imsi)).unwrap();
    assert_eq!(octets(&pdu), expected);
    assert_eq!(
        ie(&mut pdu.to_tree(), "UEPagingID")["iMSI"],
        tree!("208930000000001")
    );
    // As JER writes it, with the filler that says it is not digits.
    let same = added("UEPagingID", tree!({"iMSI": "02980300000000f1"})).unwrap();
    assert_eq!(octets(&same), expected);
    let temporary = tree!({"s-TMSI": {"mMEC": 1, "m-TMSI": "0xC0FFEE01"}});
    let pdu = added("UEPagingID", temporary).unwrap();
    let identity = &ie(&mut pdu.to_tree(), "UEPagingID")["s-TMSI"].clone();
    assert_eq!(identity, &tree!({"mMEC": 1, "m-TMSI": 0xC0FFEE01u32}));
}

#[test]
fn every_path_of_every_message_selects_its_value_and_is_one_that_its_message_can_have() {
    for (name, wire) in fixtures() {
        let tree = S1APPDU::decode(&wire).unwrap().to_tree();
        for (path, value) in inspect::paths(&tree) {
            assert!(!path.contains("protocolIEs"), "{name} {path}");
            assert_eq!(select(&tree, &path), [value], "{name} {path}");
            let checked = inspect::check_path(name, &path);
            assert!(checked.is_ok(), "{name} {path}: {checked:?}");
        }
    }
}

#[test]
fn the_messages_of_the_fixtures_go_by_the_names_that_asn1_gives_them() {
    for (name, wire) in fixtures() {
        let pdu = S1APPDU::decode(&wire).unwrap();
        assert_eq!(name_of(&pdu), Some(name));
    }
}

#[test]
fn the_messages_of_the_fixtures_have_the_criticality_that_asn1_assigns() {
    let mut ies = 0;
    for (name, wire) in fixtures() {
        let pdu = S1APPDU::decode(&wire).unwrap();
        let set = S1ap::object_set_named(name).unwrap();
        assert_eq!(pdu.criticality(), set.criticality, "{name}");
        let tree = pdu.to_tree();
        assert!(tree.get("criticality").is_none(), "{name}");
        let assigned = set.ies.unwrap_or_default();
        let Ok(listed) = pdu.protocol_ies() else {
            continue;
        };
        for ie in &listed.0 {
            let mut assigned = assigned.iter().filter(|(known, ..)| *known == ie.id.0);
            let assigned = assigned.next().map(|(_, criticality, _)| *criticality);
            assert_eq!(Some(ie.criticality), assigned, "{name}");
            ies += 1;
        }
    }
    assert!(ies > 200, "{ies}");
}

#[test]
fn every_pdu_is_its_octets_through_its_tree_whatever_its_ies_hold() {
    let mut octets = 0;
    let mut same = |wire: &[u8]| {
        let Ok(pdu) = S1APPDU::decode(wire) else {
            return;
        };
        let tree = pdu.to_tree();
        assert_eq!(S1APPDU::from_tree(&tree).as_ref(), Ok(&pdu), "{tree}");
        octets += tree.to_string().matches("\"octets\"").count();
    };
    for file in [
        include_str!("fixtures/messages.tsv"),
        include_str!("fixtures/vectors.tsv"),
        include_str!("fixtures/captured.tsv"),
        include_str!("fixtures/later.tsv"),
        include_str!("fixtures/fuzzed.tsv"),
    ] {
        let lines = file
            .lines()
            .filter(|line| !line.starts_with('#') && !line.is_empty());
        for line in lines {
            let wire = hex::decode(line.split('\t').nth(1).unwrap()).unwrap();
            same(&wire);
            let Ok(pdu) = S1APPDU::decode(&wire) else {
                continue;
            };
            let Ok(ies) = pdu.protocol_ies() else {
                continue;
            };
            let ies = ies.into_owned();
            for at in 0..ies.0.len() {
                let mut changed = ies.clone();
                let mut value = changed.0[at].value.to_vec();
                let Some(last) = value.last_mut() else {
                    continue;
                };
                *last ^= 1;
                changed.0[at].value = oxirush_s1ap::Octets::owned(value);
                let mut pdu = pdu.clone();
                pdu.set_protocol_ies(changed).unwrap();
                same(&pdu.encode().unwrap());
            }
        }
    }
    assert!(octets > 50, "{octets}");
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
    for (id, name) in S1ap::ie_names() {
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

/// The handover container of NGAP that the IE may carry is written by the name of
/// its type once the program registers the tables of oxirush-ngap: before, the name is
/// refused with the call that registers them. No test of this file registers them.
#[test]
fn a_container_of_the_other_protocol_says_the_call_that_registers_its_tables() {
    let tree = oxirush_s1ap::tree!({"message": "HandoverRequired", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"Source-ToTarget-TransparentContainer": {"SourceNGRANNode-ToTargetNGRANNode-TransparentContainer": {}}},
    ]});
    let error = S1APPDU::from_tree(&tree).unwrap_err().to_string();
    let call = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer is a type of NGAP: call oxirush_ngap::inspect::register()";
    assert!(error.contains(call), "{error}");
    let path = "/s1ap/Source-ToTarget-TransparentContainer/SourceNGRANNode-ToTargetNGRANNode-TransparentContainer/rRC";
    let error = inspect::check_path("HandoverRequired", path)
        .unwrap_err()
        .to_string();
    assert!(error.contains(call), "{error}");
}
