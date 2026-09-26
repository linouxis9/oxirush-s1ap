//! Convenience helpers for building common S1AP types.
//!
//! These helpers cover PLMN/TBCD encoding, E-UTRAN identifiers, security
//! bit strings, and common MME/eNB identities used by S1AP consumers.

use rasn::types::{BitString, FixedBitString, FixedOctetString};

use crate::s1ap::*;

/// Convert an integer to a most-significant-bit-first bit string of `width` bits.
///
/// # Panics
///
/// Panics when `width` exceeds 64.
pub fn int_to_bitvec(value: u64, width: usize) -> BitString {
    assert!(width <= 64, "width must be <= 64");
    (0..width)
        .rev()
        .map(|bit| (value >> bit) & 1 == 1)
        .collect()
}

/// Convert bytes to a most-significant-bit-first bit string.
pub fn bytes_to_bitvec(bytes: &[u8]) -> BitString {
    bytes
        .iter()
        .flat_map(|byte| (0..8u8).rev().map(move |bit| (byte >> bit) & 1 == 1))
        .collect()
}

fn fixed_bit_string<const N: usize>(value: u64) -> FixedBitString<N> {
    let mut bits = FixedBitString::<N>::ZERO;
    for index in 0..N {
        bits.set(index, (value >> (N - index - 1)) & 1 == 1);
    }
    bits
}

/// Encode MCC/MNC as a 3-octet TBCD `PLMNidentity`.
///
/// MCC must contain three decimal digits; MNC must contain two or three.
pub fn plmn(mcc: &str, mnc: &str) -> PLMNidentity {
    assert!(
        mcc.len() == 3 && mcc.bytes().all(|byte| byte.is_ascii_digit()),
        "MCC must be 3 ASCII digits"
    );
    assert!(
        (mnc.len() == 2 || mnc.len() == 3) && mnc.bytes().all(|byte| byte.is_ascii_digit()),
        "MNC must be 2 or 3 ASCII digits"
    );

    let mcc_digits: Vec<u8> = mcc.bytes().map(|byte| byte - b'0').collect();
    let mnc_digits: Vec<u8> = mnc.bytes().map(|byte| byte - b'0').collect();
    let mut bytes = [0u8; 3];
    bytes[0] = (mcc_digits[1] << 4) | mcc_digits[0];
    if mnc_digits.len() == 2 {
        bytes[1] = 0xF0 | mcc_digits[2];
        bytes[2] = (mnc_digits[1] << 4) | mnc_digits[0];
    } else {
        bytes[1] = (mnc_digits[2] << 4) | mcc_digits[2];
        bytes[2] = (mnc_digits[1] << 4) | mnc_digits[0];
    }
    PLMNidentity(TBCDSTRING(FixedOctetString::new(bytes)))
}

/// Decode a `PLMNidentity` into MCC and MNC strings.
pub fn plmn_from(identity: &PLMNidentity) -> (String, String) {
    let bytes = &identity.0.0;
    let mcc0 = bytes[0] & 0x0F;
    let mcc1 = (bytes[0] >> 4) & 0x0F;
    let mcc2 = bytes[1] & 0x0F;
    let mnc_hi = (bytes[1] >> 4) & 0x0F;
    let mnc0 = bytes[2] & 0x0F;
    let mnc1 = (bytes[2] >> 4) & 0x0F;

    let mcc = format!("{mcc0}{mcc1}{mcc2}");
    let mnc = if mnc_hi == 0xF {
        format!("{mnc0}{mnc1}")
    } else {
        format!("{mnc0}{mnc1}{mnc_hi}")
    };
    (mcc, mnc)
}

/// Build a Globally Unique MME Identifier.
pub fn gummei(plmn_identity: PLMNidentity, mme_group_id: u16, mme_code: u8) -> GUMMEI {
    GUMMEI::new(
        plmn_identity,
        MMEGroupID(FixedOctetString::new(mme_group_id.to_be_bytes())),
        MMECode(FixedOctetString::new([mme_code])),
        None,
    )
}

/// Build an LTE Tracking Area Identity.
///
/// # Panics
///
/// Panics unless `tac` contains exactly two octets.
pub fn tai(plmn_identity: PLMNidentity, tac: &[u8]) -> TAI {
    let tac: [u8; 2] = tac
        .try_into()
        .expect("LTE TAC must contain exactly 2 octets");
    TAI::new(plmn_identity, TAC(FixedOctetString::new(tac)), None)
}

/// Build an E-UTRAN Cell Global Identifier from a 20-bit macro eNB ID and 8-bit cell ID.
pub fn eutran_cgi(plmn_identity: PLMNidentity, enb_id: u32, cell_id: u8) -> EUTRANCGI {
    let eci = (((enb_id & 0x000F_FFFF) as u64) << 8) | u64::from(cell_id);
    EUTRANCGI::new(
        plmn_identity,
        CellIdentity(fixed_bit_string::<28>(eci)),
        None,
    )
}

/// Build a `GlobalENBID` for a 20-bit macro eNB identifier.
pub fn global_enb_id(plmn_identity: PLMNidentity, enb_id: u32) -> GlobalENBID {
    GlobalENBID::new(
        plmn_identity,
        ENBID::macroENB_ID(int_to_bitvec(u64::from(enb_id & 0x000F_FFFF), 20)),
        None,
    )
}

/// Build LTE UE security capabilities from encryption and integrity octets.
pub fn ue_security_capabilities(capabilities: &[u8]) -> UESecurityCapabilities {
    let encryption = capabilities.first().copied().unwrap_or(0);
    let integrity = capabilities.get(1).copied().unwrap_or(0);
    let algorithms = |byte: u8| {
        (0..16u8)
            .map(|index| index < 8 && (byte >> (7 - index)) & 1 == 1)
            .collect()
    };

    UESecurityCapabilities::new(
        EncryptionAlgorithms(algorithms(encryption)),
        IntegrityProtectionAlgorithms(algorithms(integrity)),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_s1ap;

    #[test]
    fn plmn_round_trips_for_two_digit_mnc() {
        let identity = plmn("208", "93");
        assert_eq!(&identity.0.0[..], &[0x02, 0xF8, 0x39]);
        assert_eq!(plmn_from(&identity), ("208".into(), "93".into()));
    }

    #[test]
    fn plmn_round_trips_for_three_digit_mnc() {
        let identity = plmn("999", "070");
        assert_eq!(plmn_from(&identity), ("999".into(), "070".into()));
    }

    #[test]
    fn common_identifiers_have_the_required_widths() {
        let plmn = plmn("208", "93");
        let cgi = eutran_cgi(plmn.clone(), 1, 1);
        assert_eq!(cgi.cell_id.0[..28].len(), 28);
        let enb = global_enb_id(plmn, 1);
        let ENBID::macroENB_ID(bits) = enb.e_nb_id else {
            panic!("expected macro eNB ID");
        };
        assert_eq!(bits.len(), 20);
    }

    #[test]
    fn pdu_encode_decode_round_trip() {
        let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,
            REJECT MME_UE_S1AP_ID(1u32),
            REJECT eNB_UE_S1AP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        );
        let bytes = pdu.encode().expect("encode PDU");
        let decoded = S1AP_PDU::decode(&bytes).expect("decode PDU");
        assert_eq!(pdu, decoded);
        assert_eq!(decoded.procedure_name(), "UEContextReleaseRequest");
        assert!(decoded.is_initiating());
    }
}
