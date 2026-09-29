//! Independent APER vectors for every procedure outcome in 3GPP TS 36.413 v19.2.0.
use oxirush_s1ap::s1ap::*;
use rasn::types::Any;
use std::collections::BTreeSet;

fn value(pdu: &S1AP_PDU) -> &Any {
    match pdu {
        S1AP_PDU::initiatingMessage(message) => &message.value,
        S1AP_PDU::successfulOutcome(message) => &message.value,
        S1AP_PDU::unsuccessfulOutcome(message) => &message.value,
        _ => panic!("unexpected S1AP_PDU alternative"),
    }
}

fn check_typed<T: rasn::Decode + rasn::Encode>(pdu: &S1AP_PDU, name: &str) {
    let raw = value(pdu);
    let decoded: T = pdu
        .decode_value()
        .unwrap_or_else(|error| panic!("{name}: {error}"));
    assert_eq!(encode_open_type(&decoded).unwrap(), *raw, "{name}");
    for cut in 0..raw.as_bytes().len() {
        let partial = Any::new(raw.as_bytes()[..cut].to_vec());
        assert!(
            decode_open_type::<T>(&partial).is_err(),
            "{name}: prefix {cut}"
        );
    }
    for octet in [0, 0xff] {
        let mut bytes = raw.as_bytes().to_vec();
        bytes.push(octet);
        assert!(
            decode_open_type::<T>(&Any::new(bytes.clone())).is_err(),
            "{name}: trailing octet"
        );
        let mut extra = pdu.clone();
        match &mut extra {
            S1AP_PDU::initiatingMessage(message) => message.value = Any::new(bytes),
            S1AP_PDU::successfulOutcome(message) => message.value = Any::new(bytes),
            S1AP_PDU::unsuccessfulOutcome(message) => message.value = Any::new(bytes),
            _ => panic!("unexpected S1AP_PDU alternative"),
        }
        assert!(
            extra.decode_value::<T>().is_err(),
            "{name}: convenience API trailing octet"
        );
    }
}

fn check_message(name: &str, pdu: &S1AP_PDU) {
    match name {
        "HandoverRequired" => check_typed::<HandoverRequired>(pdu, name),
        "HandoverCommand" => check_typed::<HandoverCommand>(pdu, name),
        "HandoverPreparationFailure" => check_typed::<HandoverPreparationFailure>(pdu, name),
        "HandoverRequest" => check_typed::<HandoverRequest>(pdu, name),
        "HandoverRequestAcknowledge" => check_typed::<HandoverRequestAcknowledge>(pdu, name),
        "HandoverFailure" => check_typed::<HandoverFailure>(pdu, name),
        "PathSwitchRequest" => check_typed::<PathSwitchRequest>(pdu, name),
        "PathSwitchRequestAcknowledge" => check_typed::<PathSwitchRequestAcknowledge>(pdu, name),
        "PathSwitchRequestFailure" => check_typed::<PathSwitchRequestFailure>(pdu, name),
        "E-RABSetupRequest" => check_typed::<ERABSetupRequest>(pdu, name),
        "E-RABSetupResponse" => check_typed::<ERABSetupResponse>(pdu, name),
        "E-RABModifyRequest" => check_typed::<ERABModifyRequest>(pdu, name),
        "E-RABModifyResponse" => check_typed::<ERABModifyResponse>(pdu, name),
        "E-RABReleaseCommand" => check_typed::<ERABReleaseCommand>(pdu, name),
        "E-RABReleaseResponse" => check_typed::<ERABReleaseResponse>(pdu, name),
        "InitialContextSetupRequest" => check_typed::<InitialContextSetupRequest>(pdu, name),
        "InitialContextSetupResponse" => check_typed::<InitialContextSetupResponse>(pdu, name),
        "InitialContextSetupFailure" => check_typed::<InitialContextSetupFailure>(pdu, name),
        "HandoverCancel" => check_typed::<HandoverCancel>(pdu, name),
        "HandoverCancelAcknowledge" => check_typed::<HandoverCancelAcknowledge>(pdu, name),
        "KillRequest" => check_typed::<KillRequest>(pdu, name),
        "KillResponse" => check_typed::<KillResponse>(pdu, name),
        "Reset" => check_typed::<Reset>(pdu, name),
        "ResetAcknowledge" => check_typed::<ResetAcknowledge>(pdu, name),
        "S1SetupRequest" => check_typed::<S1SetupRequest>(pdu, name),
        "S1SetupResponse" => check_typed::<S1SetupResponse>(pdu, name),
        "S1SetupFailure" => check_typed::<S1SetupFailure>(pdu, name),
        "UEContextModificationRequest" => check_typed::<UEContextModificationRequest>(pdu, name),
        "UEContextModificationResponse" => check_typed::<UEContextModificationResponse>(pdu, name),
        "UEContextModificationFailure" => check_typed::<UEContextModificationFailure>(pdu, name),
        "UEContextReleaseCommand" => check_typed::<UEContextReleaseCommand>(pdu, name),
        "UEContextReleaseComplete" => check_typed::<UEContextReleaseComplete>(pdu, name),
        "ENBConfigurationUpdate" => check_typed::<ENBConfigurationUpdate>(pdu, name),
        "ENBConfigurationUpdateAcknowledge" => {
            check_typed::<ENBConfigurationUpdateAcknowledge>(pdu, name)
        }
        "ENBConfigurationUpdateFailure" => check_typed::<ENBConfigurationUpdateFailure>(pdu, name),
        "MMEConfigurationUpdate" => check_typed::<MMEConfigurationUpdate>(pdu, name),
        "MMEConfigurationUpdateAcknowledge" => {
            check_typed::<MMEConfigurationUpdateAcknowledge>(pdu, name)
        }
        "MMEConfigurationUpdateFailure" => check_typed::<MMEConfigurationUpdateFailure>(pdu, name),
        "WriteReplaceWarningRequest" => check_typed::<WriteReplaceWarningRequest>(pdu, name),
        "WriteReplaceWarningResponse" => check_typed::<WriteReplaceWarningResponse>(pdu, name),
        "HandoverNotify" => check_typed::<HandoverNotify>(pdu, name),
        "E-RABReleaseIndication" => check_typed::<ERABReleaseIndication>(pdu, name),
        "Paging" => check_typed::<Paging>(pdu, name),
        "DownlinkNASTransport" => check_typed::<DownlinkNASTransport>(pdu, name),
        "InitialUEMessage" => check_typed::<InitialUEMessage>(pdu, name),
        "UplinkNASTransport" => check_typed::<UplinkNASTransport>(pdu, name),
        "ErrorIndication" => check_typed::<ErrorIndication>(pdu, name),
        "NASNonDeliveryIndication" => check_typed::<NASNonDeliveryIndication>(pdu, name),
        "UEContextReleaseRequest" => check_typed::<UEContextReleaseRequest>(pdu, name),
        "DownlinkS1cdma2000tunnelling" => check_typed::<DownlinkS1cdma2000tunnelling>(pdu, name),
        "UplinkS1cdma2000tunnelling" => check_typed::<UplinkS1cdma2000tunnelling>(pdu, name),
        "UECapabilityInfoIndication" => check_typed::<UECapabilityInfoIndication>(pdu, name),
        "ENBStatusTransfer" => check_typed::<ENBStatusTransfer>(pdu, name),
        "MMEStatusTransfer" => check_typed::<MMEStatusTransfer>(pdu, name),
        "DeactivateTrace" => check_typed::<DeactivateTrace>(pdu, name),
        "TraceStart" => check_typed::<TraceStart>(pdu, name),
        "TraceFailureIndication" => check_typed::<TraceFailureIndication>(pdu, name),
        "CellTrafficTrace" => check_typed::<CellTrafficTrace>(pdu, name),
        "LocationReportingControl" => check_typed::<LocationReportingControl>(pdu, name),
        "LocationReportingFailureIndication" => {
            check_typed::<LocationReportingFailureIndication>(pdu, name)
        }
        "LocationReport" => check_typed::<LocationReport>(pdu, name),
        "OverloadStart" => check_typed::<OverloadStart>(pdu, name),
        "OverloadStop" => check_typed::<OverloadStop>(pdu, name),
        "ENBDirectInformationTransfer" => check_typed::<ENBDirectInformationTransfer>(pdu, name),
        "MMEDirectInformationTransfer" => check_typed::<MMEDirectInformationTransfer>(pdu, name),
        "ENBConfigurationTransfer" => check_typed::<ENBConfigurationTransfer>(pdu, name),
        "MMEConfigurationTransfer" => check_typed::<MMEConfigurationTransfer>(pdu, name),
        "PrivateMessage" => check_typed::<PrivateMessage>(pdu, name),
        "UERadioCapabilityMatchRequest" => check_typed::<UERadioCapabilityMatchRequest>(pdu, name),
        "UERadioCapabilityMatchResponse" => {
            check_typed::<UERadioCapabilityMatchResponse>(pdu, name)
        }
        "E-RABModificationIndication" => check_typed::<ERABModificationIndication>(pdu, name),
        "E-RABModificationConfirm" => check_typed::<ERABModificationConfirm>(pdu, name),
        "UEContextModificationIndication" => {
            check_typed::<UEContextModificationIndication>(pdu, name)
        }
        "UEContextModificationConfirm" => check_typed::<UEContextModificationConfirm>(pdu, name),
        "UEContextSuspendRequest" => check_typed::<UEContextSuspendRequest>(pdu, name),
        "UEContextSuspendResponse" => check_typed::<UEContextSuspendResponse>(pdu, name),
        "UEContextResumeRequest" => check_typed::<UEContextResumeRequest>(pdu, name),
        "UEContextResumeResponse" => check_typed::<UEContextResumeResponse>(pdu, name),
        "UEContextResumeFailure" => check_typed::<UEContextResumeFailure>(pdu, name),
        "UERadioCapabilityIDMappingRequest" => {
            check_typed::<UERadioCapabilityIDMappingRequest>(pdu, name)
        }
        "UERadioCapabilityIDMappingResponse" => {
            check_typed::<UERadioCapabilityIDMappingResponse>(pdu, name)
        }
        "S1RemovalRequest" => check_typed::<S1RemovalRequest>(pdu, name),
        "S1RemovalResponse" => check_typed::<S1RemovalResponse>(pdu, name),
        "S1RemovalFailure" => check_typed::<S1RemovalFailure>(pdu, name),
        "DownlinkUEAssociatedLPPaTransport" => {
            check_typed::<DownlinkUEAssociatedLPPaTransport>(pdu, name)
        }
        "UplinkUEAssociatedLPPaTransport" => {
            check_typed::<UplinkUEAssociatedLPPaTransport>(pdu, name)
        }
        "DownlinkNonUEAssociatedLPPaTransport" => {
            check_typed::<DownlinkNonUEAssociatedLPPaTransport>(pdu, name)
        }
        "UplinkNonUEAssociatedLPPaTransport" => {
            check_typed::<UplinkNonUEAssociatedLPPaTransport>(pdu, name)
        }
        "PWSRestartIndication" => check_typed::<PWSRestartIndication>(pdu, name),
        "RerouteNASRequest" => check_typed::<RerouteNASRequest>(pdu, name),
        "PWSFailureIndication" => check_typed::<PWSFailureIndication>(pdu, name),
        "ConnectionEstablishmentIndication" => {
            check_typed::<ConnectionEstablishmentIndication>(pdu, name)
        }
        "NASDeliveryIndication" => check_typed::<NASDeliveryIndication>(pdu, name),
        "RetrieveUEInformation" => check_typed::<RetrieveUEInformation>(pdu, name),
        "UEInformationTransfer" => check_typed::<UEInformationTransfer>(pdu, name),
        "ENBCPRelocationIndication" => check_typed::<ENBCPRelocationIndication>(pdu, name),
        "MMECPRelocationIndication" => check_typed::<MMECPRelocationIndication>(pdu, name),
        "SecondaryRATDataUsageReport" => check_typed::<SecondaryRATDataUsageReport>(pdu, name),
        "HandoverSuccess" => check_typed::<HandoverSuccess>(pdu, name),
        "ENBEarlyStatusTransfer" => check_typed::<ENBEarlyStatusTransfer>(pdu, name),
        "MMEEarlyStatusTransfer" => check_typed::<MMEEarlyStatusTransfer>(pdu, name),
        _ => panic!("missing message type: {name}"),
    }
}

#[test]
fn all_procedure_outcomes_round_trip_and_reject_incomplete_encodings() {
    let mut names = BTreeSet::new();
    let mut choices = BTreeSet::new();
    for line in include_str!("fixtures/messages.tsv").lines() {
        if line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4);
        let name = fields[0];
        assert!(names.insert(name), "duplicate message: {name}");
        let wire = hex::decode(fields[1]).unwrap();
        let pdu = S1AP_PDU::decode(&wire).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(pdu.encode().unwrap(), wire, "{name}");
        assert_eq!(pdu.procedure_code().to_string(), fields[2], "{name}");
        let choice = match &pdu {
            S1AP_PDU::initiatingMessage(_) => "0",
            S1AP_PDU::successfulOutcome(_) => "1",
            S1AP_PDU::unsuccessfulOutcome(_) => "2",
            _ => panic!("unexpected S1AP_PDU alternative"),
        };
        assert_eq!(choice, fields[3], "{name}");
        choices.insert(choice);
        check_message(name, &pdu);
        for cut in 0..wire.len() {
            assert!(
                S1AP_PDU::decode(&wire[..cut]).is_err(),
                "{name}: prefix {cut}"
            );
        }
        for octet in [0, 0xff] {
            let mut extra = wire.clone();
            extra.push(octet);
            assert!(S1AP_PDU::decode(&extra).is_err(), "{name}: trailing octet");
        }
    }
    assert_eq!(names.len(), 101);
    assert_eq!(choices.len(), 3);
}

#[test]
fn complete_open_types_require_a_zero_octet_even_for_null() {
    for bytes in [vec![], vec![0xff], vec![0, 0], vec![0, 0xff]] {
        assert!(decode_open_type::<()>(&Any::new(bytes)).is_err());
    }
    decode_open_type::<()>(&Any::new(vec![0])).unwrap();
    assert_eq!(encode_open_type(&()).unwrap().as_bytes(), [0]);
    // Fewer than eight unused bits retain the decoder's receive policy.
    assert!(decode_open_type::<bool>(&Any::new(vec![0xff])).unwrap());
}
