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

/// The octets of the PLMN identity of an MCC and an MNC: the three digits of `mcc`, and
/// the two or three of `mnc`, each a number from 0 to 9.
///
/// TS 36.413 clause 9.2.3.8 has two digits in each octet, "bits 4 to 1 of octet n encoding
/// digit 2n-1" and "bits 8 to 5 of octet n encoding digit 2n", of "3 digits from MCC
/// followed by either a filler digit plus 2 digits from MNC" or "3 digits from MNC",
/// where the filler is 1111. So octet 2 has the first digit of a three-digit MNC, and
/// octet 3 its second and its third: 310-410 is `13 40 01`. This is not the order of
/// TS 24.008, which a NAS message has, where octet 2 has the third digit of the MNC.
///
/// # Panics
///
/// Panics if `mnc` is not two or three digits.
pub(crate) fn plmn_octets(mcc: [u8; 3], mnc: &[u8]) -> [u8; 3] {
    // The six digits of the clause: a filler stands before an MNC of two digits.
    let [d1, d2, d3] = mcc;
    let [d4, d5, d6] = match *mnc {
        [first, second] => [0x0F, first, second],
        [first, second, third] => [first, second, third],
        _ => panic!("an MNC has two or three digits"),
    };
    // Digit 2n-1 in bits 4 to 1 of octet n, and digit 2n in its bits 8 to 5.
    [d2 << 4 | d1, d4 << 4 | d3, d6 << 4 | d5]
}

/// The half octets of the MCC and those of the MNC in the octets of a PLMN identity, in
/// the order of [`plmn_octets`]: the MNC has two after a filler, and three otherwise.
pub(crate) fn plmn_digits(octets: [u8; 3]) -> ([u8; 3], Vec<u8>) {
    let [first, second, third] = octets;
    let mnc = match second >> 4 {
        0x0F => vec![third & 0x0F, third >> 4],
        digit => vec![digit, third & 0x0F, third >> 4],
    };
    ([first & 0x0F, first >> 4, second & 0x0F], mnc)
}

/// Encode MCC/MNC as the three octets of a `PLMNidentity`, in the order of
/// TS 36.413 clause 9.2.3.8: `plmn("310", "410")` is `13 40 01`, and `plmn("208", "93")`
/// is `02 f8 39`.
///
/// MCC must contain three decimal digits; MNC must contain two or three.
///
/// # Panics
///
/// Panics if `mcc` is not three ASCII digits or `mnc` not two or three.
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
    let mcc_digits = [mcc_digits[0], mcc_digits[1], mcc_digits[2]];
    let octets = plmn_octets(mcc_digits, &mnc_digits);
    PLMNidentity(TBCDSTRING(FixedOctetString::new(octets)))
}

/// Decode a `PLMNidentity` into MCC and MNC strings, in the order of
/// TS 36.413 clause 9.2.3.8.
pub fn plmn_from(identity: &PLMNidentity) -> (String, String) {
    let octets = &identity.0.0;
    let (mcc, mnc) = plmn_digits([octets[0], octets[1], octets[2]]);
    let digits = |digits: &[u8]| digits.iter().map(u8::to_string).collect();
    (digits(&mcc), digits(&mnc))
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
///
/// Each octet is the first octet of an S1AP bitmap, whose leading bit is
/// 128-EEA1 or 128-EIA1 (TS 36.413 §9.2.1.40). The octets of the NAS UE
/// network capability IE lead with EEA0 and EIA0 (TS 24.301 §9.9.3.34), so
/// shift them left by one bit first.
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

    #[test]
    fn security_capability_octets_lead_with_the_first_algorithm() {
        // 128-EEA1 and 128-EEA2; 128-EIA2.
        let capabilities = ue_security_capabilities(&[0xC0, 0x40]);
        let leading = |bits: &BitString| bits.iter().take(3).map(|bit| *bit).collect::<Vec<_>>();
        assert_eq!(
            leading(&capabilities.encryption_algorithms.0),
            [true, true, false]
        );
        assert_eq!(
            leading(&capabilities.integrity_protection_algorithms.0),
            [false, true, false]
        );
    }
    use crate::build_s1ap;

    #[test]
    fn plmn_round_trips_for_two_digit_mnc() {
        let identity = plmn("208", "93");
        assert_eq!(&identity.0.0[..], &[0x02, 0xF8, 0x39]);
        assert_eq!(plmn_from(&identity), ("208".into(), "93".into()));
    }

    #[test]
    fn plmn_round_trips_for_three_digit_mnc() {
        // The digits 9 9 9 0 7 0, two in each octet, the second over the first.
        let identity = plmn("999", "070");
        assert_eq!(&identity.0.0[..], &[0x99, 0x09, 0x07]);
        assert_eq!(plmn_from(&identity), ("999".into(), "070".into()));
    }

    #[test]
    fn common_identifiers_have_the_required_widths() {
        let plmn = plmn("208", "93");
        // The 20-bit eNB ID and the 8-bit cell: a 28-bit cell identity,
        // octet-aligned after the PLMN (X.691 (02/2021) §16.10).
        let cgi = eutran_cgi(plmn.clone(), 0x12345, 0x67);
        assert_eq!(cgi.cell_id.0[..28], int_to_bitvec(0x123_4567, 28)[..]);
        assert_eq!(
            rasn::aper::encode(&cgi).unwrap(),
            [0x00, 0x02, 0xf8, 0x39, 0x12, 0x34, 0x56, 0x70]
        );
        let enb = global_enb_id(plmn, 0x12345);
        let ENBID::macroENB_ID(bits) = enb.e_nb_id else {
            panic!("expected macro eNB ID");
        };
        assert_eq!(bits, int_to_bitvec(0x12345, 20));
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
