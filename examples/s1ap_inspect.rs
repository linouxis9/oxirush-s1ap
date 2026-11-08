//! Decode an S1AP PDU, print its values with their paths, edit one IE, add two and encode
//! it again.
//!
//! ```sh
//! cargo run --example s1ap_inspect --features inspect
//! ```

use oxirush_s1ap::tree;
use oxirush_s1ap::{S1ap, inspect, s1ap::S1APPDU};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // INITIAL UE MESSAGE: eNB-UE-S1AP-ID, NAS-PDU, TAI, EUTRAN-CGI and
    // RRC-Establishment-Cause.
    let wire = hex::decode(
        "000c402c000005000800020000001a000403076002004300060002f839595c006440080002f839000000000086400100",
    )?;
    let pdu = S1APPDU::decode(&wire)?;
    let mut tree = pdu.to_tree();

    // The message by the name that ASN.1 gives it, then each value with the path that
    // selects it: `/s1ap` stands for the IEs of the message, and an IE goes by its name.
    let set = S1ap::object_set(pdu.direction(), pdu.procedure_code());
    let name = set.expect("a message of the specification").name;
    println!("{name}");
    for (path, value) in inspect::paths(&tree) {
        println!("{path} = {value}");
    }

    // The PLMN identity is "208-93", and the TAC and the cell identity are numbers.
    let plmn = "/s1ap/TAI/pLMNidentity";
    let tac = "/s1ap/TAI/tAC";
    assert_eq!(inspect::select(&tree, plmn)?, [&tree!("208-93")]);
    assert_eq!(inspect::select(&tree, tac)?, [&tree!(0x595c)]);

    // Edit an IE through its value. The other IEs are encoded to the octets received.
    inspect::set(&mut tree, tac, tree!(7))?;
    inspect::set(&mut tree, plmn, tree!("001-01"))?;

    // Add an IE at the end, by its name: its value has the type of the IE. A number is
    // also written in hexadecimal.
    let identity = tree!({"S-TMSI": {"mMEC": 1, "m-TMSI": "0xC0FFEE01"}});
    inspect::insert(&mut tree, "/s1ap/-", identity)?;

    // Add given octets as an IE, before the establishment cause.
    let unknown = tree!({"60000": {"octets": "C0FFEE"}, "criticality": "ignore"});
    inspect::insert(&mut tree, "/s1ap/RRC-Establishment-Cause", unknown)?;

    // Encode the tree again.
    let edited = S1APPDU::from_tree(&tree)?;
    println!("{}", hex::encode_upper(edited.encode()?));

    // The IE that was not edited is the octets that were received.
    let (before, after) = (pdu.protocol_ies()?, edited.protocol_ies()?);
    assert_eq!(before.0[1].value, after.0[1].value);

    Ok(())
}
