//! Convenience helpers for building common S1AP types.
//!
//! These helpers cover PLMN/TBCD encoding, E-UTRAN identifiers, security
//! bit strings, and common MME/eNB identities used by S1AP consumers, and the
//! seconds of a Time to Wait ([`TimeToWait::seconds`]).
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
//! use oxirush_s1ap::{Bits, s1ap::ENBID};
//!
//! // A PLMN from its MCC and its MNC, and what is identified within it
//! let network = plmn("208", "93").unwrap();
//! let mme = gummei(network, 1, 1); // group, code
//! let tracking_area = tai(network, [0x00, 0x01]); // TAC
//! let enb = ENBID::macroENB_ID(Bits::new(0x12345, 20).unwrap());
//! let cell = eutran_cgi(network, &enb, 7).unwrap(); // eNB, cell
//!
//! // A bit string from octets, and the algorithms of a UE: the first octet
//! // of its encryption bitmap, then of its integrity bitmap
//! let algorithm_mask = oxirush_s1ap::BitString::from([0xe0]);
//! let capabilities = ue_security_capabilities(0xe0, 0xe0);
//!
//! // What is no MCC, and a cell that does not fit in the 8 bits after a macro eNB ID
//! println!("{}", plmn("20", "93").unwrap_err());
//! println!("{}", eutran_cgi(network, &enb, 0x100).unwrap_err());
//! # assert_eq!(printed, r##"`mcc` has 2 digits, and must have 3
//! # the cell of the eNB 256 does not fit in 8 bits
//! # "##);
//! ```
//!
//! ```text
//! `mcc` has 2 digits, and must have 3
//! the cell of the eNB 256 does not fit in 8 bits
//! ```
//!
//! A PLMN identity has its digits in the order of TS 36.413 clause 9.2.3.8:
//! `plmn("310", "410")` is the octets `13 40 01`, where a NAS message has
//! `13 00 14`. The `Plmn` of [`identity`](crate::identity) reads the MCC and the
//! MNC of a PLMN identity.

use oxirush_aper::{BitString, Bits};
use oxirush_utils::identity::Plmn;

use crate::s1ap::*;
use crate::{Error, ErrorKind};

/// `value`, which is `what`, when it fits in `bits` bits.
fn fits(what: &str, value: u64, bits: u32) -> Result<u64, Error> {
    match bits >= 64 || value >> bits == 0 {
        true => Ok(value),
        false => Err(Error::new(
            ErrorKind::Argument,
            format!("{what} {value} does not fit in {bits} bits"),
        )),
    }
}

/// The PLMN identity of an MCC and an MNC, in the order of TS 36.413 clause 9.2.3.8:
/// `plmn("310", "410")` is `13 40 01`, and `plmn("208", "93")` is `02 f8 39`. The
/// `Plmn` of oxirush-utils converts to and from a `PLMNidentity`, and reads its MCC and
/// its MNC.
///
/// An error of the kind `Argument` unless `mcc` is three decimal digits and `mnc` two
/// or three.
///
/// ```
/// use oxirush_s1ap::helpers::plmn;
/// use oxirush_s1ap::identity::Plmn;
///
/// let network = plmn("310", "410")?;
/// assert_eq!(network.0.0, [0x13, 0x40, 0x01]);
/// let read = Plmn::try_from(network).unwrap();
/// assert_eq!((read.mcc(), read.mnc()), ("310", "410"));
/// # Ok::<(), oxirush_s1ap::Error>(())
/// ```
pub fn plmn(mcc: &str, mnc: &str) -> Result<PLMNidentity, Error> {
    let plmn =
        Plmn::new(mcc, mnc).map_err(|error| Error::new(ErrorKind::Argument, error.to_string()))?;
    Ok(PLMNidentity::from(plmn))
}

/// Build a Globally Unique MME Identifier: a PLMN, an MME Group ID and an MME Code.
///
/// ```
/// use oxirush_s1ap::helpers::{gummei, plmn};
///
/// // MME Group 2 and MME Code 1, in PLMN 208-93
/// let mme = gummei(plmn("208", "93")?, 2, 1);
/// assert_eq!((mme.mme_group_id.0, mme.mme_code.0), ([0x00, 0x02], [0x01]));
/// # Ok::<(), oxirush_s1ap::Error>(())
/// ```
pub fn gummei(plmn_identity: PLMNidentity, mme_group_id: u16, mme_code: u8) -> GUMMEI {
    GUMMEI {
        plmn_identity,
        mme_group_id: MMEGroupID(mme_group_id.to_be_bytes()),
        mme_code: MMECode([mme_code]),
        ie_extensions: None,
    }
}

/// Build an LTE Tracking Area Identity: a PLMN and the two octets of a TAC.
///
/// ```
/// use oxirush_s1ap::helpers::{plmn, tai};
///
/// // TAC 1 of PLMN 208-93
/// let area = tai(plmn("208", "93")?, [0x00, 0x01]);
/// assert_eq!(area.plmn_identity.0.0, [0x02, 0xf8, 0x39]);
/// assert_eq!(area.tac.0, [0x00, 0x01]);
/// # Ok::<(), oxirush_s1ap::Error>(())
/// ```
pub fn tai(plmn_identity: PLMNidentity, tac: [u8; 2]) -> TAI {
    TAI {
        plmn_identity,
        tac: TAC(tac),
        ie_extensions: None,
    }
}

/// Build an E-UTRAN Cell Global Identifier from an eNB ID and the cell that the rest of
/// the 28 bits of the Cell Identity hold: the eNB ID is its leftmost bits, 20, 28, 18 or
/// 21 as its alternative has them (TS 36.413 clauses 9.2.1.37 and 9.2.1.38), and a macro
/// eNB ID leaves 8 bits for the cell, a home eNB ID none.
///
/// An error of the kind `Argument` for a cell that does not fit in the bits after the
/// eNB ID.
///
/// ```
/// use oxirush_s1ap::helpers::{eutran_cgi, plmn};
/// use oxirush_s1ap::{Bits, s1ap::ENBID};
///
/// // Cell 1 of the macro eNB 0x12345, of 20 bits: 8 bits are left for the cell
/// let enb = ENBID::macroENB_ID(Bits::new(0x12345, 20)?);
/// let cell = eutran_cgi(plmn("208", "93")?, &enb, 1)?;
/// assert_eq!(cell.cell_id.0.value(), 0x1234_501);
///
/// // A cell of 9 bits beside a macro eNB ID
/// let error = eutran_cgi(plmn("208", "93")?, &enb, 0x100).unwrap_err();
/// assert_eq!(error.to_string(), "the cell of the eNB 256 does not fit in 8 bits");
/// # Ok::<(), oxirush_s1ap::Error>(())
/// ```
pub fn eutran_cgi(
    plmn_identity: PLMNidentity,
    enb_id: &ENBID,
    cell_id: u16,
) -> Result<EUTRANCGI, Error> {
    let (ENBID::macroENB_ID(id)
    | ENBID::homeENB_ID(id)
    | ENBID::short_macroENB_ID(id)
    | ENBID::long_macroENB_ID(id)) = enb_id;
    let bits = id.len() as u32;
    let cell_id = fits("the cell of the eNB", cell_id.into(), 28 - bits)?;
    let eci = (id.value() << (28 - bits)) | cell_id;
    Ok(EUTRANCGI {
        plmn_identity,
        cell_id: CellIdentity(Bits::low(eci, 28)),
        ie_extensions: None,
    })
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
    UESecurityCapabilities {
        encryption_algorithms: EncryptionAlgorithms(algorithms(encryption)),
        integrity_protection_algorithms: IntegrityProtectionAlgorithms(algorithms(integrity)),
        ie_extensions: None,
    }
}

impl TimeToWait {
    /// The time that the node is to wait at least before it starts the procedure again,
    /// in seconds (TS 36.413 clause 9.2.1.61): that of an S1 SETUP FAILURE, an ENB
    /// CONFIGURATION UPDATE FAILURE or an MME CONFIGURATION UPDATE FAILURE.
    ///
    /// ```
    /// use oxirush_s1ap::s1ap::TimeToWait;
    /// use std::time::Duration;
    ///
    /// let wait = Duration::from_secs(TimeToWait::v10s.seconds());
    /// assert_eq!(wait, Duration::from_secs(10));
    /// ```
    pub fn seconds(self) -> u64 {
        match self {
            Self::v1s => 1,
            Self::v2s => 2,
            Self::v5s => 5,
            Self::v10s => 10,
            Self::v20s => 20,
            Self::v60s => 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ErrorKind, build_s1ap};

    /// TS 36.413 clause 9.2.1.61: ENUMERATED (1s, 2s, 5s, 10s, 20s, 60s).
    #[test]
    fn a_time_to_wait_is_its_seconds() {
        let seconds = TimeToWait::VALUES.map(TimeToWait::seconds);
        assert_eq!(seconds, [1, 2, 5, 10, 20, 60]);
    }

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
    fn a_plmn_identity_has_the_order_of_the_application_protocols() {
        let read = |identity: PLMNidentity| {
            let plmn = Plmn::try_from(identity).unwrap();
            (plmn.mcc().to_string(), plmn.mnc().to_string())
        };
        let identity = plmn("208", "93").unwrap();
        assert_eq!(identity.0.0, [0x02, 0xF8, 0x39]);
        assert_eq!(read(identity), ("208".into(), "93".into()));
        // The digits 9 9 9 0 7 0, two in each octet, the second over the first.
        let identity = plmn("999", "070").unwrap();
        assert_eq!(identity.0.0, [0x99, 0x09, 0x07]);
        assert_eq!(read(identity), ("999".into(), "070".into()));
        // What is no MCC or no MNC is an error of the argument.
        for (mcc, mnc) in [("20", "93"), ("208", "9300"), ("20a", "93"), ("", "")] {
            let error = plmn(mcc, mnc).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Argument, "{mcc}-{mnc}");
        }
    }

    /// Each alternative of an eNB ID of TS 36.413 clause 9.2.1.37 has its bits, the
    /// leftmost of the Cell Identity, and the cell the rest of the 28.
    #[test]
    fn each_enb_id_has_its_bits_and_the_cell_the_rest() {
        let network = plmn("208", "93").unwrap();
        for (enb, bits) in [
            (ENBID::macroENB_ID as fn(Bits) -> ENBID, 20),
            (ENBID::homeENB_ID, 28),
            (ENBID::short_macroENB_ID, 18),
            (ENBID::long_macroENB_ID, 21),
        ] {
            let id = enb(Bits::new(1, bits).unwrap());
            let cell = ((1u32 << (28 - bits)) - 1) as u16;
            let cgi = eutran_cgi(network, &id, cell).unwrap();
            assert_eq!(cgi.cell_id.0.value(), (1 << (28 - bits)) | u64::from(cell));
            let beyond = u32::from(cell) + 1;
            if let Ok(beyond) = u16::try_from(beyond) {
                assert_eq!(
                    eutran_cgi(network, &id, beyond).unwrap_err().to_string(),
                    format!(
                        "the cell of the eNB {beyond} does not fit in {} bits",
                        28 - bits
                    )
                );
            }
        }
        // The 20-bit eNB ID and the 8-bit cell: a 28-bit cell identity, octet-aligned
        // after the PLMN (X.691 (02/2021) §16.10).
        let enb = ENBID::macroENB_ID(Bits::new(0x12345, 20).unwrap());
        let cgi = eutran_cgi(network, &enb, 0x67).unwrap();
        assert_eq!(cgi.cell_id.0, Bits::new(0x123_4567, 28).unwrap());
        assert_eq!(
            oxirush_aper::encode(&cgi).unwrap(),
            [0x00, 0x02, 0xf8, 0x39, 0x12, 0x34, 0x56, 0x70]
        );
    }

    #[test]
    fn pdu_encode_decode_round_trip() {
        let pdu = build_s1ap!(IGNORE UEContextReleaseRequest,
            REJECT MME_UE_S1AP_ID(1u32),
            REJECT eNB_UE_S1AP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        )
        .expect("build PDU");
        let bytes = pdu.encode().expect("encode PDU");
        let decoded = S1APPDU::decode(&bytes).expect("decode PDU");
        assert_eq!(pdu, decoded);
        assert_eq!(decoded.kind(), crate::S1apPduKind::UEContextReleaseRequest);
        assert_eq!(decoded.direction(), crate::Direction::Initiating);
    }
}
