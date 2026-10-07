//! Decode an S1AP PDU, print its tree, edit one IE, add two and encode it again.
//!
//! ```sh
//! cargo run --example inspect --features inspect
//! ```

use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use serde_json::json;

/// The number of the IE that ASN.1 names `id-<name>`.
fn ie_id(name: &str) -> u16 {
    let known = inspect::ie_names().iter().find(|(_, known)| *known == name);
    known.expect("an IE of TS 36.413").0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // INITIAL UE MESSAGE: eNB-UE-S1AP-ID, NAS-PDU, TAI, EUTRAN-CGI and
    // RRC-Establishment-Cause.
    let wire = hex::decode(
        "000c402c000005000800020000001a000403076002004300060002f839595c006440080002f839000000000086400100",
    )?;
    let pdu = S1AP_PDU::decode(&wire)?;
    let mut tree = inspect::inspect_pdu(&pdu)?;
    // The PLMN identity is "208-93", and the TAC and the cell identity are numbers.
    println!("{}", serde_json::to_string_pretty(&tree)?);

    let ies = tree["message"]["protocolIEs"]
        .as_array_mut()
        .expect("the IEs of the message");
    // Edit an IE through its typed value. The other IEs keep the octets received.
    let tai = ies.iter_mut().find(|ie| ie["id"] == ie_id("TAI"));
    let tai = &mut tai.expect("a TAI")["value"];
    tai["tAC"] = json!(7);
    tai["pLMNidentity"] = json!("001-01");
    // Add an IE: its `value` has the type of the identifier. A number is also written
    // in hexadecimal.
    ies.push(json!({
        "id": ie_id("S-TMSI"),
        "criticality": "reject",
        "value": {"mMEC": 1, "m-TMSI": "0xC0FFEE01"},
    }));
    // Add given octets as an IE: `_raw_value`, without `value`.
    ies.push(json!({"id": 60000, "criticality": "ignore", "_raw_value": "C0FFEE"}));

    let edited = inspect::encode_pdu(&tree)?;
    println!("{}", hex::encode_upper(edited.encode()?));
    Ok(())
}
