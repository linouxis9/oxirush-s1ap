//! Ergonomic macros for building, extracting, and mutating S1AP protocol IEs.
//!
//! The bindings have the value of an IE as the octets of an open type: the object
//! set of a message does not show in its type. The macros keep that inside: callers
//! construct and receive concrete S1AP types, while values are APER-encoded into
//! and decoded from the open type at the protocol boundary.
//!
//! # Builder macros
//!
//! ## `build_s1ap!` — build a complete S1AP PDU
//!
//! IE IDs and procedure codes are those of `src/registry.rs`, the list generated
//! from the ASN.1 object sets, which the `inspect` feature reads too. Values
//! are converted with `.into()`, so primitive values such as `u32` can be passed
//! directly for generated newtypes.
//!
//! ```
//! use oxirush_s1ap::{build_s1ap, s1ap::*};
//!
//! let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
//!     IGNORE, UEContextReleaseRequest,
//!     REJECT MME_UE_S1AP_ID(1u32),
//!     REJECT eNB_UE_S1AP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! )?;
//! assert_eq!(pdu.procedure_name(), "UEContextReleaseRequest");
//! # Ok::<(), oxirush_s1ap::Error>(())
//! ```
//!
//! The macro gives a `Result`. A value that does not encode, as an integer outside
//! its constraint, is an [`Error`](crate::Error) of the kind `Encode`, and no panic.
//!
//! Arguments:
//! - `$direction` — `InitiatingMessage`, `SuccessfulOutcome`, or `UnsuccessfulOutcome`
//! - `$proc` — procedure name, used to derive its ASN.1 procedure code
//! - `$outer_crit` — outer criticality: `REJECT`, `IGNORE`, or `NOTIFY`, or none
//! - `$msg` — message struct name, such as `UEContextReleaseRequest`
//! - IEs — `$ie_crit $ie_name($ie_value)`, with the IE ID derived from its name, and
//!   `$ie_crit` as the outer criticality
//!
//! A criticality that is left out is the one that the specification assigns: that of
//! the procedure for the PDU, which is [`Message::CRITICALITY`](crate::Message) of its
//! message, and that of the object set of the message for an IE. An IE that the set does
//! not have has none, and is an error of the kind `Argument`.
//!
//! ```
//! use oxirush_s1ap::{Message, build_s1ap, s1ap::*};
//!
//! let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest, UEContextReleaseRequest,
//!     MME_UE_S1AP_ID(1u32),
//!     eNB_UE_S1AP_ID(7u32),
//!     Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! )?;
//! assert_eq!(pdu.criticality(), UEContextReleaseRequest::CRITICALITY);
//! assert!(pdu.check()?.is_empty());
//! # Ok::<(), oxirush_s1ap::Error>(())
//! ```
//!
//! ## `build_s1ap_ie!` — build one Protocol IE entry
//!
//! This is useful for conditionally included IEs or IEs built separately.
//!
//! ```
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
//!
//! let ie = build_s1ap_ie!(UEContextReleaseRequest,
//!     REJECT MME_UE_S1AP_ID(1u32)
//! )?;
//! assert_eq!(ie.criticality, Criticality::reject);
//! # Ok::<(), oxirush_s1ap::Error>(())
//! ```
//!
//! Without its criticality the IE has the one that the object set of the message assigns
//! it. An IE that goes in no message has its criticality written:
//!
//! ```
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
//!
//! let ie = build_s1ap_ie!(UEContextReleaseRequest, MME_UE_S1AP_ID(1u32))?;
//! assert_eq!(ie.criticality, Criticality::reject);
//! # Ok::<(), oxirush_s1ap::Error>(())
//! ```
//!
//! ```compile_fail
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
//!
//! let ie = build_s1ap_ie!(MME_UE_S1AP_ID(1u32));
//! ```
//!
//! IEs are named after their `id-` constant. The name of a type is an alias
//! only for the one IE of that type, and so is an alias that the crate gives
//! the type: id-eNB-UE-S1AP-ID is `eNB_UE_S1AP_ID`, `ENBUES1APID` and
//! `ENB_UE_S1AP_ID`. The type of both id-GUMMEI-ID and id-SourceMME-GUMMEI is
//! addressed by the IE names.
//!
//! ```
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
//!
//! fn source(gummei: GUMMEI) -> Result<ProtocolIEField<'static>, oxirush_s1ap::Error> {
//!     build_s1ap_ie!(PathSwitchRequest, IGNORE SourceMME_GUMMEI(gummei))
//! }
//! ```
//!
//! ```compile_fail
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
//!
//! fn source(gummei: GUMMEI) -> Result<ProtocolIEField<'static>, oxirush_s1ap::Error> {
//!     build_s1ap_ie!(PathSwitchRequest, IGNORE GUMMEI(gummei))
//! }
//! ```
//!
//! # Extraction macro
//!
//! ## `extract_s1ap_ies!` — extract IEs from a decoded S1AP message
//!
//! The macro iterates through `protocol_ies`, matches generated ASN.1 IE IDs,
//! and APER-decodes each matching open type into its concrete generated type.
//! A `req` field is the value, and an `opt` or a `lax` field an `Option` of it:
//!
//! | The IE | A `req` field | An `opt` field | A `lax` field |
//! |--------|---------------|----------------|---------------|
//! | is in the message | the value | `Some` of the value | `Some` of the value |
//! | is not in the message | an error of the kind `MissingIe` | `None` | `None` |
//! | is there and does not decode | an error of the kind `UndecodableIe` | the same error | `None` |
//!
//! Which kind a field is says what the receiver does without the IE:
//!
//! - `req`: it cannot go on. The IE is one that the message always has, or one that
//!   the handling needs.
//! - `opt`: it goes on without an IE that was not sent, and not with one that it
//!   could not read.
//! - `lax`: it goes on as if the IE had not been received, whether it was not sent
//!   or was not read: the field takes the IE if it decodes.
//!
//! A `lax` field is for an IE that the receiver may ignore. TS 36.413 clause 10.3.4.2
//! has a receiver ignore the content of an IE that it does not comprehend and that
//! was received with the criticality `ignore`, and go on as if the IE had not been
//! received. With `notify` it does the same and reports the IE, and with `reject`
//! it rejects the procedure. The macro does not look at the criticality of an IE:
//! the caller reads it in `protocol_ies`, and takes with `lax` what it may ignore.
//!
//! The error is that of the enclosing function, which returns a `Result` whose
//! error is [`Error`](crate::Error) or has a `From` of it, as
//! `Box<dyn std::error::Error>` does: the macro converts the error as `?`
//! converts one. A call with `lax` fields alone has no error, and its function
//! returns what it likes. Without `=> expression`, extraction uses `binding.0`,
//! which unwraps the usual single-field generated newtype.
//!
//! What a field holds of octets is borrowed from the message: a NAS-PDU is a slice of
//! the PDU, and is not copied. A value that outlives the message is made with
//! `into_owned()`, or copied as the example below copies its NAS-PDU.
//!
//! ```
//! use oxirush_s1ap::{Error, ErrorKind, build_s1ap_ie, extract_s1ap_ies, s1ap::*};
//!
//! fn handle(msg: &UplinkNASTransport) -> Result<Vec<u8>, Error> {
//!     extract_s1ap_ies!(msg, UplinkNASTransport,
//!         req mme_id: u32 = MME_UE_S1AP_ID(id),
//!         req nas_pdu: Vec<u8> = NAS_PDU(pdu) => pdu.0.to_vec(),
//!         opt enb_id: u32 = eNB_UE_S1AP_ID(id),
//!     );
//!     let _ = (mme_id, enb_id);
//!     Ok(nas_pdu)
//! }
//!
//! let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
//!     build_s1ap_ie!(UplinkNASTransport, REJECT MME_UE_S1AP_ID(1u32))?,
//!     build_s1ap_ie!(UplinkNASTransport, REJECT NAS_PDU(vec![0x07, 0x6a]))?,
//! ]));
//! assert_eq!(handle(&message)?, [0x07, 0x6a]);
//!
//! // Without the NAS-PDU, which a field requires
//! message.protocol_ies.0.pop();
//! assert_eq!(handle(&message).unwrap_err().kind(), ErrorKind::MissingIe);
//! # Ok::<(), Error>(())
//! ```
//!
//! ## `with_s1ap_ie_mut!` — locate and mutate one decoded S1AP IE
//!
//! This decodes the matching open type, passes the concrete value into the
//! expression, then re-encodes it. It gives a `Result<bool, Error>`: what the
//! expression says, `Ok(false)` for an IE that is not in the message, and an error
//! for an IE that does not decode (`UndecodableIe`) or a changed value that does
//! not encode (`Encode`).
//! The setter form `IeName(binding) = value` expands to `binding.0 = value`.
//!
//! When a message has the same IE twice, both macros use the same one, the first:
//! `extract_s1ap_ies!` reads it and `with_s1ap_ie_mut!` changes it, and neither decodes
//! the others. `check()` of the PDU reports the second as `Deviation::RepeatedIe`, and
//! `ies::<T>()` of the message gives every one.
//!
//! ```
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*, with_s1ap_ie_mut};
//!
//! let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
//!     build_s1ap_ie!(UplinkNASTransport, REJECT MME_UE_S1AP_ID(1u32))?,
//! ]));
//! let updated = with_s1ap_ie_mut!(message, UplinkNASTransport,
//!     MME_UE_S1AP_ID(id) = 42u32
//! )?;
//! assert!(updated);
//! # Ok::<(), oxirush_s1ap::Error>(())
//! ```

/// The identifier of the IE that the macros take by this name.
#[macro_export]
#[doc(hidden)]
macro_rules! __s1ap_ie_id {
    ($name:ident) => {
        <$crate::ie::$name as $crate::aper::protocol::Ie>::ID
    };
}

/// The code of the procedure of this name.
#[macro_export]
#[doc(hidden)]
macro_rules! __s1ap_proc_code {
    ($name:ident) => {
        $crate::registry::procedures::$name
    };
}

/// Extract typed S1AP protocol IEs into `req`, `opt` and `lax` fields.
///
/// | The IE | A `req` field | An `opt` field | A `lax` field |
/// |--------|---------------|----------------|---------------|
/// | is in the message | the value | `Some` of the value | `Some` of the value |
/// | is not in the message | an error of the kind `MissingIe` | `None` | `None` |
/// | is there and does not decode | an error of the kind `UndecodableIe` | the same error | `None` |
///
/// The enclosing function returns the error, converted as `?` converts one. An `opt`
/// field is `None` only when its IE is not in the message. A `lax` field takes the IE
/// if it decodes, and is `None` otherwise: it is for an IE that the receiver may
/// ignore, as TS 36.413 clause 10.3.4.2 has it ignore one that it does not comprehend
/// and that was received with the criticality `ignore`. The macro does not look at
/// the criticality, which the caller reads in `protocol_ies`. A call with `lax`
/// fields alone has no error, and its function need not return a `Result`.
///
/// A character string is checked when it is decoded: a name with a character that a
/// PrintableString does not have, as `_` or `@`, does not decode, where a decoder
/// that does not check its alphabet reads it. Such a name is an error for an `opt`
/// field and `None` for a `lax` one.
///
/// A field has the type of the IE, or the type of the one value that the IE is a name
/// for, as a number for an identifier. After `=>` comes an expression of the value of
/// the IE, which the field is then.
///
/// The IEs are read in one pass over the IEs of the message, which stops once each that
/// a field asks for is found, and only those are decoded. `S1AP_PDU` is taken too,
/// as the message that it holds, without `decode_value` first: the octets of its message
/// are read where they are, and a PDU of another message is an error of the kind
/// `WrongMessage` for a `req` and an `opt` field, as octets that are no container of
/// protocol IEs are an error of the kind `Decode`; `lax` fields are then `None`.
///
/// When the message has the IE twice, the field is the first one, which is the one
/// that `with_s1ap_ie_mut!` changes, and the second is not decoded. `check()` of the PDU
/// reports it as `Deviation::RepeatedIe`, and `ies::<T>()` of the message gives every
/// one.
///
/// ```
/// use oxirush_s1ap::{Error, build_s1ap, extract_s1ap_ies, s1ap::*};
///
/// fn released(pdu: &S1AP_PDU) -> Result<(u32, bool, Option<GWContextReleaseIndication>), Error> {
///     let message: UEContextReleaseRequest = pdu.decode_value()?;
///     extract_s1ap_ies!(message, UEContextReleaseRequest,
///         // The one value that the IE is a name for
///         req mme: u32 = MME_UE_S1AP_ID(id),
///         // An expression of the value after `=>`, which the field is
///         req inactive: bool = Cause(cause) =>
///             cause == Cause::radioNetwork(CauseRadioNetwork::user_inactivity),
///         // An IE that the message may not have
///         opt gateway: GWContextReleaseIndication = GWContextReleaseIndication(indication),
///     );
///     Ok((mme, inactive, gateway))
/// }
///
/// let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
///     IGNORE, UEContextReleaseRequest,
///     REJECT MME_UE_S1AP_ID(1u32),
///     REJECT eNB_UE_S1AP_ID(7u32),
///     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
/// )?;
/// assert_eq!(released(&pdu)?, (1, true, None));
/// # Ok::<(), Error>(())
/// ```
///
/// A `lax` field, on a message whose Cause decodes and on one whose Cause does not:
///
/// ```
/// # use std::fmt::Write as _;
/// # let mut printed = String::new();
/// # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
/// use oxirush_s1ap::{extract_s1ap_ies, s1ap::*};
///
/// // The cause of a request, when it has one that is read
/// fn cause(request: &UEContextReleaseRequest) -> Option<Cause> {
///     extract_s1ap_ies!(request, UEContextReleaseRequest,
///         lax cause: Cause = Cause(value) => value,
///     );
///     cause
/// }
///
/// // A UE CONTEXT RELEASE REQUEST, then the same with an octet of its Cause spoiled
/// for octets in [
///     "0012401500000300000002002a000800020007000240020280",
///     "0012401500000300000002002a000800020007000240027280",
/// ] {
///     let pdu = S1AP_PDU::decode(&hex::decode(octets).unwrap()).unwrap();
///     println!("{:?}", cause(&pdu.decode_value().unwrap()));
/// }
/// # assert_eq!(printed, r##"Some(radioNetwork(user_inactivity))
/// # None
/// # "##);
/// ```
///
/// ```text
/// Some(radioNetwork(user_inactivity))
/// None
/// ```
#[macro_export]
macro_rules! extract_s1ap_ies {
    ($($arguments:tt)*) => {
        $crate::aper::__extract_ies!($crate, s1ap, $($arguments)*)
    };
}

/// Locate, decode, mutate, and re-encode one S1AP protocol IE.
///
/// Gives a `Result<bool, Error>`: what the expression says, `Ok(false)` when the IE is
/// not in the message, and an error when the IE does not decode (`UndecodableIe`) or
/// the changed value does not encode (`Encode`). The message is as it was after an
/// error.
///
/// When the message has the IE twice, the first one is changed, which is the one that
/// `extract_s1ap_ies!` reads; the second is not decoded.
///
/// `= value` gives the one value that the IE is a name for. After `=>` comes a block
/// that changes the value of the IE, which it has by the name in parentheses, mutable
/// and its own, and gives whether it changed it, a `bool`: a value of a CHOICE is
/// matched by `&mut`, so that what the match finds is changed in place.
///
/// ```
/// use oxirush_s1ap::{Error, build_s1ap, with_s1ap_ie_mut, s1ap::*};
///
/// let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
///     IGNORE, UEContextReleaseRequest,
///     REJECT MME_UE_S1AP_ID(1u32),
///     REJECT eNB_UE_S1AP_ID(7u32),
///     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
/// )?;
/// let mut message: UEContextReleaseRequest = pdu.decode_value()?;
///
/// // The one value of an IE
/// assert!(with_s1ap_ie_mut!(message, UEContextReleaseRequest, eNB_UE_S1AP_ID(id) = 8)?);
///
/// // A block that changes the value where it is, and says whether it did
/// let changed = with_s1ap_ie_mut!(message, UEContextReleaseRequest, Cause(cause) => {
///     match &mut cause {
///         Cause::radioNetwork(why) => {
///             *why = CauseRadioNetwork::unspecified;
///             true
///         }
///         _ => false,
///     }
/// })?;
/// assert!(changed);
/// assert_eq!(message, {
///     let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
///         IGNORE, UEContextReleaseRequest,
///         REJECT MME_UE_S1AP_ID(1u32),
///         REJECT eNB_UE_S1AP_ID(8u32),
///         IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::unspecified)),
///     )?;
///     pdu.decode_value::<UEContextReleaseRequest>()?.into_owned()
/// });
/// # Ok::<(), Error>(())
/// ```
#[macro_export]
macro_rules! with_s1ap_ie_mut {
    ($($arguments:tt)*) => {
        $crate::aper::__with_ie_mut!($crate, s1ap, $($arguments)*)
    };
}

/// Build a complete `S1AP_PDU` from a direction, procedure, message, and IEs.
///
/// Gives a `Result<S1AP_PDU, Error>`: a value that does not encode into its open type,
/// such as an integer outside its constraint, is an error of the kind `Encode`. A
/// criticality that a call leaves out is the one that the specification assigns: that of
/// the procedure for the PDU, and that of the object set of the message for an IE, which
/// an IE that the set does not have has not: an error of the kind `Argument`.
///
/// ```
/// use oxirush_s1ap::{build_s1ap, s1ap::*};
///
/// // The direction, the procedure and the message, then each IE: the criticalities
/// // are those of the specification, or written before what they are of
/// let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest, UEContextReleaseRequest,
///     MME_UE_S1AP_ID(42u32),
///     eNB_UE_S1AP_ID(7u32),
///     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
/// )?;
/// assert_eq!(hex::encode(pdu.encode()?), "0012401500000300000002002a000800020007000240020280");
/// # Ok::<(), oxirush_s1ap::Error>(())
/// ```
#[macro_export]
macro_rules! build_s1ap {
    ($($arguments:tt)*) => {
        $crate::aper::__build!($crate, s1ap, "build_s1ap!", $($arguments)*)
    };
}

/// Build one S1AP protocol IE entry. Every message takes the same
/// `ProtocolIEField`: the name before the comma says where the IE goes, and is the
/// type of a message. The IE has the criticality that the object set of that message
/// assigns it, unless the call writes its own.
///
/// An IE that goes in no message is built without that name, and with its criticality.
/// Such is an item of a list of single containers, as `E-RABToBeSetupListCtxtSUReq`,
/// whose items are each one IE.
///
/// ```
/// use oxirush_s1ap::aper::BitString;
/// use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
///
/// // An IE of a message, of the criticality of its object set
/// let mme = build_s1ap_ie!(UEContextReleaseRequest, MME_UE_S1AP_ID(1u32))?;
/// assert_eq!(mme.criticality, Criticality::reject);
///
/// // One of another criticality, which a test of a peer sends
/// let ignored = build_s1ap_ie!(UEContextReleaseRequest, IGNORE MME_UE_S1AP_ID(1u32))?;
/// assert_eq!(ignored.criticality, Criticality::ignore);
///
/// // An item of a list of single containers: an E-RAB of QCI 9, and its tunnel
/// let priority = AllocationAndRetentionPriority::new(PriorityLevel(15),
///     PreEmptionCapability::shall_not_trigger_pre_emption,
///     PreEmptionVulnerability::not_pre_emptable, None);
/// let qos = ERABLevelQoSParameters::new(QCI(9), priority, None, None);
/// let bearer = ERABToBeSetupItemCtxtSUReq::new(ERABID(5), qos,
///     TransportLayerAddress(BitString::from([10, 0, 0, 1])), GTPTEID([0, 0, 0, 1]), None, None);
/// let item = build_s1ap_ie!(REJECT E_RABToBeSetupItemCtxtSUReq(bearer))?;
/// let list = build_s1ap_ie!(InitialContextSetupRequest,
///     E_RABToBeSetupListCtxtSUReq(ERABToBeSetupListCtxtSUReq(vec![item])))?;
/// assert_eq!(list.criticality, Criticality::reject);
/// # Ok::<(), oxirush_s1ap::Error>(())
/// ```
///
/// Gives a `Result<ProtocolIEField, Error>`: a value that does not encode into its
/// open type is an error of the kind `Encode`, and an IE that the object set does not
/// have, without its criticality, one of the kind `Argument`.
#[macro_export]
macro_rules! build_s1ap_ie {
    ($($arguments:tt)*) => {
        $crate::aper::__build_ie!($crate, s1ap, "build_s1ap_ie!", $($arguments)*)
    };
}

#[cfg(test)]
mod tests {
    use crate::s1ap::*;
    use crate::{Error, ErrorKind};

    #[test]
    fn mutating_macro_updates_newtype_payload() -> Result<(), Error> {
        let mut pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,
            REJECT MME_UE_S1AP_ID(1u32),
            REJECT eNB_UE_S1AP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        )?;

        let S1AP_PDU::initiatingMessage(message) = &mut pdu else {
            panic!("expected initiating message");
        };
        let request: UEContextReleaseRequest =
            oxirush_aper::decode(&message.value).expect("decode request");
        let mut request = request;

        let changed =
            with_s1ap_ie_mut!(request, UEContextReleaseRequest, MME_UE_S1AP_ID(id) = 42u32);
        assert_eq!(changed, Ok(true));

        let ie = request
            .protocol_ies
            .0
            .iter()
            .find(|ie| ie.id.0 == 0)
            .expect("MME UE ID");
        let id: MMEUES1APID = oxirush_aper::decode(&ie.value).expect("decode ID");
        assert_eq!(id.0, 42);
        Ok(())
    }

    /// The identifier of a request, as the macro reads it.
    fn id(request: &UEContextReleaseRequest) -> Result<u32, Error> {
        extract_s1ap_ies!(request, UEContextReleaseRequest,
            req id: u32 = MME_UE_S1AP_ID(id),
        );
        Ok(id)
    }

    #[test]
    fn an_ie_that_is_there_twice_is_read_and_changed_at_the_same_place() -> Result<(), Error> {
        let ids = |request: &UEContextReleaseRequest| -> Vec<Vec<u8>> {
            let ies = request.protocol_ies.0.iter();
            ies.map(|ie| ie.value.to_vec()).collect()
        };
        let entry = |id: u32| build_s1ap_ie!(UEContextReleaseRequest, REJECT MME_UE_S1AP_ID(id));
        let mut request =
            UEContextReleaseRequest::new(ProtocolIEContainer(vec![entry(1)?, entry(2)?]));
        // The first one is read, and is the one that is changed.
        assert_eq!(id(&request)?, 1);
        let before = ids(&request);
        let changed =
            with_s1ap_ie_mut!(request, UEContextReleaseRequest, MME_UE_S1AP_ID(id) = 42u32);
        assert_eq!(changed, Ok(true));
        assert_eq!(id(&request)?, 42);
        let after = ids(&request);
        assert_ne!(after[0], before[0]);
        assert_eq!(after[1], before[1]);
        Ok(())
    }

    #[test]
    fn the_first_of_two_is_read_and_a_second_that_does_not_decode_is_not() -> Result<(), Error> {
        // An entry with the identifier of this IE whose octets are not its value,
        // after one that decodes and before it.
        let undecodable = || {
            let octets = crate::Octets::from(vec![0xff]);
            ProtocolIEField::new(0u16, Criticality::reject, octets)
        };
        let entry = build_s1ap_ie!(UEContextReleaseRequest, REJECT MME_UE_S1AP_ID(1u32))?;
        // The first one decodes: both macros take it, and decode no other.
        let mut request =
            UEContextReleaseRequest::new(ProtocolIEContainer(vec![entry.clone(), undecodable()]));
        assert_eq!(id(&request)?, 1);
        let changed =
            with_s1ap_ie_mut!(request, UEContextReleaseRequest, MME_UE_S1AP_ID(id) = 42u32);
        assert_eq!(changed, Ok(true));
        assert_eq!(id(&request)?, 42);
        // The first one does not decode: it is the error of both.
        let mut request =
            UEContextReleaseRequest::new(ProtocolIEContainer(vec![undecodable(), entry.clone()]));
        let before = request.clone();
        assert_eq!(id(&request).unwrap_err().kind(), ErrorKind::UndecodableIe);
        let changed =
            with_s1ap_ie_mut!(request, UEContextReleaseRequest, MME_UE_S1AP_ID(id) = 42u32);
        assert_eq!(changed.unwrap_err().kind(), ErrorKind::UndecodableIe);
        // The message is as it was.
        assert_eq!(request, before);
        Ok(())
    }

    #[test]
    fn a_message_that_is_another_one_each_time_it_is_read_is_no_panic() -> Result<(), Error> {
        // The macro reads the expression of its message more than once: one that gives
        // a message with the IE, then one without it, has nowhere to write the IE.
        struct Each {
            messages: [UEContextReleaseRequest<'static>; 2],
            reads: usize,
        }
        impl Each {
            fn next(&mut self) -> &mut UEContextReleaseRequest<'static> {
                self.reads += 1;
                &mut self.messages[usize::from(self.reads > 2)]
            }
        }
        let with = build_s1ap_ie!(UEContextReleaseRequest, REJECT MME_UE_S1AP_ID(1u32))?;
        let mut each = Each {
            messages: [
                UEContextReleaseRequest::new(ProtocolIEContainer(vec![with])),
                UEContextReleaseRequest::new(ProtocolIEContainer(vec![])),
            ],
            reads: 0,
        };
        let changed = with_s1ap_ie_mut!(
            each.next(),
            UEContextReleaseRequest,
            MME_UE_S1AP_ID(id) = 2u32
        );
        assert_eq!(changed, Ok(false));
        assert_eq!(each.reads, 3);
        Ok(())
    }

    #[test]
    fn builder_macros_cover_ie_and_outcome_forms() -> Result<(), Error> {
        let ie = build_s1ap_ie!(
            UEContextReleaseRequest,
            REJECT MME_UE_S1AP_ID(1u32)
        )?;
        assert_eq!(ie.id.0, 0);

        // The direction and the procedure code of a PDU are those of its message.
        let initiating = build_s1ap!(InitiatingMessage, S1Setup, REJECT, S1SetupRequest,)?;
        assert!(initiating.is_initiating());
        assert_eq!(initiating.procedure_code(), 17);

        let successful = build_s1ap!(SuccessfulOutcome, S1Setup, REJECT, S1SetupResponse,)?;
        assert!(successful.is_successful());
        assert_eq!(successful.procedure_code(), 17);

        let unsuccessful = build_s1ap!(UnsuccessfulOutcome, S1Setup, REJECT, S1SetupFailure,)?;
        assert!(unsuccessful.is_unsuccessful());
        assert_eq!(unsuccessful.procedure_code(), 17);
        Ok(())
    }
}
