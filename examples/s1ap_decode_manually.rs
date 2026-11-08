//! Encode and decode S1AP PDUs **without macros** using the codec and the generated types.
//!
//! This example demonstrates the complete raw-codec workflow:
//! - Constructing an `S1APPDU` by hand (`ProtocolIEID`, `Criticality`, the octets of
//!   an open type)
//! - Encoding and decoding with the functions of the aligned PER runtime
//! - Decoding and pattern-matching open protocol IE values
//! - Verifying encode → decode round-trip fidelity
//!
//! `oxirush_s1ap::aper` is the runtime that the crate re-exports, `oxirush-aper`, so
//! the example needs no dependency on it.
//!
//! For the ergonomic `build_s1ap!` / `extract_s1ap_ies!` versions, see
//! `s1ap_build_pdu.rs` and `s1ap_extract_ies.rs`.

use oxirush_s1ap::aper;
use oxirush_s1ap::s1ap::*;

fn main() {
    // ── 1. Build an S1SetupResponse by hand ─────────────────────────────────
    //
    // One ServedGUMMEIs item identifies its served PLMNs, MME group IDs,
    // and MMECs.

    let plmn_identity = PLMNidentity(TBCDSTRING([0x02, 0xF8, 0x39]));
    let served_gummeis = ServedGUMMEIs(vec![ServedGUMMEIsItem {
        served_plmns: ServedPLMNs(vec![plmn_identity]),
        served_group_ids: ServedGroupIDs(vec![MMEGroupID::from([0x00, 0x01])]),
        served_mmecs: ServedMMECs(vec![MMECode::from([0x01])]),
        ie_extensions: None,
    }]);

    let mme_name = MMEname("mme-1".into());

    // The value of an IE is an open type: every entry holds its numeric ID, its
    // criticality, and the APER octets of its concrete S1AP value.
    let ies = vec![
        ProtocolIEField {
            id: ID_MMENAME,
            criticality: Criticality::Ignore,
            value: oxirush_s1ap::encode(&mme_name)
                .expect("encode MME name")
                .into(),
        },
        ProtocolIEField {
            id: ID_SERVED_GUMMEIS,
            criticality: Criticality::Reject,
            value: oxirush_s1ap::encode(&served_gummeis)
                .expect("encode served GUMMEIs")
                .into(),
        },
        ProtocolIEField {
            id: ID_RELATIVE_MMECAPACITY,
            criticality: Criticality::Ignore,
            value: oxirush_s1ap::encode(&RelativeMMECapacity(255))
                .expect("encode MME capacity")
                .into(),
        },
    ];
    let response = S1SetupResponse {
        protocol_ies: ProtocolIEContainer(ies),
    };

    // The outer PDU wraps the APER-encoded message in its direction and
    // ASN.1-derived procedure code.
    let pdu = S1APPDU::successfulOutcome(SuccessfulOutcome {
        procedure_code: ID_S1_SETUP,
        criticality: Criticality::Reject,
        value: oxirush_s1ap::encode(&response)
            .expect("encode S1 Setup Response")
            .into(),
    });

    // ── 2. Encode to APER ───────────────────────────────────────────────────
    // `pdu.encode()` is the convenience API; this deliberately uses the runtime.

    let aper_bytes = aper::encode(&pdu).expect("APER encode failed");

    println!("Encoded S1SetupResponse: {} bytes", aper_bytes.len());
    println!("APER hex: {}", hex::encode(&aper_bytes));

    // ── 3. Decode from APER ─────────────────────────────────────────────────
    // `S1APPDU::decode(&bytes)` is the equivalent convenience API.

    // The PDU borrows the octets of its message from `aper_bytes`: nothing is copied.
    let decoded: S1APPDU = aper::decode(&aper_bytes).expect("APER decode failed");

    println!("\n=== Decoded: {decoded} ===");
    println!(
        "Message: {:?}, Direction: {}\n",
        decoded.kind(),
        decoded.direction()
    );

    // ── 4. Inspect the decoded PDU and its typed open values ────────────────
    //
    // Without extraction macros, match the direction, decode the message open
    // type, iterate `protocol_ies`, then decode each recognized IE open type.

    match &decoded {
        S1APPDU::successfulOutcome(outcome) => {
            println!("Type:           SuccessfulOutcome");
            println!("Procedure code: {}", outcome.procedure_code.0);
            println!("Criticality:    {:?}", outcome.criticality);

            let response: S1SetupResponse =
                oxirush_s1ap::decode(&outcome.value).expect("decode S1 Setup Response");

            for ie in &response.protocol_ies.0 {
                match ie.id {
                    ID_MMENAME => {
                        let name: MMEname =
                            oxirush_s1ap::decode(&ie.value).expect("decode MME name");
                        println!("MME Name:       {}", name.0);
                    }

                    ID_RELATIVE_MMECAPACITY => {
                        let capacity: RelativeMMECapacity =
                            oxirush_s1ap::decode(&ie.value).expect("decode MME capacity");

                        println!("MME Capacity:   {}", capacity.0);
                    }

                    ID_SERVED_GUMMEIS => {
                        let list: ServedGUMMEIs =
                            oxirush_s1ap::decode(&ie.value).expect("decode served GUMMEIs");

                        println!("Served GUMMEIs: {} entries", list.0.len());
                        for item in &list.0 {
                            for plmn in &item.served_plmns.0 {
                                let octets = &plmn.0.0;
                                let plmn_hex: String =
                                    octets.iter().map(|b| format!("{b:02x}")).collect();
                                println!("  PLMN: {plmn_hex}");
                            }

                            let groups = item.served_group_ids.0.len();
                            let mmecs = item.served_mmecs.0.len();
                            println!("  groups: {groups}, MMECs: {mmecs}");
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
