//! Demonstrate `extract_s1ap_ies!` across representative S1AP procedures.
//!
//! The three sections decode complete S1AP PDUs and extract required/optional
//! IEs, custom expressions, nested handover choices, and an EPS NAS PDU carried
//! inside the Initial Context Setup E-RAB list.

use oxirush_s1ap::helpers::*;
use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{Error, build_s1ap, extract_s1ap_ies};

// ── 1. Simple extraction: required IDs + optional cause ────────────────────

fn handle_release_request(pdu: &S1AP_PDU) -> Result<Vec<String>, Error> {
    // The message of the PDU, as its type: another message is an error.
    let msg: UEContextReleaseRequest = pdu.decode_value()?;

    // A `req` field whose IE is absent is an error of the kind `MissingIe`.
    // An `opt` field is an Option<T>, `None` when its IE is absent.
    // An IE that is there and does not decode is an error for both.
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

fn handle_handover_required(pdu: &S1AP_PDU) -> Result<Vec<String>, Error> {
    let msg: HandoverRequired = pdu.decode_value()?;

    // Demonstrates concrete ENUMERATED values, a nested TargetID CHOICE,
    // transparent-container lengths, and custom extraction expressions.
    extract_s1ap_ies!(&msg, HandoverRequired,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        req enb_id: u32 = eNB_UE_S1AP_ID(id),

        opt ho_type: HandoverType = HandoverType(value) => value,
        opt cause_str: String = Cause(cause) => format!("{cause:?}"),

        opt target: String = TargetID(target_value) => match target_value {
            TargetID::targeteNB_ID(value) => {
                match plmn_from(&value.global_enb_id.p_lmnidentity) {
                    Some((mcc, mnc)) => format!("target eNB in PLMN {mcc}-{mnc}"),
                    None => "target eNB without a PLMN identity".to_string(),
                }
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

fn handle_initial_context_setup(pdu: &S1AP_PDU) -> Result<Vec<String>, Error> {
    let msg: InitialContextSetupRequest = pdu.decode_value()?;

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

fn main() -> Result<(), Error> {
    // ── Build test PDUs ─────────────────────────────────────────────────────

    // 1. UEContextReleaseRequest
    let release_pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,

        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;

    // 2. HandoverRequired
    let cause = Cause::radioNetwork(CauseRadioNetwork::handover_desirable_for_radio_reason);
    let handover_pdu = build_s1ap!(InitiatingMessage, HandoverPreparation,
        REJECT, HandoverRequired,

        REJECT MME_UE_S1AP_ID(100u32),
        REJECT eNB_UE_S1AP_ID(50u32),

        REJECT HandoverType(HandoverType::intralte),
        IGNORE Cause(cause),
        REJECT TargetID(target_enb()?),

        REJECT Source_ToTarget_TransparentContainer(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    )?;

    // 3. InitialContextSetupRequest. The NAS PDU is nested in its E-RAB item,
    // as specified by TS 36.413 rather than as a top-level message IE.
    let bit_rate =
        UEAggregateMaximumBitrate::new(BitRate(1_000_000_000), BitRate(500_000_000), None);
    let erabs = erab_to_be_setup_list(vec![0x07, 0x41, 0x00, 0x01]);
    let ics_pdu = build_s1ap!(InitiatingMessage, InitialContextSetup,
        REJECT, InitialContextSetupRequest,

        REJECT MME_UE_S1AP_ID(200u32),
        REJECT eNB_UE_S1AP_ID(10u32),

        REJECT UEAggregateMaximumBitrate(bit_rate),
        REJECT E_RABToBeSetupListCtxtSUReq(erabs),

        REJECT UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
        REJECT SecurityKey([0; 32]),
    )?;

    // Encode → decode round-trip, then extract.
    type Handler = fn(&S1AP_PDU) -> Result<Vec<String>, Error>;
    let cases: [(&str, &S1AP_PDU, Handler); 3] = [
        (
            "UEContextReleaseRequest",
            &release_pdu,
            handle_release_request,
        ),
        ("HandoverRequired", &handover_pdu, handle_handover_required),
        (
            "InitialContextSetupRequest",
            &ics_pdu,
            handle_initial_context_setup,
        ),
    ];

    for (name, pdu, handler) in cases {
        let bytes = pdu.encode()?;
        let decoded = S1AP_PDU::decode(&bytes)?;

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

    // The `decode_manually` example reads the IEs of a message by hand, which is
    // what `extract_s1ap_ies!` does for each field.
    Ok(())
}

fn target_enb() -> Result<TargetID<'static>, Error> {
    let network = plmn("208", "93")?;
    let enb = global_enb_id(network, 0x12345)?;
    let area = tai(network, [0x00, 0x01]);

    Ok(TargetID::targeteNB_ID(TargeteNBID::new(enb, area, None)))
}

fn erab_to_be_setup_list(nas: Vec<u8>) -> ERABToBeSetupListCtxtSUReq<'static> {
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
        ERABID(5),
        qos,
        TransportLayerAddress([10, 0, 0, 1].into()),
        GTPTEID::from([0, 0, 0, 1]),
        Some(NASPDU::from(nas)),
        None,
    );

    // An item of the list is an IE itself, alone in its container.
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
