//! Ergonomic macros for building, extracting, and mutating S1AP protocol IEs.
//!
//! `rasn-compiler` represents S1AP information-object open types as
//! [`rasn::types::Any`]. The macros keep that representation internal: callers
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
//! ```ignore
//! use oxirush_s1ap::{build_s1ap, s1ap::*};
//!
//! let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
//!     IGNORE, UEContextReleaseRequest,
//!     REJECT MME_UE_S1AP_ID(1u32),
//!     REJECT eNB_UE_S1AP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! );
//! ```
//!
//! Arguments:
//! - `$direction` — `InitiatingMessage`, `SuccessfulOutcome`, or `UnsuccessfulOutcome`
//! - `$proc` — procedure name, used to derive its ASN.1 procedure code
//! - `$outer_crit` — outer criticality: `REJECT`, `IGNORE`, or `NOTIFY`
//! - `$msg` — message struct name, such as `UEContextReleaseRequest`
//! - IEs — `$ie_crit $ie_name($ie_value)`, with the IE ID derived from its name
//!
//! ## `build_s1ap_ie!` — build one Protocol IE entry
//!
//! This is useful for conditionally included IEs or IEs built separately.
//!
//! ```ignore
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
//!
//! let ie = build_s1ap_ie!(UEContextReleaseRequest,
//!     REJECT MME_UE_S1AP_ID(1u32)
//! );
//! ```
//!
//! IEs are named after their `id-` constant. The name of a type is an alias
//! only for the one IE of that type: the type of both id-GUMMEI-ID and
//! id-SourceMME-GUMMEI is addressed by the IE names.
//!
//! ```
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
//!
//! fn source(gummei: GUMMEI) -> ProtocolIEField {
//!     build_s1ap_ie!(PathSwitchRequest, IGNORE SourceMME_GUMMEI(gummei))
//! }
//! ```
//!
//! ```compile_fail
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
//!
//! fn source(gummei: GUMMEI) -> ProtocolIEField {
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
//! Required fields are unwrapped; a missing or invalid required field returns
//! [`MissingIeError`] from the enclosing function. Optional fields remain
//! `Option<T>`.
//!
//! The enclosing function must return `Result<_, MissingIeError>` or a type that
//! implements `From<MissingIeError>`. Without `=> expression`, extraction uses
//! `binding.0`, which unwraps the usual single-field generated newtype.
//!
//! ```ignore
//! use oxirush_s1ap::{extract_s1ap_ies, macros::MissingIeError, s1ap::*};
//!
//! fn handle(msg: &UplinkNASTransport) -> Result<Vec<u8>, MissingIeError> {
//!     extract_s1ap_ies!(msg, UplinkNASTransport,
//!         req mme_id: u32 = MME_UE_S1AP_ID(id),
//!         req nas_pdu: Vec<u8> = NAS_PDU(pdu) => pdu.0.to_vec(),
//!         opt enb_id: u32 = eNB_UE_S1AP_ID(id),
//!     );
//!     let _ = (mme_id, enb_id);
//!     Ok(nas_pdu)
//! }
//! ```
//!
//! ## `with_s1ap_ie_mut!` — locate and mutate one decoded S1AP IE
//!
//! This decodes the matching open type, passes the concrete value into the
//! expression, then re-encodes it. It returns `true` only when the IE was found,
//! decoded and re-encoded successfully, and the expression returned `true`.
//! The setter form `IeName(binding) = value` expands to `binding.0 = value`.
//!
//! ```ignore
//! use oxirush_s1ap::{s1ap::*, with_s1ap_ie_mut};
//!
//! let updated = with_s1ap_ie_mut!(message, UplinkNASTransport,
//!     MME_UE_S1AP_ID(id) = 42u32
//! );
//! assert!(updated);
//! ```

use core::fmt;

/// Error returned by `extract_s1ap_ies!` when a required IE is absent or invalid.
#[derive(Debug, Clone)]
pub struct MissingIeError {
    /// Name of the required field as written in the macro invocation.
    pub ie_name: &'static str,
}

impl fmt::Display for MissingIeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "missing or invalid required S1AP IE: {}",
            self.ie_name
        )
    }
}

impl std::error::Error for MissingIeError {}

/// The procedures of the protocol, which `src/registry.rs` lists from the ASN.1: the
/// code and the name of each, then for each of its messages the direction, the kind,
/// the name that ASN.1 gives it and its type.
///
/// The codes that the macros take by the names of the procedures, the kinds of a PDU
/// and, with the `inspect` feature, the messages of a tree all come from these lines.
macro_rules! procedures {
    ($($code:literal $procedure:ident {
        $(InitiatingMessage $initiating:ident $initiating_name:literal $initiating_type:path;)?
        $(SuccessfulOutcome $successful:ident $successful_name:literal $successful_type:path;)?
        $(UnsuccessfulOutcome $unsuccessful:ident $unsuccessful_name:literal $unsuccessful_type:path;)?
    })*) => {
        /// The procedure codes, by the names of the procedures.
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        pub mod procedures {
            $(pub const $procedure: u8 = $code;)*
        }

        /// The direction and the procedure of a PDU: each variant is one message.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum S1apPduKind {
            $($(#[doc = concat!("`", $initiating_name, "`.")] $initiating,)?)*
            $($(#[doc = concat!("`", $successful_name, "`.")] $successful,)?)*
            $($(#[doc = concat!("`", $unsuccessful_name, "`.")] $unsuccessful,)?)*
            /// A procedure that the specification does not have in that direction.
            Other { direction: &'static str, procedure_code: u8 },
        }

        impl S1apPduKind {
            /// Return this kind's S1AP procedure code.
            pub fn procedure_code(&self) -> u8 {
                match self {
                    $(
                        $(Self::$initiating => $code,)?
                        $(Self::$successful => $code,)?
                        $(Self::$unsuccessful => $code,)?
                    )*
                    Self::Other { procedure_code, .. } => *procedure_code,
                }
            }
        }

        impl $crate::s1ap::S1APPDU {
            /// Return the ASN.1 procedure name.
            pub fn procedure_name(&self) -> &'static str {
                match self.procedure_code() {
                    $($code => stringify!($procedure),)*
                    _ => "Unknown",
                }
            }

            /// Return the canonical direction/procedure kind.
            pub fn kind(&self) -> S1apPduKind {
                let procedure_code = self.procedure_code();
                let (direction, kind) = match self {
                    Self::initiatingMessage(_) => ("Initiating", match procedure_code {
                        $($($code => Some(S1apPduKind::$initiating),)?)*
                        _ => None,
                    }),
                    Self::successfulOutcome(_) => ("Successful", match procedure_code {
                        $($($code => Some(S1apPduKind::$successful),)?)*
                        _ => None,
                    }),
                    Self::unsuccessfulOutcome(_) => ("Unsuccessful", match procedure_code {
                        $($($code => Some(S1apPduKind::$unsuccessful),)?)*
                        _ => None,
                    }),
                };
                kind.unwrap_or(S1apPduKind::Other { direction, procedure_code })
            }
        }

        /// The messages: the direction, the procedure code, the name that ASN.1 gives it
        /// and the type of each.
        #[cfg(feature = "inspect")]
        pub(crate) const MESSAGES: &[(&str, u8, &str, fn() -> $crate::inspect::Typed)] = &[
            $($(("InitiatingMessage", $code, $initiating_name,
                $crate::inspect::Typed::of::<$initiating_type>),)?)*
            $($(("SuccessfulOutcome", $code, $successful_name,
                $crate::inspect::Typed::of::<$successful_type>),)?)*
            $($(("UnsuccessfulOutcome", $code, $unsuccessful_name,
                $crate::inspect::Typed::of::<$unsuccessful_type>),)?)*
        ];
    };
}
pub(crate) use procedures;

/// The IEs of the protocol, which `src/registry.rs` lists from the ASN.1: the identifier,
/// the name that ASN.1 gives it and the type of each, the type that its octets contain,
/// then the names that the macros take it by. An identifier that has several types has
/// its name alone.
///
/// The identifier and the type that a name stands for in the macros and, with the
/// `inspect` feature, the names and the types of a tree all come from these lines.
macro_rules! ies {
    ($($id:literal $name:literal
        $($ie:path $(, $contents:path)? $(=> $own:ident $($alias:ident)?)?)?;
    )*) => {
        /// The identifier and the type of each IE, by the names that the macros take.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub mod ies {
            use $crate::macros::Ie;
            $($($(
                pub type $own = Ie<$id, $ie>;
                $(pub type $alias = Ie<$id, $ie>;)?
            )?)?)*
        }

        #[cfg(feature = "inspect")]
        pub(crate) const IE_NAMES: &[(u16, &str)] = &[$(($id, $name),)*];

        #[cfg(feature = "inspect")]
        pub(crate) fn ie(id: u16) -> Result<$crate::inspect::Typed, String> {
            match id {
                $($($id => Ok($crate::inspect::Typed::of::<$ie>()),)?)*
                _ => Err(format!("S1AP IE {id} is unknown or has several types")),
            }
        }

        #[cfg(feature = "inspect")]
        #[allow(clippy::match_single_binding)]
        pub(crate) fn ie_contents(id: u16) -> Option<$crate::inspect::Typed> {
            match id {
                $($($($id => Some($crate::inspect::Typed::of::<$contents>()),)?)?)*
                _ => None,
            }
        }
    };
}
pub(crate) use ies;

/// An IE as the macros name it: a type of the `ies` of the registry, which has the
/// identifier of the IE and the type of its value.
#[doc(hidden)]
pub struct Ie<const ID: u16, T>(core::marker::PhantomData<T>);

/// What a name of the macros stands for.
#[doc(hidden)]
pub trait Named {
    /// The identifier of the IE.
    const ID: u16;
    /// The type of its value.
    type Type;
}

impl<const ID: u16, T> Named for Ie<ID, T> {
    const ID: u16 = ID;
    type Type = T;
}

/// The identifier of the IE that the macros take by this name.
#[macro_export]
#[doc(hidden)]
macro_rules! __s1ap_ie_id {
    ($name:ident) => {
        <$crate::registry::ies::$name as $crate::macros::Named>::ID
    };
}

/// The open type of a value of the IE that the macros take by this name.
#[macro_export]
#[doc(hidden)]
macro_rules! __s1ap_encode_ie {
    ($name:ident, $value:expr) => {{
        let value: <$crate::registry::ies::$name as $crate::macros::Named>::Type = ($value).into();
        $crate::s1ap::encode_open_type(&value)
    }};
}

/// The value in an open type of the IE that the macros take by this name.
#[macro_export]
#[doc(hidden)]
macro_rules! __s1ap_decode_ie {
    ($name:ident, $value:expr) => {
        $crate::s1ap::decode_open_type::<
            <$crate::registry::ies::$name as $crate::macros::Named>::Type,
        >($value)
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

/// Extract typed S1AP protocol IEs with `req` and `opt` semantics.
#[macro_export]
macro_rules! extract_s1ap_ies {
    ($msg_var:expr, $msg:ident,
        $($kind:ident $name:ident : $ty:ty = $ie_name:ident ($bind:ident) $(=> $expr:expr)?),+
        $(,)?
    ) => {
        $( let mut $name: Option<$ty> = None; )+
        for _ie in &$msg_var.protocol_ies.0 {
            $(
                if _ie.id.0 == $crate::__s1ap_ie_id!($ie_name) {
                    // `$bind` is out of scope where `$name` is assigned, as
                    // both may have the same name.
                    let _value: Option<$ty> = match $crate::__s1ap_decode_ie!($ie_name, &_ie.value) {
                        Ok($bind) => Some($crate::extract_s1ap_ies!(@val $bind $(, $expr)?)),
                        Err(_) => None,
                    };
                    if _value.is_some() {
                        $name = _value;
                    }
                }
            )+
        }
        $( $crate::extract_s1ap_ies!(@check $kind $name); )+
    };
    (@val $bind:ident) => { $bind.0 };
    (@val $_bind:ident, $expr:expr) => { $expr };
    (@check req $name:ident) => {
        let $name = match $name {
            Some(value) => value,
            None => {
                return Err($crate::macros::MissingIeError {
                    ie_name: stringify!($name),
                });
            }
        };
    };
    (@check opt $name:ident) => {};
}

/// Locate, decode, mutate, and re-encode one S1AP protocol IE.
#[macro_export]
macro_rules! with_s1ap_ie_mut {
    ($msg_var:expr, $msg:ident, $ie_name:ident($bind:ident) = $value:expr $(,)?) => {{
        $crate::with_s1ap_ie_mut!($msg_var, $msg, $ie_name($bind) => {
            $bind.0 = $value;
            true
        })
    }};
    ($msg_var:expr, $msg:ident, $ie_name:ident($bind:ident) => $expr:expr $(,)?) => {{
        let mut matched = false;
        for ie in &mut $msg_var.protocol_ies.0 {
            if ie.id.0 == $crate::__s1ap_ie_id!($ie_name) {
                if let Ok(mut $bind) = $crate::__s1ap_decode_ie!($ie_name, &ie.value) {
                    let result = $expr;
                    if let Ok(value) = $crate::s1ap::encode_open_type(&$bind) {
                        ie.value = value;
                        matched = result;
                    }
                }
                break;
            }
        }
        matched
    }};
}

/// Build a complete `S1AP_PDU` from a direction, procedure, message, and IEs.
///
/// # Panics
///
/// Panics if a value cannot be APER-encoded into its open type, such as an
/// integer outside its constraint.
#[macro_export]
macro_rules! build_s1ap {
    ($direction:ident, $proc:ident,
     $outer_crit:ident, $msg:ident,
     $($ie_crit:ident $ie_name:ident ($($ie_value:tt)+)),*
     $(,)?
    ) => {
        $crate::__paste::paste! {{
            let ies = vec![
                $( $crate::s1ap::ProtocolIEField {
                    id: $crate::s1ap::ProtocolIEID($crate::__s1ap_ie_id!($ie_name)),
                    criticality: $crate::build_s1ap!(@criticality $ie_crit),
                    value: $crate::__s1ap_encode_ie!($ie_name, ($($ie_value)+))
                        .expect("failed to APER-encode S1AP IE open type"),
                }, )*
            ];
            let message = $crate::s1ap::$msg::new(
                $crate::s1ap::ProtocolIEContainer(ies),
            );
            let value = $crate::s1ap::encode_open_type(&message)
                .expect("failed to APER-encode S1AP message open type");
            $crate::build_s1ap!(@pdu $direction, $proc, $outer_crit, value)
        }}
    };
    (@criticality REJECT) => { $crate::s1ap::Criticality::reject };
    (@criticality IGNORE) => { $crate::s1ap::Criticality::ignore };
    (@criticality NOTIFY) => { $crate::s1ap::Criticality::notify };
    (@pdu InitiatingMessage, $proc:ident, $criticality:ident, $value:expr) => {
        $crate::s1ap::S1AP_PDU::initiatingMessage($crate::s1ap::InitiatingMessage {
            procedure_code: $crate::s1ap::ProcedureCode($crate::__s1ap_proc_code!($proc)),
            criticality: $crate::build_s1ap!(@criticality $criticality),
            value: $value,
        })
    };
    (@pdu SuccessfulOutcome, $proc:ident, $criticality:ident, $value:expr) => {
        $crate::s1ap::S1AP_PDU::successfulOutcome($crate::s1ap::SuccessfulOutcome {
            procedure_code: $crate::s1ap::ProcedureCode($crate::__s1ap_proc_code!($proc)),
            criticality: $crate::build_s1ap!(@criticality $criticality),
            value: $value,
        })
    };
    (@pdu UnsuccessfulOutcome, $proc:ident, $criticality:ident, $value:expr) => {
        $crate::s1ap::S1AP_PDU::unsuccessfulOutcome($crate::s1ap::UnsuccessfulOutcome {
            procedure_code: $crate::s1ap::ProcedureCode($crate::__s1ap_proc_code!($proc)),
            criticality: $crate::build_s1ap!(@criticality $criticality),
            value: $value,
        })
    };
}

/// Build one S1AP protocol IE entry. Every message takes the same
/// `ProtocolIEField`: the message name says where the IE goes and is not used.
///
/// # Panics
///
/// Panics if the value cannot be APER-encoded into its open type.
#[macro_export]
macro_rules! build_s1ap_ie {
    ($msg:ident, $criticality:ident $ie_name:ident ($($value:tt)+)) => {
        $crate::__paste::paste! {
            $crate::s1ap::ProtocolIEField {
                id: $crate::s1ap::ProtocolIEID($crate::__s1ap_ie_id!($ie_name)),
                criticality: $crate::build_s1ap!(@criticality $criticality),
                value: $crate::__s1ap_encode_ie!($ie_name, ($($value)+))
                    .expect("failed to APER-encode S1AP IE open type"),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::s1ap::*;

    #[test]
    fn mutating_macro_updates_newtype_payload() {
        let mut pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,
            REJECT MME_UE_S1AP_ID(1u32),
            REJECT eNB_UE_S1AP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        );

        let S1AP_PDU::initiatingMessage(message) = &mut pdu else {
            panic!("expected initiating message");
        };
        let request: UEContextReleaseRequest =
            rasn::aper::decode(message.value.as_bytes()).expect("decode request");
        let mut request = request;

        assert!(with_s1ap_ie_mut!(
            request,
            UEContextReleaseRequest,
            MME_UE_S1AP_ID(id) = 42u32
        ));

        let ie = request
            .protocol_ies
            .0
            .iter()
            .find(|ie| ie.id.0 == 0)
            .expect("MME UE ID");
        let id: MMEUES1APID = rasn::aper::decode(ie.value.as_bytes()).expect("decode ID");
        assert_eq!(id.0, 42);
    }

    #[test]
    fn builder_macros_cover_ie_and_outcome_forms() {
        let ie = build_s1ap_ie!(
            UEContextReleaseRequest,
            REJECT MME_UE_S1AP_ID(1u32)
        );
        assert_eq!(ie.id.0, 0);

        let successful = build_s1ap!(SuccessfulOutcome, S1Setup, REJECT, S1SetupResponse,);
        assert!(successful.is_successful());
        assert_eq!(successful.procedure_code(), 17);

        let unsuccessful = build_s1ap!(UnsuccessfulOutcome, S1Setup, REJECT, S1SetupFailure,);
        assert!(unsuccessful.is_unsuccessful());
        assert_eq!(unsuccessful.procedure_code(), 17);
    }
}
