//! Encode and decode S1AP PDUs **without macros** using rasn and the generated types.
//!
//! This example demonstrates the complete raw-codec workflow:
//! - Constructing an `S1AP_PDU` by hand (`ProtocolIEID`, `Criticality`, `Any`)
//! - Encoding and decoding with the raw rasn APER functions
//! - Decoding and pattern-matching open protocol IE values
//! - Verifying encode → decode round-trip fidelity
//!
//! For the ergonomic `build_s1ap!` / `extract_s1ap_ies!` versions, see
//! `build_pdu.rs` and `extract_ies.rs`.

use rasn::types::{FixedOctetString, PrintableString};

use oxirush_s1ap::s1ap::*;

fn main() {
    // ── 1. Build an S1SetupResponse by hand ─────────────────────────────────
    //
    // One ServedGUMMEIs item identifies its served PLMNs, MME group IDs,
    // and MMECs.

    let plmn_identity = PLMNidentity(TBCDSTRING(FixedOctetString::new([0x02, 0xF8, 0x39])));
    let served_gummeis = ServedGUMMEIs(vec![ServedGUMMEIsItem::new(
        ServedPLMNs(vec![plmn_identity]),
        ServedGroupIDs(vec![MMEGroupID::from([0x00, 0x01])]),
        ServedMMECs(vec![MMECode::from([0x01])]),
        None,
    )]);
    let mme_name = MMEname(PrintableString::try_from("OxiRushMME").expect("valid MME name"));

    // In rasn's stable opaque-open-type representation, every entry holds its
    // numeric ID, criticality, and the APER bytes of its concrete S1AP value.
    let ies = vec![
        AnonymousS1SetupResponseProtocolIEs::new(
            ID_MMENAME,
            Criticality::ignore,
            encode_open_type(&mme_name).expect("encode MME name"),
        ),
        AnonymousS1SetupResponseProtocolIEs::new(
            ID_SERVED_GUMMEIS,
            Criticality::reject,
            encode_open_type(&served_gummeis).expect("encode served GUMMEIs"),
        ),
        AnonymousS1SetupResponseProtocolIEs::new(
            ID_RELATIVE_MMECAPACITY,
            Criticality::ignore,
            encode_open_type(&RelativeMMECapacity(255)).expect("encode MME capacity"),
        ),
    ];
    let response = S1SetupResponse::new(S1SetupResponseProtocolIEs(ies));

    // The outer PDU wraps the APER-encoded message in its direction and
    // ASN.1-derived procedure code.
    let pdu = S1AP_PDU::successfulOutcome(SuccessfulOutcome::new(
        ID_S1_SETUP,
        Criticality::reject,
        encode_open_type(&response).expect("encode S1 Setup Response"),
    ));

    // ── 2. Encode to APER ───────────────────────────────────────────────────
    // `pdu.encode()` is the convenience API; this deliberately uses raw rasn.

    let aper_bytes = rasn::aper::encode(&pdu).expect("APER encode failed");
    println!("Encoded S1SetupResponse: {} bytes", aper_bytes.len());
    println!("APER hex: {}", hex::encode(&aper_bytes));

    // ── 3. Decode from APER ─────────────────────────────────────────────────
    // `S1AP_PDU::decode(&bytes)` is the equivalent convenience API.

    let decoded: S1AP_PDU = rasn::aper::decode(&aper_bytes).expect("APER decode failed");

    println!("\n=== Decoded: {decoded} ===");
    println!(
        "Procedure: {}, Direction: {}\n",
        decoded.procedure_name(),
        decoded.direction()
    );

    // ── 4. Inspect the decoded PDU and its typed open values ────────────────
    //
    // Without extraction macros, match the direction, decode the message open
    // type, iterate `protocol_ies`, then decode each recognized IE open type.

    match &decoded {
        S1AP_PDU::successfulOutcome(outcome) => {
            println!("Type:           SuccessfulOutcome");
            println!("Procedure code: {}", outcome.procedure_code.0);
            println!("Criticality:    {:?}", outcome.criticality);

            let response: S1SetupResponse =
                decode_open_type(&outcome.value).expect("decode S1 Setup Response");
            for ie in &response.protocol_ies.0 {
                match ie.id {
                    ID_MMENAME => {
                        let name: MMEname = decode_open_type(&ie.value).expect("decode MME name");
                        println!(
                            "MME Name:       {}",
                            String::from_utf8_lossy(name.0.as_bytes())
                        );
                    }
                    ID_RELATIVE_MMECAPACITY => {
                        let capacity: RelativeMMECapacity =
                            decode_open_type(&ie.value).expect("decode MME capacity");
                        println!("MME Capacity:   {}", capacity.0);
                    }
                    ID_SERVED_GUMMEIS => {
                        let list: ServedGUMMEIs =
                            decode_open_type(&ie.value).expect("decode served GUMMEIs");
                        println!("Served GUMMEIs: {} entries", list.0.len());
                        for item in &list.0 {
                            for plmn in &item.served_plmns.0 {
                                println!(
                                    "  PLMN: {}",
                                    plmn.0
                                        .0
                                        .iter()
                                        .map(|b| format!("{b:02x}"))
                                        .collect::<String>()
                                );
                            }
                            println!(
                                "  groups: {}, MMECs: {}",
                                item.served_group_ids.0.len(),
                                item.served_mmecs.0.len()
                            );
                        }
                    }
                    _ => println!("  (other IE: id={})", ie.id.0),
                }
            }
        }
        _ => println!("Unexpected PDU type"),
    }

    // ── 5. Verify round-trip ────────────────────────────────────────────────

    let re_encoded = decoded.encode().expect("re-encode failed");
    assert_eq!(aper_bytes, re_encoded, "Round-trip mismatch!");
    println!("\nRound-trip OK ({} bytes)", re_encoded.len());
}
