//! Build S1AP PDUs using the `build_s1ap!` and `build_s1ap_ie!` ergonomic macros.
//!
//! The examples cover simple and complex context setup, UE release, handover
//! outcomes, failure handling, and standalone IE construction. The macro
//! invocations hide numeric IDs and open-type encoding; the commented
//! construction at the end shows the equivalent generated types.

use oxirush_s1ap::helpers::*;
use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{Error, build_s1ap, build_s1ap_ie};

fn main() -> Result<(), Error> {
    // ── 1. Simple: InitialContextSetupResponse ─────────────────────────────

    let mme_ue_id: u32 = 1;
    let enb_ue_id: u32 = 0;

    // The direction, the procedure, its criticality and the message, then
    // each IE with its criticality.
    let pdu = build_s1ap!(SuccessfulOutcome, InitialContextSetup,
        REJECT, InitialContextSetupResponse,

        IGNORE MME_UE_S1AP_ID(mme_ue_id),
        IGNORE eNB_UE_S1AP_ID(enb_ue_id),
        IGNORE E_RABSetupListCtxtSURes(erab_setup_list()),
    )?;

    println!("=== 1. InitialContextSetupResponse ===");
    print_and_encode(&pdu)?;

    // ── 2. UEContextReleaseRequest with Cause ──────────────────────────────

    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,

        REJECT MME_UE_S1AP_ID(mme_ue_id),
        REJECT eNB_UE_S1AP_ID(enb_ue_id),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;

    println!("\n=== 2. UEContextReleaseRequest ===");
    print_and_encode(&pdu)?;

    // ── 3. Complex: InitialContextSetupRequest with EPS bearer + security ──
    //
    // S1AP carries the optional EPS NAS PDU inside an E-RAB item rather than as
    // a top-level InitialContextSetupRequest IE. This demonstrates that nested
    // open container, fixed-size security key, GUMMEI, and LTE capabilities.

    let network = plmn("208", "93")?;
    let erabs = erab_to_be_setup_list(vec![0x07, 0x41, 0x00]);
    let bit_rate =
        UEAggregateMaximumBitrate::new(BitRate(1_000_000_000), BitRate(500_000_000), None);

    let pdu = build_s1ap!(InitiatingMessage, InitialContextSetup,
        REJECT, InitialContextSetupRequest,

        // The UE, on each side of the interface
        REJECT MME_UE_S1AP_ID(mme_ue_id),
        REJECT eNB_UE_S1AP_ID(enb_ue_id),

        // Its bit rate and its bearers
        REJECT UEAggregateMaximumBitrate(bit_rate),
        REJECT E_RABToBeSetupListCtxtSUReq(erabs),

        // Its security capabilities and its key
        REJECT UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
        REJECT SecurityKey([0; 32]),

        // Its MME
        IGNORE GUMMEI_ID(gummei(network, 1, 1)),
    )?;

    println!("\n=== 3. InitialContextSetupRequest (complex) ===");
    print_and_encode(&pdu)?;

    // ── 4. Handover: procedure name differs from message name ──────────────
    //
    // HandoverPreparation procedure → HandoverRequired message. The target eNB
    // identity is mandatory in S1AP, and the transparent container uses the
    // ASN.1-derived underscore spelling accepted by the macro.

    let target_enb = global_enb_id(network, 0x12345)?;
    let target_area = tai(network, [0x00, 0x01]);
    let target = TargetID::targeteNB_ID(TargeteNBID::new(target_enb, target_area, None));
    let cause = Cause::radioNetwork(CauseRadioNetwork::handover_desirable_for_radio_reason);

    let pdu = build_s1ap!(InitiatingMessage, HandoverPreparation,
        REJECT, HandoverRequired,

        REJECT MME_UE_S1AP_ID(mme_ue_id),
        REJECT eNB_UE_S1AP_ID(enb_ue_id),

        // The kind of handover, its cause and its target
        REJECT HandoverType(HandoverType::intralte),
        IGNORE Cause(cause),
        REJECT TargetID(target),

        // The container that goes to the target
        REJECT Source_ToTarget_TransparentContainer(vec![0x00, 0x01, 0x02]),
    )?;

    println!("\n=== 4. HandoverRequired (procedure=HandoverPreparation) ===");
    print_and_encode(&pdu)?;

    // ── 5. SuccessfulOutcome + UnsuccessfulOutcome ─────────────────────────

    let pdu = build_s1ap!(SuccessfulOutcome, HandoverResourceAllocation,
        REJECT, HandoverRequestAcknowledge,

        IGNORE MME_UE_S1AP_ID(mme_ue_id),
        IGNORE eNB_UE_S1AP_ID(enb_ue_id),

        IGNORE E_RABAdmittedList(erab_admitted_list()),
        REJECT Target_ToSource_TransparentContainer(vec![0xAA, 0xBB]),
    )?;

    println!("\n=== 5. HandoverRequestAcknowledge (SuccessfulOutcome) ===");
    print_and_encode(&pdu)?;

    let pdu = build_s1ap!(UnsuccessfulOutcome, S1Setup,
        REJECT, S1SetupFailure,

        IGNORE Cause(Cause::misc(CauseMisc::unknown_PLMN)),
    )?;

    println!("\n=== 5. S1SetupFailure (UnsuccessfulOutcome) ===");
    print_and_encode(&pdu)?;

    // ── 6. build_s1ap_ie! for conditional IE construction ──────────────────

    let cause_ie = build_s1ap_ie!(UEContextReleaseRequest, IGNORE
        Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity))
    )?;

    println!("\n=== 6. Single IE (via build_s1ap_ie!) ===");
    println!(
        "IE ID: {}, Criticality: {:?}",
        cause_ie.id.0, cause_ie.criticality
    );

    // ── Equivalent hand-written code (for comparison) ──────────────────────
    // Without macros, the same InitialContextSetupResponse (example 1) is:
    //
    //   let response = InitialContextSetupResponse::new(
    //       ProtocolIEContainer(vec![
    //           ProtocolIEField::new(
    //               ProtocolIEID(0),
    //               Criticality::ignore,
    //               encode_open_type(&MMEUES1APID(1))?,
    //           ),
    //           ProtocolIEField::new(
    //               ProtocolIEID(8),
    //               Criticality::ignore,
    //               encode_open_type(&ENBUES1APID(0))?,
    //           ),
    //           ProtocolIEField::new(
    //               ProtocolIEID(51),
    //               Criticality::ignore,
    //               encode_open_type(&erab_setup_list())?,
    //           ),
    //       ]),
    //   );
    //   let pdu = S1AP_PDU::successfulOutcome(SuccessfulOutcome::new(
    //       ProcedureCode(9),
    //       Criticality::reject,
    //       encode_open_type(&response)?,
    //   ));
    Ok(())
}

fn erab_setup_list() -> ERABSetupListCtxtSURes<'static> {
    let item = ERABSetupItemCtxtSURes::new(
        ERABID(5),
        TransportLayerAddress([10, 0, 0, 2].into()),
        GTPTEID::from([0, 0, 0, 2]),
        None,
    );

    // An item of the list is an IE itself, alone in its container.
    ERABSetupListCtxtSURes(vec![ProtocolIEField::new(
        ID_E_RABSETUP_ITEM_CTXT_SURES.0,
        Criticality::ignore,
        encode_open_type(&item).expect("encode E-RAB setup item"),
    )])
}

fn erab_admitted_list() -> ERABAdmittedList<'static> {
    let item = ERABAdmittedItem::new(
        ERABID(5),
        TransportLayerAddress([10, 0, 0, 3].into()),
        GTPTEID::from([0, 0, 0, 3]),
        None,
        None,
        None,
        None,
        None,
    );

    ERABAdmittedList(vec![ProtocolIEField::new(
        ID_E_RABADMITTED_ITEM.0,
        Criticality::ignore,
        encode_open_type(&item).expect("encode admitted E-RAB item"),
    )])
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

    ERABToBeSetupListCtxtSUReq(vec![ProtocolIEField::new(
        ID_E_RABTO_BE_SETUP_ITEM_CTXT_SUREQ.0,
        Criticality::reject,
        encode_open_type(&item).expect("encode E-RAB item"),
    )])
}

fn print_and_encode(pdu: &S1AP_PDU) -> Result<(), Error> {
    // Display example: "InitiatingMessage S1Setup (code=17)".
    println!("{pdu}");
    println!(
        "  procedure: {}  direction: {}  code: {}",
        pdu.procedure_name(),
        pdu.direction(),
        pdu.procedure_code()
    );

    let bytes = pdu.encode()?;
    println!("  APER ({} bytes): {}", bytes.len(), hex::encode(&bytes));
    Ok(())
}
