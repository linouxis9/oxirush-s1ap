//! Decode an S1AP PDU, print its values with their paths, edit one IE, add two and encode
//! it again.
//!
//! ```sh
//! cargo run --example inspect --features inspect
//! ```

use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // INITIAL UE MESSAGE: eNB-UE-S1AP-ID, NAS-PDU, TAI, EUTRAN-CGI and
    // RRC-Establishment-Cause.
    let wire = hex::decode(
        "000c402c000005000800020000001a000403076002004300060002f839595c006440080002f839000000000086400100",
    )?;
    let pdu = S1AP_PDU::decode(&wire)?;
    let mut tree = inspect::inspect_pdu(&pdu)?;
    // The message by the name that ASN.1 gives it, then each value with the path that
    // selects it: `/s1ap` stands for the IEs of the message, and an IE goes by its name.
    let name = inspect::message_name(&pdu).expect("a message of TS 36.413");
    println!("{name}");
    for (path, value) in inspect::paths(&tree) {
        println!("{path} = {value}");
    }

    // The PLMN identity is "208-93", and the TAC and the cell identity are numbers.
    let plmn = inspect::select(&tree, "/s1ap/TAI/value/pLMNidentity")?;
    assert_eq!(plmn, [&json!("208-93")]);
    assert_eq!(
        inspect::select(&tree, "/s1ap/TAI/value/tAC")?,
        [&json!(0x595c)]
    );

    // Edit an IE through its typed value. The other IEs keep the octets received.
    inspect::set(&mut tree, "/s1ap/TAI/value/tAC", json!(7))?;
    inspect::set(&mut tree, "/s1ap/TAI/value/pLMNidentity", json!("001-01"))?;
    // Add an IE at the end: its `value` has the type of the identifier. A number is
    // also written in hexadecimal.
    let identity = json!({"mMEC": 1, "m-TMSI": "0xC0FFEE01"});
    let identity = json!({"id": "S-TMSI", "criticality": "reject", "value": identity});
    inspect::insert(&mut tree, "/s1ap/-", identity)?;
    // Add given octets as an IE, before the establishment cause: its `octets`, without
    // `value`.
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "C0FFEE"});
    inspect::insert(&mut tree, "/s1ap/RRC-Establishment-Cause", unknown)?;

    let edited = inspect::encode_pdu(&tree)?;
    println!("{}", hex::encode_upper(edited.encode()?));
    // The IE that was not edited is the octets that were received.
    let tree = inspect::inspect_pdu(&edited)?;
    let nas = inspect::select(&tree, "/s1ap/NAS-PDU/octets")?;
    assert_eq!(nas, [&json!("03076002")]);
    Ok(())
}
