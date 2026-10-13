//! Convenience helpers for building common S1AP types.
//!
//! These helpers cover PLMN/TBCD encoding, E-UTRAN identifiers, security
//! bit strings, and common MME/eNB identities used by S1AP consumers.
//!
//! None of them panics. One that takes a number for a field of a fixed size
//! gives an [`Error`] of the kind `Argument` for a number that does not fit,
//! and cuts none short.

use oxirush_aper::{BitString, Bits};
use oxirush_utils::identity::Plmn;

use crate::Error;
use crate::s1ap::*;

/// `value`, which is `what`, when it fits in `bits` bits.
fn fits(what: &str, value: u64, bits: u32) -> Result<u64, Error> {
    match bits >= 64 || value >> bits == 0 {
        true => Ok(value),
        false => Err(Error::argument(format!(
            "{what} {value} does not fit in {bits} bits"
        ))),
    }
}

/// The octets of the PLMN identity of an MCC and an MNC: the three digits of `mcc`, and
/// the two or three of `mnc`, each a number from 0 to 9. `None` for an MNC of another
/// length, and for a digit over 9.
///
/// The order is that of TS 36.413 clause 9.2.3.8, which `Plmn::to_ap_octets` of oxirush-utils
/// writes: octet 2 has the first digit of a three-digit MNC, and octet 3 its second
/// and its third, so that 310-410 is `13 40 01`. This is not the order of TS 24.008,
/// which a NAS message has, where octet 2 has the third digit of the MNC.
pub(crate) fn plmn_octets(mcc: [u8; 3], mnc: &[u8]) -> Option<[u8; 3]> {
    Some(Plmn::from_digits(mcc, mnc).ok()?.to_ap_octets())
}

/// The digits of the MCC and those of the MNC in the octets of a PLMN identity, in the
/// order of [`plmn_octets`]: the MNC has two after a filler, and three otherwise. `None`
/// for octets that are not a PLMN identity: a half octet is no digit, where it is not
/// the filler before an MNC of two digits.
pub(crate) fn plmn_digits(octets: [u8; 3]) -> Option<(String, String)> {
    let plmn = Plmn::from_ap_octets(octets).ok()?;
    Some((plmn.mcc().to_string(), plmn.mnc().to_string()))
}

/// The digits that `text` is, when it is ASCII digits alone.
fn digits(text: &str) -> Option<Vec<u8>> {
    let digits = text.bytes();
    let digits = digits.map(|digit| digit.is_ascii_digit().then(|| digit - b'0'));
    digits.collect()
}

/// Encode MCC/MNC as the three octets of a `PLMNidentity`, in the order of
/// TS 36.413 clause 9.2.3.8: `plmn("310", "410")` is `13 40 01`, and `plmn("208", "93")`
/// is `02 f8 39`.
///
/// An error of the kind `Argument` unless `mcc` is three decimal digits and `mnc` two
/// or three.
pub fn plmn(mcc: &str, mnc: &str) -> Result<PLMNidentity, Error> {
    let refused = |what: &str, text: &str, has: &str| {
        Error::argument(format!(
            "{text:?} is not {what}: it has {has} digits, 0 to 9"
        ))
    };
    let mcc_digits = digits(mcc).and_then(|digits| <[u8; 3]>::try_from(digits).ok());
    let mcc_digits = mcc_digits.ok_or_else(|| refused("an MCC", mcc, "three"))?;
    let octets = digits(mnc).and_then(|mnc| plmn_octets(mcc_digits, &mnc));
    let octets = octets.ok_or_else(|| refused("an MNC", mnc, "two or three"))?;
    Ok(PLMNidentity(TBCDSTRING(octets)))
}

/// Decode a `PLMNidentity` into MCC and MNC strings, in the order of
/// TS 36.413 clause 9.2.3.8.
///
/// `None` for octets that are not a PLMN identity: a half octet is no digit, where it
/// is not the filler before an MNC of two digits. `ab cd ef` is none, and so is
/// `02 f8 f9`.
pub fn plmn_from(identity: &PLMNidentity) -> Option<(String, String)> {
    plmn_digits(identity.0.0)
}

/// Build a Globally Unique MME Identifier.
pub fn gummei(plmn_identity: PLMNidentity, mme_group_id: u16, mme_code: u8) -> GUMMEI<'static> {
    GUMMEI::new(
        plmn_identity,
        MMEGroupID(mme_group_id.to_be_bytes()),
        MMECode([mme_code]),
        None,
    )
}

/// Build an LTE Tracking Area Identity: a PLMN and the two octets of a TAC.
pub fn tai(plmn_identity: PLMNidentity, tac: [u8; 2]) -> TAI<'static> {
    TAI::new(plmn_identity, TAC(tac), None)
}

/// Build an E-UTRAN Cell Global Identifier from a 20-bit macro eNB ID and 8-bit cell ID.
///
/// An error of the kind `Argument` for an eNB ID that does not fit in 20 bits, which
/// the 8 bits of the cell follow in the 28 of a Cell Identity (TS 36.413 clause
/// 9.2.1.38).
pub fn eutran_cgi(
    plmn_identity: PLMNidentity,
    enb_id: u32,
    cell_id: u8,
) -> Result<EUTRANCGI<'static>, Error> {
    let enb_id = fits("the macro eNB ID", enb_id.into(), 20)?;
    let eci = (enb_id << 8) | u64::from(cell_id);
    Ok(EUTRANCGI::new(
        plmn_identity,
        CellIdentity(Bits::low(eci, 28)),
        None,
    ))
}

/// Build a `GlobalENBID` for a 20-bit macro eNB identifier.
///
/// An error of the kind `Argument` for an eNB ID that does not fit in 20 bits
/// (TS 36.413 clause 9.2.1.37).
pub fn global_enb_id(
    plmn_identity: PLMNidentity,
    enb_id: u32,
) -> Result<GlobalENBID<'static>, Error> {
    let enb_id = fits("the macro eNB ID", enb_id.into(), 20)?;
    Ok(GlobalENBID::new(
        plmn_identity,
        ENBID::macroENB_ID(Bits::low(enb_id, 20)),
        None,
    ))
}

/// Build LTE UE security capabilities from the first octet of the encryption
/// bitmap and the first octet of the integrity bitmap.
///
/// Each is the first octet of an S1AP bitmap, whose leading bit is
/// 128-EEA1 or 128-EIA1 (TS 36.413 §9.2.1.40). The octets of the NAS UE
/// network capability IE lead with EEA0 and EIA0 (TS 24.301 §9.9.3.34), so
/// shift them left by one bit first.
pub fn ue_security_capabilities(encryption: u8, integrity: u8) -> UESecurityCapabilities<'static> {
    // Sixteen bits, of which the first eight are the algorithms.
    let algorithms = |byte: u8| BitString::from([byte, 0]);
    UESecurityCapabilities::new(
        EncryptionAlgorithms(algorithms(encryption)),
        IntegrityProtectionAlgorithms(algorithms(integrity)),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ErrorKind, build_s1ap};

    #[test]
    fn security_capability_octets_lead_with_the_first_algorithm() {
        // 128-EEA1 and 128-EEA2; 128-EIA2.
        let capabilities = ue_security_capabilities(0xC0, 0x40);
        let leading = |bits: &BitString| (0..3).map(|at| bits.get(at).unwrap()).collect::<Vec<_>>();
        assert_eq!(
            leading(&capabilities.encryption_algorithms.0),
            [true, true, false]
        );
        assert_eq!(
            leading(&capabilities.integrity_protection_algorithms.0),
            [false, true, false]
        );
    }

    #[test]
    fn plmn_round_trips_for_two_digit_mnc() {
        let identity = plmn("208", "93").unwrap();
        assert_eq!(identity.0.0, [0x02, 0xF8, 0x39]);
        assert_eq!(plmn_from(&identity), Some(("208".into(), "93".into())));
    }

    #[test]
    fn plmn_round_trips_for_three_digit_mnc() {
        // The digits 9 9 9 0 7 0, two in each octet, the second over the first.
        let identity = plmn("999", "070").unwrap();
        assert_eq!(identity.0.0, [0x99, 0x09, 0x07]);
        assert_eq!(plmn_from(&identity), Some(("999".into(), "070".into())));
    }

    #[test]
    fn what_is_no_mcc_or_no_mnc_is_an_error_of_its_argument() {
        // Two digits and four for an MCC, one and four for an MNC, and what is no digit:
        // a letter, a sign, a digit of another script.
        for (mcc, mnc) in [
            ("20", "93"),
            ("2080", "93"),
            ("208", "9"),
            ("208", "9300"),
            ("20a", "93"),
            ("208", "+3"),
            ("208", "9٣"),
            ("", ""),
        ] {
            let error = plmn(mcc, mnc).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Argument, "{mcc}-{mnc}");
        }
        assert_eq!(
            plmn("20", "93").unwrap_err().to_string(),
            "\"20\" is not an MCC: it has three digits, 0 to 9"
        );
        assert_eq!(
            plmn("208", "9").unwrap_err().to_string(),
            "\"9\" is not an MNC: it has two or three digits, 0 to 9"
        );
        // The octets of the digits alone, for the notation of a PLMN identity.
        assert_eq!(plmn_octets([2, 0, 8], &[9, 3]), Some([0x02, 0xF8, 0x39]));
        assert_eq!(plmn_octets([2, 0, 8], &[9]), None);
        assert_eq!(plmn_octets([2, 0, 10], &[9, 3]), None);
    }

    #[test]
    fn octets_that_are_no_plmn_identity_have_no_mcc_and_no_mnc() {
        // No digit at all, a filler in the MNC, a filler in the MCC, a filler before
        // an MNC that has another one, and a digit over 9.
        for octets in [
            [0xab, 0xcd, 0xef],
            [0x02, 0xf8, 0xf9],
            [0x0f, 0xf8, 0x39],
            [0x02, 0xf8, 0x3f],
            [0x02, 0xa8, 0x39],
        ] {
            let identity = PLMNidentity(TBCDSTRING::from(octets));
            assert_eq!(plmn_from(&identity), None, "{octets:02x?}");
        }
    }

    #[test]
    fn common_identifiers_have_the_required_widths() {
        let plmn = plmn("208", "93").unwrap();
        // The 20-bit eNB ID and the 8-bit cell: a 28-bit cell identity,
        // octet-aligned after the PLMN (X.691 (02/2021) §16.10).
        let cgi = eutran_cgi(plmn, 0x12345, 0x67).unwrap();
        assert_eq!(cgi.cell_id.0, Bits::new(0x123_4567, 28).unwrap());
        assert_eq!(
            oxirush_aper::encode(&cgi).unwrap(),
            [0x00, 0x02, 0xf8, 0x39, 0x12, 0x34, 0x56, 0x70]
        );
        let enb = global_enb_id(plmn, 0x12345).unwrap();
        let ENBID::macroENB_ID(bits) = enb.e_nb_id else {
            panic!("expected macro eNB ID");
        };
        assert_eq!(bits, Bits::new(0x12345, 20).unwrap());
    }

    #[test]
    fn a_number_that_does_not_fit_its_field_is_an_error_and_is_not_cut_short() {
        let network = || plmn("208", "93").unwrap();
        let argument = |error: Error| {
            assert_eq!(error.kind(), ErrorKind::Argument);
            error.to_string()
        };

        // A macro eNB ID of 20 bits, alone and before the 8 bits of a cell: the last
        // value, and the one after it.
        let ENBID::macroENB_ID(last) = global_enb_id(network(), 0xf_ffff).unwrap().e_nb_id else {
            panic!("expected macro eNB ID");
        };
        assert_eq!((last.value(), last.len()), (0xf_ffff, 20));
        assert_eq!(
            argument(global_enb_id(network(), 0x10_0000).unwrap_err()),
            "the macro eNB ID 1048576 does not fit in 20 bits"
        );
        assert!(eutran_cgi(network(), 0xf_ffff, 0xff).is_ok());
        assert_eq!(
            argument(eutran_cgi(network(), 0x10_0000, 1).unwrap_err()),
            "the macro eNB ID 1048576 does not fit in 20 bits"
        );
    }

    #[test]
    fn pdu_encode_decode_round_trip() {
        let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,
            REJECT MME_UE_S1AP_ID(1u32),
            REJECT eNB_UE_S1AP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        )
        .expect("build PDU");
        let bytes = pdu.encode().expect("encode PDU");
        let decoded = S1AP_PDU::decode(&bytes).expect("decode PDU");
        assert_eq!(pdu, decoded);
        assert_eq!(decoded.procedure_name(), "UEContextReleaseRequest");
        assert!(decoded.is_initiating());
    }
}
