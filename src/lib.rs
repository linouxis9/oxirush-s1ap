/*
   OxiRush
   Copyright 2025 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! A codec for S1AP, the S1 Application Protocol of 3GPP TS 36.413, in aligned PER.
//!
//! ```rust
//! use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, s1ap::*};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Decode a UE CONTEXT RELEASE REQUEST
//!     let bytes = hex::decode("0012401500000300000002002a000800020007000240020280")?;
//!     let pdu = S1AP_PDU::decode(&bytes)?;
//!     assert_eq!((pdu.procedure_code(), pdu.is_initiating()), (18, true));
//!     println!("{}", pdu.procedure_name());
//!     # assert_eq!(pdu.procedure_name(), "UEContextReleaseRequest");
//!
//!     // Read its IEs by name. An `opt` field is `None` when its IE is not there,
//!     // and what follows `=>` is what the field keeps of the decoded value
//!     let request: UEContextReleaseRequest = pdu.decode_value()?;
//!     extract_s1ap_ies!(request, UEContextReleaseRequest,
//!         opt mme_id: u32 = MME_UE_S1AP_ID(id),
//!         opt enb_id: u32 = eNB_UE_S1AP_ID(id),
//!         opt cause: Cause = Cause(cause) => cause,
//!     );
//!     println!("MME={mme_id:?} eNB={enb_id:?} cause={cause:?}");
//!     # assert_eq!(format!("MME={mme_id:?} eNB={enb_id:?} cause={cause:?}"), "MME=Some(42) eNB=Some(7) cause=Some(radioNetwork(user_inactivity))");
//!
//!     // Build it with another cause: the direction, the procedure, the criticality
//!     // of the PDU and the message, then each IE with its criticality
//!     let rebuilt = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
//!         IGNORE, UEContextReleaseRequest,
//!
//!         REJECT MME_UE_S1AP_ID(42u32),
//!         REJECT eNB_UE_S1AP_ID(7u32),
//!         IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
//!     )?;
//!
//!     // Encode it
//!     let encoded = hex::encode(rebuilt.encode()?);
//!     assert_eq!(encoded, "0012401500000300000002002a0008000200070002400202a0");
//!     Ok(())
//! }
//! ```
//!
//! ```text
//! UEContextReleaseRequest
//! MME=Some(42) eNB=Some(7) cause=Some(radioNetwork(user_inactivity))
//! ```
//!
//! The types of [`s1ap`] are generated from the ASN.1 modules of the specification and
//! checked in: `rasn-compiler` resolves the modules, and the generator of the crate
//! writes each type with its codec on [`oxirush-aper`](aper).
//!
//! # Tasks
//!
//! The [README](https://github.com/linouxis9/oxirush-s1ap#readme) has an example of
//! each, with what it prints.
//!
//! | Task | Items |
//! |------|-------|
//! | Answer an eNB, from S1 SETUP to the INITIAL CONTEXT SETUP | [`build_s1ap!`], [`build_s1ap_ie!`], [`extract_s1ap_ies!`], [`helpers`], [`S1AP_PDU::ie`](s1ap::S1APPDU::ie) |
//! | Decode a PDU | [`S1AP_PDU::decode`](s1ap::S1APPDU::decode), [`S1AP_PDU::decode_borrowed`](s1ap::S1APPDU::decode_borrowed), [`S1AP_PDU::kind`](s1ap::S1APPDU::kind), [`S1AP_PDU::decode_value`](s1ap::S1APPDU::decode_value) |
//! | Build a PDU | [`build_s1ap!`], [`S1AP_PDU::of_message`](s1ap::S1APPDU::of_message) |
//! | Read the IEs of a message | [`extract_s1ap_ies!`], [`ie`], [`with_s1ap_ie_mut!`], [`build_s1ap_ie!`] |
//! | A list of single containers, and a handover container, typed | [`S1AP_PDU::items`](s1ap::S1APPDU::items), [`s1ap::decode_open_type`], [`s1ap::encode_open_type`] |
//! | Check a message, and answer one that is faulty | [`S1AP_PDU::check`](s1ap::S1APPDU::check), [`S1AP_PDU::check_structure`](s1ap::S1APPDU::check_structure), [`Deviation`], [`S1AP_PDU::criticality_diagnostics_for_response`](s1ap::S1APPDU::criticality_diagnostics_for_response) |
//! | A PDU as JSON | [`S1AP_PDU::to_tree`](s1ap::S1APPDU::to_tree), [`S1AP_PDU::to_view`](s1ap::S1APPDU::to_view), [`S1AP_PDU::with_view`](s1ap::S1APPDU::with_view), [`inspect`] |
//! | Write a message from its tree | [`S1AP_PDU::from_tree`](s1ap::S1APPDU::from_tree) |
//!
//! Every function and macro that can fail returns an [`Error`]: its [`ErrorKind`] is
//! what a caller matches on, and its `Display` says what is wrong.
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_s1ap::{ErrorKind, s1ap::*};
//!
//! // A UE CONTEXT RELEASE REQUEST is not decoded as an S1 SETUP REQUEST
//! let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
//! let pdu = S1AP_PDU::decode(&bytes).unwrap();
//! let wrong = pdu.decode_value::<S1SetupRequest>().unwrap_err();
//! assert_eq!(wrong.kind(), ErrorKind::WrongMessage);
//! println!("{wrong}");
//!
//! // Octets that end early
//! let short = S1AP_PDU::decode(&bytes[..10]).unwrap_err();
//! assert_eq!(short.kind(), ErrorKind::Decode);
//! println!("{short}");
//! # assert_eq!(printed, r##"the message of this PDU is UEContextReleaseRequest, not S1SetupRequest: kind() says which message a PDU holds
//! # these octets do not decode as S1AP-PDU: the octets end before the value does, in its field `value`
//! # "##);
//! ```
//!
//! ```text
//! the message of this PDU is UEContextReleaseRequest, not S1SetupRequest: kind() says which message a PDU holds
//! these octets do not decode as S1AP-PDU: the octets end before the value does, in its field `value`
//! ```
//!
//! Every call that can fail returns an `oxirush_s1ap::Error`. Its `kind()` is
//! what a caller matches on, and the documentation of `ErrorKind` lists the
//! kinds; `error.ie()` names the IE of an error of the macros.
//!
//! The codec builds, encodes and decodes a message whatever its IEs are. `check()` of
//! a PDU reports how it deviates from what the ASN.1 assigns its message, each as a
//! [`Deviation`]: a criticality of the PDU or of an IE that is not the assigned one, an
//! IE that the message cannot have, has twice or has out of order, a mandatory IE that
//! is missing, an IE whose value does not decode or is of a later version of its type,
//! an encoding in another form than the canonical one, and a procedure that the
//! specification does not have. `criticality_diagnostics()` of the PDU gathers them
//! into the Criticality Diagnostics IE of the answer.
//!
//! # Values that borrow, and values that are kept
//!
//! A value that is decoded borrows the octets that it holds from what it was decoded
//! from: the NAS-PDU of a message is a slice of the PDU, and nothing is copied. A type
//! that holds octets has a lifetime for that, as `InitialUEMessage<'a>` and
//! `NASPDU<'a>`. A type that holds none, as `Cause`, `TAI`, `EUTRANCGI` or `GUMMEI`,
//! has no lifetime. The extension container of a value owns its octets, as a peer
//! seldom sends one, so that it gives the value no lifetime.
//!
//! | To | Write |
//! | --- | --- |
//! | Read a message where it was received | `let pdu = S1AP_PDU::decode(&octets)?;` then `pdu.decode_value()?`: the message borrows from `pdu`, which is a variable of its own |
//! | Keep a value after the PDU is gone | `value.into_owned()`, which gives the same type with the lifetime `'static`, as `NASPDU<'static>`: this is the type to write in a struct |
//! | Build a value | From data that it owns: `NASPDU::from(vec)` is a `NASPDU<'static>` |
//! | Read a PDU in the buffer that it was received in | [`S1AP_PDU::decode_borrowed`](s1ap::S1APPDU::decode_borrowed), which copies nothing: the PDU borrows from the buffer |
//!
//! [`S1AP_PDU`](s1ap::S1AP_PDU) is the PDU that owns its octets, `S1APPDU<'static>`,
//! and has no lifetime to write.
//!
//! ```
//! use oxirush_s1ap::{Error, build_s1ap, extract_s1ap_ies, s1ap::*};
//!
//! /// What a node keeps of a UE after its first message.
//! struct Ue {
//!     enb_id: u32,
//!     // The context owns these octets: `'static`, and no lifetime on `Ue`
//!     nas: NASPDU<'static>,
//! }
//!
//! fn admit(pdu: &S1AP_PDU) -> Result<Ue, Error> {
//!     // `message` borrows from `pdu`, and `nas` from `message`
//!     let message: InitialUEMessage = pdu.decode_value()?;
//!     extract_s1ap_ies!(message, InitialUEMessage,
//!         req enb_id: u32 = eNB_UE_S1AP_ID(id),
//!         req nas: NASPDU = NAS_PDU(nas) => nas,
//!     );
//!     // Kept beyond the PDU: the octets are copied here, once
//!     Ok(Ue { enb_id, nas: nas.into_owned() })
//! }
//!
//! let pdu = build_s1ap!(InitiatingMessage, initialUEMessage, IGNORE, InitialUEMessage,
//!     REJECT eNB_UE_S1AP_ID(7u32),
//!     REJECT NAS_PDU(vec![0x07, 0x41, 0x71]),
//! )?;
//! let ue = admit(&pdu)?;
//! drop(pdu);
//! assert_eq!((ue.enb_id, &ue.nas.0[..]), (7, &[0x07, 0x41, 0x71][..]));
//! # Ok::<(), Error>(())
//! ```
//!
//! A message may be held across an `.await`. The future of this handler owns the
//! buffer, the PDU borrows from the buffer, the message from the PDU and the NAS-PDU
//! from the message. The future is `Send`, as every value of the bindings is.
//!
//! ```
//! use oxirush_s1ap::{Error, build_s1ap, extract_s1ap_ies, s1ap::*};
//!
//! async fn deliver(nas: &[u8]) {
//!     assert_eq!(nas, [0x07, 0x5e, 0x03]);
//! }
//!
//! async fn handle(received: Vec<u8>) -> Result<(), Error> {
//!     let pdu = S1AP_PDU::decode_borrowed(&received)?;
//!     let message: UplinkNASTransport = pdu.decode_value()?;
//!     extract_s1ap_ies!(message, UplinkNASTransport,
//!         req nas: NASPDU = NAS_PDU(nas) => nas,
//!     );
//!     // `nas` is a slice of `received`, here and after the await
//!     deliver(&nas.0).await;
//!     assert!(nas.0.is_borrowed());
//!     Ok(())
//! }
//!
//! let transport = build_s1ap!(InitiatingMessage, uplinkNASTransport, IGNORE, UplinkNASTransport,
//!     REJECT MME_UE_S1AP_ID(1u32),
//!     REJECT eNB_UE_S1AP_ID(7u32),
//!     REJECT NAS_PDU(vec![0x07, 0x5e, 0x03]),
//! )?;
//! let handler = handle(transport.encode()?);
//!
//! // A runtime may move the handler to another thread
//! fn is_send(_: &impl Send) {}
//! is_send(&handler);
//! # let mut handler = std::pin::pin!(handler);
//! # let mut context = std::task::Context::from_waker(std::task::Waker::noop());
//! # assert!(handler.as_mut().poll(&mut context).is_ready());
//! # Ok::<(), Error>(())
//! ```
//!
//! # The names of the fields
//!
//! A field has the words of the identifier that ASN.1 gives its member, in lower case
//! with `_` between them, as a reader of the specification says them: `pLMNidentity` is
//! `plmn_identity`, `tAC` is `tac`, `iE-Extensions` is `ie_extensions`, `eNB-ID` is
//! `enb_id`, `mME-Code` is `mme_code`, `gTP-TEID` is `gtp_teid`.
//!
//! 1. A hyphen ends a word, and so does a lower-case letter or a digit before a
//!    capital: `procedureCode` is `procedure_code`.
//! 2. The first letter of an identifier belongs to the capitals after it: the
//!    identifier starts in lower case whatever its first word is.
//! 3. Capitals in a row are an abbreviation, and the last of them starts a word when a
//!    lower-case letter follows. When the capitals without it are no abbreviation of
//!    the list and the capitals with it are one, they end the abbreviation:
//!    `nRCellIdentity` is `nr_cell_identity`, and `pLMNIdentity` is `plmn_identity` as
//!    `pLMNidentity` is. An `s` after them is their plural: `protocolIEs` is
//!    `protocol_ies`.
//! 4. Capitals that are several abbreviations of the list are these:
//!    `eNBX2TransportLayerAddresses` is `enb_x2_transport_layer_addresses`.
//! 5. An abbreviation that the specifications write with a lower-case letter is one
//!    word wherever it is: `a2X`, `AIoT`, `IoT`, `QoS`, `QoE`, `ProSe`, `eNB`, `gNB`,
//!    `IPsec`, `IPv4`, `IPv6`, `NRPPa`, `LPPa`, `n3IWF`, `eDRX`, `eCGI`. `gNB-ID` is
//!    `gnb_id`.
//! 6. `fiveG` and `fiveGS` are `five_g` and `five_gs`, and `5GS` is one word.
//!
//! The list of the abbreviations is `ABBREVIATIONS` of `generator/src/names.rs` in
//! oxirush-aper, with the identifiers that the specifications write otherwise than
//! their others, as `parentTImeSource`. `build/fields.tsv` of the repository has each
//! field of each type with the name that rasn-compiler gives it, which the fields had
//! before this rule and have in 0.2.0 for every field of a type that 0.2.0 has but one,
//! the `excludedCellsToAddModList` of `InterSystemMeasurementItem`. The JSON forms keep
//! the identifiers of ASN.1.
//!
//! A `SEQUENCE` is a struct that a literal builds, and its `new` takes the members in
//! their order. An extensible `CHOICE` or `ENUMERATED` is `#[non_exhaustive]`: a later
//! version of the specification adds alternatives and values to it, which a `match`
//! leaves room for with a `_` arm. A member that a later version adds to a `SEQUENCE`
//! changes its struct and its `new`, and comes with a breaking release of the crate.
//!
//! # The types of the values
//!
//! | ASN.1 | Rust |
//! | --- | --- |
//! | `INTEGER` with a constraint | The native number that holds its range: `MMEUES1APID(pub u32)` for `(0..4294967295)` |
//! | `INTEGER` whose constraint has an extension marker | `u64`: `ERABID(pub u64)` for `(0..15, ...)` |
//! | `OCTET STRING (SIZE (n))` | `[u8; n]`: `TAC(pub [u8; 2])` |
//! | Another `OCTET STRING`, and an open type | [`Octets`]: `NASPDU<'a>(pub Octets<'a>)` |
//! | `BIT STRING` of 64 bits at most, without an extension marker | [`Bits`], a number with a length, which is `Copy`. A type of one size is made of the number: `CellIdentity::new(0x1234507)?` |
//! | Another `BIT STRING` | [`BitString`], which borrows its octets when it is longer than 64 bits |
//! | `PrintableString`, `VisibleString`, `UTF8String` | `Cow<'a, str>`: `MMEname<'a>` |
//! | `ENUMERATED`, `CHOICE`, `SEQUENCE`, `SEQUENCE OF` | An enum, an enum, a struct, and a `Vec` in a struct |
//!
//! A bit string is a [`Bits`] when its type says that 64 bits hold it. A string that
//! may be longer, and one whose size has an extension marker, which a later release
//! may send longer, is a [`BitString`]:
//!
//! | [`Bits`] | [`BitString`] |
//! | --- | --- |
//! | `ENBID::macroENB_ID` (20 bits), `ENBID::homeENB_ID` (28) | `TransportLayerAddress` (1 to 160, ...) |
//! | `CellIdentity` (28), `NRCellIdentity` (36), `CSGId` (27) | `SecurityKey` (256) |
//! | `MaskedIMEISV` (64), `GNBID` (22 to 32) | `EncryptionAlgorithms`, `IntegrityProtectionAlgorithms`, `NRencryptionAlgorithms` and `NRintegrityProtectionAlgorithms` (16, ...) |
//! | `UEIdentityIndexValue` (10), `MessageIdentifier`, `SerialNumber` (16), `InterfacesToTrace` (8) | `EnGNBID` (22 to 32, ...), `ReceiveStatusofULPDCPSDUs` (4096) |
//!
//! The bindings hold an `INTEGER` in 64 bits. A number that an extension marker lets
//! a later release send beyond that does not decode, with an error of the kind
//! `Decode` that says `the number does not fit in 64 bits`. The PDU still decodes:
//! its IEs are octets until one is read, and the JSON forms of the `inspect` feature
//! show an IE that does not decode as the octets received, and write them back.
//!
//! # The OxiRush family
//!
//! - [oxirush-utils](https://docs.rs/oxirush-utils): the 3GPP identities, and the JSON tree with its paths that the others share.
//! - [oxirush-aper](https://docs.rs/oxirush-aper): aligned PER, the runtime of the NGAP and S1AP codecs.
//! - [oxirush-nas](https://docs.rs/oxirush-nas): the NAS messages of 5GS and EPS.
//! - [oxirush-security](https://docs.rs/oxirush-security): NAS security, AKA and the derivation of keys.
//! - [oxirush-ngap](https://docs.rs/oxirush-ngap): NGAP, between a gNB and an AMF.
//! - [oxirush-s1ap](https://docs.rs/oxirush-s1ap): S1AP, between an eNB and an MME.
//! - [oxirush-gtp-u](https://docs.rs/oxirush-gtp-u): GTP-U tunnels, with a test UPF.
//! - [oxirush-sctp](https://docs.rs/oxirush-sctp): SCTP sockets, which carry NGAP and S1AP.

#![forbid(unsafe_code)]

extern crate alloc;

pub mod helpers;
// What the fuzz targets and the oracle take from the inside of the crate, with
// `--cfg oxirush_s1ap_hooks`: the module `hooks`.
oxirush_aper::__hooks! { cfg oxirush_s1ap_hooks, bindings s1ap, pdu S1APPDU, owned S1AP_PDU }
#[cfg(feature = "inspect")]
pub mod inspect;
#[cfg(feature = "inspect")]
mod inspect_registry;
pub mod macros;
/// The types of the ASN.1 modules of TS 36.413, as the generator writes them:
/// [`S1AP_PDU`](s1ap::S1AP_PDU) and its three directions, each message, each IE and
/// each type of a value, which decode and encode themselves in aligned PER.
///
/// ```
/// use oxirush_s1ap::s1ap::{MMEUES1APID, decode_open_type, encode_open_type};
///
/// // `MME-UE-S1AP-ID ::= INTEGER (0..4294967295)`, as the value of its IE holds it
/// let octets = encode_open_type(&MMEUES1APID(42))?;
/// assert_eq!(octets, [0x00, 0x2a]);
/// assert_eq!(decode_open_type::<MMEUES1APID>(&octets)?, MMEUES1APID(42));
/// # Ok::<(), oxirush_s1ap::Error>(())
/// ```
// The bindings are formatted where they are generated, by the rustfmt of the toolchain
// that the generator runs with: a later one breaks some of their long lines otherwise.
#[rustfmt::skip]
pub mod s1ap;
mod pdu;
#[doc(hidden)]
pub mod registry;

/// The aligned PER runtime that the bindings are made of, at the version that this
/// crate is built with. The values of the bindings are its types, such as
/// [`Octets`], [`Bits`] and [`BitString`],
/// and the error of the crate is its error: a crate that names them takes them from
/// here, and needs no dependency on `oxirush-aper` of its own.
pub use oxirush_aper as aper;
/// The identities of 3GPP networks as values, of the `oxirush-utils` that this crate
/// is built with: a `PLMNidentity` converts to and from its `Plmn`, which has the MCC and
/// the MNC.
pub use oxirush_utils::identity;
/// The trees of `oxirush-utils`, of the version that this crate is built with: a PDU and
/// each value of the bindings as JSON whose members keep the order of the specification,
/// [`tree::Value`], and `tree!`, which writes one in the code as `serde_json::json!` does.
/// A crate that reads or writes them takes them from here.
#[cfg(feature = "inspect")]
pub use oxirush_utils::tree;

pub use oxirush_aper::protocol::Message;
pub use oxirush_aper::{
    AsnType, BitString, Bits, Decode, Encode, Error, ErrorKind, Fault, Faults, Octets,
};
/// The IEs of the protocol, one marker type each, by the names that the macros take:
/// `ies::<T>()` and `ie::<T>()` of a PDU and of a message take them, as
/// `pdu.ie::<ie::Cause>()`. Each implements [`aper::protocol::Ie`], which gives its
/// identifier and the type of its value.
pub use registry::ie;
pub use registry::{Deviation, S1ap};
pub use s1ap::S1apPduKind;

/// The examples of the README, which the documentation tests compile and run.
#[cfg(all(doctest, feature = "serde"))]
#[doc = include_str!("../README.md")]
struct Readme;

/// Version of oxirush-s1ap.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
