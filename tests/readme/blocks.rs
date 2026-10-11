// The Rust blocks of README.md, each as a function. `tests/readme.rs` writes
// this file and says how: do not edit it.
#![allow(unused)]

use std::cell::RefCell;

thread_local!(static PRINTED: RefCell<String> = const { RefCell::new(String::new()) });

// What a block prints is kept, to be compared with what the README shows.
macro_rules! println {
    () => {
        PRINTED.with(|printed| printed.borrow_mut().push('\n'))
    };
    ($($argument:tt)*) => {
        PRINTED.with(|printed| {
            let line = format!($($argument)*);
            let mut printed = printed.borrow_mut();
            printed.push_str(&line);
            printed.push('\n');
        })
    };
}

/// What `block` prints.
pub fn printed(block: fn()) -> String {
    PRINTED.with(|printed| printed.borrow_mut().clear());
    block();
    PRINTED.with(|printed| printed.take())
}

pub const BLOCKS: &[fn()] = &[
    block_1,
    block_2,
    block_3,
    block_4,
    block_5,
    block_6,
    block_7,
    block_8,
    block_9,
    block_10,
    block_11,
    block_12,
    block_13,
    block_14,
    block_15,
    block_16,
    block_17,
    block_18,
];

/// The Rust block 1 of README.md.
fn block_1() {
    use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, s1ap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Decode a UE CONTEXT RELEASE REQUEST
        let bytes = hex::decode("0012401500000300000002002a000800020007000240020280")?;
        let pdu = S1AP_PDU::decode(&bytes)?;
        assert_eq!((pdu.procedure_code(), pdu.is_initiating()), (18, true));
        println!("{}", pdu.procedure_name());

        // Read its IEs by name. An `opt` field is `None` when its IE is not there,
        // and what follows `=>` is what the field keeps of the decoded value
        let request: UEContextReleaseRequest = pdu.decode_value()?;
        extract_s1ap_ies!(request, UEContextReleaseRequest,
            opt mme_id: u32 = MME_UE_S1AP_ID(id),
            opt enb_id: u32 = eNB_UE_S1AP_ID(id),
            opt cause: Cause = Cause(cause) => cause,
        );
        println!("MME={mme_id:?} eNB={enb_id:?} cause={cause:?}");

        // Build it with another cause: the direction, the procedure, the criticality
        // of the PDU and the message, then each IE with its criticality
        let rebuilt = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,

            REJECT MME_UE_S1AP_ID(42u32),
            REJECT eNB_UE_S1AP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
        )?;

        // Encode it
        let encoded = hex::encode(rebuilt.encode()?);
        assert_eq!(encoded, "0012401500000300000002002a0008000200070002400202a0");
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 2 of README.md.
fn block_2() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
    use serde_json::json;

    // The same message as a tree: its name and its IEs by name
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode(&bytes).unwrap();
    println!("{}", inspect::message_tree(&pdu).unwrap());

    // Its view is edited at a path, and is a PDU again
    let mut view = inspect::message_view(&pdu).unwrap();
    let lost = json!("radio-connection-with-ue-lost");
    inspect::set(&mut view, "/s1ap/Cause/radioNetwork", lost).unwrap();
    let edited = inspect::message_from_view(&view).unwrap();
    println!("{}", inspect::message_tree(&edited).unwrap());

    // Its octets are those that the typed example built
    let octets = hex::encode(edited.encode().unwrap());
    assert_eq!(octets, "0012401500000300000002002a0008000200070002400202a0");
}

/// The Rust block 3 of README.md.
fn block_3() {
    use oxirush_s1ap::{S1apPduKind, s1ap::*};

    fn receive(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        let pdu = S1AP_PDU::decode(bytes)?;

        Ok(match pdu.kind() {
            S1apPduKind::Initiating_UEContextReleaseRequest => {
                let request: UEContextReleaseRequest = pdu.decode_value()?;
                format!("a release request with {} IEs", request.protocol_ies.0.len())
            }
            S1apPduKind::Other { direction, procedure_code } => {
                format!("procedure {procedure_code} has no {direction}")
            }
            kind => format!("{kind:?} is not handled here"),
        })
    }

    // A UE CONTEXT RELEASE REQUEST, then an S1 SETUP FAILURE
    let request = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    println!("{}", receive(&request).unwrap());

    let failure = hex::decode("401100080000010002400145").unwrap();
    println!("{}", receive(&failure).unwrap());
}

/// The Rust block 4 of README.md.
fn block_4() {
    use oxirush_s1ap::{Error, extract_s1ap_ies, s1ap::*};

    fn handle(request: &UEContextReleaseRequest) -> Result<(), Error> {
        extract_s1ap_ies!(request, UEContextReleaseRequest,
            req mme_id: u32 = MME_UE_S1AP_ID(id),
            req enb_id: u32 = eNB_UE_S1AP_ID(id),
            req cause: Cause = Cause(value) => value,
        );

        println!("MME={mme_id} eNB={enb_id} cause={cause:?}");
        Ok(())
    }
}

/// The Rust block 5 of README.md.
fn block_5() {
    use oxirush_s1ap::{build_s1ap_ie, s1ap::*, with_s1ap_ie_mut};

    // A message with one IE, which is built alone
    let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
        build_s1ap_ie!(UplinkNASTransport, REJECT MME_UE_S1AP_ID(1u32)).unwrap(),
    ]));

    // The IE is decoded, changed and encoded again in its place
    let updated = with_s1ap_ie_mut!(message, UplinkNASTransport,
        MME_UE_S1AP_ID(id) = 42u32
    );
    assert_eq!(updated, Ok(true));
}

/// The Rust block 6 of README.md.
fn block_6() {
    use oxirush_s1ap::helpers::*;

    // A PLMN from its MCC and its MNC, and what is identified within it
    let network = plmn("208", "93").unwrap();
    let mme = gummei(network.clone(), 1, 1); // group, code
    let tracking_area = tai(network.clone(), [0x00, 0x01]); // TAC
    let cell = eutran_cgi(network.clone(), 0x12345, 7).unwrap(); // macro eNB, cell
    let enb = global_enb_id(network.clone(), 0x12345).unwrap();

    // A bit string from octets, and the algorithms of a UE: the first octet
    // of its encryption bitmap, then of its integrity bitmap
    let algorithm_mask = bytes_to_bitvec(&[0xe0]);
    let capabilities = ue_security_capabilities(0xe0, 0xe0);

    // What is no MCC, and a macro eNB ID that does not fit in its 20 bits
    println!("{}", plmn("20", "93").unwrap_err());
    println!("{}", global_enb_id(network, 0x100000).unwrap_err());
}

/// The Rust block 7 of README.md.
fn block_7() {
    use oxirush_s1ap::{ErrorKind, s1ap::*};

    // A UE CONTEXT RELEASE REQUEST is not decoded as an S1 SETUP REQUEST
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode(&bytes).unwrap();
    let wrong = pdu.decode_value::<S1SetupRequest>().unwrap_err();
    assert_eq!(wrong.kind(), ErrorKind::WrongMessage);
    println!("{wrong}");

    // Octets that end early: the message, and what rasn said
    let short = S1AP_PDU::decode(&bytes[..10]).unwrap_err();
    assert_eq!(short.kind(), ErrorKind::Decode);
    println!("{short}");
    println!("{}", std::error::Error::source(&short).unwrap());
}

/// The Rust block 8 of README.md.
fn block_8() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};

    // UE CONTEXT RELEASE REQUEST
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode(&bytes).unwrap();

    // Its tree, then its third IE as the view has it
    println!("{}", inspect::message_tree(&pdu).unwrap());
    let view = inspect::message_view(&pdu).unwrap();
    println!("{}", view["message"]["protocolIEs"][2]);
}

/// The Rust block 9 of README.md.
fn block_9() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
    use serde_json::json;

    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode(&bytes).unwrap();
    let view = inspect::message_view(&pdu).unwrap();

    // `/s1ap` is the IEs of the message, and an IE goes by its name
    let select = |path| inspect::select(&view, path).unwrap();
    assert_eq!(select("/s1ap/eNB-UE-S1AP-ID/value"), [&json!(7)]);
    assert_eq!(select("/s1ap/Cause/radioNetwork"), [&json!("user-inactivity")]);
    assert_eq!(select("/s1ap/Cause/octets"), [&json!("0280")]);

    // By its position, by its name whatever its case and its separators, by its
    // identifier, and each IE with `*`
    assert_eq!(select("/s1ap/1/value"), select("/S1AP/enb_ue_s1ap_id/value"));
    assert_eq!(select("/s1ap/@id=8/value"), [&json!(7)]);
    assert_eq!(select("/s1ap/*/criticality").len(), 3);

    // Nothing where the message could have a value and has none
    assert!(select("/s1ap/Cause/nas").is_empty());
    assert!(select("/s1ap/GUMMEI-ID/mME-Code").is_empty());

    // Each value with the path that selects it
    for (path, value) in inspect::paths(&view) {
        println!("{path} = {value}");
    }
}

/// The Rust block 10 of README.md.
fn block_10() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
    use serde_json::json;

    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode(&bytes).unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();

    // A value, and another alternative of a CHOICE in place of the one that is there
    inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", json!(9)).unwrap();
    inspect::set(&mut view, "/s1ap/Cause/nas", json!("Normal Release")).unwrap();
    let edited = inspect::message_from_view(&view).unwrap();
    println!("{}", inspect::message_tree(&edited).unwrap());

    // What was not edited is the octets that were received
    let after = inspect::message_view(&edited).unwrap();
    let octets = inspect::select(&after, "/s1ap/MME-UE-S1AP-ID/octets").unwrap();
    assert_eq!(octets, [&json!("002A")]);

    // A name that the type does not have is refused, and the view is as it was
    let wrong = inspect::set(&mut view, "/s1ap/Cause/radioNetwrok", json!("unspecified"));
    println!("{}", wrong.unwrap_err());
}

/// The Rust block 11 of README.md.
fn block_11() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
    use serde_json::json;

    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode(&bytes).unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();

    // An IE is added at the end, by its name and its typed value
    let name = json!({"id": "eNBname", "criticality": "ignore", "value": "enb-1"});
    inspect::insert(&mut view, "/s1ap/-", name).unwrap();

    // Before another IE, by its number and its octets
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "c0ffee"});
    inspect::insert(&mut view, "/s1ap/Cause", unknown).unwrap();

    // An IE is taken out by its name
    inspect::remove(&mut view, "/s1ap/MME-UE-S1AP-ID").unwrap();

    let edited = inspect::message_from_view(&view).unwrap();
    println!("{}", inspect::message_tree(&edited).unwrap()["ies"]);
}

/// The Rust block 12 of README.md.
fn block_12() {
    use oxirush_s1ap::inspect;
    use serde_json::json;

    // A tree: the name of the message, whatever its case and its separators, then its IEs
    let request = json!({
        "message": "ue context release request",
        "ies": [
            {"MME-UE-S1AP-ID": 42},
            {"eNB-UE-S1AP-ID": 7},
            {"Cause": {"radioNetwork": "user-inactivity"}},
            // An IE that the message does not have says its criticality
            {"eNBname": "enb-1", "criticality": "ignore"},
        ],
    });

    let pdu = inspect::message_from_tree(&request).unwrap();
    println!("{}", hex::encode(pdu.encode().unwrap()));

    // The name gave the direction, the procedure code and the criticalities
    let view = inspect::message_view(&pdu).unwrap();
    println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
    println!("{}", json!(inspect::select(&view, "/s1ap/*/criticality").unwrap()));
}

/// The Rust block 13 of README.md.
fn block_13() {
    use oxirush_s1ap::inspect;
    use serde_json::json;

    // `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
    let request = json!({"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
    ]});
    let pdu = inspect::message_from_tree(&request).unwrap();
    println!("{}", hex::encode(pdu.encode().unwrap()));

    // A view shows an IE that does not decode by its octets, and says why
    let mut view = inspect::message_view(&pdu).unwrap();
    println!("{}", view["message"]["protocolIEs"][1]);

    // In a view, the `octets` of an IE are set in place of its value
    inspect::set(&mut view, "/s1ap/MME-UE-S1AP-ID/octets", json!("00ff")).unwrap();
    let edited = inspect::message_from_view(&view).unwrap();
    println!("{}", inspect::message_tree(&edited).unwrap());
}

/// The Rust block 14 of README.md.
fn block_14() {
    use oxirush_s1ap::inspect;
    use serde_json::json;

    // A message that has one IE twice
    let twice = json!({"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 1},
        {"eNB-UE-S1AP-ID": 7},
        {"MME-UE-S1AP-ID": 2},
    ]});
    let pdu = inspect::message_from_tree(&twice).unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();

    // The name selects both, in the order of the message
    let both = inspect::select(&view, "/s1ap/MME-UE-S1AP-ID/value").unwrap();
    assert_eq!(both, [&json!(1), &json!(2)]);

    // Each has its position, which is how `paths` names it, and its name may follow
    inspect::set(&mut view, "/s1ap/2/MME-UE-S1AP-ID/value", json!(3)).unwrap();

    // After a position, the name of another IE is an error that says which IE is there
    println!("{}", inspect::select(&view, "/s1ap/2/eNB-UE-S1AP-ID/value").unwrap_err());

    // The values, each with its path
    for (path, value) in inspect::paths(&view) {
        if path.ends_with("/value") {
            println!("{path} = {value}");
        }
    }
}

/// The Rust block 15 of README.md.
fn block_15() {
    use oxirush_s1ap::inspect;
    use serde_json::{Value, json};

    // A message whose one IE is a list of two items
    let command = json!({"message": "E-RABReleaseCommand", "ies": [
        {"E-RABToBeReleasedList": [
            {"E-RABItem": {"e-RAB-ID": 5, "cause": {"nas": "normal-release"}}},
            {"E-RABItem": {"e-RAB-ID": 6, "cause": {"nas": "normal-release"}}},
        ]},
    ]});
    let pdu = inspect::message_from_tree(&command).unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();

    // An item by its position or by its name, which selects each item of that name
    let select = |view, path| -> Vec<Value> {
        let selected = inspect::select(view, path).unwrap();
        selected.into_iter().cloned().collect()
    };
    assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/1/e-RAB-ID"), [json!(6)]);
    assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/E-RABItem/e-RAB-ID"), [json!(5), json!(6)]);

    // The same value by the path that says each `value`
    assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/value/1/value/e-RAB-ID"), [json!(6)]);

    // And by the names that the tree shows, joined: the item by its position, then its name
    assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/1/E-RABItem/e-RAB-ID"), [json!(6)]);

    // The criticality of an item is the one of its object set
    assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/0/criticality"), [json!("ignore")]);

    // An item is added at the end, as the IE that it is, and one is taken out
    let item = json!({"e-RAB-ID": 7, "cause": {"radioNetwork": "unspecified"}});
    let item = json!({"id": "E-RABItem", "criticality": "ignore", "value": item});
    inspect::insert(&mut view, "/s1ap/E-RABToBeReleasedList/-", item).unwrap();
    inspect::remove(&mut view, "/s1ap/E-RABToBeReleasedList/0").unwrap();

    let edited = inspect::message_from_view(&view).unwrap();
    println!("{}", inspect::message_tree(&edited).unwrap());
}

/// The Rust block 16 of README.md.
fn block_16() {
    use oxirush_s1ap::inspect;

    // The names that the value at a path can have, as the specification spells them
    let names = inspect::enumerated_at("UEContextReleaseRequest", "/s1ap/Cause/nas").unwrap();
    println!("{:?}", names.unwrap());

    // A value of another type has none
    let choice = inspect::enumerated_at("UEContextReleaseRequest", "/s1ap/Cause/value");
    assert_eq!(choice, Ok(None));
}

/// The Rust block 17 of README.md.
fn block_17() {
    use oxirush_s1ap::inspect;

    // Whether a path can select anything in a message of a name, without a view
    let check = |path| inspect::check_path("UEContextReleaseRequest", path);
    assert_eq!(check("/s1ap/Cause/nas"), Ok(()));
    assert_eq!(check("/s1ap/*/radioNetwork"), Ok(()));

    // A name that is written wrong, and an IE that the message does not have
    for wrong in ["/s1ap/Cause/radioNetwrok", "/s1ap/eNB-UE-S1AP-ID/valeu", "/s1ap/GUMMEI-ID"] {
        println!("{}", check(wrong).unwrap_err());
    }
}

/// The Rust block 18 of README.md.
fn block_18() {
    use oxirush_s1ap::{ErrorKind, inspect};
    use serde_json::json;

    // Written: the name of the type with its value, in place of the octets
    let name = "SourceeNB-ToTargeteNB-TransparentContainer";
    let required = json!({"message": "HandoverRequired", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"Source-ToTarget-TransparentContainer": {name: {
            "rRC-Container": "0102",
            "targetCell-ID": {"pLMNidentity": "208-93", "cell-ID": 4660},
            "uE-HistoryInformation": [{"uTRAN-Cell": "00"}],
        }}},
    ]});

    // A tree shows the octets that the container was encoded as
    let pdu = inspect::message_from_tree(&required).unwrap();
    println!("{}", inspect::message_tree(&pdu).unwrap()["ies"][2]);

    // Read: a path goes through the name of the type
    let mut view = inspect::message_view(&pdu).unwrap();
    let container = format!("/s1ap/Source-ToTarget-TransparentContainer/{name}");
    let cell = format!("{container}/targetCell-ID/cell-ID");
    assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));

    // The path is an error until `open` has decoded the octets, and selects the value after that
    assert_eq!(inspect::select(&view, &cell).unwrap_err().kind(), ErrorKind::NotOpened);
    inspect::open(&mut view, &cell).unwrap();
    assert_eq!(inspect::select(&view, &cell).unwrap(), [&json!(4660)]);

    // What the octets carry, decoded
    let decoded = format!("{container}/decoded");
    println!("{}", inspect::select(&view, &decoded).unwrap()[0]);

    // An edit goes into it, and what is not edited is sent as it was received
    inspect::set(&mut view, &cell, json!(7)).unwrap();
    let edited = inspect::message_from_view(&view).unwrap();
    println!("{}", inspect::message_tree(&edited).unwrap()["ies"][2]);
}
