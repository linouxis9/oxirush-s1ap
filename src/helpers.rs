//! Convenience helpers for building common S1AP types.
//!
//! These helpers cover PLMN/TBCD encoding, E-UTRAN identifiers, security
//! bit strings, and common MME/eNB identities used by S1AP consumers.
//!
//! None of them panics. One that takes a number for a field of a fixed size
//! gives an [`Error`] of the kind `Argument` for a number that does not fit,
//! and cuts none short.
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_s1ap::helpers::*;
//!
//! // A PLMN from its MCC and its MNC, and what is identified within it
//! let network = plmn("208", "93").unwrap();
//! let mme = gummei(network, 1, 1); // group, code
//! let tracking_area = tai(network, [0x00, 0x01]); // TAC
//! let cell = eutran_cgi(network, EnbKind::Macro, 0x12345, 7).unwrap(); // eNB, cell
//! let enb = global_enb_id(network, EnbKind::Macro, 0x12345).unwrap();
//! let home = global_enb_id(network, EnbKind::Home, 0x123_4567).unwrap();
//!
//! // A bit string from octets, and the algorithms of a UE: the first octet
//! // of its encryption bitmap, then of its integrity bitmap
//! let algorithm_mask = oxirush_s1ap::BitString::from([0xe0]);
//! let capabilities = ue_security_capabilities(0xe0, 0xe0);
//!
//! // What is no MCC, and a macro eNB ID that does not fit in its 20 bits
//! println!("{}", plmn("20", "93").unwrap_err());
//! println!("{}", global_enb_id(network, EnbKind::Macro, 0x100000).unwrap_err());
//! # assert_eq!(printed, r##""20" is not an MCC: it has three digits, 0 to 9
//! # the macro eNB ID 1048576 does not fit in 20 bits
//! # "##);
//! ```
//!
//! ```text
//! "20" is not an MCC: it has three digits, 0 to 9
//! the macro eNB ID 1048576 does not fit in 20 bits
//! ```
//!
//! A PLMN identity has its digits in the order of TS 36.413 clause 9.2.3.8:
//! `plmn("310", "410")` is the octets `13 40 01`, where a NAS message has
//! `13 00 14`. [`plmn_from`] reads the MCC and the MNC of a PLMN identity.

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
///
/// ```
/// use oxirush_s1ap::helpers::{plmn, plmn_from};
///
/// let network = plmn("310", "410")?;
/// assert_eq!(network.0.0, [0x13, 0x40, 0x01]);
/// assert_eq!(plmn_from(&network), Some(("310".into(), "410".into())));
/// # Ok::<(), oxirush_s1ap::Error>(())
/// ```
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
///
/// ```
/// use oxirush_s1ap::helpers::plmn_from;
/// use oxirush_s1ap::s1ap::{PLMNidentity, TBCDSTRING};
///
/// let network = PLMNidentity(TBCDSTRING([0x02, 0xf8, 0x39]));
/// assert_eq!(plmn_from(&network), Some(("208".into(), "93".into())));
/// assert_eq!(plmn_from(&PLMNidentity(TBCDSTRING([0xab, 0xcd, 0xef]))), None);
/// ```
pub fn plmn_from(identity: &PLMNidentity) -> Option<(String, String)> {
    plmn_digits(identity.0.0)
}

/// Build a Globally Unique MME Identifier.
pub fn gummei(plmn_identity: PLMNidentity, mme_group_id: u16, mme_code: u8) -> GUMMEI {
    GUMMEI::new(
        plmn_identity,
        MMEGroupID(mme_group_id.to_be_bytes()),
        MMECode([mme_code]),
        None,
    )
}

/// Build an LTE Tracking Area Identity: a PLMN and the two octets of a TAC.
pub fn tai(plmn_identity: PLMNidentity, tac: [u8; 2]) -> TAI {
    TAI::new(plmn_identity, TAC(tac), None)
}

/// The kinds of eNB ID of TS 36.413 clause 9.2.1.37, each of the bits that it has: the
/// leftmost bits of the 28 of the Cell Identity of each cell that the eNB serves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnbKind {
    /// A macro eNB ID, of 20 bits, which leaves 8 bits for the cell.
    Macro,
    /// A home eNB ID, of 28 bits: the Cell Identity of its one cell.
    Home,
    /// A short macro eNB ID, of 18 bits, which leaves 10 bits for the cell.
    ShortMacro,
    /// A long macro eNB ID, of 21 bits, which leaves 7 bits for the cell.
    LongMacro,
}

impl EnbKind {
    /// The bits of an eNB ID of this kind.
    pub const fn bits(self) -> u32 {
        match self {
            Self::Macro => 20,
            Self::Home => 28,
            Self::ShortMacro => 18,
            Self::LongMacro => 21,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Macro => "the macro eNB ID",
            Self::Home => "the home eNB ID",
            Self::ShortMacro => "the short macro eNB ID",
            Self::LongMacro => "the long macro eNB ID",
        }
    }
}

/// Build the eNB ID of a kind.
///
/// An error of the kind `Argument` for an eNB ID that does not fit in the bits of its
/// kind.
pub fn enb_id(kind: EnbKind, enb_id: u32) -> Result<ENBID, Error> {
    let bits = kind.bits();
    let id = Bits::low(fits(kind.name(), enb_id.into(), bits)?, bits as usize);
    Ok(match kind {
        EnbKind::Macro => ENBID::macroENB_ID(id),
        EnbKind::Home => ENBID::homeENB_ID(id),
        EnbKind::ShortMacro => ENBID::short_macroENB_ID(id),
        EnbKind::LongMacro => ENBID::long_macroENB_ID(id),
    })
}

/// Build an E-UTRAN Cell Global Identifier from an eNB ID of a kind and the cell that
/// the rest of the 28 bits of the Cell Identity hold: the eNB ID is its leftmost bits
/// (TS 36.413 clauses 9.2.1.37 and 9.2.1.38), and a macro eNB ID leaves 8 bits for the
/// cell, a home eNB ID none.
///
/// An error of the kind `Argument` for an eNB ID that does not fit in the bits of its
/// kind, and for a cell that does not fit in the bits after it.
pub fn eutran_cgi(
    plmn_identity: PLMNidentity,
    kind: EnbKind,
    enb_id: u32,
    cell_id: u16,
) -> Result<EUTRANCGI, Error> {
    let bits = kind.bits();
    let enb_id = fits(kind.name(), enb_id.into(), bits)?;
    let cell_id = fits("the cell of the eNB", cell_id.into(), 28 - bits)?;
    let eci = (enb_id << (28 - bits)) | cell_id;
    Ok(EUTRANCGI::new(
        plmn_identity,
        CellIdentity(Bits::low(eci, 28)),
        None,
    ))
}

/// Build a `GlobalENBID` for an eNB ID of a kind (TS 36.413 clause 9.2.1.37).
///
/// An error of the kind `Argument` for an eNB ID that does not fit in the bits of its
/// kind.
pub fn global_enb_id(
    plmn_identity: PLMNidentity,
    kind: EnbKind,
    enb_id: u32,
) -> Result<GlobalENBID, Error> {
    Ok(GlobalENBID::new(
        plmn_identity,
        self::enb_id(kind, enb_id)?,
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

    /// Each kind of eNB ID of TS 36.413 clause 9.2.1.37 has its bits, the leftmost of the
    /// Cell Identity, and the cell the rest of the 28.
    #[test]
    fn each_kind_of_enb_id_has_its_bits_and_the_cell_the_rest() {
        let network = plmn("208", "93").unwrap();
        for (kind, bits) in [
            (EnbKind::Macro, 20),
            (EnbKind::Home, 28),
            (EnbKind::ShortMacro, 18),
            (EnbKind::LongMacro, 21),
        ] {
            assert_eq!(kind.bits(), bits);
            let most = (1u32 << bits) - 1;
            let id = match enb_id(kind, most).unwrap() {
                ENBID::macroENB_ID(id) => (EnbKind::Macro, id),
                ENBID::homeENB_ID(id) => (EnbKind::Home, id),
                ENBID::short_macroENB_ID(id) => (EnbKind::ShortMacro, id),
                ENBID::long_macroENB_ID(id) => (EnbKind::LongMacro, id),
            };
            assert_eq!(
                (id.0, id.1.len(), id.1.value()),
                (kind, bits as usize, u64::from(most))
            );
            assert!(enb_id(kind, most + 1).is_err());
            let global = global_enb_id(network, kind, 1).unwrap();
            assert_eq!(global.enb_id, enb_id(kind, 1).unwrap());
            // The eNB ID, then the cell.
            let cell = (1u16 << (28 - bits)) - 1;
            let cgi = eutran_cgi(network, kind, 1, cell).unwrap();
            assert_eq!(cgi.cell_id.0.value(), (1 << (28 - bits)) | u64::from(cell));
            assert!(eutran_cgi(network, kind, 1, cell + 1).is_err());
            assert!(eutran_cgi(network, kind, most + 1, 0).is_err());
        }
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
        let cgi = eutran_cgi(plmn, EnbKind::Macro, 0x12345, 0x67).unwrap();
        assert_eq!(cgi.cell_id.0, Bits::new(0x123_4567, 28).unwrap());
        assert_eq!(
            oxirush_aper::encode(&cgi).unwrap(),
            [0x00, 0x02, 0xf8, 0x39, 0x12, 0x34, 0x56, 0x70]
        );
        let enb = global_enb_id(plmn, EnbKind::Macro, 0x12345).unwrap();
        let ENBID::macroENB_ID(bits) = enb.enb_id else {
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
        let ENBID::macroENB_ID(last) = global_enb_id(network(), EnbKind::Macro, 0xf_ffff)
            .unwrap()
            .enb_id
        else {
            panic!("expected macro eNB ID");
        };
        assert_eq!((last.value(), last.len()), (0xf_ffff, 20));
        assert_eq!(
            argument(global_enb_id(network(), EnbKind::Macro, 0x10_0000).unwrap_err()),
            "the macro eNB ID 1048576 does not fit in 20 bits"
        );
        assert!(eutran_cgi(network(), EnbKind::Macro, 0xf_ffff, 0xff).is_ok());
        assert_eq!(
            argument(eutran_cgi(network(), EnbKind::Macro, 0x10_0000, 1).unwrap_err()),
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
