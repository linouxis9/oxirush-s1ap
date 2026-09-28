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
//! IE IDs and procedure codes are generated from the ASN.1 object sets. Values
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
//! fn source(gummei: GUMMEI) -> AnonymousPathSwitchRequestProtocolIEs {
//!     build_s1ap_ie!(PathSwitchRequest, IGNORE SourceMME_GUMMEI(gummei))
//! }
//! ```
//!
//! ```compile_fail
//! use oxirush_s1ap::{build_s1ap_ie, s1ap::*};
//!
//! fn source(gummei: GUMMEI) -> AnonymousPathSwitchRequestProtocolIEs {
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
                $( $crate::s1ap::[< Anonymous $msg ProtocolIEs >] {
                    id: $crate::s1ap::ProtocolIEID($crate::__s1ap_ie_id!($ie_name)),
                    criticality: $crate::build_s1ap!(@criticality $ie_crit),
                    value: $crate::__s1ap_encode_ie!($ie_name, ($($ie_value)+))
                        .expect("failed to APER-encode S1AP IE open type"),
                }, )*
            ];
            let message = $crate::s1ap::$msg::new(
                $crate::s1ap::[< $msg ProtocolIEs >](ies),
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

/// Build one S1AP Protocol IE entry for a message type.
///
/// # Panics
///
/// Panics if the value cannot be APER-encoded into its open type.
#[macro_export]
macro_rules! build_s1ap_ie {
    ($msg:ident, $criticality:ident $ie_name:ident ($($value:tt)+)) => {
        $crate::__paste::paste! {
            $crate::s1ap::[< Anonymous $msg ProtocolIEs >] {
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
