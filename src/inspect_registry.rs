// Auto-generated from ASN.1 by the generator of oxirush-aper; do not edit.
pub(crate) use crate::registry::{
    IE_NAMES, IE_TYPES, ITEM_CRITICALITIES, MESSAGE_IE_CRITICALITIES, MESSAGES,
    PROCEDURE_CRITICALITIES, ie, ie_contents,
};
use oxirush_aper::inspect::Form;
pub(crate) const PROTOCOL: &str = "S1AP";
pub(crate) const ROOT: &str = "s1ap";
oxirush_aper::__transfers! {}
oxirush_aper::__carried! {
    104 "SourceeNB-ToTargeteNB-TransparentContainer" crate::s1ap::SourceeNBToTargeteNBTransparentContainer;
    123 "TargeteNB-ToSourceeNB-TransparentContainer" crate::s1ap::TargeteNBToSourceeNBTransparentContainer;
}
pub(crate) fn type_form(name: &str) -> Option<Form> {
    Some(match name {
        // CI
        "CI" => Form::Number(16),
        // CellIdentity
        "CellIdentity" => Form::Number(28),
        // FiveGSTAC
        "FiveGSTAC" => Form::Number(24),
        // GTP-TEID
        "GTPTEID" => Form::Number(32),
        // IMSI
        "IMSI" => Form::Digits,
        // LAC
        "LAC" => Form::Number(16),
        // MME-Code
        "MMECode" => Form::Number(8),
        // MME-Group-ID
        "MMEGroupID" => Form::Number(16),
        // M-TMSI
        "MTMSI" => Form::Number(32),
        // NRCellIdentity
        "NRCellIdentity" => Form::Number(36),
        // PLMNidentity
        "PLMNidentity" => Form::Plmn,
        // Port-Number
        "PortNumber" => Form::Number(16),
        // RAC
        "RAC" => Form::Number(8),
        // TAC
        "TAC" => Form::Number(16),
        // TransportLayerAddress
        "TransportLayerAddress" => Form::Address,
        // UL-NAS-Count
        "ULNASCount" => Form::Number(5),
        _ => return None,
    })
}
/// The names of the values of each ENUMERATED, by the name of its type in the bindings, in the order of the names of the types.
pub(crate) const ENUMERATIONS: &[(&str, &[&str])] = &[
    (
        "AdditionalCSFallbackIndicator",
        &["no-restriction", "restriction"],
    ),
    (
        "AerialUEsubscriptionInformation",
        &["allowed", "not-allowed"],
    ),
    ("BearerType", &["non-IP"]),
    ("BluetoothMeasConfig", &["setup"]),
    ("BluetoothMeasurementConfigurationBtRssi", &["true"]),
    ("CEModeBRestricted", &["restricted", "not-restricted"]),
    ("CEModeBSupportIndicator", &["supported"]),
    ("CNDomain", &["ps", "cs"]),
    ("CNType", &["fiveGCForbidden", "epc-Forbiddden"]),
    (
        "CSFallbackIndicator",
        &["cs-fallback-required", "cs-fallback-high-priority"],
    ),
    ("CSGMembershipStatus", &["member", "not-member"]),
    (
        "CauseMisc",
        &[
            "control-processing-overload",
            "not-enough-user-plane-processing-resources",
            "hardware-failure",
            "om-intervention",
            "unspecified",
            "unknown-PLMN",
        ],
    ),
    (
        "CauseNas",
        &[
            "normal-release",
            "authentication-failure",
            "detach",
            "unspecified",
            "csg-subscription-expiry",
            "uE-not-in-PLMN-serving-area",
            "iab-not-authorized",
        ],
    ),
    (
        "CauseProtocol",
        &[
            "transfer-syntax-error",
            "abstract-syntax-error-reject",
            "abstract-syntax-error-ignore-and-notify",
            "message-not-compatible-with-receiver-state",
            "semantic-error",
            "abstract-syntax-error-falsely-constructed-message",
            "unspecified",
        ],
    ),
    (
        "CauseRadioNetwork",
        &[
            "unspecified",
            "tx2relocoverall-expiry",
            "successful-handover",
            "release-due-to-eutran-generated-reason",
            "handover-cancelled",
            "partial-handover",
            "ho-failure-in-target-EPC-eNB-or-target-system",
            "ho-target-not-allowed",
            "tS1relocoverall-expiry",
            "tS1relocprep-expiry",
            "cell-not-available",
            "unknown-targetID",
            "no-radio-resources-available-in-target-cell",
            "unknown-mme-ue-s1ap-id",
            "unknown-enb-ue-s1ap-id",
            "unknown-pair-ue-s1ap-id",
            "handover-desirable-for-radio-reason",
            "time-critical-handover",
            "resource-optimisation-handover",
            "reduce-load-in-serving-cell",
            "user-inactivity",
            "radio-connection-with-ue-lost",
            "load-balancing-tau-required",
            "cs-fallback-triggered",
            "ue-not-available-for-ps-service",
            "radio-resources-not-available",
            "failure-in-radio-interface-procedure",
            "invalid-qos-combination",
            "interrat-redirection",
            "interaction-with-other-procedure",
            "unknown-E-RAB-ID",
            "multiple-E-RAB-ID-instances",
            "encryption-and-or-integrity-protection-algorithms-not-supported",
            "s1-intra-system-handover-triggered",
            "s1-inter-system-handover-triggered",
            "x2-handover-triggered",
            "redirection-towards-1xRTT",
            "not-supported-QCI-value",
            "invalid-CSG-Id",
            "release-due-to-pre-emption",
            "n26-interface-not-available",
            "insufficient-ue-capabilities",
            "maximum-bearer-pre-emption-rate-exceeded",
            "up-integrity-protection-not-possible",
            "release-due-to-discontinuous-coverage",
        ],
    ),
    (
        "CauseTransport",
        &["transport-resource-unavailable", "unspecified"],
    ),
    ("Cdma2000HORequiredIndication", &["true"]),
    ("Cdma2000HOStatus", &["hOSuccess", "hOFailure"]),
    ("Cdma2000RATType", &["hRPD", "onexRTT"]),
    ("CellAccessMode", &["hybrid"]),
    ("CellSize", &["verysmall", "small", "medium", "large"]),
    ("CoarseUELocationRequested", &["true"]),
    ("ConcurrentWarningMessageIndicator", &["true"]),
    ("CoverageLevel", &["extendedcoverage"]),
    ("Criticality", &["reject", "ignore", "notify"]),
    ("DAPSRequestInfoDAPSIndicator", &["dAPS-HO-required"]),
    (
        "DAPSResponseInfoDapsresponseindicator",
        &["dAPS-HO-accepted", "dAPS-HO-not-accepted"],
    ),
    ("DLForwarding", &["dL-Forwarding-proposed"]),
    ("DLNASPDUDeliveryAckRequest", &["requested"]),
    (
        "DataForwardingNotPossible",
        &["data-Forwarding-not-Possible"],
    ),
    ("DirectForwardingPathAvailability", &["directPathAvailable"]),
    ("EDTSession", &["true"]),
    ("EmergencyIndicator", &["true"]),
    ("EndIndication", &["no-further-data", "further-data-exists"]),
    ("EnhancedCoverageRestricted", &["restricted"]),
    ("EthernetType", &["true"]),
    ("EventTriggerOutOfCoverage", &["true"]),
    (
        "EventType",
        &[
            "direct",
            "change-of-serve-cell",
            "stop-change-of-serve-cell",
        ],
    ),
    (
        "ExpectedHOInterval",
        &[
            "sec15",
            "sec30",
            "sec60",
            "sec90",
            "sec120",
            "sec180",
            "long-time",
        ],
    ),
    (
        "ForbiddenInterRATs",
        &[
            "all",
            "geran",
            "utran",
            "cdma2000",
            "geranandutran",
            "cdma2000andutran",
        ],
    ),
    ("GUMMEIType", &["native", "mapped", "mappedFrom5G"]),
    ("GWContextReleaseIndication", &["true"]),
    ("HandoverFlag", &["handoverPreparation"]),
    (
        "HandoverType",
        &[
            "intralte",
            "ltetoutran",
            "ltetogeran",
            "utrantolte",
            "gerantolte",
            "eps-to-5gs",
            "fivegs-to-eps",
        ],
    ),
    ("IABAuthorized", &["authorized", "not-authorized"]),
    ("IABNodeIndication", &["true"]),
    ("IABSupported", &["true"]),
    ("IMSvoiceEPSfallbackfrom5G", &["true"]),
    (
        "IntegrityProtectionIndication",
        &["required", "preferred", "not-needed"],
    ),
    ("IntegrityProtectionResult", &["performed", "not-performed"]),
    (
        "InterSystemMeasurementItemSubcarrierSpacingSSB",
        &[
            "kHz15", "kHz30", "kHz60", "kHz120", "kHz240", "kHz480", "kHz960",
        ],
    ),
    ("KillAllWarningMessages", &["true"]),
    ("LTEMIndication", &["lte-m"]),
    (
        "LinksToLog",
        &["uplink", "downlink", "both-uplink-and-downlink"],
    ),
    (
        "ListeningSubframePatternPatternPeriod",
        &["ms1280", "ms2560", "ms5120", "ms10240"],
    ),
    (
        "LoggingDuration",
        &["m10", "m20", "m40", "m60", "m90", "m120"],
    ),
    (
        "LoggingInterval",
        &[
            "ms1280", "ms2560", "ms5120", "ms10240", "ms20480", "ms30720", "ms40960", "ms61440",
        ],
    ),
    (
        "M1ReportingTrigger",
        &["periodic", "a2eventtriggered", "a2eventtriggered-periodic"],
    ),
    (
        "M3period",
        &[
            "ms100", "ms1000", "ms10000", "ms1024", "ms1280", "ms2048", "ms2560", "ms5120",
            "ms10240", "min1",
        ],
    ),
    (
        "M4ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"],
    ),
    (
        "M4period",
        &["ms1024", "ms2048", "ms5120", "ms10240", "min1"],
    ),
    (
        "M5ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"],
    ),
    (
        "M5period",
        &["ms1024", "ms2048", "ms5120", "ms10240", "min1"],
    ),
    (
        "M6ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"],
    ),
    (
        "M6delayThreshold",
        &[
            "ms30", "ms40", "ms50", "ms60", "ms70", "ms80", "ms90", "ms100", "ms150", "ms300",
            "ms500", "ms750",
        ],
    ),
    (
        "M6reportInterval",
        &["ms1024", "ms2048", "ms5120", "ms10240"],
    ),
    (
        "M7ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"],
    ),
    (
        "MDTActivation",
        &[
            "immediate-MDT-only",
            "immediate-MDT-and-Trace",
            "logged-MDT-only",
            "logged-MBSFN-MDT",
        ],
    ),
    ("MMERelaySupportIndicator", &["true"]),
    ("ManagementBasedMDTAllowed", &["allowed"]),
    (
        "MutingAvailabilityIndication",
        &["available", "unavailable"],
    ),
    (
        "MutingPatternInformationMutingPatternPeriod",
        &["ms0", "ms1280", "ms2560", "ms5120", "ms10240"],
    ),
    ("NBIoTDefaultPagingDRX", &["v128", "v256", "v512", "v1024"]),
    (
        "NBIoTPagingDRX",
        &["v32", "v64", "v128", "v256", "v512", "v1024"],
    ),
    (
        "NBIoTPagingEDRXCycle",
        &[
            "hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128",
            "hf256", "hf512", "hf1024",
        ],
    ),
    (
        "NBIoTPagingTimeWindow",
        &[
            "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13",
            "s14", "s15", "s16",
        ],
    ),
    ("NRrestrictionin5GS", &["nRrestrictedin5GS"]),
    (
        "NRrestrictioninEPSasSecondaryRAT",
        &["nRrestrictedinEPSasSecondaryRAT"],
    ),
    ("NextPagingAreaScope", &["same", "changed"]),
    ("NotifySourceeNB", &["notifySource"]),
    (
        "OverloadAction",
        &[
            "reject-non-emergency-mo-dt",
            "reject-rrc-cr-signalling",
            "permit-emergency-sessions-and-mobile-terminated-services-only",
            "permit-high-priority-sessions-and-mobile-terminated-services-only",
            "reject-delay-tolerant-access",
            "permit-high-priority-sessions-and-exception-reporting-and-mobile-terminated-services-only",
            "not-accept-mo-data-or-delay-tolerant-access-from-CP-CIoT",
        ],
    ),
    ("PSServiceNotAvailable", &["ps-service-not-available"]),
    ("PagingCause", &["voice"]),
    ("PagingDRX", &["v32", "v64", "v128", "v256"]),
    (
        "PagingEDRXCycle",
        &[
            "hfhalf", "hf1", "hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32",
            "hf64", "hf128", "hf256",
        ],
    ),
    (
        "PagingPriority",
        &[
            "priolevel1",
            "priolevel2",
            "priolevel3",
            "priolevel4",
            "priolevel5",
            "priolevel6",
            "priolevel7",
            "priolevel8",
        ],
    ),
    (
        "PagingProbabilityInformation",
        &[
            "p00", "p05", "p10", "p15", "p20", "p25", "p30", "p35", "p40", "p45", "p50", "p55",
            "p60", "p65", "p70", "p75", "p80", "p85", "p90", "p95", "p100",
        ],
    ),
    (
        "PagingTimeWindow",
        &[
            "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13",
            "s14", "s15", "s16",
        ],
    ),
    ("PedestrianUE", &["authorized", "not-authorized"]),
    ("PendingDataIndication", &["true"]),
    (
        "PreEmptionCapability",
        &["shall-not-trigger-pre-emption", "may-trigger-pre-emption"],
    ),
    (
        "PreEmptionVulnerability",
        &["not-pre-emptable", "pre-emptable"],
    ),
    ("Presence", &["optional", "conditional", "mandatory"]),
    ("PrivacyIndicator", &["immediate-MDT", "logged-MDT"]),
    (
        "ProSeDirectCommunication",
        &["authorized", "not-authorized"],
    ),
    ("ProSeDirectDiscovery", &["authorized", "not-authorized"]),
    (
        "ProSeUEtoNetworkRelaying",
        &["authorized", "not-authorized"],
    ),
    ("RACSIndication", &["true"]),
    (
        "RATType",
        &[
            "nbiot",
            "nbiot-leo",
            "nbiot-meo",
            "nbiot-geo",
            "nbiot-othersat",
            "eutran-leo",
            "eutran-meo",
            "eutran-geo",
            "eutran-othersat",
        ],
    ),
    (
        "RRCEstablishmentCause",
        &[
            "emergency",
            "highPriorityAccess",
            "mt-Access",
            "mo-Signalling",
            "mo-Data",
            "delay-TolerantAccess",
            "mo-VoiceCall",
            "mo-ExceptionData",
        ],
    ),
    (
        "Range",
        &[
            "m50", "m80", "m180", "m200", "m350", "m400", "m500", "m700", "m1000",
        ],
    ),
    ("RelayNodeIndicator", &["true"]),
    (
        "ReportAmountMDT",
        &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "rinfinity"],
    ),
    ("ReportArea", &["ecgi"]),
    (
        "ReportIntervalMDT",
        &[
            "ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "min1",
            "min6", "min12", "min30", "min60",
        ],
    ),
    ("RequestTypeAdditionalInfo", &["includePSCell"]),
    ("ResetAll", &["reset-all"]),
    (
        "SONInformationRequest",
        &[
            "x2TNL-Configuration-Info",
            "time-Synchronisation-Info",
            "activate-Muting",
            "deactivate-Muting",
        ],
    ),
    ("SRVCCHOIndication", &["pSandCS", "cSonly"]),
    ("SRVCCOperationNotPossible", &["notPossible"]),
    ("SRVCCOperationPossible", &["possible"]),
    ("SecondaryRATDataUsageRequest", &["requested"]),
    ("SecondaryRATType", &["nR", "unlicensed"]),
    ("SensorMeasConfig", &["setup"]),
    ("SensorNameConfigUncompensatedBarometricConfig", &["true"]),
    (
        "ServiceType",
        &["qMC-for-streaming-service", "qMC-for-MTSI-service"],
    ),
    (
        "SourceOfUEActivityBehaviourInformation",
        &["subscription-information", "statistics"],
    ),
    (
        "SubscriptionBasedUEDifferentiationInfoBatteryIndication",
        &[
            "battery-powered",
            "battery-powered-not-rechargeable-or-replaceable",
            "not-battery-powered",
        ],
    ),
    (
        "SubscriptionBasedUEDifferentiationInfoPeriodicCommunicationIndicator",
        &["periodically", "ondemand"],
    ),
    (
        "SubscriptionBasedUEDifferentiationInfoStationaryIndication",
        &["stationary", "mobile"],
    ),
    (
        "SubscriptionBasedUEDifferentiationInfoTrafficProfile",
        &["single-packet", "dual-packets", "multiple-packets"],
    ),
    ("SynchronisationStatus", &["synchronous", "asynchronous"]),
    ("TimeRefDistribution", &["true"]),
    (
        "TimeToTrigger",
        &[
            "ms0", "ms40", "ms64", "ms80", "ms100", "ms128", "ms160", "ms256", "ms320", "ms480",
            "ms512", "ms640", "ms1024", "ms1280", "ms2560", "ms5120",
        ],
    ),
    ("TimeToWait", &["v1s", "v2s", "v5s", "v10s", "v20s", "v60s"]),
    (
        "TraceDepth",
        &[
            "minimum",
            "medium",
            "maximum",
            "minimumWithoutVendorSpecificExtension",
            "mediumWithoutVendorSpecificExtension",
            "maximumWithoutVendorSpecificExtension",
        ],
    ),
    (
        "TriggeringMessage",
        &[
            "initiating-message",
            "successful-outcome",
            "unsuccessfull-outcome",
        ],
    ),
    ("TypeOfError", &["not-understood", "missing"]),
    ("UECapabilityInfoRequest", &["requested"]),
    ("UERetentionInformation", &["ues-retained"]),
    ("UEUserPlaneCIoTSupportIndicator", &["supported"]),
    ("UnlicensedSpectrumRestriction", &["unlicensed-restricted"]),
    ("VehicleUE", &["authorized", "not-authorized"]),
    (
        "VoiceSupportMatchIndicator",
        &["supported", "not-supported"],
    ),
    ("WLANMeasConfig", &["setup"]),
    ("WLANMeasurementConfigurationWlanRssi", &["true"]),
    ("WLANMeasurementConfigurationWlanRtt", &["true"]),
];
oxirush_aper::__types! {
    AdditionalGUTI sequence { "gUMMEI": GUMMEI, "m-TMSI": MTMSI, "iE-Extensions"? extensions };
    AllocationAndRetentionPriority sequence { "priorityLevel": _, "pre-emptionCapability": PreEmptionCapability, "pre-emptionVulnerability": PreEmptionVulnerability, "iE-Extensions"? extensions };
    AnonymousPrivateMessagePrivateIEs sequence { "id": PrivateIEID, "criticality": Criticality, "value": _ };
    AreaScopeOfMDT choice { "cellBased": CellBasedMDT, "tABased": TABasedMDT, "pLMNWide": null, "tAIBased": TAIBasedMDT };
    AreaScopeOfQMC choice { "cellBased": CellBasedQMC, "tABased": TABasedQMC, "tAIBased": TAIBasedQMC, "pLMNAreaBased": PLMNAreaBasedQMC };
    AssistanceDataForCECapableUEs sequence { "cellIdentifierAndCELevelForCECapableUEs": CellIdentifierAndCELevelForCECapableUEs, "iE-Extensions"? extensions };
    AssistanceDataForPaging sequence { "assistanceDataForRecommendedCells"? AssistanceDataForRecommendedCells, "assistanceDataForCECapableUEs"? AssistanceDataForCECapableUEs, "pagingAttemptInformation"? PagingAttemptInformation, "iE-Extensions"? extensions };
    AssistanceDataForRecommendedCells sequence { "recommendedCellsForPaging": RecommendedCellsForPaging, "iE-Extensions"? extensions };
    BPLMNs = [PLMNidentity];
    BearersSubjectToDLDiscardingItem sequence { "e-RAB-ID": _, "dL-Discarding": DLDiscarding, "iE-Extensions"? extensions };
    BearersSubjectToDLDiscardingList = [ie];
    BearersSubjectToEarlyStatusTransferItem sequence { "e-RAB-ID": _, "dLCOUNT-PDCP-SNlength": DLCOUNTPDCPSNlength, "iE-Extensions"? extensions };
    BearersSubjectToEarlyStatusTransferList = [ie];
    BearersSubjectToStatusTransferItem sequence { "e-RAB-ID": _, "uL-COUNTvalue": COUNTvalue, "dL-COUNTvalue": COUNTvalue, "receiveStatusofULPDCPSDUs"? _, "iE-Extensions"? extensions };
    BearersSubjectToStatusTransferList = [ie];
    BluetoothMeasConfigNameList = [_];
    BluetoothMeasurementConfiguration sequence { "bluetoothMeasConfig": BluetoothMeasConfig, "bluetoothMeasConfigNameList"? BluetoothMeasConfigNameList, "bt-rssi"? BluetoothMeasurementConfigurationBtRssi, "iE-Extensions"? extensions };
    BroadcastCancelledAreaList choice { "cellID-Cancelled": CellIDCancelled, "tAI-Cancelled": TAICancelled, "emergencyAreaID-Cancelled": EmergencyAreaIDCancelled };
    BroadcastCompletedAreaList choice { "cellID-Broadcast": CellIDBroadcast, "tAI-Broadcast": TAIBroadcast, "emergencyAreaID-Broadcast": EmergencyAreaIDBroadcast };
    CGI sequence { "pLMNidentity": PLMNidentity, "lAC": LAC, "cI": CI, "rAC"? RAC, "iE-Extensions"? extensions };
    CNTypeRestrictions = [CNTypeRestrictionsItem];
    CNTypeRestrictionsItem sequence { "pLMN-Identity": PLMNidentity, "cNType": CNType, "iE-Extensions"? extensions };
    COUNTValueExtended sequence { "pDCP-SNExtended": _, "hFNModified": _, "iE-Extensions"? extensions };
    COUNTvalue sequence { "pDCP-SN": _, "hFN": _, "iE-Extensions"? extensions };
    COUNTvaluePDCPSNlength18 sequence { "pDCP-SNlength18": _, "hFNforPDCP-SNlength18": _, "iE-Extensions"? extensions };
    CSGIdList = [CSGIdListItem];
    CSGIdListItem sequence { "cSG-Id": _, "iE-Extensions"? extensions };
    CSGMembershipInfo sequence { "cSGMembershipStatus": CSGMembershipStatus, "cSG-Id": _, "cellAccessMode"? CellAccessMode, "pLMNidentity"? PLMNidentity, "iE-Extensions"? extensions };
    CancelledCellinEAI = [CancelledCellinEAIItem];
    CancelledCellinEAIItem sequence { "eCGI": EUTRANCGI, "numberOfBroadcasts": _, "iE-Extensions"? extensions };
    CancelledCellinTAI = [CancelledCellinTAIItem];
    CancelledCellinTAIItem sequence { "eCGI": EUTRANCGI, "numberOfBroadcasts": _, "iE-Extensions"? extensions };
    Cause choice { "radioNetwork": CauseRadioNetwork, "transport": CauseTransport, "nas": CauseNas, "protocol": CauseProtocol, "misc": CauseMisc };
    Cdma2000OneXSRVCCInfo sequence { "cdma2000OneXMEID": _, "cdma2000OneXMSI": _, "cdma2000OneXPilot": _, "iE-Extensions"? extensions };
    CellBasedMDT sequence { "cellIdListforMDT": CellIdListforMDT, "iE-Extensions"? extensions };
    CellBasedQMC sequence { "cellIdListforQMC": CellIdListforQMC, "iE-Extensions"? extensions };
    CellIDBroadcast = [CellIDBroadcastItem];
    CellIDBroadcastItem sequence { "eCGI": EUTRANCGI, "iE-Extensions"? extensions };
    CellIDCancelled = [CellIDCancelledItem];
    CellIDCancelledItem sequence { "eCGI": EUTRANCGI, "numberOfBroadcasts": _, "iE-Extensions"? extensions };
    CellIdListforMDT = [EUTRANCGI];
    CellIdListforQMC = [EUTRANCGI];
    CellIdentifierAndCELevelForCECapableUEs sequence { "global-Cell-ID": EUTRANCGI, "cELevel": _, "iE-Extensions"? extensions };
    CellTrafficTrace sequence { "protocolIEs": ies };
    CellType sequence { "cell-Size": CellSize, "iE-Extensions"? extensions };
    CompletedCellinEAI = [CompletedCellinEAIItem];
    CompletedCellinEAIItem sequence { "eCGI": EUTRANCGI, "iE-Extensions"? extensions };
    CompletedCellinTAI = [CompletedCellinTAIItem];
    CompletedCellinTAIItem sequence { "eCGI": EUTRANCGI, "iE-Extensions"? extensions };
    ConnectedengNBItem sequence { "en-gNB-ID": EnGNBID, "supportedTAs": SupportedTAs, "iE-Extensions"? extensions };
    ConnectedengNBList = [ConnectedengNBItem];
    ConnectionEstablishmentIndication sequence { "protocolIEs": ies };
    ContextatSource sequence { "sourceNG-RAN-node-ID": GlobalRANNODEID, "rAN-UE-NGAP-ID": _, "iE-Extensions"? extensions };
    CriticalityDiagnostics sequence { "procedureCode"? _, "triggeringMessage"? TriggeringMessage, "procedureCriticality"? Criticality, "iEsCriticalityDiagnostics"? CriticalityDiagnosticsIEList, "iE-Extensions"? extensions };
    CriticalityDiagnosticsIEItem sequence { "iECriticality": Criticality, "iE-ID": _, "typeOfError": TypeOfError, "iE-Extensions"? extensions };
    CriticalityDiagnosticsIEList = [CriticalityDiagnosticsIEItem];
    DAPSRequestInfo sequence { "dAPSIndicator": DAPSRequestInfoDAPSIndicator, "iE-Extensions"? extensions };
    DAPSResponseInfo sequence { "dapsresponseindicator": DAPSResponseInfoDapsresponseindicator, "iE-Extensions"? extensions };
    DAPSResponseInfoItem sequence { "e-RAB-ID": _, "dAPSResponseInfo": DAPSResponseInfo, "iE-Extensions"? extensions };
    DAPSResponseInfoList = [ie];
    DLCOUNTPDCPSNlength choice { "dLCOUNTValuePDCP-SNlength12": COUNTvalue, "dLCOUNTValuePDCP-SNlength15": COUNTValueExtended, "dLCOUNTValuePDCP-SNlength18": COUNTvaluePDCPSNlength18 };
    DLCPSecurityInformation sequence { "dl-NAS-MAC": _, "iE-Extensions"? extensions };
    DLDiscarding choice { "discardDLCOUNTValuePDCP-SNlength12": COUNTvalue, "discardDLCOUNTValuePDCP-SNlength15": COUNTValueExtended, "discardDLCOUNTValuePDCP-SNlength18": COUNTvaluePDCPSNlength18 };
    DeactivateTrace sequence { "protocolIEs": ies };
    DownlinkNASTransport sequence { "protocolIEs": ies };
    DownlinkNonUEAssociatedLPPaTransport sequence { "protocolIEs": ies };
    DownlinkS1cdma2000tunnelling sequence { "protocolIEs": ies };
    DownlinkUEAssociatedLPPaTransport sequence { "protocolIEs": ies };
    ECGIList = [EUTRANCGI];
    ECGIListForRestart = [EUTRANCGI];
    ECGI_List = [EUTRANCGI];
    ENBCPRelocationIndication sequence { "protocolIEs": ies };
    ENBConfigurationTransfer sequence { "protocolIEs": ies };
    ENBConfigurationUpdate sequence { "protocolIEs": ies };
    ENBConfigurationUpdateAcknowledge sequence { "protocolIEs": ies };
    ENBConfigurationUpdateFailure sequence { "protocolIEs": ies };
    ENBDirectInformationTransfer sequence { "protocolIEs": ies };
    ENBEarlyStatusTransfer sequence { "protocolIEs": ies };
    ENBEarlyStatusTransferTransparentContainer sequence { "bearers-SubjectToEarlyStatusTransferList": BearersSubjectToEarlyStatusTransferList, "iE-Extensions"? extensions };
    ENBID choice { "macroENB-ID": _, "homeENB-ID": _, "short-macroENB-ID": _, "long-macroENB-ID": _ };
    ENBIndirectX2TransportLayerAddresses = [TransportLayerAddress];
    ENBStatusTransfer sequence { "protocolIEs": ies };
    ENBStatusTransferTransparentContainer sequence { "bearers-SubjectToStatusTransferList": BearersSubjectToStatusTransferList, "iE-Extensions"? extensions };
    ENBX2ExtTLA sequence { "iPsecTLA"? TransportLayerAddress, "gTPTLAa"? ENBX2GTPTLAs, "iE-Extensions"? extensions };
    ENBX2ExtTLAs = [ENBX2ExtTLA];
    ENBX2GTPTLAs = [TransportLayerAddress];
    ENBX2TLAs = [TransportLayerAddress];
    ENDCSONConfigurationTransfer sequence { "transfertype": ENDCSONTransferType, "sONInformation": SONInformation, "x2TNLConfigInfo"? X2TNLConfigurationInfo, "iE-Extensions"? extensions };
    ENDCSONTransferType choice { "request": ENDCTransferTypeRequest, "reply": ENDCTransferTypeReply };
    ENDCSONeNBIdentification sequence { "globaleNBID": GlobalENBID, "selectedTAI": TAI, "iE-Extensions"? extensions };
    ENDCSONengNBIdentification sequence { "globalengNBID": GlobalEnGNBID, "selectedTAI": TAI, "iE-Extensions"? extensions };
    ENDCTransferTypeReply sequence { "sourceengNB": ENDCSONengNBIdentification, "targeteNB": ENDCSONeNBIdentification, "iE-Extensions"? extensions };
    ENDCTransferTypeRequest sequence { "sourceeNB": ENDCSONeNBIdentification, "targetengNB": ENDCSONengNBIdentification, "targeteNB"? ENDCSONeNBIdentification, "associatedTAI"? TAI, "broadcast5GSTAI"? FiveGSTAI, "iE-Extensions"? extensions };
    EPLMNs = [PLMNidentity];
    ERABAdmittedItem sequence { "e-RAB-ID": _, "transportLayerAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "dL-transportLayerAddress"? TransportLayerAddress, "dL-gTP-TEID"? GTPTEID, "uL-TransportLayerAddress"? TransportLayerAddress, "uL-GTP-TEID"? GTPTEID, "iE-Extensions"? extensions };
    ERABAdmittedList = [ie];
    ERABDataForwardingItem sequence { "e-RAB-ID": _, "dL-transportLayerAddress"? TransportLayerAddress, "dL-gTP-TEID"? GTPTEID, "uL-TransportLayerAddress"? TransportLayerAddress, "uL-GTP-TEID"? GTPTEID, "iE-Extensions"? extensions };
    ERABFailedToResumeItemResumeReq sequence { "e-RAB-ID": _, "cause": Cause, "iE-Extensions"? extensions };
    ERABFailedToResumeItemResumeRes sequence { "e-RAB-ID": _, "cause": Cause, "iE-Extensions"? extensions };
    ERABFailedToResumeListResumeReq = [ie];
    ERABFailedToResumeListResumeRes = [ie];
    ERABFailedToSetupItemHOReqAck sequence { "e-RAB-ID": _, "cause": Cause, "iE-Extensions"? extensions };
    ERABFailedtoSetupListHOReqAck = [ie];
    ERABInformationList = [ie];
    ERABInformationListItem sequence { "e-RAB-ID": _, "dL-Forwarding"? DLForwarding, "iE-Extensions"? extensions };
    ERABItem sequence { "e-RAB-ID": _, "cause": Cause, "iE-Extensions"? extensions };
    ERABLevelQoSParameters sequence { "qCI": _, "allocationRetentionPriority": AllocationAndRetentionPriority, "gbrQosInformation"? GBRQosInformation, "iE-Extensions"? extensions };
    ERABList = [ie];
    ERABModificationConfirm sequence { "protocolIEs": ies };
    ERABModificationIndication sequence { "protocolIEs": ies };
    ERABModifyItemBearerModConf sequence { "e-RAB-ID": _, "iE-Extensions"? extensions };
    ERABModifyItemBearerModRes sequence { "e-RAB-ID": _, "iE-Extensions"? extensions };
    ERABModifyListBearerModConf = [ie];
    ERABModifyListBearerModRes = [ie];
    ERABModifyRequest sequence { "protocolIEs": ies };
    ERABModifyResponse sequence { "protocolIEs": ies };
    ERABNotToBeModifiedItemBearerModInd sequence { "e-RAB-ID": _, "transportLayerAddress": TransportLayerAddress, "dL-GTP-TEID": GTPTEID, "iE-Extensions"? extensions };
    ERABNotToBeModifiedListBearerModInd = [ie];
    ERABReleaseCommand sequence { "protocolIEs": ies };
    ERABReleaseIndication sequence { "protocolIEs": ies };
    ERABReleaseItemBearerRelComp sequence { "e-RAB-ID": _, "iE-Extensions"? extensions };
    ERABReleaseListBearerRelComp = [ie];
    ERABReleaseResponse sequence { "protocolIEs": ies };
    ERABSecurityResultItem sequence { "e-RAB-ID": _, "securityResult": SecurityResult, "iE-Extensions"? extensions };
    ERABSecurityResultList = [ie];
    ERABSetupItemBearerSURes sequence { "e-RAB-ID": _, "transportLayerAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "iE-Extensions"? extensions };
    ERABSetupItemCtxtSURes sequence { "e-RAB-ID": _, "transportLayerAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "iE-Extensions"? extensions };
    ERABSetupListBearerSURes = [ie];
    ERABSetupListCtxtSURes = [ie];
    ERABSetupRequest sequence { "protocolIEs": ies };
    ERABSetupResponse sequence { "protocolIEs": ies };
    ERABSubjecttoDataForwardingList = [ie];
    ERABToBeModifiedItemBearerModInd sequence { "e-RAB-ID": _, "transportLayerAddress": TransportLayerAddress, "dL-GTP-TEID": GTPTEID, "iE-Extensions"? extensions };
    ERABToBeModifiedItemBearerModReq sequence { "e-RAB-ID": _, "e-RABLevelQoSParameters": ERABLevelQoSParameters, "nAS-PDU": _, "iE-Extensions"? extensions };
    ERABToBeModifiedListBearerModInd = [ie];
    ERABToBeModifiedListBearerModReq = [ie];
    ERABToBeSetupItemBearerSUReq sequence { "e-RAB-ID": _, "e-RABlevelQoSParameters": ERABLevelQoSParameters, "transportLayerAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "nAS-PDU": _, "iE-Extensions"? extensions };
    ERABToBeSetupItemCtxtSUReq sequence { "e-RAB-ID": _, "e-RABlevelQoSParameters": ERABLevelQoSParameters, "transportLayerAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "nAS-PDU"? _, "iE-Extensions"? extensions };
    ERABToBeSetupItemHOReq sequence { "e-RAB-ID": _, "transportLayerAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "e-RABlevelQosParameters": ERABLevelQoSParameters, "iE-Extensions"? extensions };
    ERABToBeSetupListBearerSUReq = [ie];
    ERABToBeSetupListCtxtSUReq = [ie];
    ERABToBeSetupListHOReq = [ie];
    ERABToBeSwitchedDLItem sequence { "e-RAB-ID": _, "transportLayerAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "iE-Extensions"? extensions };
    ERABToBeSwitchedDLList = [ie];
    ERABToBeSwitchedULItem sequence { "e-RAB-ID": _, "transportLayerAddress": TransportLayerAddress, "gTP-TEID": GTPTEID, "iE-Extensions"? extensions };
    ERABToBeSwitchedULList = [ie];
    ERABToBeUpdatedItem sequence { "e-RAB-ID": _, "securityIndication"? SecurityIndication, "iE-Extensions"? extensions };
    ERABToBeUpdatedList = [ie];
    ERABUsageReportItem sequence { "startTimestamp": _, "endTimestamp": _, "usageCountUL": _, "usageCountDL": _, "iE-Extensions"? extensions };
    ERABUsageReportList = [ie];
    EUTRANCGI sequence { "pLMNidentity": PLMNidentity, "cell-ID": CellIdentity, "iE-Extensions"? extensions };
    EmergencyAreaIDBroadcast = [EmergencyAreaIDBroadcastItem];
    EmergencyAreaIDBroadcastItem sequence { "emergencyAreaID": _, "completedCellinEAI": CompletedCellinEAI, "iE-Extensions"? extensions };
    EmergencyAreaIDCancelled = [EmergencyAreaIDCancelledItem];
    EmergencyAreaIDCancelledItem sequence { "emergencyAreaID": _, "cancelledCellinEAI": CancelledCellinEAI, "iE-Extensions"? extensions };
    EmergencyAreaIDList = [_];
    EmergencyAreaIDListForRestart = [_];
    EnGNBID = bits;
    EncryptionAlgorithms = bits;
    ErrorIndication sequence { "protocolIEs": ies };
    EventL1LoggedMDTConfig sequence { "l1Threshold": MeasurementThresholdL1LoggedMDT, "hysteresis": _, "timeToTrigger": TimeToTrigger, "iE-Extensions"? extensions };
    EventTrigger choice { "outOfCoverage": EventTriggerOutOfCoverage, "eventL1LoggedMDTConfig": EventL1LoggedMDTConfig, "choice-Extensions": ie };
    ExpectedUEActivityBehaviour sequence { "expectedActivityPeriod"? _, "expectedIdlePeriod"? _, "sourceofUEActivityBehaviourInformation"? SourceOfUEActivityBehaviourInformation, "iE-Extensions"? extensions };
    ExpectedUEBehaviour sequence { "expectedActivity"? ExpectedUEActivityBehaviour, "expectedHOInterval"? ExpectedHOInterval, "iE-Extensions"? extensions };
    FiveGSTAI sequence { "pLMNidentity": PLMNidentity, "fiveGSTAC": FiveGSTAC, "iE-Extensions"? extensions };
    ForbiddenLACs = [LAC];
    ForbiddenLAs = [ForbiddenLAsItem];
    ForbiddenLAsItem sequence { "pLMN-Identity": PLMNidentity, "forbiddenLACs": ForbiddenLACs, "iE-Extensions"? extensions };
    ForbiddenTACs = [TAC];
    ForbiddenTAs = [ForbiddenTAsItem];
    ForbiddenTAsItem sequence { "pLMN-Identity": PLMNidentity, "forbiddenTACs": ForbiddenTACs, "iE-Extensions"? extensions };
    GBRQosInformation sequence { "e-RAB-MaximumBitrateDL": _, "e-RAB-MaximumBitrateUL": _, "e-RAB-GuaranteedBitrateDL": _, "e-RAB-GuaranteedBitrateUL": _, "iE-Extensions"? extensions };
    GERANCellID sequence { "lAI": LAI, "rAC": RAC, "cI": CI, "iE-Extensions"? extensions };
    GNB sequence { "global-gNB-ID": GlobalGNBID, "iE-Extensions"? extensions };
    GNBID = bits;
    GNBIdentity choice { "gNB-ID": GNBID };
    GUMMEI sequence { "pLMN-Identity": PLMNidentity, "mME-Group-ID": MMEGroupID, "mME-Code": MMECode, "iE-Extensions"? extensions };
    GUMMEIList = [GUMMEI];
    GlobalENBID sequence { "pLMNidentity": PLMNidentity, "eNB-ID": ENBID, "iE-Extensions"? extensions };
    GlobalEnGNBID sequence { "pLMNidentity": PLMNidentity, "en-gNB-ID": EnGNBID, "iE-Extensions"? extensions };
    GlobalGNBID sequence { "pLMN-Identity": PLMNidentity, "gNB-ID": GNBIdentity, "iE-Extensions"? extensions };
    GlobalRANNODEID choice { "gNB": GNB, "ng-eNB": NGENB };
    HandoverCancel sequence { "protocolIEs": ies };
    HandoverCancelAcknowledge sequence { "protocolIEs": ies };
    HandoverCommand sequence { "protocolIEs": ies };
    HandoverFailure sequence { "protocolIEs": ies };
    HandoverNotify sequence { "protocolIEs": ies };
    HandoverPreparationFailure sequence { "protocolIEs": ies };
    HandoverRequest sequence { "protocolIEs": ies };
    HandoverRequestAcknowledge sequence { "protocolIEs": ies };
    HandoverRequired sequence { "protocolIEs": ies };
    HandoverRestrictionList sequence { "servingPLMN": PLMNidentity, "equivalentPLMNs"? EPLMNs, "forbiddenTAs"? ForbiddenTAs, "forbiddenLAs"? ForbiddenLAs, "forbiddenInterRATs"? ForbiddenInterRATs, "iE-Extensions"? extensions };
    HandoverSuccess sequence { "protocolIEs": ies };
    ImmediateMDT sequence { "measurementsToActivate": _, "m1reportingTrigger": M1ReportingTrigger, "m1thresholdeventA2"? M1ThresholdEventA2, "m1periodicReporting"? M1PeriodicReporting, "iE-Extensions"? extensions };
    InformationOnRecommendedCellsAndENBsForPaging sequence { "recommendedCellsForPaging": RecommendedCellsForPaging, "recommendENBsForPaging": RecommendedENBsForPaging, "iE-Extensions"? extensions };
    InitialContextSetupFailure sequence { "protocolIEs": ies };
    InitialContextSetupRequest sequence { "protocolIEs": ies };
    InitialContextSetupResponse sequence { "protocolIEs": ies };
    InitialUEMessage sequence { "protocolIEs": ies };
    InitiatingMessage sequence { "procedureCode": _, "criticality": Criticality, "value": _ };
    IntegrityProtectionAlgorithms = bits;
    InterSystemInformationTransferType choice { "rIMTransfer": RIMTransfer };
    InterSystemMeasurementItem sequence { "freqBandIndicatorNR": _, "sSBfrequencies": _, "subcarrierSpacingSSB": InterSystemMeasurementItemSubcarrierSpacingSSB, "maxRSIndexCellQual"? _, "sMTC"? _, "threshRS-Index-r15"? _, "sSBToMeasure"? _, "sSRSSIMeasurement"? _, "quantityConfigNR-R15"? _, "excludedCellsToAddModList"? _, "iE-Extensions"? extensions };
    InterSystemMeasurementList = [InterSystemMeasurementItem];
    InterSystemMeasurementParameters sequence { "measurementDuration": _, "interSystemMeasurementList"? InterSystemMeasurementList, "iE-Extensions"? extensions };
    IntersystemMeasurementConfiguration sequence { "rSRP"? _, "rSRQ"? _, "sINR"? _, "interSystemMeasurementParameters": InterSystemMeasurementParameters, "iE-Extensions"? extensions };
    KillRequest sequence { "protocolIEs": ies };
    KillResponse sequence { "protocolIEs": ies };
    LAI sequence { "pLMNidentity": PLMNidentity, "lAC": LAC, "iE-Extensions"? extensions };
    LTENTNTAIInformation sequence { "servingPLMN": PLMNidentity, "tACList-In-LTE-NTN": TACListInLTENTN, "uE-Location-Derived-TAC"? TAC, "iE-Extensions"? extensions };
    LastVisitedCellItem choice { "e-UTRAN-Cell": LastVisitedEUTRANCellInformation, "uTRAN-Cell": _, "gERAN-Cell": LastVisitedGERANCellInformation, "nG-RAN-Cell": _ };
    LastVisitedEUTRANCellInformation sequence { "global-Cell-ID": EUTRANCGI, "cellType": CellType, "time-UE-StayedInCell": _, "iE-Extensions"? extensions };
    LastVisitedGERANCellInformation choice { "undefined": null };
    LastVisitedPSCellInformation sequence { "pSCellID"? PSCellInformation, "timeStay": _, "iE-Extensions"? extensions };
    LastVisitedPSCellList = [LastVisitedPSCellInformation];
    ListeningSubframePattern sequence { "pattern-period": ListeningSubframePatternPatternPeriod, "pattern-offset": _, "iE-Extensions"? extensions };
    LocationReport sequence { "protocolIEs": ies };
    LocationReportingControl sequence { "protocolIEs": ies };
    LocationReportingFailureIndication sequence { "protocolIEs": ies };
    LoggedMBSFNMDT sequence { "loggingInterval": LoggingInterval, "loggingDuration": LoggingDuration, "mBSFN-ResultToLog"? MBSFNResultToLog, "iE-Extensions"? extensions };
    LoggedMDT sequence { "loggingInterval": LoggingInterval, "loggingDuration": LoggingDuration, "iE-Extensions"? extensions };
    LoggedMDTTrigger choice { "periodical": null, "eventTrigger": EventTrigger };
    M1PeriodicReporting sequence { "reportInterval": ReportIntervalMDT, "reportAmount": ReportAmountMDT, "iE-Extensions"? extensions };
    M1ThresholdEventA2 sequence { "measurementThreshold": MeasurementThresholdA2, "iE-Extensions"? extensions };
    M3Configuration sequence { "m3period": M3period, "iE-Extensions"? extensions };
    M4Configuration sequence { "m4period": M4period, "m4-links-to-log": LinksToLog, "iE-Extensions"? extensions };
    M5Configuration sequence { "m5period": M5period, "m5-links-to-log": LinksToLog, "iE-Extensions"? extensions };
    M6Configuration sequence { "m6report-Interval": M6reportInterval, "m6delay-threshold"? M6delayThreshold, "m6-links-to-log": LinksToLog, "iE-Extensions"? extensions };
    M7Configuration sequence { "m7period": _, "m7-links-to-log": LinksToLog, "iE-Extensions"? extensions };
    MBSFNResultToLog = [MBSFNResultToLogInfo];
    MBSFNResultToLogInfo sequence { "mBSFN-AreaId"? _, "carrierFreq": _, "iE-Extensions"? extensions };
    MDTConfiguration sequence { "mdt-Activation": MDTActivation, "areaScopeOfMDT": AreaScopeOfMDT, "mDTMode": MDTMode, "iE-Extensions"? extensions };
    MDTMode choice { "immediateMDT": ImmediateMDT, "loggedMDT": LoggedMDT, "mDTMode-Extension": ie };
    MDTPLMNList = [PLMNidentity];
    MMECPRelocationIndication sequence { "protocolIEs": ies };
    MMEConfigurationTransfer sequence { "protocolIEs": ies };
    MMEConfigurationUpdate sequence { "protocolIEs": ies };
    MMEConfigurationUpdateAcknowledge sequence { "protocolIEs": ies };
    MMEConfigurationUpdateFailure sequence { "protocolIEs": ies };
    MMEDirectInformationTransfer sequence { "protocolIEs": ies };
    MMEEarlyStatusTransfer sequence { "protocolIEs": ies };
    MMEPagingTarget choice { "global-ENB-ID": GlobalENBID, "tAI": TAI };
    MMEStatusTransfer sequence { "protocolIEs": ies };
    MeasurementThresholdA2 choice { "threshold-RSRP": _, "threshold-RSRQ": _ };
    MeasurementThresholdL1LoggedMDT choice { "threshold-RSRP": _, "threshold-RSRQ": _, "choice-Extensions": ie };
    MutingPatternInformation sequence { "muting-pattern-period": MutingPatternInformationMutingPatternPeriod, "muting-pattern-offset"? _, "iE-Extensions"? extensions };
    NASDeliveryIndication sequence { "protocolIEs": ies };
    NASNonDeliveryIndication sequence { "protocolIEs": ies };
    NBIoTPagingEDRXInformation sequence { "nB-IoT-paging-eDRX-Cycle": NBIoTPagingEDRXCycle, "nB-IoT-pagingTimeWindow"? NBIoTPagingTimeWindow, "iE-Extensions"? extensions };
    NGENB sequence { "global-ng-eNB-ID": GlobalENBID, "iE-Extensions"? extensions };
    NRCGI sequence { "pLMNIdentity": PLMNidentity, "nRCellIdentity": NRCellIdentity, "iE-Extensions"? extensions };
    NRUESecurityCapabilities sequence { "nRencryptionAlgorithms": NRencryptionAlgorithms, "nRintegrityProtectionAlgorithms": NRintegrityProtectionAlgorithms, "iE-Extensions"? extensions };
    NRUESidelinkAggregateMaximumBitrate sequence { "uEaggregateMaximumBitRate": _, "iE-Extensions"? extensions };
    NRV2XServicesAuthorized sequence { "vehicleUE"? VehicleUE, "pedestrianUE"? PedestrianUE, "iE-Extensions"? extensions };
    NRencryptionAlgorithms = bits;
    NRintegrityProtectionAlgorithms = bits;
    OverloadResponse choice { "overloadAction": OverloadAction };
    OverloadStart sequence { "protocolIEs": ies };
    OverloadStop sequence { "protocolIEs": ies };
    PC5FlowBitRates sequence { "guaranteedFlowBitRate": _, "maximumFlowBitRate": _, "iE-Extensions"? extensions };
    PC5QoSFlowItem sequence { "pQI": _, "pc5FlowBitRates"? PC5FlowBitRates, "range"? Range, "iE-Extensions"? extensions };
    PC5QoSFlowList = [PC5QoSFlowItem];
    PC5QoSParameters sequence { "pc5QoSFlowList": PC5QoSFlowList, "pc5LinkAggregatedBitRates"? _, "iE-Extensions"? extensions };
    PLMNAreaBasedQMC sequence { "plmnListforQMC": PLMNListforQMC, "iE-Extensions"? extensions };
    PLMNListforQMC = [PLMNidentity];
    PSCellInformation sequence { "nCGI": NRCGI, "iE-Extensions"? extensions };
    PWSFailureIndication sequence { "protocolIEs": ies };
    PWSRestartIndication sequence { "protocolIEs": ies };
    PWSfailedECGIList = [EUTRANCGI];
    Paging sequence { "protocolIEs": ies };
    PagingAttemptInformation sequence { "pagingAttemptCount": _, "intendedNumberOfPagingAttempts": _, "nextPagingAreaScope"? NextPagingAreaScope, "iE-Extensions"? extensions };
    PagingEDRXInformation sequence { "paging-eDRX-Cycle": PagingEDRXCycle, "pagingTimeWindow"? PagingTimeWindow, "iE-Extensions"? extensions };
    PathSwitchRequest sequence { "protocolIEs": ies };
    PathSwitchRequestAcknowledge sequence { "protocolIEs": ies };
    PathSwitchRequestFailure sequence { "protocolIEs": ies };
    PrivateIEID choice { "local": _, "global": _ };
    PrivateMessage sequence { "privateIEs": PrivateMessagePrivateIEs };
    PrivateMessagePrivateIEs = [AnonymousPrivateMessagePrivateIEs];
    ProSeAuthorized sequence { "proSeDirectDiscovery"? ProSeDirectDiscovery, "proSeDirectCommunication"? ProSeDirectCommunication, "iE-Extensions"? extensions };
    RATRestrictions = [RATRestrictionsItem];
    RATRestrictionsItem sequence { "pLMNidentity": PLMNidentity, "rAT-RestrictionInformation": bits, "iE-Extensions"? extensions };
    RIMRoutingAddress choice { "gERAN-Cell-ID": GERANCellID, "targetRNC-ID": TargetRNCID, "eHRPD-Sector-ID": _ };
    RIMTransfer sequence { "rIMInformation": _, "rIMRoutingAddress"? RIMRoutingAddress, "iE-Extensions"? extensions };
    RLFReportInformation sequence { "uE-RLF-Report-Container": _, "uE-RLF-Report-Container-for-extended-bands"? _, "iE-Extensions"? extensions };
    ReceiveStatusOfULPDCPSDUsExtended = bits;
    ReceiveStatusOfULPDCPSDUsPDCPSNlength18 = bits;
    RecommendedCellItem sequence { "eUTRAN-CGI": EUTRANCGI, "timeStayedInCell"? _, "iE-Extensions"? extensions };
    RecommendedCellList = [ie];
    RecommendedCellsForPaging sequence { "recommendedCellList": RecommendedCellList, "iE-Extensions"? extensions };
    RecommendedENBItem sequence { "mMEPagingTarget": MMEPagingTarget, "iE-Extensions"? extensions };
    RecommendedENBList = [ie];
    RecommendedENBsForPaging sequence { "recommendedENBList": RecommendedENBList, "iE-Extensions"? extensions };
    RequestType sequence { "eventType": EventType, "reportArea": ReportArea, "iE-Extensions"? extensions };
    RequestedTNLInfo sequence { "pLMNidentity": PLMNidentity, "iE-Extensions"? extensions };
    RerouteNASRequest sequence { "protocolIEs": ies };
    Reset sequence { "protocolIEs": ies };
    ResetAcknowledge sequence { "protocolIEs": ies };
    ResetType choice { "s1-Interface": ResetAll, "partOfS1-Interface": UEAssociatedLogicalS1ConnectionListRes };
    RetrieveUEInformation sequence { "protocolIEs": ies };
    S1APPDU choice { "initiatingMessage": InitiatingMessage, "successfulOutcome": SuccessfulOutcome, "unsuccessfulOutcome": UnsuccessfulOutcome };
    S1RemovalFailure sequence { "protocolIEs": ies };
    S1RemovalRequest sequence { "protocolIEs": ies };
    S1RemovalResponse sequence { "protocolIEs": ies };
    S1SetupFailure sequence { "protocolIEs": ies };
    S1SetupRequest sequence { "protocolIEs": ies };
    S1SetupResponse sequence { "protocolIEs": ies };
    SONConfigurationTransfer sequence { "targeteNB-ID": TargeteNBID, "sourceeNB-ID": SourceeNBID, "sONInformation": SONInformation, "iE-Extensions"? extensions };
    SONInformation choice { "sONInformationRequest": SONInformationRequest, "sONInformationReply": SONInformationReply, "sONInformation-Extension": ie };
    SONInformationReply sequence { "x2TNLConfigurationInfo"? X2TNLConfigurationInfo, "iE-Extensions"? extensions };
    SONInformationReport choice { "rLFReportInformation": RLFReportInformation };
    STMSI sequence { "mMEC": MMECode, "m-TMSI": MTMSI, "iE-Extensions"? extensions };
    ScheduledCommunicationTime sequence { "dayofWeek"? bits, "timeofDayStart"? _, "timeofDayEnd"? _, "iE-Extensions"? extensions };
    SecondaryRATDataUsageReport sequence { "protocolIEs": ies };
    SecondaryRATDataUsageReportItem sequence { "e-RAB-ID": _, "secondaryRATType": SecondaryRATType, "e-RABUsageReportList": ERABUsageReportList, "iE-Extensions"? extensions };
    SecondaryRATDataUsageReportList = [ie];
    SecurityContext sequence { "nextHopChainingCount": _, "nextHopParameter": _, "iE-Extensions"? extensions };
    SecurityIndication sequence { "integrityProtectionIndication": IntegrityProtectionIndication, "iE-Extensions"? extensions };
    SecurityResult sequence { "integrityProtectionResult": IntegrityProtectionResult, "iE-Extensions"? extensions };
    SensorMeasConfigNameItem sequence { "sensorNameConfig": SensorNameConfig, "iE-Extensions"? extensions };
    SensorMeasConfigNameList = [SensorMeasConfigNameItem];
    SensorMeasurementConfiguration sequence { "sensorMeasConfig": SensorMeasConfig, "sensorMeasConfigNameList"? SensorMeasConfigNameList, "iE-Extensions"? extensions };
    SensorNameConfig choice { "uncompensatedBarometricConfig": SensorNameConfigUncompensatedBarometricConfig, "choice-Extensions": ie };
    ServedDCNs = [ServedDCNsItem];
    ServedDCNsItem sequence { "dCN-ID": _, "relativeDCNCapacity": _, "iE-Extensions"? extensions };
    ServedGUMMEIs = [ServedGUMMEIsItem];
    ServedGUMMEIsItem sequence { "servedPLMNs": ServedPLMNs, "servedGroupIDs": ServedGroupIDs, "servedMMECs": ServedMMECs, "iE-Extensions"? extensions };
    ServedGroupIDs = [MMEGroupID];
    ServedMMECs = [MMECode];
    ServedPLMNs = [PLMNidentity];
    SourceNgRanNodeID sequence { "global-RAN-NODE-ID": GlobalRANNODEID, "selected-TAI": FiveGSTAI, "iE-Extensions"? extensions };
    SourceNodeID choice { "sourceNgRanNode-ID": SourceNgRanNodeID, "sourceNodeID-Extension": ie };
    SourceeNBID sequence { "global-ENB-ID": GlobalENBID, "selected-TAI": TAI, "iE-Extensions"? extensions };
    SourceeNBToTargeteNBTransparentContainer sequence { "rRC-Container": _, "e-RABInformationList"? ERABInformationList, "targetCell-ID": EUTRANCGI, "subscriberProfileIDforRFP"? _, "uE-HistoryInformation": UEHistoryInformation, "iE-Extensions"? extensions };
    SubscriptionBasedUEDifferentiationInfo sequence { "periodicCommunicationIndicator"? SubscriptionBasedUEDifferentiationInfoPeriodicCommunicationIndicator, "periodicTime"? _, "scheduledCommunicationTime"? ScheduledCommunicationTime, "stationaryIndication"? SubscriptionBasedUEDifferentiationInfoStationaryIndication, "trafficProfile"? SubscriptionBasedUEDifferentiationInfoTrafficProfile, "batteryIndication"? SubscriptionBasedUEDifferentiationInfoBatteryIndication, "iE-Extensions"? extensions };
    SuccessfulOutcome sequence { "procedureCode": _, "criticality": Criticality, "value": _ };
    SupportedTAs = [SupportedTAsItem];
    SupportedTAsItem sequence { "tAC": TAC, "broadcastPLMNs": BPLMNs, "iE-Extensions"? extensions };
    SynchronisationInformation sequence { "sourceStratumLevel"? _, "listeningSubframePattern"? ListeningSubframePattern, "aggressoreCGI-List"? ECGI_List, "iE-Extensions"? extensions };
    TABasedMDT sequence { "tAListforMDT": TAListforMDT, "iE-Extensions"? extensions };
    TABasedQMC sequence { "tAListforQMC": TAListforQMC, "iE-Extensions"? extensions };
    TACListInLTENTN = [TAC];
    TAI sequence { "pLMNidentity": PLMNidentity, "tAC": TAC, "iE-Extensions"? extensions };
    TAIBasedMDT sequence { "tAIListforMDT": TAIListforMDT, "iE-Extensions"? extensions };
    TAIBasedQMC sequence { "tAIListforQMC": TAIListforQMC, "iE-Extensions"? extensions };
    TAIBroadcast = [TAIBroadcastItem];
    TAIBroadcastItem sequence { "tAI": TAI, "completedCellinTAI": CompletedCellinTAI, "iE-Extensions"? extensions };
    TAICancelled = [TAICancelledItem];
    TAICancelledItem sequence { "tAI": TAI, "cancelledCellinTAI": CancelledCellinTAI, "iE-Extensions"? extensions };
    TAIItem sequence { "tAI": TAI, "iE-Extensions"? extensions };
    TAIList = [ie];
    TAIListForRestart = [TAI];
    TAIListforMDT = [TAI];
    TAIListforQMC = [TAI];
    TAIListforWarning = [TAI];
    TAListforMDT = [TAC];
    TAListforQMC = [TAC];
    TargetID choice { "targeteNB-ID": TargeteNBID, "targetRNC-ID": TargetRNCID, "cGI": CGI, "targetgNgRanNode-ID": TargetNgRanNodeID };
    TargetNgRanNodeID sequence { "global-RAN-NODE-ID": GlobalRANNODEID, "selected-TAI": FiveGSTAI, "iE-Extensions"? extensions };
    TargetRNCID sequence { "lAI": LAI, "rAC"? RAC, "rNC-ID": _, "extendedRNC-ID"? _, "iE-Extensions"? extensions };
    TargeteNBID sequence { "global-ENB-ID": GlobalENBID, "selected-TAI": TAI, "iE-Extensions"? extensions };
    TargeteNBToSourceeNBTransparentContainer sequence { "rRC-Container": _, "iE-Extensions"? extensions };
    TimeBasedHandoverInformation sequence { "hOWindowStart": _, "hOWindowDuration": _, "iE-Extensions"? extensions };
    TimeSynchronisationInfo sequence { "stratumLevel": _, "synchronisationStatus": SynchronisationStatus, "iE-Extensions"? extensions };
    TraceActivation sequence { "e-UTRAN-Trace-ID": _, "interfacesToTrace": _, "traceDepth": TraceDepth, "traceCollectionEntityIPAddress": TransportLayerAddress, "iE-Extensions"? extensions };
    TraceFailureIndication sequence { "protocolIEs": ies };
    TraceStart sequence { "protocolIEs": ies };
    TransportInformation sequence { "transportLayerAddress": TransportLayerAddress, "uL-GTP-TEID": GTPTEID };
    TransportLayerAddress = bits;
    TunnelInformation sequence { "transportLayerAddress": TransportLayerAddress, "uDP-Port-Number"? PortNumber, "iE-Extensions"? extensions };
    UEAggregateMaximumBitrate sequence { "uEaggregateMaximumBitRateDL": _, "uEaggregateMaximumBitRateUL": _, "iE-Extensions"? extensions };
    UEAppLayerMeasConfig sequence { "containerForAppLayerMeasConfig": _, "areaScopeOfQMC": AreaScopeOfQMC, "iE-Extensions"? extensions };
    UEAssociatedLogicalS1ConnectionItem sequence { "mME-UE-S1AP-ID"? _, "eNB-UE-S1AP-ID"? _, "iE-Extensions"? extensions };
    UEAssociatedLogicalS1ConnectionListRes = [ie];
    UEAssociatedLogicalS1ConnectionListResAck = [ie];
    UECapabilityInfoIndication sequence { "protocolIEs": ies };
    UEContextModificationConfirm sequence { "protocolIEs": ies };
    UEContextModificationFailure sequence { "protocolIEs": ies };
    UEContextModificationIndication sequence { "protocolIEs": ies };
    UEContextModificationRequest sequence { "protocolIEs": ies };
    UEContextModificationResponse sequence { "protocolIEs": ies };
    UEContextReleaseCommand sequence { "protocolIEs": ies };
    UEContextReleaseComplete sequence { "protocolIEs": ies };
    UEContextReleaseRequest sequence { "protocolIEs": ies };
    UEContextResumeFailure sequence { "protocolIEs": ies };
    UEContextResumeRequest sequence { "protocolIEs": ies };
    UEContextResumeResponse sequence { "protocolIEs": ies };
    UEContextSuspendRequest sequence { "protocolIEs": ies };
    UEContextSuspendResponse sequence { "protocolIEs": ies };
    UEHistoryInformation = [LastVisitedCellItem];
    UEInformationTransfer sequence { "protocolIEs": ies };
    UEPagingID choice { "s-TMSI": STMSI, "iMSI": IMSI };
    UERadioCapabilityIDMappingRequest sequence { "protocolIEs": ies };
    UERadioCapabilityIDMappingResponse sequence { "protocolIEs": ies };
    UERadioCapabilityMatchRequest sequence { "protocolIEs": ies };
    UERadioCapabilityMatchResponse sequence { "protocolIEs": ies };
    UES1APIDPair sequence { "mME-UE-S1AP-ID": _, "eNB-UE-S1AP-ID": _, "iE-Extensions"? extensions };
    UES1APIDs choice { "uE-S1AP-ID-pair": UES1APIDPair, "mME-UE-S1AP-ID": _ };
    UESecurityCapabilities sequence { "encryptionAlgorithms": EncryptionAlgorithms, "integrityProtectionAlgorithms": IntegrityProtectionAlgorithms, "iE-Extensions"? extensions };
    UESidelinkAggregateMaximumBitrate sequence { "uESidelinkAggregateMaximumBitRate": _, "iE-Extensions"? extensions };
    ULCPSecurityInformation sequence { "ul-NAS-MAC": _, "ul-NAS-Count": ULNASCount, "iE-Extensions"? extensions };
    UnsuccessfulOutcome sequence { "procedureCode": _, "criticality": Criticality, "value": _ };
    UplinkNASTransport sequence { "protocolIEs": ies };
    UplinkNonUEAssociatedLPPaTransport sequence { "protocolIEs": ies };
    UplinkS1cdma2000tunnelling sequence { "protocolIEs": ies };
    UplinkUEAssociatedLPPaTransport sequence { "protocolIEs": ies };
    UserLocationInformation sequence { "eutran-cgi": EUTRANCGI, "tai": TAI, "iE-Extensions"? extensions };
    V2XServicesAuthorized sequence { "vehicleUE"? VehicleUE, "pedestrianUE"? PedestrianUE, "iE-Extensions"? extensions };
    WLANMeasConfigNameList = [_];
    WLANMeasurementConfiguration sequence { "wlanMeasConfig": WLANMeasConfig, "wlanMeasConfigNameList"? WLANMeasConfigNameList, "wlan-rssi"? WLANMeasurementConfigurationWlanRssi, "wlan-rtt"? WLANMeasurementConfigurationWlanRtt, "iE-Extensions"? extensions };
    WUSAssistanceInformation sequence { "pagingProbabilityInformation": PagingProbabilityInformation, "iE-Extensions"? extensions };
    WarningAreaList choice { "cellIDList": ECGIList, "trackingAreaListforWarning": TAIListforWarning, "emergencyAreaIDList": EmergencyAreaIDList };
    WriteReplaceWarningRequest sequence { "protocolIEs": ies };
    WriteReplaceWarningResponse sequence { "protocolIEs": ies };
    X2TNLConfigurationInfo sequence { "eNBX2TransportLayerAddresses": ENBX2TLAs, "iE-Extensions"? extensions };
}
