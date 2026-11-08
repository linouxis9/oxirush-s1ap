#![cfg(feature = "inspect")]
//! The tree of a PDU and its paths: what `to_tree` shows, what `from_tree` writes, and what
//! `select`, `set`, `remove`, `insert`, `open`, `check_path` and `enumerated_at` do with it.

use oxirush_s1ap::tree;
use oxirush_s1ap::tree::Value;
use oxirush_s1ap::{Criticality, Direction, Presence, S1ap, build_s1ap, inspect, s1ap::*};

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

/// The tree of a UE CONTEXT RELEASE REQUEST of the IEs `ies`, written and decoded again.
fn released(ies: Value) -> Result<Value, oxirush_s1ap::Error> {
    sent(&tree!({"message": "UEContextReleaseRequest", "ies": ies}))
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

/// A UE CONTEXT RELEASE REQUEST: two UE S1AP IDs and a cause.
fn release_request() -> S1APPDU<'static> {
    build_s1ap!(IGNORE UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )
    .unwrap()
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

#[test]
fn an_edit_changes_the_ies_that_it_changes_and_no_other() {
    // UE CONTEXT RELEASE REQUEST
    let wire = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1APPDU::decode(&wire).unwrap();
    let mut tree = pdu.to_tree();
    inspect::set(&mut tree, "/s1ap/Cause/radioNetwork", tree!("unspecified")).unwrap();
    inspect::remove(&mut tree, "/s1ap/MME-UE-S1AP-ID").unwrap();
    let edited = S1APPDU::from_tree(&tree).unwrap();
    assert_eq!(
        hex::encode(edited.encode().unwrap()),
        "0012400f000002000800020007000240020000"
    );
    assert_eq!(
        edited.to_tree(),
        tree!({"message": "UEContextReleaseRequest", "ies": [
            {"eNB-UE-S1AP-ID": 7},
            {"Cause": {"radioNetwork": "unspecified"}},
        ]})
    );
    // A value set at a path changes the octets of its IE alone.
    let mut tree = release_request().to_tree();
    inspect::set(&mut tree, "/s1ap/eNB-UE-S1AP-ID", tree!(9)).unwrap();
    inspect::set(
        &mut tree,
        "/s1ap/Cause/radioNetwork",
        tree!("Release Due To E-UTRAN Generated Reason"),
    )
    .unwrap();
    let edited = S1APPDU::from_tree(&tree).unwrap();
    let (before, after) = (octets_of(&release_request()), octets_of(&edited));
    assert_eq!(after[0], before[0]);
    assert_eq!(after[1], [0, 9]);
    assert_ne!(after[2], before[2]);
    let cause = select(&edited.to_tree(), "/s1ap/Cause/radioNetwork");
    assert_eq!(cause, [tree!("release-due-to-eutran-generated-reason")]);
}

#[test]
fn repeated_ies_are_edited_one_by_one_in_their_order() {
    let pdu = build_s1ap!(IGNORE UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT MME_UE_S1AP_ID(2u32),
    )
    .unwrap();
    let mut tree = pdu.to_tree();
    assert_eq!(
        inspect::paths(&tree),
        [
            ("/s1ap/0".to_string(), tree!(1)),
            ("/s1ap/eNB-UE-S1AP-ID".to_string(), tree!(7)),
            ("/s1ap/2".to_string(), tree!(2)),
        ]
    );
    assert_eq!(select(&tree, "/s1ap/MME-UE-S1AP-ID"), [tree!(1), tree!(2)]);
    inspect::set(&mut tree, "/s1ap/2", tree!(3)).unwrap();
    let edited = S1APPDU::from_tree(&tree).unwrap();
    let tree = edited.to_tree();
    assert_eq!(select(&tree, "/s1ap/MME-UE-S1AP-ID"), [tree!(1), tree!(3)]);
    let (before, after) = (octets_of(&pdu), octets_of(&edited));
    assert_eq!(&before[..2], &after[..2]);
}

#[test]
fn an_ie_whose_value_has_no_place_for_its_octets_is_its_octets() {
    // Independent complete warning request from unknown_extensions.rs. The first TAI has
    // a future SEQUENCE addition.
    let wire = hex::decode(
        "00240031000005006f00028000007000028000007140122000018002f83900010101000002f839000200720002000a007300020001",
    )
    .unwrap();
    let pdu = S1APPDU::decode(&wire).unwrap();
    let mut tree = pdu.to_tree();
    assert_eq!(S1APPDU::from_tree(&tree).unwrap().encode().unwrap(), wire);
    let ies = tree["ies"].as_array().unwrap();
    let octets = |entry: &Value| {
        entry
            .as_object()
            .unwrap()
            .values()
            .next()
            .unwrap()
            .get("octets")
            .is_some()
    };
    assert!(octets(&ies[2]), "{tree}");
    // A typed edit cannot drop the addition: there is no value to edit.
    let path = "/s1ap/2/trackingAreaListforWarning/0/tAC";
    let error = inspect::set(&mut tree, path, tree!(3)).unwrap_err();
    assert_eq!(error.kind(), oxirush_s1ap::ErrorKind::UndecodableIe);
}

#[test]
fn an_ie_is_written_by_its_name_with_its_value_or_its_octets() {
    let tree = released(tree!([
        {"MME-UE-S1AP-ID": 1},
        {"eNB-UE-S1AP-ID": {"octets": "0009"}},
        {"60000": {"octets": "C0FFEE"}, "criticality": "ignore"},
    ]))
    .unwrap();
    assert_eq!(tree["ies"][1], tree!({"eNB-UE-S1AP-ID": 9}));
    assert_eq!(
        tree["ies"][2],
        tree!({"60000": {"octets": "c0ffee"}, "criticality": "ignore"})
    );
    let with_cause = release_request().to_tree();
    for cause in ["Cause", "2", "cause"] {
        let written = released(tree!([
            {"MME-UE-S1AP-ID": 1},
            {"eNB-UE-S1AP-ID": 7},
            {(cause): {"radioNetwork": "user-inactivity"}},
        ]));
        assert_eq!(written.unwrap(), with_cause, "{cause}");
    }
    let cause = tree!({"radioNetwork": "no-such-cause"});
    assert!(released(tree!([{"Cause": cause}])).is_err());
    let error = released(tree!([{"60000": 5, "criticality": "ignore"}])).unwrap_err();
    assert!(error.to_string().contains("as its octets"), "{error}");
    // An ENUMERATED by its name and an OCTET STRING by its octets, in either case.
    let typed = build_s1ap!(IGNORE UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        IGNORE RRC_Establishment_Cause(RRCEstablishmentCause::mo_Signalling),
        REJECT NAS_PDU(vec![0x07, 0x6a]),
    )
    .unwrap();
    for octets in ["076a", "076A"] {
        let written = S1APPDU::from_tree(&tree!({"message": "UEContextReleaseRequest", "ies": [
            {"MME-UE-S1AP-ID": 1},
            {"RRC-Establishment-Cause": "mo-Signalling", "criticality": "ignore"},
            {"NAS-PDU": octets, "criticality": "reject"},
        ]}));
        assert_eq!(written.unwrap(), typed, "{octets}");
    }
}

#[test]
fn an_ie_that_does_not_decode_is_its_octets() {
    let request = UEContextReleaseRequest {
        protocol_ies: ProtocolIEContainer(vec![ProtocolIEField {
            id: ProtocolIEID(60000),
            criticality: Criticality::Ignore,
            value: vec![0xC0, 0xFF, 0xEE].into(),
        }]),
    };
    let pdu = S1APPDU::initiatingMessage(InitiatingMessage {
        procedure_code: ProcedureCode(18),
        criticality: Criticality::Ignore,
        value: oxirush_s1ap::encode(&request).unwrap().into(),
    });
    let mut tree = pdu.to_tree();
    let ie = tree!({"60000": {"octets": "c0ffee"}, "criticality": "ignore"});
    assert_eq!(tree["ies"][0], ie);
    assert_eq!(S1APPDU::from_tree(&tree).unwrap(), pdu);
    inspect::set(&mut tree, "/s1ap/0", tree!({"octets": "DECAFBAD"})).unwrap();
    let edited = S1APPDU::from_tree(&tree).unwrap().to_tree();
    assert_eq!(select(&edited, "/s1ap/0/octets"), [tree!("decafbad")]);
    inspect::set(&mut tree, "/s1ap/0", tree!(5)).unwrap();
    assert!(S1APPDU::from_tree(&tree).is_err());
}

#[test]
fn a_message_is_written_by_its_name_and_its_ies_by_theirs() {
    let request = tree!({
        "message": "ue context release request",
        "ies": [
            {"mme-ue-s1ap-id": 1},
            {"eNB-UE-S1AP-ID": 2},
            {"Cause": {"radioNetwork": "user-inactivity"}},
            {"65000": {"octets": "00"}, "criticality": "reject"},
        ],
    });
    let pdu = S1APPDU::from_tree(&request).unwrap();
    assert_eq!(name_of(&pdu), Some("UEContextReleaseRequest"));
    assert_eq!(pdu.criticality(), Criticality::Ignore);
    let ies = pdu.protocol_ies().unwrap();
    let criticalities: Vec<_> = ies.0.iter().map(|ie| ie.criticality.name()).collect();
    assert_eq!(criticalities, ["reject", "reject", "ignore", "reject"]);
    let shown = pdu.to_tree();
    assert_eq!(shown["ies"][0], tree!({"MME-UE-S1AP-ID": 1}));
    assert_eq!(
        shown["ies"][3],
        tree!({"65000": {"octets": "00"}, "criticality": "reject"})
    );
    let refused = |tree: Value| S1APPDU::from_tree(&tree).unwrap_err().to_string();
    let named = |message: &str, ie: &str| tree!({"message": message, "ies": [{ie: 1}]});
    assert!(
        refused(named("UEContextReleaseRequst", "MME-UE-S1AP-ID")).contains("is not a message")
    );
    assert!(refused(named("UEContextReleaseRequest", "MME-UE-ID")).contains("is not an IE"));
    assert!(
        refused(named("UEContextReleaseRequest", "CSG-Id")).contains("says its \"criticality\"")
    );
    assert!(refused(tree!({"message": "Paging", "ies": [{}]})).contains("one IE"));
    assert!(refused(tree!({"message": "Paging", "ie": []})).contains("is no member"));
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
fn a_message_is_shown_and_written_by_name_to_any_depth() {
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
    // An item of another form than a tree's is refused.
    let entry = tree!({"id": 17, "criticality": "reject", "value": &bearer});
    assert!(written(&request(&entry)).is_err());
    // An IE of an extension container says its criticality.
    let mut extended = bearer.clone();
    extended["iE-Extensions"] = tree!([{"BearerType": "non-IP", "criticality": "reject"}]);
    let extended = request(&tree!({"E-RABToBeSetupItemBearerSUReq": extended}));
    let pdu = S1APPDU::from_tree(&extended).unwrap();
    let tree = pdu.to_tree();
    let extension = "/s1ap/E-RABToBeSetupListBearerSUReq/0/iE-Extensions/BearerType";
    assert_eq!(select(&tree, extension), [tree!("non-IP")]);
    assert_eq!(
        select(&tree, &format!("{extension}/criticality")),
        [tree!("reject")]
    );
    assert_eq!(tree, extended);
    let mut unassigned = bearer.clone();
    unassigned["iE-Extensions"] = tree!([{"BearerType": "non-IP"}]);
    let unassigned = request(&tree!({"E-RABToBeSetupItemBearerSUReq": unassigned}));
    let error = written(&unassigned).unwrap_err().to_string();
    assert!(
        error.contains("BearerType says its \"criticality\""),
        "{error}"
    );
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
    // An IE that a value holds alone goes by its name after the member that holds it.
    let node = tree!({
        "global-ENB-ID": {"pLMNidentity": "208-93", "eNB-ID": {"macroENB-ID": "000010"}},
        "selected-TAI": {"pLMNidentity": "208-93", "tAC": 1},
    });
    let report = tree!({"rLFReportInformation": {"uE-RLF-Report-Container": "00"}});
    let transfer = tree!({"message": "ENBConfigurationTransfer", "ies": [
        {"SONConfigurationTransferECT": {
            "targeteNB-ID": &node,
            "sourceeNB-ID": &node,
            "sONInformation": {"sONInformation-Extension": {"SON-Information-Report": report}},
        }},
    ]});
    let pdu = S1APPDU::from_tree(&transfer).unwrap();
    let tree = pdu.to_tree();
    let held = "/s1ap/SONConfigurationTransferECT/sONInformation/sONInformation-Extension";
    assert!(select(&tree, &format!("{held}/criticality")).is_empty());
    let container =
        format!("{held}/SON-Information-Report/rLFReportInformation/uE-RLF-Report-Container");
    assert_eq!(select(&tree, &container), [tree!("00")]);
    assert_eq!(tree, transfer);
    let error = written(&request(&tree!({"a": 1, "b": 2})))
        .unwrap_err()
        .to_string();
    assert!(error.contains("one IE"), "{error}");
}

#[test]
fn the_items_of_a_list_go_by_their_names_or_their_positions() {
    let name = "E-RABSetupRequest";
    let tree = S1APPDU::decode(&fixture(name)).unwrap().to_tree();
    let list = "/s1ap/E-RABToBeSetupListBearerSUReq";
    let tunnel = select(&tree, &format!("{list}/0/gTP-TEID"));
    assert_eq!(tunnel, [tree!(2778534635u32)]);
    for path in [
        format!("{list}/0"),
        format!("{list}/0").to_lowercase(),
        format!("{list}/E-RABToBeSetupItemBearerSUReq"),
        format!("{list}/@id=17"),
        format!("{list}/0/E-RABToBeSetupItemBearerSUReq"),
    ] {
        let selected = select(&tree, &format!("{path}/gTP-TEID"));
        assert_eq!(selected, tunnel, "{path}");
        let checked = inspect::check_path(name, &format!("{path}/gTP-TEID"));
        assert_eq!(checked, Ok(()), "{path}");
    }
    // An item has the criticality that its object set assigns, which the tree leaves out.
    assert!(select(&tree, &format!("{list}/0/criticality")).is_empty());
    assert_eq!(select(&tree, &format!("{list}/*")).len(), 1);
    let capability = "e-RABlevelQoSParameters/allocationRetentionPriority/pre-emptionCapability";
    let item = format!("{list}/E-RABToBeSetupItemBearerSUReq");
    let names = inspect::enumerated_at(name, &format!("{item}/{capability}")).unwrap();
    assert!(names.is_some_and(|names| names.contains(&"may-trigger-pre-emption")));
    assert_eq!(
        inspect::enumerated_at(name, &format!("{list}/@id=17/{capability}")),
        Ok(names)
    );
    // Edits take the same paths: a member is set, an item is added and one taken out.
    let mut edited = tree.clone();
    let second = tree["ies"][2]["E-RABToBeSetupListBearerSUReq"][0].clone();
    inspect::set(&mut edited, &format!("{list}/0/e-RAB-ID"), tree!(5)).unwrap();
    inspect::insert(&mut edited, &format!("{list}/-"), second.clone()).unwrap();
    inspect::insert(&mut edited, &format!("{list}/0"), second).unwrap();
    inspect::set(&mut edited, &format!("{list}/0/nAS-PDU"), tree!("0760")).unwrap();
    inspect::remove(&mut edited, &format!("{list}/2")).unwrap();
    let after = sent(&edited).unwrap();
    let bearers = select(&after, &format!("{list}/*/e-RAB-ID"));
    assert_eq!(bearers, [tree!(0), tree!(5)]);
    assert_eq!(
        select(&after, &format!("{list}/0/nAS-PDU")),
        [tree!("0760")]
    );
    // A name that the value does not have is refused with its members.
    let before = edited.clone();
    for (path, reason) in [
        (
            format!("{list}/E-RABToBeSetupItemBearerSUReq/misspelled"),
            "is not a member of ERABToBeSetupItemBearerSUReq",
        ),
        (
            format!("{list}/misspelled"),
            "\"misspelled\" is not an IE of S1AP",
        ),
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
            inspect::set(&mut edited, &path, tree!(1))
                .unwrap_err()
                .to_string(),
            inspect::check_path(name, &path).unwrap_err().to_string(),
        ] {
            assert!(error.contains(reason), "{path}: {error}");
        }
    }
    assert_eq!(edited, before);
}

#[test]
fn the_container_of_a_handover_is_written_and_selected_by_the_type_that_it_carries() {
    // Between eNBs, the Source to Target Transparent Container of a HANDOVER REQUIRED
    // carries a Source eNB to Target eNB Transparent Container: TS 36.413 clause
    // 9.2.1.56 says so, and the ASN.1 has an OCTET STRING.
    let name = "SourceeNB-ToTargeteNB-TransparentContainer";
    let container = tree!({
        "rRC-Container": "0102",
        "targetCell-ID": {"pLMNidentity": "208-93", "cell-ID": 4660},
        "uE-HistoryInformation": [{"uTRAN-Cell": "00"}],
    });
    let required = |container: Value| {
        tree!({"message": "HandoverRequired", "ies": [
            {"MME-UE-S1AP-ID": 1},
            {"eNB-UE-S1AP-ID": 2},
            {"Source-ToTarget-TransparentContainer": container},
        ]})
    };
    let pdu = S1APPDU::from_tree(&required(tree!({name: &container}))).unwrap();
    // The octets are the APER encoding of the type, which the bindings read.
    let message: HandoverRequired = pdu.decode_value().unwrap();
    let octets: SourceToTargetTransparentContainer =
        oxirush_s1ap::decode(&message.protocol_ies.0[2].value).unwrap();
    let carried: SourceeNBToTargeteNBTransparentContainer =
        oxirush_s1ap::aper::decode(&octets.0).unwrap();
    assert_eq!(carried.rrc_container.0, [1, 2]);
    // Nothing says what the octets carry: a tree has them as they are.
    let mut tree = pdu.to_tree();
    let ie = "/s1ap/Source-ToTarget-TransparentContainer";
    let hexadecimal = tree!(hex::encode(&octets.0));
    assert_eq!(tree, required(hexadecimal.clone()));
    // A path names the type, and the octets are opened before it selects.
    let cell = format!("{ie}/{name}/targetCell-ID/cell-ID");
    assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));
    let error = inspect::select(&tree, &cell).unwrap_err().to_string();
    assert!(error.contains("open decodes them"), "{error}");
    inspect::open(&mut tree, &cell).unwrap();
    assert_eq!(select(&tree, &cell), [tree!(4660)]);
    assert_eq!(
        select(&tree, &format!("{ie}/{name}")),
        std::slice::from_ref(&container)
    );
    let loose = format!("{ie}/{}/rrc_container", name.to_lowercase());
    assert_eq!(select(&tree, &loose), [tree!("0102")]);
    let listed = format!("{ie}/{name}/rRC-Container");
    let paths = inspect::paths(&tree);
    assert!(paths.iter().any(|(path, _)| *path == listed), "{paths:?}");
    for (path, value) in &paths {
        assert_eq!(select(&tree, path), std::slice::from_ref(value), "{path}");
    }
    assert_eq!(S1APPDU::from_tree(&tree).unwrap(), pdu);
    // An edit goes into it.
    inspect::set(&mut tree, &cell, tree!(7)).unwrap();
    let mut after = sent(&tree).unwrap();
    inspect::open(&mut after, &cell).unwrap();
    assert_eq!(select(&after, &cell), [tree!(7)]);
    // Octets that are not of the type are opened as their octets, and sent as they came.
    let other = S1APPDU::from_tree(&required(tree!("ff"))).unwrap();
    let mut tree = other.to_tree();
    inspect::open(&mut tree, &cell).unwrap();
    assert_eq!(
        select(&tree, &format!("{ie}/{name}")),
        [tree!({"octets": "ff"})]
    );
    assert_eq!(S1APPDU::from_tree(&tree).unwrap(), other);
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
fn an_enumerated_value_is_named_whatever_its_case_and_separators() {
    let causes = |cause: &str, establishment: &str| {
        S1APPDU::from_tree(&tree!({"message": "UEContextReleaseRequest", "ies": [
            {"Cause": {"radioNetwork": cause}},
            {"RRC-Establishment-Cause": establishment, "criticality": "ignore"},
        ]}))
    };
    let exact = causes("user-inactivity", "mo-Signalling").unwrap();
    let loose = causes("User_Inactivity", "MO SIGNALLING").unwrap();
    assert_eq!(loose, exact);
    assert!(causes("mo-Signalling", "mo-Signalling").is_err());
    assert!(causes("user-inactivit", "mo-Signalling").is_err());
}

#[test]
fn the_ies_of_a_message_have_paths_by_their_names() {
    let tree = release_request().to_tree();
    let printed: Vec<_> = (inspect::paths(&tree).iter())
        .map(|(path, value)| format!("{path} = {value}"))
        .collect();
    assert_eq!(
        printed,
        [
            "/s1ap/MME-UE-S1AP-ID = 1",
            "/s1ap/eNB-UE-S1AP-ID = 7",
            "/s1ap/Cause/radioNetwork = \"user-inactivity\"",
        ]
    );
    for path in [
        "/s1ap/eNB-UE-S1AP-ID",
        "/s1ap/enb_ue_s1ap_id",
        "/s1ap/@id=8",
        "/s1ap/1",
        "/ies/1",
        "/ies/1/eNB-UE-S1AP-ID",
        "/S1AP/ENB UE S1AP ID",
    ] {
        assert_eq!(select(&tree, path), [tree!(7)], "{path}");
    }
    let shown = released(tree!([{"eNB-UE-S1AP-ID": 7, "criticality": "ignore"}])).unwrap();
    let criticality = (
        "/s1ap/eNB-UE-S1AP-ID/criticality".to_string(),
        tree!("ignore"),
    );
    assert!(inspect::paths(&shown).contains(&criticality));
    // An edit writes a member by the name that ASN.1 gives it.
    let mut edited = tree.clone();
    inspect::set(&mut edited, "/s1ap/cause/NAS", tree!("detach")).unwrap();
    assert_eq!(select(&edited, "/s1ap/Cause"), [tree!({"nas": "detach"})]);
    assert!(S1APPDU::from_tree(&edited).is_ok());
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
fn ies_are_added_and_taken_out_at_a_path() {
    let mut tree = release_request().to_tree();
    let name = tree!({"eNBname": "enb-1", "criticality": "ignore"});
    inspect::insert(&mut tree, "/s1ap/-", name).unwrap();
    let unknown = tree!({"60000": {"octets": "c0ffee"}, "criticality": "ignore"});
    inspect::insert(&mut tree, "/s1ap/Cause", unknown).unwrap();
    inspect::remove(&mut tree, "/s1ap/MME-UE-S1AP-ID").unwrap();
    inspect::set(
        &mut tree,
        "/s1ap/eNB-UE-S1AP-ID",
        tree!({"octets": "400102"}),
    )
    .unwrap();
    let pdu = S1APPDU::from_tree(&tree).unwrap();
    let ids: Vec<_> = (pdu.protocol_ies().unwrap().0.iter())
        .map(|ie| ie.id.0)
        .collect();
    assert_eq!(ids, [8, 60000, 2, 60]);
    let tree = pdu.to_tree();
    assert_eq!(select(&tree, "/s1ap/eNB-UE-S1AP-ID"), [tree!(258)]);
    assert_eq!(select(&tree, "/s1ap/eNBname"), [tree!("enb-1")]);
    assert_eq!(select(&tree, "/s1ap/@id=60000/octets"), [tree!("c0ffee")]);
    assert!(select(&tree, "/s1ap/MME-UE-S1AP-ID").is_empty());
}

#[test]
fn a_path_that_names_nothing_is_refused_and_changes_nothing() {
    let mut tree = release_request().to_tree();
    let before = tree.clone();
    for (path, reason) in [
        ("/s1ap/NoSuchIE", "\"NoSuchIE\" is not an IE of S1AP"),
        (
            "/s1ap/Cause/misspelled",
            "\"misspelled\" is not a member of Cause, which has radioNetwork, transport, nas, \
             protocol, misc",
        ),
        (
            "/s1ap/Cause/radioNetwork/deeper",
            "\"deeper\" is not a member of this value, which has none",
        ),
        ("/s1ap/+0", "\"+0\" is not an IE of S1AP"),
        ("s1ap/Cause", "a path starts with \"/\""),
        ("/s1ap/@id=70000", "is not the identifier of an IE"),
    ] {
        let error = inspect::select(&tree, path).unwrap_err().to_string();
        assert!(error.contains(reason), "{path}: {error}");
    }
    for (path, reason) in [
        ("/s1ap/NoSuchIE", "is not an IE of S1AP"),
        ("/s1ap/GWContextReleaseIndication", "at /s1ap/- to add it"),
        ("/s1ap/7", "the path selects nothing"),
    ] {
        let error = inspect::set(&mut tree, path, tree!(1))
            .unwrap_err()
            .to_string();
        assert!(error.contains(reason), "{path}: {error}");
    }
    assert!(inspect::remove(&mut tree, "/s1ap/GWContextReleaseIndication").is_err());
    assert_eq!(tree, before);
}

#[test]
fn a_message_goes_by_the_name_that_asn1_gives_it() {
    for (name, wire) in fixtures() {
        let pdu = S1APPDU::decode(&wire).unwrap();
        assert_eq!(name_of(&pdu), Some(name));
    }
    for set in S1ap::object_sets() {
        let found = Some((set.direction, set.procedure_code));
        let named = |name: &str| {
            S1ap::object_set_named(name).map(|set| (set.direction, set.procedure_code))
        };
        assert_eq!(named(set.name), found, "{}", set.name);
        assert_eq!(named(&set.name.to_lowercase()), found);
        assert_eq!(named(&set.name.replace('-', "")), found);
    }
    assert_eq!(S1ap::object_set_named("NoSuchMessage"), None);
    let mut tree = tree!({"message": "ue context release request", "ies": []});
    inspect::insert(&mut tree, "/s1ap/-", tree!({"eNB-UE-S1AP-ID": 7})).unwrap();
    let pdu = S1APPDU::from_tree(&tree).unwrap();
    assert_eq!(name_of(&pdu), Some("UEContextReleaseRequest"));
    assert_eq!(pdu.direction(), Direction::Initiating);
    inspect::set(&mut tree, "/criticality", tree!("Reject")).unwrap();
    let pdu = S1APPDU::from_tree(&tree).unwrap();
    assert_eq!(pdu.to_tree()["criticality"], tree!("reject"));
}

#[test]
fn null_takes_an_optional_member_out() {
    let pdu = S1APPDU::decode(&fixture("InitialContextSetupRequest"))
        .unwrap()
        .into_owned();
    let tree = pdu.to_tree();
    let nas = "/s1ap/E-RABToBeSetupListCtxtSUReq/0/nAS-PDU";
    let mut edited = tree.clone();
    inspect::set(&mut edited, nas, tree!("0102")).unwrap();
    let mut edited = sent(&edited).unwrap();
    assert_eq!(select(&edited, nas), [tree!("0102")]);
    inspect::set(&mut edited, nas, Value::Null).unwrap();
    assert!(select(&sent(&edited).unwrap(), nas).is_empty());
    let error = inspect::set(&mut edited, nas, Value::Null)
        .unwrap_err()
        .to_string();
    assert!(error.contains("the path selects nothing"), "{error}");
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
fn an_ie_that_a_value_holds_alone_has_the_criticality_of_its_object_sets() {
    let id = |name: &str| {
        let mut names = S1ap::ie_names().iter();
        names.find(|(_, known)| *known == name).unwrap().0
    };
    // The items of a list of E-RABs, in a request and in its response.
    let item = |name: &str| S1ap::item_criticality(id(name));
    assert_eq!(S1ap::item_criticality(52), Some(Criticality::Reject));
    assert_eq!(
        item("E-RABToBeSetupItemCtxtSUReq"),
        Some(Criticality::Reject)
    );
    assert_eq!(item("E-RABSetupItemCtxtSURes"), Some(Criticality::Ignore));
    // A list that a parameterized type makes of single containers, and the alternative
    // that extends a CHOICE.
    assert_eq!(item("E-RABToBeSetupItemHOReq"), Some(Criticality::Reject));
    assert_eq!(item("LoggedMBSFNMDT"), Some(Criticality::Ignore));
    // RESET has this item with `reject` and RESET ACKNOWLEDGE with `ignore`.
    assert_eq!(item("UE-associatedLogicalS1-ConnectionItem"), None);
    // The IEs of a message are in no single container.
    assert_eq!(item("MME-UE-S1AP-ID"), None);
    assert_eq!(S1ap::item_criticality(u16::MAX), None);
}

#[test]
fn a_member_that_is_absent_selects_nothing_and_one_that_its_type_has_not_is_an_error() {
    let tree = release_request().to_tree();
    let select = |path: &str| inspect::select(&tree, path);
    let cause = tree!("user-inactivity");
    assert_eq!(select("/s1ap/Cause/radioNetwork").unwrap(), [&cause]);
    assert!(select("/s1ap/Cause/nas").unwrap().is_empty());
    let error = select("/s1ap/Cause/nas/deeper").unwrap_err().to_string();
    assert!(error.contains("which has none"), "{error}");
    assert_eq!(select("/s1ap/*/radioNetwork").unwrap(), [&cause]);
    assert_eq!(select("/s1ap/Cause/*").unwrap(), [&cause]);
    let error = select("/s1ap/*/misspelled").unwrap_err().to_string();
    assert!(error.contains("is not a member of"), "{error}");
    // What follows an IE that is not there is still a path of its type.
    let setup = S1APPDU::decode(&fixture("E-RABSetupRequest"))
        .unwrap()
        .to_tree();
    let ambr = "/s1ap/uEaggregateMaximumBitrate";
    assert!(
        inspect::select(&setup, &format!("{ambr}/uEaggregateMaximumBitRateDL"))
            .unwrap()
            .is_empty()
    );
    let error = inspect::select(&setup, &format!("{ambr}/misspelled"))
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("is not a member of UEAggregateMaximumBitrate"),
        "{error}"
    );
}

#[test]
fn a_procedure_that_the_specification_does_not_have_has_a_tree_of_its_ies() {
    use oxirush_s1ap::ErrorKind;
    let mut octets = release_request().encode().unwrap();
    octets[1] = 200;
    let pdu = S1APPDU::decode(&octets).unwrap();
    assert_eq!(pdu.procedure_code(), 200);
    let mut tree = pdu.to_tree();
    assert_eq!(
        tree,
        tree!({
            "procedure_code": 200,
            "direction": "InitiatingMessage",
            "ies": [{"MME-UE-S1AP-ID": 1, "criticality": "reject"},
                    {"eNB-UE-S1AP-ID": 7, "criticality": "reject"},
                    {"Cause": {"radioNetwork": "user-inactivity"}, "criticality": "ignore"}],
            "criticality": "ignore",
        })
    );
    assert_eq!(S1APPDU::from_tree(&tree).unwrap(), pdu);
    assert_eq!(select(&tree, "/s1ap/eNB-UE-S1AP-ID"), [tree!(7)]);
    inspect::set(&mut tree, "/s1ap/eNB-UE-S1AP-ID", tree!(9)).unwrap();
    let edited = S1APPDU::from_tree(&tree).unwrap();
    assert_eq!(edited.procedure_code(), 200);
    assert_eq!(
        select(&edited.to_tree(), "/s1ap/eNB-UE-S1AP-ID"),
        [tree!(9)]
    );
    let mut unsaid = tree.clone();
    unsaid.as_object_mut().unwrap().shift_remove("criticality");
    assert_eq!(
        S1APPDU::from_tree(&unsaid).unwrap_err().kind(),
        ErrorKind::Value
    );
    // A message that is no container of protocol IEs has its octets.
    let pdu = S1APPDU::decode(&[0x00, 200, 0x00, 0x03, 0x00, 0x00, 0x01]).unwrap();
    let tree = pdu.to_tree();
    assert_eq!(tree["octets"], tree!("000001"));
    assert_eq!(S1APPDU::from_tree(&tree).unwrap(), pdu);
}

#[test]
fn an_edit_that_one_place_refuses_is_done_at_none() {
    let mut tree = released(tree!([
        {"MME-UE-S1AP-ID": 1},
        {"Cause": {"radioNetwork": "user-inactivity"}},
    ]))
    .unwrap();
    let before = tree.clone();
    let everywhere = "/s1ap/*/radioNetwork";
    let error = inspect::set(&mut tree, everywhere, tree!("unspecified"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("which has none"), "{error}");
    assert_eq!(tree, before);
    assert!(inspect::remove(&mut tree, everywhere).is_err());
    assert_eq!(tree, before);
    inspect::set(&mut tree, "/s1ap/Cause/radioNetwork", tree!("unspecified")).unwrap();
    assert_eq!(
        select(&tree, "/s1ap/Cause/radioNetwork"),
        [tree!("unspecified")]
    );
}

#[test]
fn an_edit_names_a_member_that_the_type_has_and_keeps_what_it_always_has() {
    let mut tree = release_request().to_tree();
    let before = tree.clone();
    let error = inspect::set(&mut tree, "/s1ap/Cause/misspelled", tree!(1))
        .unwrap_err()
        .to_string();
    assert!(error.contains("is not a member of Cause"), "{error}");
    let error = inspect::remove(&mut tree, "/s1ap/Cause/radioNetwork")
        .unwrap_err()
        .to_string();
    assert!(error.contains("a CHOICE has one alternative"), "{error}");
    assert_eq!(tree, before);
    inspect::set(&mut tree, "/s1ap/Cause/nas", tree!("normal-release")).unwrap();
    let written = sent(&tree).unwrap();
    assert_eq!(
        select(&written, "/s1ap/Cause"),
        [tree!({"nas": "normal-release"})]
    );
    let mut setup = S1APPDU::decode(&fixture("E-RABSetupRequest"))
        .unwrap()
        .to_tree();
    let item = "/s1ap/E-RABToBeSetupListBearerSUReq/0";
    let error = inspect::remove(&mut setup, &format!("{item}/e-RAB-ID"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("its type always has it"), "{error}");
    let error = inspect::set(&mut setup, &format!("{item}/e-RAB-ID"), Value::Null)
        .unwrap_err()
        .to_string();
    assert!(error.contains("its type always has it"), "{error}");
}

#[test]
fn a_path_is_checked_against_a_message_without_a_tree() {
    let list = "/s1ap/E-RABToBeSetupListBearerSUReq";
    for (message, path) in [
        ("UEContextReleaseRequest", "/s1ap/Cause/nas"),
        ("ue-context-release-request", "/s1ap/cause/radio_network"),
        ("UEContextReleaseRequest", "/s1ap/*/radioNetwork"),
        ("UEContextReleaseRequest", "/s1ap/0/criticality"),
        ("UEContextReleaseRequest", "/s1ap/-"),
        ("UEContextReleaseRequest", "/criticality"),
        ("UEContextReleaseRequest", "/s1ap/@id=60000/anything"),
        ("E-RABSetupRequest", &format!("{list}/0/gTP-TEID")),
        (
            "E-RABSetupRequest",
            &format!("{list}/E-RABToBeSetupItemBearerSUReq/gTP-TEID"),
        ),
        (
            "E-RABSetupRequest",
            &format!("{list}/0/E-RABToBeSetupItemBearerSUReq/gTP-TEID"),
        ),
        ("E-RABSetupRequest", &format!("{list}/0/criticality")),
    ] {
        let checked = inspect::check_path(message, path);
        assert!(checked.is_ok(), "{message} {path}: {checked:?}");
    }
    for (message, path, reason) in [
        ("NoSuchMessage", "/s1ap/Cause", "is not a message of S1AP"),
        (
            "UEContextReleaseRequest",
            "/s1ap/GUMMEI-ID",
            "is not an IE of UEContextReleaseRequest, which has MME-UE-S1AP-ID",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/Cause/misspelled",
            "is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, \
             at /s1ap/Cause/misspelled",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/eNB-UE-S1AP-ID/deeper",
            "which has none",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/Cause/value/nas",
            "is not a member of Cause",
        ),
        (
            "UEContextReleaseRequest",
            "/s1ap/NoSuchIE",
            "is not an IE of S1AP",
        ),
        (
            "E-RABSetupRequest",
            &format!("{list}/E-RABToBeSetupItemBearerSUReq/misspelled"),
            "is not a member of ERABToBeSetupItemBearerSUReq",
        ),
    ] {
        let error = inspect::check_path(message, path).unwrap_err().to_string();
        assert!(error.contains(reason), "{message} {path}: {error}");
    }
}

#[test]
fn a_private_message_has_no_ies_under_the_root() {
    let tree = S1APPDU::decode(&fixture("PrivateMessage"))
        .unwrap()
        .to_tree();
    let reason = "a message PrivateMessage has no IEs: its tree has its octets";
    let error = inspect::select(&tree, "/s1ap/Cause")
        .unwrap_err()
        .to_string();
    assert!(error.contains(reason), "{error}");
    let error = inspect::check_path("PrivateMessage", "/s1ap/Cause")
        .unwrap_err()
        .to_string();
    assert!(error.contains(reason), "{error}");
}

/// A report of the data that a secondary RAT carried, with this count for the uplink.
fn usage_report(count: u64) -> Value {
    tree!({"message": "SecondaryRATDataUsageReport", "ies": [
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
/// is written, read, edited and encoded as any other.
#[test]
fn an_integer_of_two_to_the_63_or_more_is_a_number() {
    let report = "/s1ap/SecondaryRATDataUsageReportList/0/e-RABUsageReportList/0";
    let count = format!("{report}/usageCountUL");
    let below = S1APPDU::from_tree(&usage_report((1 << 63) - 1)).unwrap();
    let below = hex::encode(below.encode().unwrap());
    assert_eq!(below.matches("7fffffffffffffff").count(), 1);
    for (number, octets) in [
        (1u64 << 63, "8000000000000000"),
        (u64::MAX, "ffffffffffffffff"),
    ] {
        let pdu = S1APPDU::from_tree(&usage_report(number)).unwrap();
        let wire = pdu.encode().unwrap();
        assert_eq!(
            hex::encode(&wire),
            below.replace("7fffffffffffffff", octets)
        );
        let mut tree = S1APPDU::decode(&wire).unwrap().to_tree();
        assert_eq!(select(&tree, &count), [tree!(number)]);
        assert_eq!(tree, usage_report(number));
        inspect::set(&mut tree, &count, tree!(number - 1)).unwrap();
        let edited = S1APPDU::from_tree(&tree).unwrap();
        assert_eq!(edited.to_tree(), usage_report(number - 1));
    }
    let mut beyond = usage_report(1);
    let digits = tree!("18446744073709551616");
    inspect::set(&mut beyond, &count, digits).unwrap();
    let error = S1APPDU::from_tree(&beyond).unwrap_err();
    assert_eq!(error.kind(), oxirush_s1ap::ErrorKind::Value);
    assert_eq!(
        error.to_string(),
        "this is not a value of E-RABUsageReportItem: a number from 0 to 18446744073709551615 \
         is expected, and \"18446744073709551616\" is written, at usageCountUL"
    );
}

/// The bindings hold an INTEGER in 64 bits. An IE with a number that a later release
/// makes larger than that is not read with its type: a tree has the octets that were
/// received, and writes them back as they are.
#[test]
fn an_ie_with_an_integer_beyond_64_bits_keeps_the_octets_received() {
    let attempts = PagingAttemptInformation {
        paging_attempt_count: PagingAttemptCount(u64::MAX),
        intended_number_of_paging_attempts: IntendedNumberOfPagingAttempts(1),
        next_paging_area_scope: None,
        ie_extensions: None,
    };
    let largest = AssistanceDataForPaging {
        assistance_data_for_recommended_cells: None,
        assistance_data_for_ce_capable_ues: None,
        paging_attempt_information: Some(attempts),
        ie_extensions: None,
    };
    let largest = oxirush_s1ap::encode(&largest).unwrap();
    let nine = [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let at = largest
        .windows(9)
        .position(|octets| octets == nine)
        .unwrap();
    let mut beyond = largest.to_vec();
    beyond[at..at + 9].copy_from_slice(&[0x01, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mut pdu = S1APPDU::from_tree(&tree!({"message": "Paging", "ies": [
        {"UEIdentityIndexValue": "0a40"},
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
    let tree = S1APPDU::decode(&wire).unwrap().to_tree();
    let octets = tree!({"octets": hex::encode(&beyond)});
    assert_eq!(tree["ies"][4], tree!({"AssistanceDataForPaging": octets}));
    assert_eq!(select(&tree, "/s1ap/CNDomain"), [tree!("ps")]);
    assert_eq!(S1APPDU::from_tree(&tree).unwrap().encode().unwrap(), wire);
}

/// A BIT STRING whose type gives one size and an extension marker has its length when
/// it is of another size (ITU-T X.697 (02/2021) §24.3), and is its octets alone when it
/// is of that size. Both are read, edited and written: the octets are those that the
/// bindings encode.
#[test]
fn a_bit_string_of_another_size_than_its_type_gives_has_its_length() {
    let algorithms = "/s1ap/UESecurityCapabilities/encryptionAlgorithms";
    let message = "UEContextModificationRequest";
    let mut tree = modification(16).to_tree();
    assert_eq!(select(&tree, algorithms), [tree!("c000")]);
    inspect::set(&mut tree, algorithms, tree!("8000")).unwrap();
    assert_eq!(select(&sent(&tree).unwrap(), algorithms), [tree!("8000")]);
    for bits in [20, 24] {
        let pdu = modification(bits);
        let wire = pdu.encode().unwrap();
        let mut tree = S1APPDU::decode(&wire).unwrap().to_tree();
        let shown = tree!({"value": "c00000", "length": bits});
        assert_eq!(select(&tree, algorithms), [shown]);
        let length = format!("{algorithms}/length");
        assert_eq!(select(&tree, &length), [tree!(bits)]);
        let paths = inspect::paths(&tree);
        assert!(paths.iter().any(|(path, _)| *path == length), "{paths:?}");
        assert_eq!(inspect::check_path(message, &length), Ok(()));
        assert_eq!(S1APPDU::from_tree(&tree).unwrap().encode().unwrap(), wire);
        inspect::set(&mut tree, algorithms, tree!("c000")).unwrap();
        assert_eq!(S1APPDU::from_tree(&tree).unwrap(), modification(16));
    }
    let mut tree = modification(16).to_tree();
    let longer = tree!({"value": "c00000", "length": 24});
    inspect::set(&mut tree, algorithms, longer).unwrap();
    assert_eq!(S1APPDU::from_tree(&tree).unwrap(), modification(24));
    inspect::set(&mut tree, &format!("{algorithms}/length"), tree!(20)).unwrap();
    assert_eq!(S1APPDU::from_tree(&tree).unwrap(), modification(20));
    let written = |algorithms: Value| {
        S1APPDU::from_tree(&tree!({"message": message, "ies": [
            {"MME-UE-S1AP-ID": 1},
            {"eNB-UE-S1AP-ID": 1},
            {"UESecurityCapabilities": {
                "encryptionAlgorithms": algorithms,
                "integrityProtectionAlgorithms": "c000",
            }},
        ]}))
    };
    assert!(written(tree!("c000")).is_ok());
    let error = written(tree!("c00000")).unwrap_err();
    assert_eq!(error.kind(), oxirush_s1ap::ErrorKind::Value);
    assert_eq!(
        error.to_string(),
        "this is not a value of UESecurityCapabilities: 3 octets are written for a bit string \
         of 16 bits, which has 2, at encryptionAlgorithms"
    );
    assert!(written(tree!({"value": "c000", "length": 20})).is_err());
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
fn a_message_has_the_ies_and_the_criticalities_of_its_object_set() {
    let id = |name: &str| {
        let mut names = S1ap::ie_names().iter();
        names.find(|(_, known)| *known == name).unwrap().0
    };
    let request = S1ap::object_set_named("InitialContextSetupRequest").unwrap();
    assert_eq!(request.criticality, Criticality::Reject);
    let ies = request.ies.unwrap();
    let of = |name: &str| {
        let ie = ies.iter().find(|(ie, ..)| *ie == id(name));
        ie.map(|(_, criticality, presence)| (*criticality, *presence))
    };
    assert_eq!(ies[0].0, id("MME-UE-S1AP-ID"));
    assert_eq!(
        of("MME-UE-S1AP-ID"),
        Some((Criticality::Reject, Presence::Mandatory))
    );
    assert_eq!(
        of("E-RABToBeSetupListCtxtSUReq"),
        Some((Criticality::Reject, Presence::Mandatory))
    );
    assert_eq!(
        of("TraceActivation"),
        Some((Criticality::Ignore, Presence::Optional))
    );
    assert_eq!(of("E-RABToBeSetupListBearerSUReq"), None);
    // An IE has the criticality that the set of each message gives it.
    let response = S1ap::object_set_named("InitialContextSetupResponse").unwrap();
    let first = response.ies.unwrap()[0];
    assert_eq!(
        first,
        (
            id("MME-UE-S1AP-ID"),
            Criticality::Ignore,
            Presence::Mandatory
        )
    );
    // A name is taken as that of a message is, and a message without IEs has none.
    let named = S1ap::object_set_named("initial-context-setup-request");
    assert_eq!(named, Some(request));
    assert_eq!(S1ap::object_set_named("PrivateMessage").unwrap().ies, None);
    assert_eq!(S1ap::object_set_named("NoSuchMessage"), None);
    // The procedure has its own criticality, which is that of each of its messages.
    let procedure = |name: &str| S1ap::object_set_named(name).map(|set| set.criticality);
    assert_eq!(
        procedure("initial-context-setup-response"),
        Some(Criticality::Reject)
    );
    assert_eq!(procedure("Paging"), Some(Criticality::Ignore));
    assert_eq!(procedure("ErrorIndication"), Some(Criticality::Ignore));
    // Every message is found by its direction and its procedure code, and has the IEs of
    // its set, each of them known.
    for set in S1ap::object_sets() {
        assert_eq!(
            S1ap::object_set(set.direction, set.procedure_code),
            Some(set)
        );
        let ies = set.ies.unwrap_or_default();
        assert!(
            set.name == "PrivateMessage" || !ies.is_empty(),
            "{}",
            set.name
        );
        for (ie, ..) in ies {
            let known = S1ap::ie_names().iter().any(|(known, _)| known == ie);
            assert!(known, "{} {ie}", set.name);
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

#[test]
fn a_kind_that_the_specification_does_not_have_says_the_direction_of_its_pdu() {
    let pdu = S1APPDU::unsuccessfulOutcome(UnsuccessfulOutcome {
        procedure_code: ProcedureCode(13),
        criticality: Criticality::Ignore,
        value: oxirush_s1ap::Octets::from(vec![0]),
    });
    let oxirush_s1ap::S1apPduKind::Other {
        direction,
        procedure_code,
    } = pdu.kind()
    else {
        panic!("{:?}", pdu.kind());
    };
    assert_eq!((direction, procedure_code), (pdu.direction(), 13));
}

/// A request to modify a UE context, with UE security capabilities whose encryption
/// algorithms are this many bits.
fn modification(bits: usize) -> S1APPDU<'static> {
    // The first two bits of a string of this length.
    let mask = |length: usize| {
        let mut octets = vec![0; length.div_ceil(8)];
        octets[0] = 0xc0;
        oxirush_s1ap::BitString::new(octets, length).unwrap()
    };
    let capabilities = UESecurityCapabilities {
        encryption_algorithms: EncryptionAlgorithms(mask(bits)),
        integrity_protection_algorithms: IntegrityProtectionAlgorithms(mask(16)),
        ie_extensions: None,
    };
    build_s1ap!(REJECT UEContextModificationRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(1u32),
        REJECT UESecurityCapabilities(capabilities),
    )
    .unwrap()
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
