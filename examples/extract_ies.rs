//! Demonstrate `extract_s1ap_ies!` across representative S1AP procedures.
//!
//! The three sections decode complete S1AP PDUs and extract required/optional
//! IEs, custom expressions, nested handover choices, and an EPS NAS PDU carried
//! inside the Initial Context Setup E-RAB list.

use rasn::types::FixedBitString;

use oxirush_s1ap::helpers::*;
use oxirush_s1ap::macros::MissingIeError;
use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{build_s1ap, extract_s1ap_ies};

// ── 1. Simple extraction: required IDs + optional cause ────────────────────

fn handle_release_request(pdu: &S1AP_PDU) -> Result<Vec<String>, MissingIeError> {
    let Some(msg): Option<UEContextReleaseRequest> =
        decode_initiating(pdu, ID_UECONTEXT_RELEASE_REQUEST.0)
    else {
        return Ok(vec![]);
    };

    // `req` fields return Err(MissingIeError) if absent or invalid.
    // `opt` fields become Option<T>.
    // Without `=> expr`, extraction defaults to `.0` (newtype unwrap).
    extract_s1ap_ies!(&msg, UEContextReleaseRequest,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        opt cause: String = Cause(c) => format!("{c:?}"),
    );

    let mut result = vec![
        format!("MME-UE-S1AP-ID: {mme_id}"),
        format!("eNB-UE-S1AP-ID: {enb_id}"),
    ];
    if let Some(cause) = cause {
        result.push(format!("Cause: {cause}"));
    }
    Ok(result)
}

// ── 2. Complex extraction: handover with pattern matching ──────────────────

fn handle_handover_required(pdu: &S1AP_PDU) -> Result<Vec<String>, MissingIeError> {
    let Some(msg): Option<HandoverRequired> = decode_initiating(pdu, ID_HANDOVER_PREPARATION.0)
    else {
        return Ok(vec![]);
    };

    // Demonstrates concrete ENUMERATED values, a nested TargetID CHOICE,
    // transparent-container lengths, and custom extraction expressions.
    extract_s1ap_ies!(&msg, HandoverRequired,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        opt ho_type: HandoverType = HandoverType(value) => value,
        opt cause_str: String = Cause(cause) => format!("{cause:?}"),
        opt target: String = TargetID(target_value) => match target_value {
            TargetID::targeteNB_ID(value) => {
                let (mcc, mnc) = plmn_from(&value.global_enb_id.p_lmnidentity);
                format!("target eNB in PLMN {mcc}-{mnc}")
            },
            _ => "non-eNB target".to_string(),
        },
        opt container_len: usize =
            Source_ToTarget_TransparentContainer(container) => container.0.len(),
    );

    Ok(vec![
        format!("MME-UE-S1AP-ID: {mme_id}"),
        format!("eNB-UE-S1AP-ID: {enb_id}"),
        format!("HandoverType: {ho_type:?}"),
        format!("Cause: {cause_str:?}"),
        format!("Target: {target:?}"),
        format!("S2T container bytes: {container_len:?}"),
    ])
}

// ── 3. InitialContextSetupRequest: many IEs + nested EPS bearer ────────────

fn handle_initial_context_setup(pdu: &S1AP_PDU) -> Result<Vec<String>, MissingIeError> {
    let Some(msg): Option<InitialContextSetupRequest> =
        decode_initiating(pdu, ID_INITIAL_CONTEXT_SETUP.0)
    else {
        return Ok(vec![]);
    };

    extract_s1ap_ies!(&msg, InitialContextSetupRequest,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        opt erabs: ERABToBeSetupListCtxtSUReq =
            E_RABToBeSetupListCtxtSUReq(list) => list,
        opt ambr_dl: u64 = UEAggregateMaximumBitrate(ambr) =>
            ambr.u_eaggregate_maximum_bit_rate_dl.0,
    );

    let erab_count = erabs.as_ref().map_or(0, |list| list.0.len());
    let nas_len = erabs.as_ref().and_then(nested_nas_len).unwrap_or(0);
    Ok(vec![
        format!("MME-UE-S1AP-ID: {mme_id}"),
        format!("eNB-UE-S1AP-ID: {enb_id}"),
        format!("E-RABs: {erab_count}"),
        format!("EPS NAS PDU: {nas_len} bytes"),
        format!("DL AMBR: {ambr_dl:?} bps"),
    ])
}

fn main() {
    // ── Build test PDUs ─────────────────────────────────────────────────────

    // 1. UEContextReleaseRequest
    let release_pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    );

    // 2. HandoverRequired
    let handover_pdu = build_s1ap!(InitiatingMessage, HandoverPreparation,
        REJECT, HandoverRequired,
        REJECT MME_UE_S1AP_ID(100u32),
        REJECT eNB_UE_S1AP_ID(50u32),
        REJECT HandoverType(HandoverType::intralte),
        IGNORE Cause(Cause::radioNetwork(
            CauseRadioNetwork::handover_desirable_for_radio_reason,
        )),
        REJECT TargetID(target_enb()),
        REJECT Source_ToTarget_TransparentContainer(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    );

    // 3. InitialContextSetupRequest. The NAS PDU is nested in its E-RAB item,
    // as specified by TS 36.413 rather than as a top-level message IE.
    let ics_pdu = build_s1ap!(InitiatingMessage, InitialContextSetup,
        REJECT, InitialContextSetupRequest,
        REJECT MME_UE_S1AP_ID(200u32),
        REJECT eNB_UE_S1AP_ID(10u32),
        REJECT UEAggregateMaximumBitrate(UEAggregateMaximumBitrate::new(
            BitRate(1_000_000_000),
            BitRate(500_000_000),
            None,
        )),
        REJECT E_RABToBeSetupListCtxtSUReq(erab_to_be_setup_list(vec![
            0x07, 0x41, 0x00, 0x01,
        ])),
        REJECT UESecurityCapabilities(ue_security_capabilities(&[0xe0, 0xe0])),
        REJECT SecurityKey(SecurityKey(FixedBitString::<256>::ZERO)),
    );

    // Encode → decode round-trip, then extract.
    for (name, pdu, handler) in [
        (
            "UEContextReleaseRequest",
            &release_pdu,
            handle_release_request as fn(&S1AP_PDU) -> Result<Vec<String>, MissingIeError>,
        ),
        ("HandoverRequired", &handover_pdu, handle_handover_required),
        (
            "InitialContextSetupRequest",
            &ics_pdu,
            handle_initial_context_setup,
        ),
    ] {
        let bytes = pdu.encode().expect("encode failed");
        let decoded = S1AP_PDU::decode(&bytes).expect("decode failed");

        println!("=== {name} ({decoded}) ===");
        match handler(&decoded) {
            Ok(lines) => {
                for line in &lines {
                    println!("  {line}");
                }
            }
            Err(error) => eprintln!("  Error: {error}"),
        }
        println!();
    }

    // ── Equivalent hand-written extraction (for comparison) ────────────────
    // Without `extract_s1ap_ies!`, release-request extraction would manually:
    //
    //   let mut mme_id = None;
    //   for ie in &msg.protocol_ies.0 {
    //       if ie.id == ID_MME_UE_S1AP_ID {
    //           if let Ok(value) = decode_open_type::<MMEUES1APID>(&ie.value) {
    //               mme_id = Some(value.0);
    //           }
    //       }
    //   }
    //   let mme_id = mme_id.ok_or(MissingIeError { ie_name: "mme_id" })?;
}

fn decode_initiating<T: rasn::Decode>(pdu: &S1AP_PDU, procedure_code: u8) -> Option<T> {
    match pdu {
        S1AP_PDU::initiatingMessage(message) if message.procedure_code.0 == procedure_code => {
            decode_open_type(&message.value).ok()
        }
        _ => None,
    }
}

fn target_enb() -> TargetID {
    let network = plmn("208", "93");
    TargetID::targeteNB_ID(TargeteNBID::new(
        global_enb_id(network.clone(), 0x12345),
        tai(network, &[0x00, 0x01]),
        None,
    ))
}

fn erab_to_be_setup_list(nas: Vec<u8>) -> ERABToBeSetupListCtxtSUReq {
    let qos = ERABLevelQoSParameters::new(
        QCI(9),
        AllocationAndRetentionPriority::new(
            PriorityLevel(15),
            PreEmptionCapability::shall_not_trigger_pre_emption,
            PreEmptionVulnerability::not_pre_emptable,
            None,
        ),
        None,
        None,
    );
    let item = ERABToBeSetupItemCtxtSUReq::new(
        ERABID(5u8.into()),
        qos,
        TransportLayerAddress(bytes_to_bitvec(&[10, 0, 0, 1])),
        GTPTEID::from([0, 0, 0, 1]),
        Some(NASPDU::from(nas)),
        None,
    );
    ERABToBeSetupListCtxtSUReq(vec![ProtocolIEField::new(
        ID_E_RABTO_BE_SETUP_ITEM_CTXT_SUREQ.0,
        Criticality::reject,
        encode_open_type(&item).expect("encode E-RAB item"),
    )])
}

fn nested_nas_len(list: &ERABToBeSetupListCtxtSUReq) -> Option<usize> {
    list.0.iter().find_map(|entry| {
        let item: ERABToBeSetupItemCtxtSUReq = decode_open_type(&entry.value).ok()?;
        item.n_as_pdu.map(|nas| nas.0.len())
    })
}
