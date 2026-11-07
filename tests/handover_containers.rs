#![cfg(feature = "inspect")]
//! The handover containers of NGAP in the messages of a handover from EPS to 5GS (TS
//! 36.413 clauses 9.2.1.56 and 9.2.1.57, TS 23.502 clause 4.11.1.2.2): the source eNB
//! sends the Source NG-RAN Node to Target NG-RAN Node Transparent Container of TS 38.413
//! in the HANDOVER REQUIRED, and the MME the Target NG-RAN Node to Source NG-RAN Node
//! Transparent Container in the HANDOVER COMMAND. Once the program registers the tables
//! of oxirush-ngap, a tree writes each by the name of its type and the names of NGAP, and
//! a view opens, reads and edits it: NGAP encodes it as it encodes the container of its
//! own HANDOVER REQUEST.

use oxirush_ngap::ngap::NGAP_PDU;
use oxirush_s1ap::inspect::Value;
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU, tree};

const SOURCE: &str = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
const TARGET: &str = "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer";

/// The container of the source node: an RRC container, the target cell, and the cell
/// that the UE stayed in.
fn source() -> Value {
    let cell = tree!({"nR-CGI": {"pLMNIdentity": "208-93", "nRCellIdentity": 4660}});
    tree!({
        "rRCContainer": "0102",
        "targetCell-ID": (cell.clone()),
        "uEHistoryInformation": [{"lastVisitedCellInformation": {"nGRANCell": {
            "globalCellID": (cell),
            "cellType": {"cellSize": "medium"},
            "timeUEStayedInCell": 30,
        }}}],
    })
}

/// The octets that oxirush-ngap writes for the container `name` with the value `value`,
/// in the IE `ie` of its own message `message`.
fn by_ngap(message: &str, ie: &str, name: &str, value: Value) -> Value {
    let pdu = NGAP_PDU::from_tree(&tree!({"message": message, "ies": [
        {"AMF-UE-NGAP-ID": 1},
        {(ie): {(name): (value)}},
    ]}))
    .unwrap();
    pdu.to_tree()["ies"][1][ie].clone()
}

/// The octets of the IE `ie` of a tree of S1AP.
fn octets(pdu: &S1AP_PDU, ie: &str) -> Value {
    let tree = pdu.to_tree();
    let ies = tree["ies"].as_array().unwrap();
    let found = ies.iter().find(|entry| entry.get(ie).is_some());
    found.unwrap()[ie].clone()
}

#[test]
fn the_handover_required_to_5gs_carries_the_container_of_the_source_node() {
    oxirush_ngap::inspect::register();
    let required = S1AP_PDU::from_tree(&tree!({"message": "HandoverRequired", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"HandoverType": "eps-to-5gs"},
        {"Cause": {"radioNetwork": "handover-desirable-for-radio-reason"}},
        {"Source-ToTarget-TransparentContainer": {(SOURCE): (source())}},
    ]}))
    .unwrap();
    let container = "Source-ToTarget-TransparentContainer";
    let expected = by_ngap(
        "HandoverRequest",
        "SourceToTarget-TransparentContainer",
        SOURCE,
        source(),
    );
    assert_eq!(octets(&required, container), expected);

    // Read: a path goes through the name of the type, into the members of NGAP
    let path = format!("/s1ap/{container}/{SOURCE}");
    let size = format!(
        "{path}/uEHistoryInformation/0/lastVisitedCellInformation/nGRANCell/cellType/cellSize"
    );
    assert_eq!(inspect::check_path("HandoverRequired", &size), Ok(()));
    let sizes = inspect::enumerated_at("HandoverRequired", &size).unwrap();
    assert_eq!(sizes, Some(&["verysmall", "small", "medium", "large"][..]));
    let mut view = required.to_view().unwrap();
    inspect::open(&mut view, &size).unwrap();
    assert_eq!(inspect::select(&view, &size).unwrap(), [&tree!("medium")]);
    let cell = format!("{path}/targetCell-ID/nR-CGI/nRCellIdentity");
    assert_eq!(inspect::select(&view, &cell).unwrap(), [&tree!(4660)]);
    for (path, value) in inspect::paths(&view) {
        assert_eq!(inspect::select(&view, &path).unwrap(), [&value], "{path}");
    }
    assert_eq!(S1AP_PDU::from_view(&view).unwrap(), required);

    // Edited by the type of NGAP
    inspect::set(&mut view, &size, tree!("verysmall")).unwrap();
    let edited = S1AP_PDU::from_view(&view).unwrap();
    let mut small = source();
    let visited = &mut small["uEHistoryInformation"][0]["lastVisitedCellInformation"];
    visited["nGRANCell"]["cellType"]["cellSize"] = tree!("verysmall");
    let expected = by_ngap(
        "HandoverRequest",
        "SourceToTarget-TransparentContainer",
        SOURCE,
        small,
    );
    assert_eq!(octets(&edited, container), expected);
    inspect::set(&mut view, &size, tree!("huge")).unwrap();
    assert!(S1AP_PDU::from_view(&view).is_err());
}

#[test]
fn the_handover_command_to_5gs_carries_the_container_of_the_target_node() {
    oxirush_ngap::inspect::register();
    let target = tree!({"rRCContainer": "0a0b0c"});
    let command = S1AP_PDU::from_tree(&tree!({"message": "HandoverCommand", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"HandoverType": "eps-to-5gs"},
        {"Target-ToSource-TransparentContainer": {(TARGET): (target.clone())}},
    ]}))
    .unwrap();
    let container = "Target-ToSource-TransparentContainer";
    let expected = by_ngap(
        "HandoverRequestAcknowledge",
        "TargetToSource-TransparentContainer",
        TARGET,
        target,
    );
    assert_eq!(octets(&command, container), expected);
    assert_eq!(expected, tree!("00030a0b0c"));

    let rrc = format!("/s1ap/{container}/{TARGET}/rRCContainer");
    let mut view = command.to_view().unwrap();
    inspect::open(&mut view, &rrc).unwrap();
    assert_eq!(inspect::select(&view, &rrc).unwrap(), [&tree!("0a0b0c")]);
    inspect::set(&mut view, &rrc, tree!("ff")).unwrap();
    let edited = S1AP_PDU::from_view(&view).unwrap();
    assert_eq!(octets(&edited, container), tree!("0001ff"));
    // A container of E-UTRAN is still written by the name of its type
    let own = "TargeteNB-ToSourceeNB-TransparentContainer";
    let written = S1AP_PDU::from_tree(&tree!({"message": "HandoverCommand", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"HandoverType": "intralte"},
        {"Target-ToSource-TransparentContainer": {(own): {"rRC-Container": "0a0b0c"}}},
    ]}));
    assert!(written.is_ok());
}
