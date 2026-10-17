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

// The messages of `readers`, whose program oxirush-aper has (`src/bin/readers.rs` of its
// `protocols/benches/`) and includes this file: the four that `protocol.rs` writes, each
// with its IEs, the first first.

readers!(initial_context_setup_request InitialContextSetupRequest {
    MME_UE_S1AP_ID: MMEUES1APID,
    eNB_UE_S1AP_ID: ENBUES1APID,
    uEaggregateMaximumBitrate: UEAggregateMaximumBitrate,
    E_RABToBeSetupListCtxtSUReq: ERABToBeSetupListCtxtSUReq<'a>,
    UESecurityCapabilities: UESecurityCapabilities<'a>,
    SecurityKey: SecurityKey<'a>,
    HandoverRestrictionList: HandoverRestrictionList,
    UERadioCapability: UERadioCapability<'a>,
    Masked_IMEISV: MaskedIMEISV,
});

readers!(e_rab_setup_request ERABSetupRequest {
    MME_UE_S1AP_ID: MMEUES1APID,
    eNB_UE_S1AP_ID: ENBUES1APID,
    uEaggregateMaximumBitrate: UEAggregateMaximumBitrate,
    E_RABToBeSetupListBearerSUReq: ERABToBeSetupListBearerSUReq<'a>,
});

readers!(handover_required HandoverRequired {
    MME_UE_S1AP_ID: MMEUES1APID,
    eNB_UE_S1AP_ID: ENBUES1APID,
    HandoverType: HandoverType,
    Cause: Cause,
    TargetID: TargetID,
    Direct_Forwarding_Path_Availability: DirectForwardingPathAvailability,
    Source_ToTarget_TransparentContainer: SourceToTargetTransparentContainer<'a>,
});

readers!(paging Paging {
    UEIdentityIndexValue: UEIdentityIndexValue,
    UEPagingID: UEPagingID<'a>,
    pagingDRX: PagingDRX,
    CNDomain: CNDomain,
    TAIList: TAIList<'a>,
    PagingPriority: PagingPriority,
});

/// The E-RABs of the list of an E-RAB SETUP REQUEST, by `decode_value`, the macro and the
/// decoding of each item.
fn bearers_decoded(pdu: &Pdu) {
    let message: types::ERABSetupRequest = pdu.decode_value().unwrap();
    let read = || -> Result<(), Error> {
        extract_ies!(message, ERABSetupRequest,
            req list: types::ERABToBeSetupListBearerSUReq = E_RABToBeSetupListBearerSUReq(list) => list,
        );
        for item in &list.0 {
            let bearer: types::ERABToBeSetupItemBearerSUReq = types::decode_open_type(&item.value)?;
            black_box(bearer);
        }
        Ok(())
    };
    read().unwrap();
}

/// The same E-RABs, walked in the PDU by `items::<L>()`.
#[cfg(feature = "readers")]
fn bearers_walked(pdu: &Pdu) {
    for bearer in pdu.items::<codec::ie::E_RABToBeSetupListBearerSUReq>() {
        black_box(bearer.unwrap());
    }
}

/// The operations on the E-RABs of an E-RAB SETUP REQUEST.
const BEARERS: Operations = &[
    ("decode_value, the macro, then each E-RAB", bearers_decoded),
    #[cfg(feature = "readers")]
    ("items::<L>() of the PDU: each E-RAB", bearers_walked),
];

/// The operations on each message, by its name in `written()`, and the check that the
/// readers read what the macro reads.
const MESSAGES: &[Message] = &[
    (
        "initial-context-setup-request",
        initial_context_setup_request::OPERATIONS,
        initial_context_setup_request::same,
    ),
    (
        "e-rab-setup-request",
        e_rab_setup_request::OPERATIONS,
        e_rab_setup_request::same,
    ),
    (
        "handover-required",
        handover_required::OPERATIONS,
        handover_required::same,
    ),
    ("paging", paging::OPERATIONS, paging::same),
    ("e-rab-setup-request", BEARERS, nothing),
];
