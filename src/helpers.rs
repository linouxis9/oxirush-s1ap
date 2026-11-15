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
//! // A bit string from octets, and the algorithms of a UE: the value of its
//! // NAS UE network capability IE
//! let algorithm_mask = oxirush_s1ap::BitString::from([0xe0]);
//! let capabilities = ue_security_capabilities(&[0xf0, 0xf0]);
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

/// Build the UE Security Capabilities of a UE from the value of its NAS UE
/// network capability IE (TS 24.301 §9.9.3.34), whose first two octets are
/// its EPS encryption and integrity algorithms.
///
/// A NAS octet leads with EEA0 or EIA0, which an S1AP bitmap leaves out: bits
/// 7 to 1 of the octet are the first to seventh bits of the bitmap, which
/// starts at 128-EEA1 or 128-EIA1 (TS 36.413 §9.2.1.40). An octet that the
/// value does not have gives a bitmap of no algorithm. Each bitmap has sixteen
/// bits.
///
/// ```
/// use oxirush_s1ap::helpers::ue_security_capabilities;
///
/// // EEA0 to 128-EEA3 and EIA0 to 128-EIA3, then the UMTS algorithms
/// let capabilities = ue_security_capabilities(&[0xf0, 0xf0, 0x80, 0x80]);
/// assert_eq!(capabilities.encryption_algorithms.0.as_octets(), [0xe0, 0x00]);
/// assert_eq!(capabilities.integrity_protection_algorithms.0.as_octets(), [0xe0, 0x00]);
/// ```
pub fn ue_security_capabilities(nas: &[u8]) -> UESecurityCapabilities<'static> {
    let bitmap = |octet: usize| BitString::from([nas.get(octet).copied().unwrap_or(0) << 1, 0]);
    UESecurityCapabilities {
        encryption_algorithms: EncryptionAlgorithms(bitmap(0)),
        integrity_protection_algorithms: IntegrityProtectionAlgorithms(bitmap(1)),
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
