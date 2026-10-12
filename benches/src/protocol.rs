/*
   OxiRush
   Copyright 2025-2026 Valentin D'Emmanuele

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

//! What the benchmarks have of S1AP: the type of a PDU, four messages of the
//! size that a network sends, and the operations that take the type of a
//! message. `lib.rs`, which is the same for oxirush-ngap, has the rest.

pub use oxirush_s1ap as codec;

use crate::{Typed, Written, done};
use codec::s1ap;
use codec::{Error, build_s1ap, extract_s1ap_ies};
use serde_json::{Value, json};

/// A PDU of the protocol, which owns its octets.
pub type Pdu = s1ap::S1AP_PDU;

/// A PDU that borrows its message from the octets that it is decoded from.
pub type Borrowed<'a> = s1ap::S1APPDU<'a>;

/// The fixtures of the crate: a line for each message, with its name and its
/// octets in hexadecimal.
pub const FIXTURES: &str = include_str!("../../tests/fixtures/messages.tsv");

/// The root of the paths of the IEs of a message.
pub const ROOT: &str = "/s1ap";

/// The operations that take the type of a message: its direction, its
/// procedure, the criticality of its PDU and its type, then each of its IEs
/// as the macros name it, with its criticality and the type of its value.
macro_rules! typed {
    ($direction:ident $procedure:ident $criticality:ident $message:ident {
        $($ie_criticality:ident $ie:ident: $type:ident $(<$borrows:lifetime>)?,)*
    }) => {{
        /// The values of the IEs of the message, in their order. Those that hold
        /// octets borrow them.
        type Values<'a> = ($(s1ap::$type $(<$borrows>)?,)*);

        #[allow(non_snake_case)]
        fn read<'a>(message: &'a s1ap::$message<'a>) -> Result<Values<'a>, Error> {
            extract_s1ap_ies!(message, $message,
                $(req $ie: s1ap::$type $(<$borrows>)? = $ie(value) => value,)*
            );
            Ok(($($ie,)*))
        }

        /// The same values, which borrow nothing.
        #[allow(non_snake_case)]
        fn owned(($($ie,)*): Values<'_>) -> Values<'static> {
            ($(typed!(@owned $ie $(, $borrows)?),)*)
        }

        #[allow(non_snake_case)]
        fn build(($($ie,)*): Values<'_>) -> Result<Pdu, Error> {
            build_s1ap!($direction, $procedure, $criticality, $message,
                $($ie_criticality $ie($ie),)*
            )
        }

        fn message(pdu: &Pdu) -> s1ap::$message<'_> {
            pdu.decode_value().unwrap()
        }

        Typed {
            decode_value: |pdu| done(pdu.decode_value::<s1ap::$message<'_>>()),
            extract: |pdu| {
                let message = message(pdu);
                Box::new(move || done(read(&message)))
            },
            build: |pdu| {
                let mut values = Some(owned(read(&message(pdu)).unwrap()));
                Box::new(move || done(build(values.take().unwrap())))
            },
            rebuilt: |pdu| build(read(&message(pdu)).unwrap()).unwrap(),
        }
    }};
    (@owned $value:ident) => {
        $value
    };
    (@owned $value:ident, $borrows:lifetime) => {
        $value.into_owned()
    };
}

/// `length` octets in hexadecimal, which are not all the same.
fn octets(length: usize) -> String {
    (0..length)
        .map(|at| format!("{:02X}", (at * 7 + 3) % 251))
        .collect()
}

/// What an E-RAB is set up with: its QoS, the tunnel of its gateway and a
/// NAS-PDU of 96 octets.
fn e_rab(id: u8) -> Value {
    json!({
        "e-RAB-ID": id,
        "e-RABlevelQoSParameters": {
            "qCI": 9,
            "allocationRetentionPriority": {
                "priorityLevel": 8,
                "pre-emptionCapability": "shall-not-trigger-pre-emption",
                "pre-emptionVulnerability": "not-pre-emptable",
            },
        },
        "transportLayerAddress": format!("10.0.0.{id}"),
        "gTP-TEID": 0x0100_0000 + u32::from(id),
        "nAS-PDU": octets(96),
    })
}

/// The bit rates of a UE.
fn ue_bit_rates() -> Value {
    json!({
        "uEaggregateMaximumBitRateDL": 2_000_000_000,
        "uEaggregateMaximumBitRateUL": 1_000_000_000,
    })
}

/// An INITIAL CONTEXT SETUP REQUEST with eight E-RABs and the radio
/// capability of the UE.
fn initial_context_setup_request() -> Written {
    let e_rab = |id: u8| json!({"E-RABToBeSetupItemCtxtSUReq": e_rab(id)});
    let tree = json!({"message": "InitialContextSetupRequest", "ies": [
        {"MME-UE-S1AP-ID": 4_000_000_000u32},
        {"eNB-UE-S1AP-ID": 16_000_000},
        {"uEaggregateMaximumBitrate": ue_bit_rates()},
        {"E-RABToBeSetupListCtxtSUReq": (5..=12).map(e_rab).collect::<Vec<_>>()},
        {"UESecurityCapabilities": {
            "encryptionAlgorithms": "E000",
            "integrityProtectionAlgorithms": "E000",
        }},
        {"SecurityKey": octets(32)},
        {"HandoverRestrictionList": {"servingPLMN": "208-93"}},
        {"UERadioCapability": octets(400)},
        {"Masked-IMEISV": "3554900000FFFF10"},
    ]});
    Written {
        name: "initial-context-setup-request",
        what: "INITIAL CONTEXT SETUP REQUEST with 8 E-RABs",
        tree,
        path: "/s1ap/E-RABToBeSetupListCtxtSUReq/7/E-RABToBeSetupItemCtxtSUReq\
               /e-RABlevelQoSParameters/allocationRetentionPriority/priorityLevel",
        edit: (
            "/s1ap/E-RABToBeSetupListCtxtSUReq/7/E-RABToBeSetupItemCtxtSUReq\
             /e-RABlevelQoSParameters/allocationRetentionPriority/priorityLevel",
            json!(3),
        ),
        typed: typed!(InitiatingMessage InitialContextSetup REJECT InitialContextSetupRequest {
            REJECT MME_UE_S1AP_ID: MMEUES1APID,
            REJECT eNB_UE_S1AP_ID: ENBUES1APID,
            REJECT uEaggregateMaximumBitrate: UEAggregateMaximumBitrate<'a>,
            REJECT E_RABToBeSetupListCtxtSUReq: ERABToBeSetupListCtxtSUReq<'a>,
            REJECT UESecurityCapabilities: UESecurityCapabilities<'a>,
            REJECT SecurityKey: SecurityKey<'a>,
            IGNORE HandoverRestrictionList: HandoverRestrictionList<'a>,
            IGNORE UERadioCapability: UERadioCapability<'a>,
            IGNORE Masked_IMEISV: MaskedIMEISV,
        }),
    }
}

/// An E-RAB SETUP REQUEST with eight E-RABs.
fn e_rab_setup_request() -> Written {
    let e_rab = |id: u8| json!({"E-RABToBeSetupItemBearerSUReq": e_rab(id)});
    let tree = json!({"message": "E-RABSetupRequest", "ies": [
        {"MME-UE-S1AP-ID": 4_000_000_000u32},
        {"eNB-UE-S1AP-ID": 16_000_000},
        {"uEaggregateMaximumBitrate": ue_bit_rates()},
        {"E-RABToBeSetupListBearerSUReq": (5..=12).map(e_rab).collect::<Vec<_>>()},
    ]});
    Written {
        name: "e-rab-setup-request",
        what: "E-RAB SETUP REQUEST with 8 E-RABs",
        tree,
        path: "/s1ap/E-RABToBeSetupListBearerSUReq/7/E-RABToBeSetupItemBearerSUReq/gTP-TEID",
        edit: (
            "/s1ap/E-RABToBeSetupListBearerSUReq/7/E-RABToBeSetupItemBearerSUReq/gTP-TEID",
            json!(0x0200_000c),
        ),
        typed: typed!(InitiatingMessage E_RABSetup REJECT ERABSetupRequest {
            REJECT MME_UE_S1AP_ID: MMEUES1APID,
            REJECT eNB_UE_S1AP_ID: ENBUES1APID,
            REJECT uEaggregateMaximumBitrate: UEAggregateMaximumBitrate<'a>,
            REJECT E_RABToBeSetupListBearerSUReq: ERABToBeSetupListBearerSUReq<'a>,
        }),
    }
}

/// How many octets the RRC container of the HANDOVER REQUIRED has, for a
/// transparent container of 4,000 octets.
const RRC_CONTAINER: usize = 3_967;

/// A HANDOVER REQUIRED between two eNBs, with a transparent container of
/// 4,000 octets, most of which is its RRC container.
fn handover_required() -> Written {
    let cell = |identity: u32| json!({"pLMNidentity": "208-93", "cell-ID": identity});
    let container = json!({"SourceeNB-ToTargeteNB-TransparentContainer": {
        "rRC-Container": octets(RRC_CONTAINER),
        "e-RABInformationList": [
            {"E-RABInformationListItem": {"e-RAB-ID": 5}},
            {"E-RABInformationListItem": {"e-RAB-ID": 6}},
        ],
        "targetCell-ID": cell(0x0004_0001),
        "uE-HistoryInformation": [{"e-UTRAN-Cell": {
            "global-Cell-ID": cell(0x0003_0001),
            "cellType": {"cell-Size": "medium"},
            "time-UE-StayedInCell": 600,
        }}],
    }});
    let tree = json!({"message": "HandoverRequired", "ies": [
        {"MME-UE-S1AP-ID": 4_000_000_000u32},
        {"eNB-UE-S1AP-ID": 16_000_000},
        {"HandoverType": "intralte"},
        {"Cause": {"radioNetwork": "handover-desirable-for-radio-reason"}},
        {"TargetID": {"targeteNB-ID": {
            "global-ENB-ID": {"pLMNidentity": "208-93", "eNB-ID": {"macroENB-ID": "000040"}},
            "selected-TAI": {"pLMNidentity": "208-93", "tAC": 1},
        }}},
        {"Direct-Forwarding-Path-Availability": "directPathAvailable"},
        {"Source-ToTarget-TransparentContainer": container},
    ]});
    Written {
        name: "handover-required",
        what: "HANDOVER REQUIRED with a transparent container of 4,000 octets",
        tree,
        path: "/s1ap/TargetID/targeteNB-ID/selected-TAI/tAC",
        edit: ("/s1ap/TargetID/targeteNB-ID/selected-TAI/tAC", json!(2)),
        typed: typed!(InitiatingMessage HandoverPreparation REJECT HandoverRequired {
            REJECT MME_UE_S1AP_ID: MMEUES1APID,
            REJECT eNB_UE_S1AP_ID: ENBUES1APID,
            REJECT HandoverType: HandoverType,
            IGNORE Cause: Cause,
            REJECT TargetID: TargetID<'a>,
            IGNORE Direct_Forwarding_Path_Availability: DirectForwardingPathAvailability,
            REJECT Source_ToTarget_TransparentContainer: SourceToTargetTransparentContainer<'a>,
        }),
    }
}

/// A PAGING for sixteen tracking areas.
fn paging() -> Written {
    let area = |tac: u16| json!({"TAIItem": {"tAI": {"pLMNidentity": "208-93", "tAC": tac}}});
    let tree = json!({"message": "Paging", "ies": [
        {"UEIdentityIndexValue": "0A40"},
        {"UEPagingID": {"s-TMSI": {"mMEC": 1, "m-TMSI": 3_661_752_294u32}}},
        {"pagingDRX": "v128"},
        {"CNDomain": "ps"},
        {"TAIList": (1..=16).map(area).collect::<Vec<_>>()},
        {"PagingPriority": "priolevel1"},
    ]});
    Written {
        name: "paging",
        what: "PAGING for 16 tracking areas",
        tree,
        path: "/s1ap/TAIList/15/TAIItem/tAI/tAC",
        edit: ("/s1ap/TAIList/15/TAIItem/tAI/tAC", json!(17)),
        typed: typed!(InitiatingMessage Paging IGNORE Paging {
            IGNORE UEIdentityIndexValue: UEIdentityIndexValue,
            IGNORE UEPagingID: UEPagingID<'a>,
            IGNORE pagingDRX: PagingDRX,
            IGNORE CNDomain: CNDomain,
            IGNORE TAIList: TAIList<'a>,
            IGNORE PagingPriority: PagingPriority,
        }),
    }
}

/// The messages of the size that a network sends.
pub(crate) fn written() -> Vec<Written> {
    vec![
        initial_context_setup_request(),
        e_rab_setup_request(),
        handover_required(),
        paging(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use codec::inspect;

    /// How many octets a string of a view is, which has them in hexadecimal.
    fn length(view: &Value, path: &str) -> usize {
        let selected = inspect::select(view, path).unwrap();
        selected[0].as_str().unwrap().len() / 2
    }

    /// How many entries a list of a view has.
    fn entries(view: &Value, path: &str) -> usize {
        inspect::select(view, path).unwrap().len()
    }

    /// The messages have the sizes that their inputs are named after.
    #[test]
    fn the_written_messages_have_the_sizes_that_they_say() {
        let view = |written: Written| Pdu::from_tree(&written.tree).unwrap().to_view().unwrap();

        let setup = view(initial_context_setup_request());
        let e_rabs = "/s1ap/E-RABToBeSetupListCtxtSUReq/value/*";
        assert_eq!(entries(&setup, e_rabs), 8);

        let e_rabs = view(e_rab_setup_request());
        let listed = "/s1ap/E-RABToBeSetupListBearerSUReq/value/*";
        assert_eq!(entries(&e_rabs, listed), 8);

        let handover = view(handover_required());
        let container = "/s1ap/Source-ToTarget-TransparentContainer/value";
        assert_eq!(length(&handover, container), 4_000);

        let paging = view(paging());
        assert_eq!(entries(&paging, "/s1ap/TAIList/value/*"), 16);
    }
}
