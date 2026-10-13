//! `Debug` prints no octet of a key: of K_eNB and of the next hop NH, which are a
//! `SecurityKey`, it shows the length, as does every value that holds one. The octets of
//! an IE and of a message whose type holds a key are shown by their length too. A tree
//! and a view are the data of a message, and have the key.

use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{Error, build_s1ap, extract_s1ap_ies};

/// A key of which every octet is `5a`, and one of which every octet is `a5`.
const KENB: [u8; 32] = [0x5a; 32];
const NH: [u8; 32] = [0xa5; 32];

/// What `Debug` printed has no four octets of either key in hexadecimal, in either
/// case, nor as numbers.
fn has_no_key(printed: &str) -> bool {
    let lower = printed.to_ascii_lowercase();
    !["5a5a5a5a", "a5a5a5a5", "90, 90, 90", "165, 165, 165"]
        .iter()
        .any(|key| lower.contains(key))
}

#[test]
fn debug_of_a_key_and_of_what_holds_one_shows_its_length() {
    let key = SecurityKey::from(KENB);
    assert_eq!(format!("{key:?}"), "SecurityKey(256 bits)");
    let context = SecurityContext::new(1, SecurityKey::from(NH), None);
    let printed = format!("{context:?}");
    assert!(has_no_key(&printed), "{printed}");
    assert!(
        printed.contains("next_hop_parameter: SecurityKey(256 bits)"),
        "{printed}"
    );
    // The key is the value that it was, and compares as such.
    assert_eq!(key, SecurityKey::from(KENB));
    assert_ne!(key, SecurityKey::from(NH));
}

#[test]
fn debug_of_a_message_with_a_key_shows_the_length_of_its_octets() -> Result<(), Error> {
    // An INITIAL CONTEXT SETUP REQUEST with K_eNB
    let pdu = build_s1ap!(InitiatingMessage, InitialContextSetup, REJECT, InitialContextSetupRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT SecurityKey(SecurityKey::from(KENB)),
    )?;
    let octets = pdu.encode()?;
    assert!(hex::encode(&octets).contains(&"5a".repeat(32)));

    // The PDU, which owns its octets or borrows them
    for pdu in [
        S1AP_PDU::decode(&octets)?,
        S1AP_PDU::decode_borrowed(&octets)?,
    ] {
        let printed = format!("{pdu:?}");
        assert!(has_no_key(&printed), "{printed}");
        assert!(
            printed.contains(&format!("value: {} octets", octets.len() - 4)),
            "{printed}"
        );
        assert_eq!(
            pdu.to_string(),
            "InitiatingMessage InitialContextSetup (code=9)"
        );

        // Its IEs: the two identifiers are printed, and the key is 32 octets
        let ies = pdu.protocol_ies()?;
        let printed = format!("{ies:?}");
        assert!(has_no_key(&printed), "{printed}");
        assert!(printed.contains("value: 0001"), "{printed}");
        assert!(
            printed.contains("id: ProtocolIEID(73), criticality: reject, value: 32 octets"),
            "{printed}"
        );

        // Its message, and the key that is read from it
        let message: InitialContextSetupRequest = pdu.decode_value()?;
        let printed = format!("{message:?}");
        assert!(has_no_key(&printed), "{printed}");
        extract_s1ap_ies!(message, InitialContextSetupRequest,
            req key: SecurityKey = SecurityKey(key) => key,
        );
        assert_eq!(format!("{key:?}"), "SecurityKey(256 bits)");
        assert_eq!(key, SecurityKey::from(KENB));
    }
    Ok(())
}

#[test]
fn debug_of_a_message_with_a_security_context_shows_no_next_hop() -> Result<(), Error> {
    // A PATH SWITCH REQUEST ACKNOWLEDGE with the next hop NH
    let context = SecurityContext::new(1, SecurityKey::from(NH), None);
    let pdu = build_s1ap!(SuccessfulOutcome, PathSwitchRequest, REJECT, PathSwitchRequestAcknowledge,
        IGNORE MME_UE_S1AP_ID(1u32),
        IGNORE eNB_UE_S1AP_ID(7u32),
        REJECT SecurityContext(context),
    )?;
    assert!(hex::encode(pdu.encode()?).contains(&"a5".repeat(32)));
    let printed = format!("{pdu:?} {:?}", pdu.protocol_ies()?);
    assert!(has_no_key(&printed), "{printed}");
    assert!(
        printed.contains("id: ProtocolIEID(40), criticality: reject, value: 33 octets"),
        "{printed}"
    );
    Ok(())
}

#[test]
fn debug_of_a_message_without_a_key_shows_its_octets() -> Result<(), Error> {
    // An UPLINK NAS TRANSPORT: its object set has no IE that holds a key
    let pdu = build_s1ap!(InitiatingMessage, uplinkNASTransport, IGNORE, UplinkNASTransport,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT NAS_PDU(vec![0x7e, 0x00, 0x5d]),
    )?;
    let printed = format!("{pdu:?}");
    assert!(printed.contains("7E005D"), "{printed}");
    let printed = format!("{:?}", pdu.protocol_ies()?);
    assert!(printed.contains("value: 037E005D"), "{printed}");
    Ok(())
}

#[cfg(feature = "inspect")]
#[test]
fn a_tree_and_a_view_have_the_key() -> Result<(), Error> {
    let pdu = build_s1ap!(InitiatingMessage, InitialContextSetup, REJECT, InitialContextSetupRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT SecurityKey(SecurityKey::from(KENB)),
    )?;
    let key = "5a".repeat(32);
    for written in [pdu.to_tree()?, pdu.to_view()?] {
        let written = written.to_string().to_ascii_lowercase();
        assert!(written.contains(&key), "{written}");
    }
    Ok(())
}
