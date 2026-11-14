// Auto-generated from ASN.1 by the generator of oxirush-aper; do not edit.
pub(crate) use crate::registry::{
    IE_NAMES, IE_TYPES, ITEM_CRITICALITIES, MESSAGES, ie, ie_contents,
};
use oxirush_aper::inspect::Form;
pub(crate) const PROTOCOL: &str = "S1AP";
pub(crate) const ROOT: &str = "s1ap";
oxirush_aper::__transfers! {}
oxirush_aper::__carried! {
    104 "SourceeNB-ToTargeteNB-TransparentContainer" crate::s1ap::SourceeNBToTargeteNBTransparentContainer;
    123 "TargeteNB-ToSourceeNB-TransparentContainer" crate::s1ap::TargeteNBToSourceeNBTransparentContainer;
    of "NGAP" 104 "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
    of "NGAP" 123 "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer";
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
oxirush_aper::__types! {
    types {
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
    enumerations {
        AdditionalCSFallbackIndicator ["no-restriction", "restriction"];
        AerialUEsubscriptionInformation ["allowed", "not-allowed"];
        BearerType ["non-IP"];
        BluetoothMeasConfig ["setup"];
        BluetoothMeasurementConfigurationBtRssi ["true"];
        CEModeBRestricted ["restricted", "not-restricted"];
        CEModeBSupportIndicator ["supported"];
        CNDomain ["ps", "cs"];
        CNType ["fiveGCForbidden", "epc-Forbiddden"];
        CSFallbackIndicator ["cs-fallback-required", "cs-fallback-high-priority"];
        CSGMembershipStatus ["member", "not-member"];
        CauseMisc ["control-processing-overload", "not-enough-user-plane-processing-resources", "hardware-failure", "om-intervention", "unspecified", "unknown-PLMN"];
        CauseNas ["normal-release", "authentication-failure", "detach", "unspecified", "csg-subscription-expiry", "uE-not-in-PLMN-serving-area", "iab-not-authorized"];
        CauseProtocol ["transfer-syntax-error", "abstract-syntax-error-reject", "abstract-syntax-error-ignore-and-notify", "message-not-compatible-with-receiver-state", "semantic-error", "abstract-syntax-error-falsely-constructed-message", "unspecified"];
        CauseRadioNetwork ["unspecified", "tx2relocoverall-expiry", "successful-handover", "release-due-to-eutran-generated-reason", "handover-cancelled", "partial-handover", "ho-failure-in-target-EPC-eNB-or-target-system", "ho-target-not-allowed", "tS1relocoverall-expiry", "tS1relocprep-expiry", "cell-not-available", "unknown-targetID", "no-radio-resources-available-in-target-cell", "unknown-mme-ue-s1ap-id", "unknown-enb-ue-s1ap-id", "unknown-pair-ue-s1ap-id", "handover-desirable-for-radio-reason", "time-critical-handover", "resource-optimisation-handover", "reduce-load-in-serving-cell", "user-inactivity", "radio-connection-with-ue-lost", "load-balancing-tau-required", "cs-fallback-triggered", "ue-not-available-for-ps-service", "radio-resources-not-available", "failure-in-radio-interface-procedure", "invalid-qos-combination", "interrat-redirection", "interaction-with-other-procedure", "unknown-E-RAB-ID", "multiple-E-RAB-ID-instances", "encryption-and-or-integrity-protection-algorithms-not-supported", "s1-intra-system-handover-triggered", "s1-inter-system-handover-triggered", "x2-handover-triggered", "redirection-towards-1xRTT", "not-supported-QCI-value", "invalid-CSG-Id", "release-due-to-pre-emption", "n26-interface-not-available", "insufficient-ue-capabilities", "maximum-bearer-pre-emption-rate-exceeded", "up-integrity-protection-not-possible", "release-due-to-discontinuous-coverage"];
        CauseTransport ["transport-resource-unavailable", "unspecified"];
        Cdma2000HORequiredIndication ["true"];
        Cdma2000HOStatus ["hOSuccess", "hOFailure"];
        Cdma2000RATType ["hRPD", "onexRTT"];
        CellAccessMode ["hybrid"];
        CellSize ["verysmall", "small", "medium", "large"];
        CoarseUELocationRequested ["true"];
        ConcurrentWarningMessageIndicator ["true"];
        CoverageLevel ["extendedcoverage"];
        Criticality ["reject", "ignore", "notify"];
        DAPSRequestInfoDAPSIndicator ["dAPS-HO-required"];
        DAPSResponseInfoDapsresponseindicator ["dAPS-HO-accepted", "dAPS-HO-not-accepted"];
        DLForwarding ["dL-Forwarding-proposed"];
        DLNASPDUDeliveryAckRequest ["requested"];
        DataForwardingNotPossible ["data-Forwarding-not-Possible"];
        DirectForwardingPathAvailability ["directPathAvailable"];
        EDTSession ["true"];
        EmergencyIndicator ["true"];
        EndIndication ["no-further-data", "further-data-exists"];
        EnhancedCoverageRestricted ["restricted"];
        EthernetType ["true"];
        EventTriggerOutOfCoverage ["true"];
        EventType ["direct", "change-of-serve-cell", "stop-change-of-serve-cell"];
        ExpectedHOInterval ["sec15", "sec30", "sec60", "sec90", "sec120", "sec180", "long-time"];
        ForbiddenInterRATs ["all", "geran", "utran", "cdma2000", "geranandutran", "cdma2000andutran"];
        GUMMEIType ["native", "mapped", "mappedFrom5G"];
        GWContextReleaseIndication ["true"];
        HandoverFlag ["handoverPreparation"];
        HandoverType ["intralte", "ltetoutran", "ltetogeran", "utrantolte", "gerantolte", "eps-to-5gs", "fivegs-to-eps"];
        IABAuthorized ["authorized", "not-authorized"];
        IABNodeIndication ["true"];
        IABSupported ["true"];
        IMSvoiceEPSfallbackfrom5G ["true"];
        IntegrityProtectionIndication ["required", "preferred", "not-needed"];
        IntegrityProtectionResult ["performed", "not-performed"];
        InterSystemMeasurementItemSubcarrierSpacingSSB ["kHz15", "kHz30", "kHz60", "kHz120", "kHz240", "kHz480", "kHz960"];
        KillAllWarningMessages ["true"];
        LTEMIndication ["lte-m"];
        LinksToLog ["uplink", "downlink", "both-uplink-and-downlink"];
        ListeningSubframePatternPatternPeriod ["ms1280", "ms2560", "ms5120", "ms10240"];
        LoggingDuration ["m10", "m20", "m40", "m60", "m90", "m120"];
        LoggingInterval ["ms1280", "ms2560", "ms5120", "ms10240", "ms20480", "ms30720", "ms40960", "ms61440"];
        M1ReportingTrigger ["periodic", "a2eventtriggered", "a2eventtriggered-periodic"];
        M3period ["ms100", "ms1000", "ms10000", "ms1024", "ms1280", "ms2048", "ms2560", "ms5120", "ms10240", "min1"];
        M4ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"];
        M4period ["ms1024", "ms2048", "ms5120", "ms10240", "min1"];
        M5ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"];
        M5period ["ms1024", "ms2048", "ms5120", "ms10240", "min1"];
        M6ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"];
        M6delayThreshold ["ms30", "ms40", "ms50", "ms60", "ms70", "ms80", "ms90", "ms100", "ms150", "ms300", "ms500", "ms750"];
        M6reportInterval ["ms1024", "ms2048", "ms5120", "ms10240"];
        M7ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"];
        MDTActivation ["immediate-MDT-only", "immediate-MDT-and-Trace", "logged-MDT-only", "logged-MBSFN-MDT"];
        MMERelaySupportIndicator ["true"];
        ManagementBasedMDTAllowed ["allowed"];
        MutingAvailabilityIndication ["available", "unavailable"];
        MutingPatternInformationMutingPatternPeriod ["ms0", "ms1280", "ms2560", "ms5120", "ms10240"];
        NBIoTDefaultPagingDRX ["v128", "v256", "v512", "v1024"];
        NBIoTPagingDRX ["v32", "v64", "v128", "v256", "v512", "v1024"];
        NBIoTPagingEDRXCycle ["hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128", "hf256", "hf512", "hf1024"];
        NBIoTPagingTimeWindow ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16"];
        NRrestrictionin5GS ["nRrestrictedin5GS"];
        NRrestrictioninEPSasSecondaryRAT ["nRrestrictedinEPSasSecondaryRAT"];
        NextPagingAreaScope ["same", "changed"];
        NotifySourceeNB ["notifySource"];
        OverloadAction ["reject-non-emergency-mo-dt", "reject-rrc-cr-signalling", "permit-emergency-sessions-and-mobile-terminated-services-only", "permit-high-priority-sessions-and-mobile-terminated-services-only", "reject-delay-tolerant-access", "permit-high-priority-sessions-and-exception-reporting-and-mobile-terminated-services-only", "not-accept-mo-data-or-delay-tolerant-access-from-CP-CIoT"];
        PSServiceNotAvailable ["ps-service-not-available"];
        PagingCause ["voice"];
        PagingDRX ["v32", "v64", "v128", "v256"];
        PagingEDRXCycle ["hfhalf", "hf1", "hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128", "hf256"];
        PagingPriority ["priolevel1", "priolevel2", "priolevel3", "priolevel4", "priolevel5", "priolevel6", "priolevel7", "priolevel8"];
        PagingProbabilityInformation ["p00", "p05", "p10", "p15", "p20", "p25", "p30", "p35", "p40", "p45", "p50", "p55", "p60", "p65", "p70", "p75", "p80", "p85", "p90", "p95", "p100"];
        PagingTimeWindow ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16"];
        PedestrianUE ["authorized", "not-authorized"];
        PendingDataIndication ["true"];
        PreEmptionCapability ["shall-not-trigger-pre-emption", "may-trigger-pre-emption"];
        PreEmptionVulnerability ["not-pre-emptable", "pre-emptable"];
        Presence ["optional", "conditional", "mandatory"];
        PrivacyIndicator ["immediate-MDT", "logged-MDT"];
        ProSeDirectCommunication ["authorized", "not-authorized"];
        ProSeDirectDiscovery ["authorized", "not-authorized"];
        ProSeUEtoNetworkRelaying ["authorized", "not-authorized"];
        RACSIndication ["true"];
        RATType ["nbiot", "nbiot-leo", "nbiot-meo", "nbiot-geo", "nbiot-othersat", "eutran-leo", "eutran-meo", "eutran-geo", "eutran-othersat"];
        RRCEstablishmentCause ["emergency", "highPriorityAccess", "mt-Access", "mo-Signalling", "mo-Data", "delay-TolerantAccess", "mo-VoiceCall", "mo-ExceptionData"];
        Range ["m50", "m80", "m180", "m200", "m350", "m400", "m500", "m700", "m1000"];
        RelayNodeIndicator ["true"];
        ReportAmountMDT ["r1", "r2", "r4", "r8", "r16", "r32", "r64", "rinfinity"];
        ReportArea ["ecgi"];
        ReportIntervalMDT ["ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "min1", "min6", "min12", "min30", "min60"];
        RequestTypeAdditionalInfo ["includePSCell"];
        ResetAll ["reset-all"];
        SONInformationRequest ["x2TNL-Configuration-Info", "time-Synchronisation-Info", "activate-Muting", "deactivate-Muting"];
        SRVCCHOIndication ["pSandCS", "cSonly"];
        SRVCCOperationNotPossible ["notPossible"];
        SRVCCOperationPossible ["possible"];
        SecondaryRATDataUsageRequest ["requested"];
        SecondaryRATType ["nR", "unlicensed"];
        SensorMeasConfig ["setup"];
        SensorNameConfigUncompensatedBarometricConfig ["true"];
        ServiceType ["qMC-for-streaming-service", "qMC-for-MTSI-service"];
        SourceOfUEActivityBehaviourInformation ["subscription-information", "statistics"];
        SubscriptionBasedUEDifferentiationInfoBatteryIndication ["battery-powered", "battery-powered-not-rechargeable-or-replaceable", "not-battery-powered"];
        SubscriptionBasedUEDifferentiationInfoPeriodicCommunicationIndicator ["periodically", "ondemand"];
        SubscriptionBasedUEDifferentiationInfoStationaryIndication ["stationary", "mobile"];
        SubscriptionBasedUEDifferentiationInfoTrafficProfile ["single-packet", "dual-packets", "multiple-packets"];
        SynchronisationStatus ["synchronous", "asynchronous"];
        TimeRefDistribution ["true"];
        TimeToTrigger ["ms0", "ms40", "ms64", "ms80", "ms100", "ms128", "ms160", "ms256", "ms320", "ms480", "ms512", "ms640", "ms1024", "ms1280", "ms2560", "ms5120"];
        TimeToWait ["v1s", "v2s", "v5s", "v10s", "v20s", "v60s"];
        TraceDepth ["minimum", "medium", "maximum", "minimumWithoutVendorSpecificExtension", "mediumWithoutVendorSpecificExtension", "maximumWithoutVendorSpecificExtension"];
        TriggeringMessage ["initiating-message", "successful-outcome", "unsuccessfull-outcome"];
        TypeOfError ["not-understood", "missing"];
        UECapabilityInfoRequest ["requested"];
        UERetentionInformation ["ues-retained"];
        UEUserPlaneCIoTSupportIndicator ["supported"];
        UnlicensedSpectrumRestriction ["unlicensed-restricted"];
        VehicleUE ["authorized", "not-authorized"];
        VoiceSupportMatchIndicator ["supported", "not-supported"];
        WLANMeasConfig ["setup"];
        WLANMeasurementConfigurationWlanRssi ["true"];
        WLANMeasurementConfigurationWlanRtt ["true"];
    }
}
use oxirush_aper::UNBOUNDED;
#[allow(unused_imports)]
use oxirush_aper::inspect::schema::{Field as F, Kind as K, Member as M, Text, Type as T};
/// The types of the protocol, in the order of the names that ASN.1 gives them, which the walker of the trees reads: a type is named by its row.
#[rustfmt::skip]
pub(crate) static SCHEMA: &[T] = &[
/* 0 */ T { name: "Additional-GUTI", kind: K::Sequence { names: &["gUMMEI", "m-TMSI", "iE-Extensions"], members: &[M { field: "gummei", ty: F::Named(250), optional: false }, M { field: "m_tmsi", ty: F::Named(329), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 1 */ T { name: "AdditionalCSFallbackIndicator", kind: K::Enumerated { names: &["no-restriction", "restriction"], root: 2, extensible: true } },
/* 2 */ T { name: "AdditionalRRMPriorityIndex", kind: K::Alias { field: F::Bits(32, 32), form: None, secret: false } },
/* 3 */ T { name: "AerialUEsubscriptionInformation", kind: K::Enumerated { names: &["allowed", "not-allowed"], root: 2, extensible: true } },
/* 4 */ T { name: "AllocationAndRetentionPriority", kind: K::Sequence { names: &["priorityLevel", "pre-emptionCapability", "pre-emptionVulnerability", "iE-Extensions"], members: &[M { field: "priority_level", ty: F::Named(449), optional: false }, M { field: "pre_emption_capability", ty: F::Named(446), optional: false }, M { field: "pre_emption_vulnerability", ty: F::Named(447), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 5 */ T { name: "AreaScopeOfMDT", kind: K::Choice { names: &["cellBased", "tABased", "pLMNWide", "tAIBased"], alternatives: &[F::Named(66), F::Named(568), F::Null, F::Named(577)], root: 3, extensible: true } },
/* 6 */ T { name: "AreaScopeOfQMC", kind: K::Choice { names: &["cellBased", "tABased", "tAIBased", "pLMNAreaBased"], alternatives: &[F::Named(67), F::Named(569), F::Named(578), F::Named(421)], root: 4, extensible: true } },
/* 7 */ T { name: "AssistanceDataForCECapableUEs", kind: K::Sequence { names: &["cellIdentifierAndCELevelForCECapableUEs", "iE-Extensions"], members: &[M { field: "cell_identifier_and_ce_level_for_ce_capable_ues", ty: F::Named(74), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 8 */ T { name: "AssistanceDataForPaging", kind: K::Sequence { names: &["assistanceDataForRecommendedCells", "assistanceDataForCECapableUEs", "pagingAttemptInformation", "iE-Extensions"], members: &[M { field: "assistance_data_for_recommended_cells", ty: F::Named(9), optional: true }, M { field: "assistance_data_for_ce_capable_ues", ty: F::Named(7), optional: true }, M { field: "paging_attempt_information", ty: F::Named(434), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 9 */ T { name: "AssistanceDataForRecommendedCells", kind: K::Sequence { names: &["recommendedCellsForPaging", "iE-Extensions"], members: &[M { field: "recommended_cells_for_paging", ty: F::Named(485), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 10 */ T { name: "BPLMNs", kind: K::Alias { field: F::List(&F::Named(423), 1, 6), form: None, secret: false } },
/* 11 */ T { name: "BearerType", kind: K::Enumerated { names: &["non-IP"], root: 1, extensible: true } },
/* 12 */ T { name: "Bearers-SubjectToDLDiscarding-Item", kind: K::Sequence { names: &["e-RAB-ID", "dL-Discarding", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "dl_discarding", ty: F::Named(106), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 13 */ T { name: "Bearers-SubjectToDLDiscardingList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 14 */ T { name: "Bearers-SubjectToEarlyStatusTransfer-Item", kind: K::Sequence { names: &["e-RAB-ID", "dLCOUNT-PDCP-SNlength", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "dl_count_pdcp_sn_length", ty: F::Named(105), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 15 */ T { name: "Bearers-SubjectToEarlyStatusTransferList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 16 */ T { name: "Bearers-SubjectToStatusTransfer-Item", kind: K::Sequence { names: &["e-RAB-ID", "uL-COUNTvalue", "dL-COUNTvalue", "receiveStatusofULPDCPSDUs", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "ul_count_value", ty: F::Named(36), optional: false }, M { field: "dl_count_value", ty: F::Named(36), optional: false }, M { field: "receive_statusof_ul_pdcp_sdus", ty: F::Named(482), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 17 */ T { name: "Bearers-SubjectToStatusTransferList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 18 */ T { name: "BitRate", kind: K::Alias { field: F::Unsigned(0, 10000000000, false), form: None, secret: false } },
/* 19 */ T { name: "BluetoothMeasConfig", kind: K::Enumerated { names: &["setup"], root: 1, extensible: true } },
/* 20 */ T { name: "BluetoothMeasConfigNameList", kind: K::Alias { field: F::List(&F::Named(23), 1, 4), form: None, secret: false } },
/* 21 */ T { name: "BluetoothMeasurementConfiguration", kind: K::Sequence { names: &["bluetoothMeasConfig", "bluetoothMeasConfigNameList", "bt-rssi", "iE-Extensions"], members: &[M { field: "bluetooth_meas_config", ty: F::Named(19), optional: false }, M { field: "bluetooth_meas_config_name_list", ty: F::Named(20), optional: true }, M { field: "bt_rssi", ty: F::Named(22), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 22 */ T { name: "BluetoothMeasurementConfigurationBtRssi", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 23 */ T { name: "BluetoothName", kind: K::Alias { field: F::Octets(1, 248, false), form: None, secret: false } },
/* 24 */ T { name: "BroadcastCancelledAreaList", kind: K::Choice { names: &["cellID-Cancelled", "tAI-Cancelled", "emergencyAreaID-Cancelled"], alternatives: &[F::Named(70), F::Named(575), F::Named(210)], root: 3, extensible: true } },
/* 25 */ T { name: "BroadcastCompletedAreaList", kind: K::Choice { names: &["cellID-Broadcast", "tAI-Broadcast", "emergencyAreaID-Broadcast"], alternatives: &[F::Named(68), F::Named(573), F::Named(208)], root: 3, extensible: true } },
/* 26 */ T { name: "CE-ModeBRestricted", kind: K::Enumerated { names: &["restricted", "not-restricted"], root: 2, extensible: true } },
/* 27 */ T { name: "CE-mode-B-SupportIndicator", kind: K::Enumerated { names: &["supported"], root: 1, extensible: true } },
/* 28 */ T { name: "CELevel", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 29 */ T { name: "CGI", kind: K::Sequence { names: &["pLMNidentity", "lAC", "cI", "rAC", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "lac", ty: F::Named(305), optional: false }, M { field: "ci", ty: F::Named(30), optional: false }, M { field: "rac", ty: F::Named(466), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 30 */ T { name: "CI", kind: K::Alias { field: F::OctetArray(2), form: Some(Form::Number(16)), secret: false } },
/* 31 */ T { name: "CNDomain", kind: K::Enumerated { names: &["ps", "cs"], root: 2, extensible: false } },
/* 32 */ T { name: "CNType", kind: K::Enumerated { names: &["fiveGCForbidden", "epc-Forbiddden"], root: 1, extensible: true } },
/* 33 */ T { name: "CNTypeRestrictions", kind: K::Alias { field: F::List(&F::Named(34), 1, 16), form: None, secret: false } },
/* 34 */ T { name: "CNTypeRestrictions-Item", kind: K::Sequence { names: &["pLMN-Identity", "cNType", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "cn_type", ty: F::Named(32), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 35 */ T { name: "COUNTValueExtended", kind: K::Sequence { names: &["pDCP-SNExtended", "hFNModified", "iE-Extensions"], members: &[M { field: "pdcp_sn_extended", ty: F::Named(419), optional: false }, M { field: "hfn_modified", ty: F::Named(259), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 36 */ T { name: "COUNTvalue", kind: K::Sequence { names: &["pDCP-SN", "hFN", "iE-Extensions"], members: &[M { field: "pdcp_sn", ty: F::Named(418), optional: false }, M { field: "hfn", ty: F::Named(258), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 37 */ T { name: "COUNTvaluePDCP-SNlength18", kind: K::Sequence { names: &["pDCP-SNlength18", "hFNforPDCP-SNlength18", "iE-Extensions"], members: &[M { field: "pdcp_sn_length18", ty: F::Named(420), optional: false }, M { field: "hfn_for_pdcp_sn_length18", ty: F::Named(260), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 38 */ T { name: "CSFallbackIndicator", kind: K::Enumerated { names: &["cs-fallback-required", "cs-fallback-high-priority"], root: 1, extensible: true } },
/* 39 */ T { name: "CSG-Id", kind: K::Alias { field: F::Bits(27, 27), form: None, secret: false } },
/* 40 */ T { name: "CSG-IdList", kind: K::Alias { field: F::List(&F::Named(41), 1, 256), form: None, secret: false } },
/* 41 */ T { name: "CSG-IdList-Item", kind: K::Sequence { names: &["cSG-Id", "iE-Extensions"], members: &[M { field: "csg_id", ty: F::Named(39), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 42 */ T { name: "CSGMembershipInfo", kind: K::Sequence { names: &["cSGMembershipStatus", "cSG-Id", "cellAccessMode", "pLMNidentity", "iE-Extensions"], members: &[M { field: "csg_membership_status", ty: F::Named(43), optional: false }, M { field: "csg_id", ty: F::Named(39), optional: false }, M { field: "cell_access_mode", ty: F::Named(65), optional: true }, M { field: "plmn_identity", ty: F::Named(423), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 43 */ T { name: "CSGMembershipStatus", kind: K::Enumerated { names: &["member", "not-member"], root: 2, extensible: false } },
/* 44 */ T { name: "CancelledCellinEAI", kind: K::Alias { field: F::List(&F::Named(45), 1, 65535), form: None, secret: false } },
/* 45 */ T { name: "CancelledCellinEAI-Item", kind: K::Sequence { names: &["eCGI", "numberOfBroadcasts", "iE-Extensions"], members: &[M { field: "ecgi", ty: F::Named(205), optional: false }, M { field: "number_of_broadcasts", ty: F::Named(406), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 46 */ T { name: "CancelledCellinTAI", kind: K::Alias { field: F::List(&F::Named(47), 1, 65535), form: None, secret: false } },
/* 47 */ T { name: "CancelledCellinTAI-Item", kind: K::Sequence { names: &["eCGI", "numberOfBroadcasts", "iE-Extensions"], members: &[M { field: "ecgi", ty: F::Named(205), optional: false }, M { field: "number_of_broadcasts", ty: F::Named(406), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 48 */ T { name: "Cause", kind: K::Choice { names: &["radioNetwork", "transport", "nas", "protocol", "misc"], alternatives: &[F::Named(52), F::Named(53), F::Named(50), F::Named(51), F::Named(49)], root: 5, extensible: true } },
/* 49 */ T { name: "CauseMisc", kind: K::Enumerated { names: &["control-processing-overload", "not-enough-user-plane-processing-resources", "hardware-failure", "om-intervention", "unspecified", "unknown-PLMN"], root: 6, extensible: true } },
/* 50 */ T { name: "CauseNas", kind: K::Enumerated { names: &["normal-release", "authentication-failure", "detach", "unspecified", "csg-subscription-expiry", "uE-not-in-PLMN-serving-area", "iab-not-authorized"], root: 4, extensible: true } },
/* 51 */ T { name: "CauseProtocol", kind: K::Enumerated { names: &["transfer-syntax-error", "abstract-syntax-error-reject", "abstract-syntax-error-ignore-and-notify", "message-not-compatible-with-receiver-state", "semantic-error", "abstract-syntax-error-falsely-constructed-message", "unspecified"], root: 7, extensible: true } },
/* 52 */ T { name: "CauseRadioNetwork", kind: K::Enumerated { names: &["unspecified", "tx2relocoverall-expiry", "successful-handover", "release-due-to-eutran-generated-reason", "handover-cancelled", "partial-handover", "ho-failure-in-target-EPC-eNB-or-target-system", "ho-target-not-allowed", "tS1relocoverall-expiry", "tS1relocprep-expiry", "cell-not-available", "unknown-targetID", "no-radio-resources-available-in-target-cell", "unknown-mme-ue-s1ap-id", "unknown-enb-ue-s1ap-id", "unknown-pair-ue-s1ap-id", "handover-desirable-for-radio-reason", "time-critical-handover", "resource-optimisation-handover", "reduce-load-in-serving-cell", "user-inactivity", "radio-connection-with-ue-lost", "load-balancing-tau-required", "cs-fallback-triggered", "ue-not-available-for-ps-service", "radio-resources-not-available", "failure-in-radio-interface-procedure", "invalid-qos-combination", "interrat-redirection", "interaction-with-other-procedure", "unknown-E-RAB-ID", "multiple-E-RAB-ID-instances", "encryption-and-or-integrity-protection-algorithms-not-supported", "s1-intra-system-handover-triggered", "s1-inter-system-handover-triggered", "x2-handover-triggered", "redirection-towards-1xRTT", "not-supported-QCI-value", "invalid-CSG-Id", "release-due-to-pre-emption", "n26-interface-not-available", "insufficient-ue-capabilities", "maximum-bearer-pre-emption-rate-exceeded", "up-integrity-protection-not-possible", "release-due-to-discontinuous-coverage"], root: 36, extensible: true } },
/* 53 */ T { name: "CauseTransport", kind: K::Enumerated { names: &["transport-resource-unavailable", "unspecified"], root: 2, extensible: true } },
/* 54 */ T { name: "Cdma2000HORequiredIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 55 */ T { name: "Cdma2000HOStatus", kind: K::Enumerated { names: &["hOSuccess", "hOFailure"], root: 2, extensible: true } },
/* 56 */ T { name: "Cdma2000OneXMEID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 57 */ T { name: "Cdma2000OneXMSI", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 58 */ T { name: "Cdma2000OneXPilot", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 59 */ T { name: "Cdma2000OneXRAND", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 60 */ T { name: "Cdma2000OneXSRVCCInfo", kind: K::Sequence { names: &["cdma2000OneXMEID", "cdma2000OneXMSI", "cdma2000OneXPilot", "iE-Extensions"], members: &[M { field: "cdma2000_one_xmeid", ty: F::Named(56), optional: false }, M { field: "cdma2000_one_xmsi", ty: F::Named(57), optional: false }, M { field: "cdma2000_one_x_pilot", ty: F::Named(58), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 61 */ T { name: "Cdma2000PDU", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 62 */ T { name: "Cdma2000RATType", kind: K::Enumerated { names: &["hRPD", "onexRTT"], root: 2, extensible: true } },
/* 63 */ T { name: "Cdma2000SectorID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 64 */ T { name: "Cell-Size", kind: K::Enumerated { names: &["verysmall", "small", "medium", "large"], root: 4, extensible: true } },
/* 65 */ T { name: "CellAccessMode", kind: K::Enumerated { names: &["hybrid"], root: 1, extensible: true } },
/* 66 */ T { name: "CellBasedMDT", kind: K::Sequence { names: &["cellIdListforMDT", "iE-Extensions"], members: &[M { field: "cell_id_listfor_mdt", ty: F::Named(72), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 67 */ T { name: "CellBasedQMC", kind: K::Sequence { names: &["cellIdListforQMC", "iE-Extensions"], members: &[M { field: "cell_id_listfor_qmc", ty: F::Named(73), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 68 */ T { name: "CellID-Broadcast", kind: K::Alias { field: F::List(&F::Named(69), 1, 65535), form: None, secret: false } },
/* 69 */ T { name: "CellID-Broadcast-Item", kind: K::Sequence { names: &["eCGI", "iE-Extensions"], members: &[M { field: "ecgi", ty: F::Named(205), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 70 */ T { name: "CellID-Cancelled", kind: K::Alias { field: F::List(&F::Named(71), 1, 65535), form: None, secret: false } },
/* 71 */ T { name: "CellID-Cancelled-Item", kind: K::Sequence { names: &["eCGI", "numberOfBroadcasts", "iE-Extensions"], members: &[M { field: "ecgi", ty: F::Named(205), optional: false }, M { field: "number_of_broadcasts", ty: F::Named(406), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 72 */ T { name: "CellIdListforMDT", kind: K::Alias { field: F::List(&F::Named(205), 1, 32), form: None, secret: false } },
/* 73 */ T { name: "CellIdListforQMC", kind: K::Alias { field: F::List(&F::Named(205), 1, 32), form: None, secret: false } },
/* 74 */ T { name: "CellIdentifierAndCELevelForCECapableUEs", kind: K::Sequence { names: &["global-Cell-ID", "cELevel", "iE-Extensions"], members: &[M { field: "global_cell_id", ty: F::Named(205), optional: false }, M { field: "ce_level", ty: F::Named(28), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 75 */ T { name: "CellIdentity", kind: K::Alias { field: F::Bits(28, 28), form: Some(Form::Number(28)), secret: false } },
/* 76 */ T { name: "CellTrafficTrace", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 77 */ T { name: "CellType", kind: K::Sequence { names: &["cell-Size", "iE-Extensions"], members: &[M { field: "cell_size", ty: F::Named(64), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 78 */ T { name: "CoarseUELocation", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 79 */ T { name: "CoarseUELocationRequested", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 80 */ T { name: "CompletedCellinEAI", kind: K::Alias { field: F::List(&F::Named(81), 1, 65535), form: None, secret: false } },
/* 81 */ T { name: "CompletedCellinEAI-Item", kind: K::Sequence { names: &["eCGI", "iE-Extensions"], members: &[M { field: "ecgi", ty: F::Named(205), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 82 */ T { name: "CompletedCellinTAI", kind: K::Alias { field: F::List(&F::Named(83), 1, 65535), form: None, secret: false } },
/* 83 */ T { name: "CompletedCellinTAI-Item", kind: K::Sequence { names: &["eCGI", "iE-Extensions"], members: &[M { field: "ecgi", ty: F::Named(205), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 84 */ T { name: "ConcurrentWarningMessageIndicator", kind: K::Enumerated { names: &["true"], root: 1, extensible: false } },
/* 85 */ T { name: "ConnectedengNBItem", kind: K::Sequence { names: &["en-gNB-ID", "supportedTAs", "iE-Extensions"], members: &[M { field: "en_gnb_id", ty: F::Named(215), optional: false }, M { field: "supported_tas", ty: F::Named(564), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 86 */ T { name: "ConnectedengNBList", kind: K::Alias { field: F::List(&F::Named(85), 1, 256), form: None, secret: false } },
/* 87 */ T { name: "ConnectionEstablishmentIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 88 */ T { name: "ContextatSource", kind: K::Sequence { names: &["sourceNG-RAN-node-ID", "rAN-UE-NGAP-ID", "iE-Extensions"], members: &[M { field: "source_ng_ran_node_id", ty: F::Named(256), optional: false }, M { field: "ran_ue_ngap_id", ty: F::Named(468), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 89 */ T { name: "Correlation-ID", kind: K::Alias { field: F::OctetArray(4), form: None, secret: false } },
/* 90 */ T { name: "Coverage-Level", kind: K::Enumerated { names: &["extendedcoverage"], root: 1, extensible: true } },
/* 91 */ T { name: "Criticality", kind: K::Enumerated { names: &["reject", "ignore", "notify"], root: 3, extensible: false } },
/* 92 */ T { name: "CriticalityDiagnostics", kind: K::Sequence { names: &["procedureCode", "triggeringMessage", "procedureCriticality", "iEsCriticalityDiagnostics", "iE-Extensions"], members: &[M { field: "procedure_code", ty: F::Named(458), optional: true }, M { field: "triggering_message", ty: F::Named(614), optional: true }, M { field: "procedure_criticality", ty: F::Named(91), optional: true }, M { field: "ies_criticality_diagnostics", ty: F::Named(94), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 93 */ T { name: "CriticalityDiagnostics-IE-Item", kind: K::Sequence { names: &["iECriticality", "iE-ID", "typeOfError", "iE-Extensions"], members: &[M { field: "ie_criticality", ty: F::Named(91), optional: false }, M { field: "ie_id", ty: F::Named(464), optional: false }, M { field: "type_of_error", ty: F::Named(616), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 94 */ T { name: "CriticalityDiagnostics-IE-List", kind: K::Alias { field: F::List(&F::Named(93), 1, 256), form: None, secret: false } },
/* 95 */ T { name: "DAPSRequestInfo", kind: K::Sequence { names: &["dAPSIndicator", "iE-Extensions"], members: &[M { field: "daps_indicator", ty: F::Named(96), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 96 */ T { name: "DAPSRequestInfoDAPSIndicator", kind: K::Enumerated { names: &["dAPS-HO-required"], root: 1, extensible: true } },
/* 97 */ T { name: "DAPSResponseInfo", kind: K::Sequence { names: &["dapsresponseindicator", "iE-Extensions"], members: &[M { field: "dapsresponseindicator", ty: F::Named(98), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 98 */ T { name: "DAPSResponseInfoDapsresponseindicator", kind: K::Enumerated { names: &["dAPS-HO-accepted", "dAPS-HO-not-accepted"], root: 2, extensible: true } },
/* 99 */ T { name: "DAPSResponseInfoItem", kind: K::Sequence { names: &["e-RAB-ID", "dAPSResponseInfo", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "daps_response_info", ty: F::Named(97), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 100 */ T { name: "DAPSResponseInfoList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 101 */ T { name: "DCN-ID", kind: K::Alias { field: F::Unsigned(0, 65535, false), form: None, secret: false } },
/* 102 */ T { name: "DL-CP-SecurityInformation", kind: K::Sequence { names: &["dl-NAS-MAC", "iE-Extensions"], members: &[M { field: "dl_nas_mac", ty: F::Named(104), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 103 */ T { name: "DL-Forwarding", kind: K::Enumerated { names: &["dL-Forwarding-proposed"], root: 1, extensible: true } },
/* 104 */ T { name: "DL-NAS-MAC", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 105 */ T { name: "DLCOUNT-PDCP-SNlength", kind: K::Choice { names: &["dLCOUNTValuePDCP-SNlength12", "dLCOUNTValuePDCP-SNlength15", "dLCOUNTValuePDCP-SNlength18"], alternatives: &[F::Named(36), F::Named(35), F::Named(37)], root: 3, extensible: true } },
/* 106 */ T { name: "DLDiscarding", kind: K::Choice { names: &["discardDLCOUNTValuePDCP-SNlength12", "discardDLCOUNTValuePDCP-SNlength15", "discardDLCOUNTValuePDCP-SNlength18"], alternatives: &[F::Named(36), F::Named(35), F::Named(37)], root: 3, extensible: true } },
/* 107 */ T { name: "DLNASPDUDeliveryAckRequest", kind: K::Enumerated { names: &["requested"], root: 1, extensible: true } },
/* 108 */ T { name: "Data-Forwarding-Not-Possible", kind: K::Enumerated { names: &["data-Forwarding-not-Possible"], root: 1, extensible: true } },
/* 109 */ T { name: "DataCodingScheme", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 110 */ T { name: "DataSize", kind: K::Alias { field: F::Unsigned(1, 4095, true), form: None, secret: false } },
/* 111 */ T { name: "DeactivateTrace", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 112 */ T { name: "Direct-Forwarding-Path-Availability", kind: K::Enumerated { names: &["directPathAvailable"], root: 1, extensible: true } },
/* 113 */ T { name: "DownlinkNASTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 114 */ T { name: "DownlinkNonUEAssociatedLPPaTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 115 */ T { name: "DownlinkS1cdma2000tunnelling", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 116 */ T { name: "DownlinkUEAssociatedLPPaTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 117 */ T { name: "E-RAB-ID", kind: K::Alias { field: F::Unsigned(0, 15, true), form: None, secret: false } },
/* 118 */ T { name: "E-RABAdmittedItem", kind: K::Sequence { names: &["e-RAB-ID", "transportLayerAddress", "gTP-TEID", "dL-transportLayerAddress", "dL-gTP-TEID", "uL-TransportLayerAddress", "uL-GTP-TEID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "gtp_teid", ty: F::Named(249), optional: false }, M { field: "dl_transport_layer_address", ty: F::Named(613), optional: true }, M { field: "dl_gtp_teid", ty: F::Named(249), optional: true }, M { field: "ul_transport_layer_address", ty: F::Named(613), optional: true }, M { field: "ul_gtp_teid", ty: F::Named(249), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 119 */ T { name: "E-RABAdmittedList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 120 */ T { name: "E-RABDataForwardingItem", kind: K::Sequence { names: &["e-RAB-ID", "dL-transportLayerAddress", "dL-gTP-TEID", "uL-TransportLayerAddress", "uL-GTP-TEID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "dl_transport_layer_address", ty: F::Named(613), optional: true }, M { field: "dl_gtp_teid", ty: F::Named(249), optional: true }, M { field: "ul_transport_layer_address", ty: F::Named(613), optional: true }, M { field: "ul_gtp_teid", ty: F::Named(249), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 121 */ T { name: "E-RABFailedToResumeItemResumeReq", kind: K::Sequence { names: &["e-RAB-ID", "cause", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "cause", ty: F::Named(48), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 122 */ T { name: "E-RABFailedToResumeItemResumeRes", kind: K::Sequence { names: &["e-RAB-ID", "cause", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "cause", ty: F::Named(48), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 123 */ T { name: "E-RABFailedToResumeListResumeReq", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 124 */ T { name: "E-RABFailedToResumeListResumeRes", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 125 */ T { name: "E-RABFailedToSetupItemHOReqAck", kind: K::Sequence { names: &["e-RAB-ID", "cause", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "cause", ty: F::Named(48), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 126 */ T { name: "E-RABFailedtoSetupListHOReqAck", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 127 */ T { name: "E-RABInformationList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 128 */ T { name: "E-RABInformationListItem", kind: K::Sequence { names: &["e-RAB-ID", "dL-Forwarding", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "dl_forwarding", ty: F::Named(103), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 129 */ T { name: "E-RABItem", kind: K::Sequence { names: &["e-RAB-ID", "cause", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "cause", ty: F::Named(48), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 130 */ T { name: "E-RABLevelQoSParameters", kind: K::Sequence { names: &["qCI", "allocationRetentionPriority", "gbrQosInformation", "iE-Extensions"], members: &[M { field: "qci", ty: F::Named(465), optional: false }, M { field: "allocation_retention_priority", ty: F::Named(4), optional: false }, M { field: "gbr_qos_information", ty: F::Named(244), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 131 */ T { name: "E-RABList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 132 */ T { name: "E-RABModificationConfirm", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 133 */ T { name: "E-RABModificationIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 134 */ T { name: "E-RABModifyItemBearerModConf", kind: K::Sequence { names: &["e-RAB-ID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 135 */ T { name: "E-RABModifyItemBearerModRes", kind: K::Sequence { names: &["e-RAB-ID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 136 */ T { name: "E-RABModifyListBearerModConf", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 137 */ T { name: "E-RABModifyListBearerModRes", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 138 */ T { name: "E-RABModifyRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 139 */ T { name: "E-RABModifyResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 140 */ T { name: "E-RABNotToBeModifiedItemBearerModInd", kind: K::Sequence { names: &["e-RAB-ID", "transportLayerAddress", "dL-GTP-TEID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "dl_gtp_teid", ty: F::Named(249), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 141 */ T { name: "E-RABNotToBeModifiedListBearerModInd", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 142 */ T { name: "E-RABReleaseCommand", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 143 */ T { name: "E-RABReleaseIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 144 */ T { name: "E-RABReleaseItemBearerRelComp", kind: K::Sequence { names: &["e-RAB-ID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 145 */ T { name: "E-RABReleaseListBearerRelComp", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 146 */ T { name: "E-RABReleaseResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 147 */ T { name: "E-RABSecurityResultItem", kind: K::Sequence { names: &["e-RAB-ID", "securityResult", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "security_result", ty: F::Named(531), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 148 */ T { name: "E-RABSecurityResultList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 149 */ T { name: "E-RABSetupItemBearerSURes", kind: K::Sequence { names: &["e-RAB-ID", "transportLayerAddress", "gTP-TEID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "gtp_teid", ty: F::Named(249), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 150 */ T { name: "E-RABSetupItemCtxtSURes", kind: K::Sequence { names: &["e-RAB-ID", "transportLayerAddress", "gTP-TEID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "gtp_teid", ty: F::Named(249), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 151 */ T { name: "E-RABSetupListBearerSURes", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 152 */ T { name: "E-RABSetupListCtxtSURes", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 153 */ T { name: "E-RABSetupRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 154 */ T { name: "E-RABSetupResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 155 */ T { name: "E-RABSubjecttoDataForwardingList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 156 */ T { name: "E-RABToBeModifiedItemBearerModInd", kind: K::Sequence { names: &["e-RAB-ID", "transportLayerAddress", "dL-GTP-TEID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "dl_gtp_teid", ty: F::Named(249), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 157 */ T { name: "E-RABToBeModifiedItemBearerModReq", kind: K::Sequence { names: &["e-RAB-ID", "e-RABLevelQoSParameters", "nAS-PDU", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "e_rab_level_qos_parameters", ty: F::Named(130), optional: false }, M { field: "nas_pdu", ty: F::Named(382), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 158 */ T { name: "E-RABToBeModifiedListBearerModInd", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 159 */ T { name: "E-RABToBeModifiedListBearerModReq", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 160 */ T { name: "E-RABToBeSetupItemBearerSUReq", kind: K::Sequence { names: &["e-RAB-ID", "e-RABlevelQoSParameters", "transportLayerAddress", "gTP-TEID", "nAS-PDU", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "e_rab_level_qos_parameters", ty: F::Named(130), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "gtp_teid", ty: F::Named(249), optional: false }, M { field: "nas_pdu", ty: F::Named(382), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 161 */ T { name: "E-RABToBeSetupItemCtxtSUReq", kind: K::Sequence { names: &["e-RAB-ID", "e-RABlevelQoSParameters", "transportLayerAddress", "gTP-TEID", "nAS-PDU", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "e_rab_level_qos_parameters", ty: F::Named(130), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "gtp_teid", ty: F::Named(249), optional: false }, M { field: "nas_pdu", ty: F::Named(382), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 162 */ T { name: "E-RABToBeSetupItemHOReq", kind: K::Sequence { names: &["e-RAB-ID", "transportLayerAddress", "gTP-TEID", "e-RABlevelQosParameters", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "gtp_teid", ty: F::Named(249), optional: false }, M { field: "e_rab_level_qos_parameters", ty: F::Named(130), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 163 */ T { name: "E-RABToBeSetupListBearerSUReq", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 164 */ T { name: "E-RABToBeSetupListCtxtSUReq", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 165 */ T { name: "E-RABToBeSetupListHOReq", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 166 */ T { name: "E-RABToBeSwitchedDLItem", kind: K::Sequence { names: &["e-RAB-ID", "transportLayerAddress", "gTP-TEID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "gtp_teid", ty: F::Named(249), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 167 */ T { name: "E-RABToBeSwitchedDLList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 168 */ T { name: "E-RABToBeSwitchedULItem", kind: K::Sequence { names: &["e-RAB-ID", "transportLayerAddress", "gTP-TEID", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "gtp_teid", ty: F::Named(249), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 169 */ T { name: "E-RABToBeSwitchedULList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 170 */ T { name: "E-RABToBeUpdatedItem", kind: K::Sequence { names: &["e-RAB-ID", "securityIndication", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "security_indication", ty: F::Named(529), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 171 */ T { name: "E-RABToBeUpdatedList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 172 */ T { name: "E-RABUsageReportItem", kind: K::Sequence { names: &["startTimestamp", "endTimestamp", "usageCountUL", "usageCountDL", "iE-Extensions"], members: &[M { field: "start_timestamp", ty: F::OctetArray(4), optional: false }, M { field: "end_timestamp", ty: F::OctetArray(4), optional: false }, M { field: "usage_count_ul", ty: F::Unsigned(0, 18446744073709551615, false), optional: false }, M { field: "usage_count_dl", ty: F::Unsigned(0, 18446744073709551615, false), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 173 */ T { name: "E-RABUsageReportList", kind: K::Alias { field: F::List(&F::Named(463), 1, 2), form: None, secret: false } },
/* 174 */ T { name: "E-UTRAN-Trace-ID", kind: K::Alias { field: F::OctetArray(8), form: None, secret: false } },
/* 175 */ T { name: "EARFCN", kind: K::Alias { field: F::Unsigned(0, 262143, true), form: None, secret: false } },
/* 176 */ T { name: "ECGI-List", kind: K::Alias { field: F::List(&F::Named(205), 1, 256), form: None, secret: false } },
/* 177 */ T { name: "ECGIList", kind: K::Alias { field: F::List(&F::Named(205), 1, 65535), form: None, secret: false } },
/* 178 */ T { name: "ECGIListForRestart", kind: K::Alias { field: F::List(&F::Named(205), 1, 256), form: None, secret: false } },
/* 179 */ T { name: "EDT-Session", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 180 */ T { name: "EN-DCSONConfigurationTransfer", kind: K::Sequence { names: &["transfertype", "sONInformation", "x2TNLConfigInfo", "iE-Extensions"], members: &[M { field: "transfertype", ty: F::Named(181), optional: false }, M { field: "son_information", ty: F::Named(515), optional: false }, M { field: "x2_tnl_config_info", ty: F::Named(687), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 181 */ T { name: "EN-DCSONTransferType", kind: K::Choice { names: &["request", "reply"], alternatives: &[F::Named(185), F::Named(184)], root: 2, extensible: true } },
/* 182 */ T { name: "EN-DCSONeNBIdentification", kind: K::Sequence { names: &["globaleNBID", "selectedTAI", "iE-Extensions"], members: &[M { field: "global_enb_id", ty: F::Named(254), optional: false }, M { field: "selected_tai", ty: F::Named(572), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 183 */ T { name: "EN-DCSONengNBIdentification", kind: K::Sequence { names: &["globalengNBID", "selectedTAI", "iE-Extensions"], members: &[M { field: "globalen_gnb_id", ty: F::Named(257), optional: false }, M { field: "selected_tai", ty: F::Named(572), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 184 */ T { name: "EN-DCTransferTypeReply", kind: K::Sequence { names: &["sourceengNB", "targeteNB", "iE-Extensions"], members: &[M { field: "source_en_gnb", ty: F::Named(183), optional: false }, M { field: "target_enb", ty: F::Named(182), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 185 */ T { name: "EN-DCTransferTypeRequest", kind: K::Sequence { names: &["sourceeNB", "targetengNB", "targeteNB", "associatedTAI", "broadcast5GSTAI", "iE-Extensions"], members: &[M { field: "source_enb", ty: F::Named(182), optional: false }, M { field: "target_en_gnb", ty: F::Named(183), optional: false }, M { field: "target_enb", ty: F::Named(182), optional: true }, M { field: "associated_tai", ty: F::Named(572), optional: true }, M { field: "broadcast_5gs_tai", ty: F::Named(235), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 186 */ T { name: "ENB-EarlyStatusTransfer-TransparentContainer", kind: K::Sequence { names: &["bearers-SubjectToEarlyStatusTransferList", "iE-Extensions"], members: &[M { field: "bearers_subject_to_early_status_transfer_list", ty: F::Named(15), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 187 */ T { name: "ENB-ID", kind: K::Choice { names: &["macroENB-ID", "homeENB-ID", "short-macroENB-ID", "long-macroENB-ID"], alternatives: &[F::Bits(20, 20), F::Bits(28, 28), F::Bits(18, 18), F::Bits(21, 21)], root: 2, extensible: true } },
/* 188 */ T { name: "ENB-StatusTransfer-TransparentContainer", kind: K::Sequence { names: &["bearers-SubjectToStatusTransferList", "iE-Extensions"], members: &[M { field: "bearers_subject_to_status_transfer_list", ty: F::Named(17), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 189 */ T { name: "ENB-UE-S1AP-ID", kind: K::Alias { field: F::Unsigned(0, 16777215, false), form: None, secret: false } },
/* 190 */ T { name: "ENBCPRelocationIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 191 */ T { name: "ENBConfigurationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 192 */ T { name: "ENBConfigurationUpdate", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 193 */ T { name: "ENBConfigurationUpdateAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 194 */ T { name: "ENBConfigurationUpdateFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 195 */ T { name: "ENBDirectInformationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 196 */ T { name: "ENBEarlyStatusTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 197 */ T { name: "ENBIndirectX2TransportLayerAddresses", kind: K::Alias { field: F::List(&F::Named(613), 1, 2), form: None, secret: false } },
/* 198 */ T { name: "ENBStatusTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 199 */ T { name: "ENBX2ExtTLA", kind: K::Sequence { names: &["iPsecTLA", "gTPTLAa", "iE-Extensions"], members: &[M { field: "ipsec_tla", ty: F::Named(613), optional: true }, M { field: "gtp_tlaa", ty: F::Named(201), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 200 */ T { name: "ENBX2ExtTLAs", kind: K::Alias { field: F::List(&F::Named(199), 1, 16), form: None, secret: false } },
/* 201 */ T { name: "ENBX2GTPTLAs", kind: K::Alias { field: F::List(&F::Named(613), 1, 16), form: None, secret: false } },
/* 202 */ T { name: "ENBX2TLAs", kind: K::Alias { field: F::List(&F::Named(613), 1, 2), form: None, secret: false } },
/* 203 */ T { name: "ENBname", kind: K::Alias { field: F::Text(Text::Printable, 1, 150, true), form: None, secret: false } },
/* 204 */ T { name: "EPLMNs", kind: K::Alias { field: F::List(&F::Named(423), 1, 15), form: None, secret: false } },
/* 205 */ T { name: "EUTRAN-CGI", kind: K::Sequence { names: &["pLMNidentity", "cell-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "cell_id", ty: F::Named(75), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 206 */ T { name: "EUTRANRoundTripDelayEstimationInfo", kind: K::Alias { field: F::Unsigned(0, 2047, false), form: None, secret: false } },
/* 207 */ T { name: "EmergencyAreaID", kind: K::Alias { field: F::OctetArray(3), form: None, secret: false } },
/* 208 */ T { name: "EmergencyAreaID-Broadcast", kind: K::Alias { field: F::List(&F::Named(209), 1, 65535), form: None, secret: false } },
/* 209 */ T { name: "EmergencyAreaID-Broadcast-Item", kind: K::Sequence { names: &["emergencyAreaID", "completedCellinEAI", "iE-Extensions"], members: &[M { field: "emergency_area_id", ty: F::Named(207), optional: false }, M { field: "completed_cellin_eai", ty: F::Named(80), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 210 */ T { name: "EmergencyAreaID-Cancelled", kind: K::Alias { field: F::List(&F::Named(211), 1, 65535), form: None, secret: false } },
/* 211 */ T { name: "EmergencyAreaID-Cancelled-Item", kind: K::Sequence { names: &["emergencyAreaID", "cancelledCellinEAI", "iE-Extensions"], members: &[M { field: "emergency_area_id", ty: F::Named(207), optional: false }, M { field: "cancelled_cellin_eai", ty: F::Named(44), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 212 */ T { name: "EmergencyAreaIDList", kind: K::Alias { field: F::List(&F::Named(207), 1, 65535), form: None, secret: false } },
/* 213 */ T { name: "EmergencyAreaIDListForRestart", kind: K::Alias { field: F::List(&F::Named(207), 1, 256), form: None, secret: false } },
/* 214 */ T { name: "EmergencyIndicator", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 215 */ T { name: "En-gNB-ID", kind: K::Alias { field: F::BitString(22, 32, true), form: None, secret: false } },
/* 216 */ T { name: "EncryptionAlgorithms", kind: K::Alias { field: F::BitString(16, 16, true), form: None, secret: false } },
/* 217 */ T { name: "EndIndication", kind: K::Enumerated { names: &["no-further-data", "further-data-exists"], root: 2, extensible: true } },
/* 218 */ T { name: "EnhancedCoverageRestricted", kind: K::Enumerated { names: &["restricted"], root: 1, extensible: true } },
/* 219 */ T { name: "ErrorIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 220 */ T { name: "Ethernet-Type", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 221 */ T { name: "EventL1LoggedMDTConfig", kind: K::Sequence { names: &["l1Threshold", "hysteresis", "timeToTrigger", "iE-Extensions"], members: &[M { field: "l1_threshold", ty: F::Named(375), optional: false }, M { field: "hysteresis", ty: F::Named(276), optional: false }, M { field: "time_to_trigger", ty: F::Named(605), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 222 */ T { name: "EventTrigger", kind: K::Choice { names: &["outOfCoverage", "eventL1LoggedMDTConfig", "choice-Extensions"], alternatives: &[F::Named(223), F::Named(221), F::Named(463)], root: 3, extensible: false } },
/* 223 */ T { name: "EventTriggerOutOfCoverage", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 224 */ T { name: "EventType", kind: K::Enumerated { names: &["direct", "change-of-serve-cell", "stop-change-of-serve-cell"], root: 3, extensible: true } },
/* 225 */ T { name: "ExpectedActivityPeriod", kind: K::Alias { field: F::Unsigned(1, 181, true), form: None, secret: false } },
/* 226 */ T { name: "ExpectedHOInterval", kind: K::Enumerated { names: &["sec15", "sec30", "sec60", "sec90", "sec120", "sec180", "long-time"], root: 7, extensible: true } },
/* 227 */ T { name: "ExpectedIdlePeriod", kind: K::Alias { field: F::Unsigned(1, 181, true), form: None, secret: false } },
/* 228 */ T { name: "ExpectedUEActivityBehaviour", kind: K::Sequence { names: &["expectedActivityPeriod", "expectedIdlePeriod", "sourceofUEActivityBehaviourInformation", "iE-Extensions"], members: &[M { field: "expected_activity_period", ty: F::Named(225), optional: true }, M { field: "expected_idle_period", ty: F::Named(227), optional: true }, M { field: "sourceof_ue_activity_behaviour_information", ty: F::Named(552), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 229 */ T { name: "ExpectedUEBehaviour", kind: K::Sequence { names: &["expectedActivity", "expectedHOInterval", "iE-Extensions"], members: &[M { field: "expected_activity", ty: F::Named(228), optional: true }, M { field: "expected_ho_interval", ty: F::Named(226), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 230 */ T { name: "Extended-UEIdentityIndexValue", kind: K::Alias { field: F::Bits(14, 14), form: None, secret: false } },
/* 231 */ T { name: "ExtendedBitRate", kind: K::Alias { field: F::Unsigned(10000000001, 4000000000000, true), form: None, secret: false } },
/* 232 */ T { name: "ExtendedRNC-ID", kind: K::Alias { field: F::Unsigned(4096, 65535, false), form: None, secret: false } },
/* 233 */ T { name: "ExtendedRepetitionPeriod", kind: K::Alias { field: F::Unsigned(4096, 131071, false), form: None, secret: false } },
/* 234 */ T { name: "FiveGSTAC", kind: K::Alias { field: F::OctetArray(3), form: Some(Form::Number(24)), secret: false } },
/* 235 */ T { name: "FiveGSTAI", kind: K::Sequence { names: &["pLMNidentity", "fiveGSTAC", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "five_gs_tac", ty: F::Named(234), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 236 */ T { name: "FiveQI", kind: K::Alias { field: F::Unsigned(0, 255, true), form: None, secret: false } },
/* 237 */ T { name: "ForbiddenInterRATs", kind: K::Enumerated { names: &["all", "geran", "utran", "cdma2000", "geranandutran", "cdma2000andutran"], root: 4, extensible: true } },
/* 238 */ T { name: "ForbiddenLACs", kind: K::Alias { field: F::List(&F::Named(305), 1, 4096), form: None, secret: false } },
/* 239 */ T { name: "ForbiddenLAs", kind: K::Alias { field: F::List(&F::Named(240), 1, 16), form: None, secret: false } },
/* 240 */ T { name: "ForbiddenLAs-Item", kind: K::Sequence { names: &["pLMN-Identity", "forbiddenLACs", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "forbidden_lacs", ty: F::Named(238), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 241 */ T { name: "ForbiddenTACs", kind: K::Alias { field: F::List(&F::Named(570), 1, 4096), form: None, secret: false } },
/* 242 */ T { name: "ForbiddenTAs", kind: K::Alias { field: F::List(&F::Named(243), 1, 16), form: None, secret: false } },
/* 243 */ T { name: "ForbiddenTAs-Item", kind: K::Sequence { names: &["pLMN-Identity", "forbiddenTACs", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "forbidden_tacs", ty: F::Named(241), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 244 */ T { name: "GBR-QosInformation", kind: K::Sequence { names: &["e-RAB-MaximumBitrateDL", "e-RAB-MaximumBitrateUL", "e-RAB-GuaranteedBitrateDL", "e-RAB-GuaranteedBitrateUL", "iE-Extensions"], members: &[M { field: "e_rab_maximum_bitrate_dl", ty: F::Named(18), optional: false }, M { field: "e_rab_maximum_bitrate_ul", ty: F::Named(18), optional: false }, M { field: "e_rab_guaranteed_bitrate_dl", ty: F::Named(18), optional: false }, M { field: "e_rab_guaranteed_bitrate_ul", ty: F::Named(18), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 245 */ T { name: "GERAN-Cell-ID", kind: K::Sequence { names: &["lAI", "rAC", "cI", "iE-Extensions"], members: &[M { field: "lai", ty: F::Named(306), optional: false }, M { field: "rac", ty: F::Named(466), optional: false }, M { field: "ci", ty: F::Named(30), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 246 */ T { name: "GNB", kind: K::Sequence { names: &["global-gNB-ID", "iE-Extensions"], members: &[M { field: "global_gnb_id", ty: F::Named(255), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 247 */ T { name: "GNB-ID", kind: K::Alias { field: F::Bits(22, 32), form: None, secret: false } },
/* 248 */ T { name: "GNB-Identity", kind: K::Choice { names: &["gNB-ID"], alternatives: &[F::Named(247)], root: 1, extensible: true } },
/* 249 */ T { name: "GTP-TEID", kind: K::Alias { field: F::OctetArray(4), form: Some(Form::Number(32)), secret: false } },
/* 250 */ T { name: "GUMMEI", kind: K::Sequence { names: &["pLMN-Identity", "mME-Group-ID", "mME-Code", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "mme_group_id", ty: F::Named(357), optional: false }, M { field: "mme_code", ty: F::Named(356), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 251 */ T { name: "GUMMEIList", kind: K::Alias { field: F::List(&F::Named(250), 1, 256), form: None, secret: false } },
/* 252 */ T { name: "GUMMEIType", kind: K::Enumerated { names: &["native", "mapped", "mappedFrom5G"], root: 2, extensible: true } },
/* 253 */ T { name: "GWContextReleaseIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 254 */ T { name: "Global-ENB-ID", kind: K::Sequence { names: &["pLMNidentity", "eNB-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "enb_id", ty: F::Named(187), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 255 */ T { name: "Global-GNB-ID", kind: K::Sequence { names: &["pLMN-Identity", "gNB-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "gnb_id", ty: F::Named(248), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 256 */ T { name: "Global-RAN-NODE-ID", kind: K::Choice { names: &["gNB", "ng-eNB"], alternatives: &[F::Named(246), F::Named(394)], root: 2, extensible: true } },
/* 257 */ T { name: "Global-en-gNB-ID", kind: K::Sequence { names: &["pLMNidentity", "en-gNB-ID", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "en_gnb_id", ty: F::Named(215), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 258 */ T { name: "HFN", kind: K::Alias { field: F::Unsigned(0, 1048575, false), form: None, secret: false } },
/* 259 */ T { name: "HFNModified", kind: K::Alias { field: F::Unsigned(0, 131071, false), form: None, secret: false } },
/* 260 */ T { name: "HFNforPDCP-SNlength18", kind: K::Alias { field: F::Unsigned(0, 16383, false), form: None, secret: false } },
/* 261 */ T { name: "HandoverCancel", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 262 */ T { name: "HandoverCancelAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 263 */ T { name: "HandoverCommand", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 264 */ T { name: "HandoverFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 265 */ T { name: "HandoverFlag", kind: K::Enumerated { names: &["handoverPreparation"], root: 1, extensible: true } },
/* 266 */ T { name: "HandoverNotify", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 267 */ T { name: "HandoverPreparationFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 268 */ T { name: "HandoverRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 269 */ T { name: "HandoverRequestAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 270 */ T { name: "HandoverRequired", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 271 */ T { name: "HandoverRestrictionList", kind: K::Sequence { names: &["servingPLMN", "equivalentPLMNs", "forbiddenTAs", "forbiddenLAs", "forbiddenInterRATs", "iE-Extensions"], members: &[M { field: "serving_plmn", ty: F::Named(423), optional: false }, M { field: "equivalent_plmns", ty: F::Named(204), optional: true }, M { field: "forbidden_tas", ty: F::Named(242), optional: true }, M { field: "forbidden_las", ty: F::Named(239), optional: true }, M { field: "forbidden_inter_rats", ty: F::Named(237), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 272 */ T { name: "HandoverSuccess", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 273 */ T { name: "HandoverType", kind: K::Enumerated { names: &["intralte", "ltetoutran", "ltetogeran", "utrantolte", "gerantolte", "eps-to-5gs", "fivegs-to-eps"], root: 5, extensible: true } },
/* 274 */ T { name: "HandoverWindowDuration", kind: K::Alias { field: F::Unsigned(1, 6000, false), form: None, secret: false } },
/* 275 */ T { name: "HandoverWindowStart", kind: K::Alias { field: F::Unsigned(0, 1048575, false), form: None, secret: false } },
/* 276 */ T { name: "Hysteresis", kind: K::Alias { field: F::Unsigned(0, 30, false), form: None, secret: false } },
/* 277 */ T { name: "IAB-Authorized", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 278 */ T { name: "IAB-Node-Indication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 279 */ T { name: "IAB-Supported", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 280 */ T { name: "IMSI", kind: K::Alias { field: F::Octets(3, 8, false), form: Some(Form::Digits), secret: false } },
/* 281 */ T { name: "IMSvoiceEPSfallbackfrom5G", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 282 */ T { name: "ImmediateMDT", kind: K::Sequence { names: &["measurementsToActivate", "m1reportingTrigger", "m1thresholdeventA2", "m1periodicReporting", "iE-Extensions"], members: &[M { field: "measurements_to_activate", ty: F::Named(376), optional: false }, M { field: "m1_reporting_trigger", ty: F::Named(331), optional: false }, M { field: "m1_thresholdevent_a2", ty: F::Named(332), optional: true }, M { field: "m1_periodic_reporting", ty: F::Named(330), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 283 */ T { name: "InformationOnRecommendedCellsAndENBsForPaging", kind: K::Sequence { names: &["recommendedCellsForPaging", "recommendENBsForPaging", "iE-Extensions"], members: &[M { field: "recommended_cells_for_paging", ty: F::Named(485), optional: false }, M { field: "recommend_enbs_for_paging", ty: F::Named(488), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 284 */ T { name: "InitialContextSetupFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 285 */ T { name: "InitialContextSetupRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 286 */ T { name: "InitialContextSetupResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 287 */ T { name: "InitialUEMessage", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 288 */ T { name: "InitiatingMessage", kind: K::Sequence { names: &["procedureCode", "criticality", "value"], members: &[M { field: "procedure_code", ty: F::Named(458), optional: false }, M { field: "criticality", ty: F::Named(91), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 289 */ T { name: "IntegrityProtectionAlgorithms", kind: K::Alias { field: F::BitString(16, 16, true), form: None, secret: false } },
/* 290 */ T { name: "IntegrityProtectionIndication", kind: K::Enumerated { names: &["required", "preferred", "not-needed"], root: 3, extensible: true } },
/* 291 */ T { name: "IntegrityProtectionResult", kind: K::Enumerated { names: &["performed", "not-performed"], root: 2, extensible: true } },
/* 292 */ T { name: "IntendedNumberOfPagingAttempts", kind: K::Alias { field: F::Unsigned(1, 16, true), form: None, secret: false } },
/* 293 */ T { name: "Inter-SystemInformationTransferType", kind: K::Choice { names: &["rIMTransfer"], alternatives: &[F::Named(474)], root: 1, extensible: true } },
/* 294 */ T { name: "InterSystemMeasurementItem", kind: K::Sequence { names: &["freqBandIndicatorNR", "sSBfrequencies", "subcarrierSpacingSSB", "maxRSIndexCellQual", "sMTC", "threshRS-Index-r15", "sSBToMeasure", "sSRSSIMeasurement", "quantityConfigNR-R15", "excludedCellsToAddModList", "iE-Extensions"], members: &[M { field: "freq_band_indicator_nr", ty: F::Unsigned(1, 1024, false), optional: false }, M { field: "ssb_frequencies", ty: F::Unsigned(0, 3279165, false), optional: false }, M { field: "subcarrier_spacing_ssb", ty: F::Named(295), optional: false }, M { field: "max_rs_index_cell_qual", ty: F::Unsigned(1, 16, false), optional: true }, M { field: "smtc", ty: F::Octets(0, UNBOUNDED, false), optional: true }, M { field: "thresh_rs_index_r15", ty: F::Octets(0, UNBOUNDED, false), optional: true }, M { field: "ssb_to_measure", ty: F::Octets(0, UNBOUNDED, false), optional: true }, M { field: "ss_rssi_measurement", ty: F::Octets(0, UNBOUNDED, false), optional: true }, M { field: "quantity_config_nr_r15", ty: F::Octets(0, UNBOUNDED, false), optional: true }, M { field: "excluded_cells_to_add_mod_list", ty: F::Octets(0, UNBOUNDED, false), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: false } },
/* 295 */ T { name: "InterSystemMeasurementItemSubcarrierSpacingSSB", kind: K::Enumerated { names: &["kHz15", "kHz30", "kHz60", "kHz120", "kHz240", "kHz480", "kHz960"], root: 5, extensible: true } },
/* 296 */ T { name: "InterSystemMeasurementList", kind: K::Alias { field: F::List(&F::Named(294), 1, 64), form: None, secret: false } },
/* 297 */ T { name: "InterSystemMeasurementParameters", kind: K::Sequence { names: &["measurementDuration", "interSystemMeasurementList", "iE-Extensions"], members: &[M { field: "measurement_duration", ty: F::Unsigned(1, 100, false), optional: false }, M { field: "inter_system_measurement_list", ty: F::Named(296), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 298 */ T { name: "InterfacesToTrace", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 299 */ T { name: "IntersystemMeasurementConfiguration", kind: K::Sequence { names: &["rSRP", "rSRQ", "sINR", "interSystemMeasurementParameters", "iE-Extensions"], members: &[M { field: "rsrp", ty: F::Unsigned(0, 127, false), optional: true }, M { field: "rsrq", ty: F::Unsigned(0, 127, false), optional: true }, M { field: "sinr", ty: F::Unsigned(0, 127, false), optional: true }, M { field: "inter_system_measurement_parameters", ty: F::Named(297), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 300 */ T { name: "IntersystemSONConfigurationTransfer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 301 */ T { name: "KillAllWarningMessages", kind: K::Enumerated { names: &["true"], root: 1, extensible: false } },
/* 302 */ T { name: "KillRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 303 */ T { name: "KillResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 304 */ T { name: "L3-Information", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 305 */ T { name: "LAC", kind: K::Alias { field: F::OctetArray(2), form: Some(Form::Number(16)), secret: false } },
/* 306 */ T { name: "LAI", kind: K::Sequence { names: &["pLMNidentity", "lAC", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "lac", ty: F::Named(305), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 307 */ T { name: "LHN-ID", kind: K::Alias { field: F::Octets(32, 256, false), form: None, secret: false } },
/* 308 */ T { name: "LPPa-PDU", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 309 */ T { name: "LTE-M-Indication", kind: K::Enumerated { names: &["lte-m"], root: 1, extensible: true } },
/* 310 */ T { name: "LTE-NTN-TAI-Information", kind: K::Sequence { names: &["servingPLMN", "tACList-In-LTE-NTN", "uE-Location-Derived-TAC", "iE-Extensions"], members: &[M { field: "serving_plmn", ty: F::Named(423), optional: false }, M { field: "tac_list_in_lte_ntn", ty: F::Named(571), optional: false }, M { field: "ue_location_derived_tac", ty: F::Named(570), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 311 */ T { name: "LastVisitedCell-Item", kind: K::Choice { names: &["e-UTRAN-Cell", "uTRAN-Cell", "gERAN-Cell", "nG-RAN-Cell"], alternatives: &[F::Named(312), F::Named(317), F::Named(313), F::Named(314)], root: 3, extensible: true } },
/* 312 */ T { name: "LastVisitedEUTRANCellInformation", kind: K::Sequence { names: &["global-Cell-ID", "cellType", "time-UE-StayedInCell", "iE-Extensions"], members: &[M { field: "global_cell_id", ty: F::Named(205), optional: false }, M { field: "cell_type", ty: F::Named(77), optional: false }, M { field: "time_ue_stayed_in_cell", ty: F::Named(599), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 313 */ T { name: "LastVisitedGERANCellInformation", kind: K::Choice { names: &["undefined"], alternatives: &[F::Null], root: 1, extensible: true } },
/* 314 */ T { name: "LastVisitedNGRANCellInformation", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 315 */ T { name: "LastVisitedPSCellInformation", kind: K::Sequence { names: &["pSCellID", "timeStay", "iE-Extensions"], members: &[M { field: "ps_cell_id", ty: F::Named(425), optional: true }, M { field: "time_stay", ty: F::Unsigned(0, 40950, false), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 316 */ T { name: "LastVisitedPSCellList", kind: K::Alias { field: F::List(&F::Named(315), 1, 8), form: None, secret: false } },
/* 317 */ T { name: "LastVisitedUTRANCellInformation", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 318 */ T { name: "Links-to-log", kind: K::Enumerated { names: &["uplink", "downlink", "both-uplink-and-downlink"], root: 3, extensible: true } },
/* 319 */ T { name: "ListeningSubframePattern", kind: K::Sequence { names: &["pattern-period", "pattern-offset", "iE-Extensions"], members: &[M { field: "pattern_period", ty: F::Named(320), optional: false }, M { field: "pattern_offset", ty: F::Unsigned(0, 10239, true), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 320 */ T { name: "ListeningSubframePatternPatternPeriod", kind: K::Enumerated { names: &["ms1280", "ms2560", "ms5120", "ms10240"], root: 4, extensible: true } },
/* 321 */ T { name: "LocationReport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 322 */ T { name: "LocationReportingControl", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 323 */ T { name: "LocationReportingFailureIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 324 */ T { name: "LoggedMBSFNMDT", kind: K::Sequence { names: &["loggingInterval", "loggingDuration", "mBSFN-ResultToLog", "iE-Extensions"], members: &[M { field: "logging_interval", ty: F::Named(328), optional: false }, M { field: "logging_duration", ty: F::Named(327), optional: false }, M { field: "mbsfn_result_to_log", ty: F::Named(348), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 325 */ T { name: "LoggedMDT", kind: K::Sequence { names: &["loggingInterval", "loggingDuration", "iE-Extensions"], members: &[M { field: "logging_interval", ty: F::Named(328), optional: false }, M { field: "logging_duration", ty: F::Named(327), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 326 */ T { name: "LoggedMDTTrigger", kind: K::Choice { names: &["periodical", "eventTrigger"], alternatives: &[F::Null, F::Named(222)], root: 2, extensible: true } },
/* 327 */ T { name: "LoggingDuration", kind: K::Enumerated { names: &["m10", "m20", "m40", "m60", "m90", "m120"], root: 6, extensible: false } },
/* 328 */ T { name: "LoggingInterval", kind: K::Enumerated { names: &["ms1280", "ms2560", "ms5120", "ms10240", "ms20480", "ms30720", "ms40960", "ms61440"], root: 8, extensible: false } },
/* 329 */ T { name: "M-TMSI", kind: K::Alias { field: F::OctetArray(4), form: Some(Form::Number(32)), secret: false } },
/* 330 */ T { name: "M1PeriodicReporting", kind: K::Sequence { names: &["reportInterval", "reportAmount", "iE-Extensions"], members: &[M { field: "report_interval", ty: F::Named(494), optional: false }, M { field: "report_amount", ty: F::Named(492), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 331 */ T { name: "M1ReportingTrigger", kind: K::Enumerated { names: &["periodic", "a2eventtriggered", "a2eventtriggered-periodic"], root: 2, extensible: true } },
/* 332 */ T { name: "M1ThresholdEventA2", kind: K::Sequence { names: &["measurementThreshold", "iE-Extensions"], members: &[M { field: "measurement_threshold", ty: F::Named(374), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 333 */ T { name: "M3Configuration", kind: K::Sequence { names: &["m3period", "iE-Extensions"], members: &[M { field: "m3_period", ty: F::Named(334), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 334 */ T { name: "M3period", kind: K::Enumerated { names: &["ms100", "ms1000", "ms10000", "ms1024", "ms1280", "ms2048", "ms2560", "ms5120", "ms10240", "min1"], root: 3, extensible: true } },
/* 335 */ T { name: "M4Configuration", kind: K::Sequence { names: &["m4period", "m4-links-to-log", "iE-Extensions"], members: &[M { field: "m4_period", ty: F::Named(337), optional: false }, M { field: "m4_links_to_log", ty: F::Named(318), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 336 */ T { name: "M4ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"], root: 8, extensible: true } },
/* 337 */ T { name: "M4period", kind: K::Enumerated { names: &["ms1024", "ms2048", "ms5120", "ms10240", "min1"], root: 5, extensible: true } },
/* 338 */ T { name: "M5Configuration", kind: K::Sequence { names: &["m5period", "m5-links-to-log", "iE-Extensions"], members: &[M { field: "m5_period", ty: F::Named(340), optional: false }, M { field: "m5_links_to_log", ty: F::Named(318), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 339 */ T { name: "M5ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"], root: 8, extensible: true } },
/* 340 */ T { name: "M5period", kind: K::Enumerated { names: &["ms1024", "ms2048", "ms5120", "ms10240", "min1"], root: 5, extensible: true } },
/* 341 */ T { name: "M6Configuration", kind: K::Sequence { names: &["m6report-Interval", "m6delay-threshold", "m6-links-to-log", "iE-Extensions"], members: &[M { field: "m6_report_interval", ty: F::Named(344), optional: false }, M { field: "m6_delay_threshold", ty: F::Named(343), optional: true }, M { field: "m6_links_to_log", ty: F::Named(318), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 342 */ T { name: "M6ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"], root: 8, extensible: true } },
/* 343 */ T { name: "M6delay-threshold", kind: K::Enumerated { names: &["ms30", "ms40", "ms50", "ms60", "ms70", "ms80", "ms90", "ms100", "ms150", "ms300", "ms500", "ms750"], root: 12, extensible: true } },
/* 344 */ T { name: "M6report-Interval", kind: K::Enumerated { names: &["ms1024", "ms2048", "ms5120", "ms10240"], root: 4, extensible: true } },
/* 345 */ T { name: "M7Configuration", kind: K::Sequence { names: &["m7period", "m7-links-to-log", "iE-Extensions"], members: &[M { field: "m7_period", ty: F::Named(347), optional: false }, M { field: "m7_links_to_log", ty: F::Named(318), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 346 */ T { name: "M7ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "infinity"], root: 8, extensible: true } },
/* 347 */ T { name: "M7period", kind: K::Alias { field: F::Unsigned(1, 60, true), form: None, secret: false } },
/* 348 */ T { name: "MBSFN-ResultToLog", kind: K::Alias { field: F::List(&F::Named(349), 1, 8), form: None, secret: false } },
/* 349 */ T { name: "MBSFN-ResultToLogInfo", kind: K::Sequence { names: &["mBSFN-AreaId", "carrierFreq", "iE-Extensions"], members: &[M { field: "mbsfn_area_id", ty: F::Unsigned(0, 255, false), optional: true }, M { field: "carrier_freq", ty: F::Named(175), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 350 */ T { name: "MDT-Activation", kind: K::Enumerated { names: &["immediate-MDT-only", "immediate-MDT-and-Trace", "logged-MDT-only", "logged-MBSFN-MDT"], root: 3, extensible: true } },
/* 351 */ T { name: "MDT-Configuration", kind: K::Sequence { names: &["mdt-Activation", "areaScopeOfMDT", "mDTMode", "iE-Extensions"], members: &[M { field: "mdt_activation", ty: F::Named(350), optional: false }, M { field: "area_scope_of_mdt", ty: F::Named(5), optional: false }, M { field: "mdt_mode", ty: F::Named(354), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 352 */ T { name: "MDT-ConfigurationNR", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 353 */ T { name: "MDT-Location-Info", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 354 */ T { name: "MDTMode", kind: K::Choice { names: &["immediateMDT", "loggedMDT", "mDTMode-Extension"], alternatives: &[F::Named(282), F::Named(325), F::Named(463)], root: 2, extensible: true } },
/* 355 */ T { name: "MDTPLMNList", kind: K::Alias { field: F::List(&F::Named(423), 1, 16), form: None, secret: false } },
/* 356 */ T { name: "MME-Code", kind: K::Alias { field: F::OctetArray(1), form: Some(Form::Number(8)), secret: false } },
/* 357 */ T { name: "MME-Group-ID", kind: K::Alias { field: F::OctetArray(2), form: Some(Form::Number(16)), secret: false } },
/* 358 */ T { name: "MME-UE-S1AP-ID", kind: K::Alias { field: F::Unsigned(0, 4294967295, false), form: None, secret: false } },
/* 359 */ T { name: "MMECPRelocationIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 360 */ T { name: "MMEConfigurationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 361 */ T { name: "MMEConfigurationUpdate", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 362 */ T { name: "MMEConfigurationUpdateAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 363 */ T { name: "MMEConfigurationUpdateFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 364 */ T { name: "MMEDirectInformationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 365 */ T { name: "MMEEarlyStatusTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 366 */ T { name: "MMEPagingTarget", kind: K::Choice { names: &["global-ENB-ID", "tAI"], alternatives: &[F::Named(254), F::Named(572)], root: 2, extensible: true } },
/* 367 */ T { name: "MMERelaySupportIndicator", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 368 */ T { name: "MMEStatusTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 369 */ T { name: "MMEname", kind: K::Alias { field: F::Text(Text::Printable, 1, 150, true), form: None, secret: false } },
/* 370 */ T { name: "MSClassmark2", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 371 */ T { name: "MSClassmark3", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 372 */ T { name: "ManagementBasedMDTAllowed", kind: K::Enumerated { names: &["allowed"], root: 1, extensible: true } },
/* 373 */ T { name: "Masked-IMEISV", kind: K::Alias { field: F::Bits(64, 64), form: None, secret: false } },
/* 374 */ T { name: "MeasurementThresholdA2", kind: K::Choice { names: &["threshold-RSRP", "threshold-RSRQ"], alternatives: &[F::Named(597), F::Named(598)], root: 2, extensible: true } },
/* 375 */ T { name: "MeasurementThresholdL1LoggedMDT", kind: K::Choice { names: &["threshold-RSRP", "threshold-RSRQ", "choice-Extensions"], alternatives: &[F::Named(597), F::Named(598), F::Named(463)], root: 3, extensible: false } },
/* 376 */ T { name: "MeasurementsToActivate", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 377 */ T { name: "MessageIdentifier", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 378 */ T { name: "MobilityInformation", kind: K::Alias { field: F::Bits(32, 32), form: None, secret: false } },
/* 379 */ T { name: "MutingAvailabilityIndication", kind: K::Enumerated { names: &["available", "unavailable"], root: 2, extensible: true } },
/* 380 */ T { name: "MutingPatternInformation", kind: K::Sequence { names: &["muting-pattern-period", "muting-pattern-offset", "iE-Extensions"], members: &[M { field: "muting_pattern_period", ty: F::Named(381), optional: false }, M { field: "muting_pattern_offset", ty: F::Unsigned(0, 10239, true), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 381 */ T { name: "MutingPatternInformationMutingPatternPeriod", kind: K::Enumerated { names: &["ms0", "ms1280", "ms2560", "ms5120", "ms10240"], root: 5, extensible: true } },
/* 382 */ T { name: "NAS-PDU", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 383 */ T { name: "NASDeliveryIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 384 */ T { name: "NASNonDeliveryIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 385 */ T { name: "NASSecurityParametersfromE-UTRAN", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 386 */ T { name: "NASSecurityParameterstoE-UTRAN", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 387 */ T { name: "NB-IoT-DefaultPagingDRX", kind: K::Enumerated { names: &["v128", "v256", "v512", "v1024"], root: 4, extensible: true } },
/* 388 */ T { name: "NB-IoT-Paging-eDRX-Cycle", kind: K::Enumerated { names: &["hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128", "hf256", "hf512", "hf1024"], root: 14, extensible: true } },
/* 389 */ T { name: "NB-IoT-Paging-eDRXInformation", kind: K::Sequence { names: &["nB-IoT-paging-eDRX-Cycle", "nB-IoT-pagingTimeWindow", "iE-Extensions"], members: &[M { field: "nb_iot_paging_edrx_cycle", ty: F::Named(388), optional: false }, M { field: "nb_iot_paging_time_window", ty: F::Named(391), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 390 */ T { name: "NB-IoT-PagingDRX", kind: K::Enumerated { names: &["v32", "v64", "v128", "v256", "v512", "v1024"], root: 6, extensible: true } },
/* 391 */ T { name: "NB-IoT-PagingTimeWindow", kind: K::Enumerated { names: &["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16"], root: 16, extensible: true } },
/* 392 */ T { name: "NB-IoT-RLF-Report-Container", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 393 */ T { name: "NB-IoT-UEIdentityIndexValue", kind: K::Alias { field: F::Bits(12, 12), form: None, secret: false } },
/* 394 */ T { name: "NG-eNB", kind: K::Sequence { names: &["global-ng-eNB-ID", "iE-Extensions"], members: &[M { field: "global_ng_enb_id", ty: F::Named(254), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 395 */ T { name: "NR-CGI", kind: K::Sequence { names: &["pLMNIdentity", "nRCellIdentity", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "nr_cell_identity", ty: F::Named(396), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 396 */ T { name: "NRCellIdentity", kind: K::Alias { field: F::Bits(36, 36), form: Some(Form::Number(36)), secret: false } },
/* 397 */ T { name: "NRUESecurityCapabilities", kind: K::Sequence { names: &["nRencryptionAlgorithms", "nRintegrityProtectionAlgorithms", "iE-Extensions"], members: &[M { field: "nr_encryption_algorithms", ty: F::Named(400), optional: false }, M { field: "nr_integrity_protection_algorithms", ty: F::Named(401), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 398 */ T { name: "NRUESidelinkAggregateMaximumBitrate", kind: K::Sequence { names: &["uEaggregateMaximumBitRate", "iE-Extensions"], members: &[M { field: "ue_aggregate_maximum_bit_rate", ty: F::Named(18), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 399 */ T { name: "NRV2XServicesAuthorized", kind: K::Sequence { names: &["vehicleUE", "pedestrianUE", "iE-Extensions"], members: &[M { field: "vehicle_ue", ty: F::Named(671), optional: true }, M { field: "pedestrian_ue", ty: F::Named(443), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 400 */ T { name: "NRencryptionAlgorithms", kind: K::Alias { field: F::BitString(16, 16, true), form: None, secret: false } },
/* 401 */ T { name: "NRintegrityProtectionAlgorithms", kind: K::Alias { field: F::BitString(16, 16, true), form: None, secret: false } },
/* 402 */ T { name: "NRrestrictionin5GS", kind: K::Enumerated { names: &["nRrestrictedin5GS"], root: 1, extensible: true } },
/* 403 */ T { name: "NRrestrictioninEPSasSecondaryRAT", kind: K::Enumerated { names: &["nRrestrictedinEPSasSecondaryRAT"], root: 1, extensible: true } },
/* 404 */ T { name: "NextPagingAreaScope", kind: K::Enumerated { names: &["same", "changed"], root: 2, extensible: true } },
/* 405 */ T { name: "NotifySourceeNB", kind: K::Enumerated { names: &["notifySource"], root: 1, extensible: true } },
/* 406 */ T { name: "NumberOfBroadcasts", kind: K::Alias { field: F::Unsigned(0, 65535, false), form: None, secret: false } },
/* 407 */ T { name: "NumberofBroadcastRequest", kind: K::Alias { field: F::Unsigned(0, 65535, false), form: None, secret: false } },
/* 408 */ T { name: "OCTET STRING", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 409 */ T { name: "OldBSS-ToNewBSS-Information", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 410 */ T { name: "OverloadAction", kind: K::Enumerated { names: &["reject-non-emergency-mo-dt", "reject-rrc-cr-signalling", "permit-emergency-sessions-and-mobile-terminated-services-only", "permit-high-priority-sessions-and-mobile-terminated-services-only", "reject-delay-tolerant-access", "permit-high-priority-sessions-and-exception-reporting-and-mobile-terminated-services-only", "not-accept-mo-data-or-delay-tolerant-access-from-CP-CIoT"], root: 3, extensible: true } },
/* 411 */ T { name: "OverloadResponse", kind: K::Choice { names: &["overloadAction"], alternatives: &[F::Named(410)], root: 1, extensible: true } },
/* 412 */ T { name: "OverloadStart", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 413 */ T { name: "OverloadStop", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 414 */ T { name: "PC5FlowBitRates", kind: K::Sequence { names: &["guaranteedFlowBitRate", "maximumFlowBitRate", "iE-Extensions"], members: &[M { field: "guaranteed_flow_bit_rate", ty: F::Named(18), optional: false }, M { field: "maximum_flow_bit_rate", ty: F::Named(18), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 415 */ T { name: "PC5QoSFlowItem", kind: K::Sequence { names: &["pQI", "pc5FlowBitRates", "range", "iE-Extensions"], members: &[M { field: "pqi", ty: F::Named(236), optional: false }, M { field: "pc5_flow_bit_rates", ty: F::Named(414), optional: true }, M { field: "range", ty: F::Named(479), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 416 */ T { name: "PC5QoSFlowList", kind: K::Alias { field: F::List(&F::Named(415), 1, 2048), form: None, secret: false } },
/* 417 */ T { name: "PC5QoSParameters", kind: K::Sequence { names: &["pc5QoSFlowList", "pc5LinkAggregatedBitRates", "iE-Extensions"], members: &[M { field: "pc5_qos_flow_list", ty: F::Named(416), optional: false }, M { field: "pc5_link_aggregated_bit_rates", ty: F::Named(18), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 418 */ T { name: "PDCP-SN", kind: K::Alias { field: F::Unsigned(0, 4095, false), form: None, secret: false } },
/* 419 */ T { name: "PDCP-SNExtended", kind: K::Alias { field: F::Unsigned(0, 32767, false), form: None, secret: false } },
/* 420 */ T { name: "PDCP-SNlength18", kind: K::Alias { field: F::Unsigned(0, 262143, false), form: None, secret: false } },
/* 421 */ T { name: "PLMNAreaBasedQMC", kind: K::Sequence { names: &["plmnListforQMC", "iE-Extensions"], members: &[M { field: "plmn_listfor_qmc", ty: F::Named(422), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 422 */ T { name: "PLMNListforQMC", kind: K::Alias { field: F::List(&F::Named(423), 1, 16), form: None, secret: false } },
/* 423 */ T { name: "PLMNidentity", kind: K::Alias { field: F::Named(587), form: Some(Form::Plmn), secret: false } },
/* 424 */ T { name: "PS-ServiceNotAvailable", kind: K::Enumerated { names: &["ps-service-not-available"], root: 1, extensible: true } },
/* 425 */ T { name: "PSCellInformation", kind: K::Sequence { names: &["nCGI", "iE-Extensions"], members: &[M { field: "ncgi", ty: F::Named(395), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 426 */ T { name: "PWSFailureIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 427 */ T { name: "PWSRestartIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 428 */ T { name: "PWSfailedECGIList", kind: K::Alias { field: F::List(&F::Named(205), 1, 256), form: None, secret: false } },
/* 429 */ T { name: "Packet-LossRate", kind: K::Alias { field: F::Unsigned(0, 1000, false), form: None, secret: false } },
/* 430 */ T { name: "Paging", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 431 */ T { name: "Paging-eDRX-Cycle", kind: K::Enumerated { names: &["hfhalf", "hf1", "hf2", "hf4", "hf6", "hf8", "hf10", "hf12", "hf14", "hf16", "hf32", "hf64", "hf128", "hf256"], root: 14, extensible: true } },
/* 432 */ T { name: "Paging-eDRXInformation", kind: K::Sequence { names: &["paging-eDRX-Cycle", "pagingTimeWindow", "iE-Extensions"], members: &[M { field: "paging_edrx_cycle", ty: F::Named(431), optional: false }, M { field: "paging_time_window", ty: F::Named(439), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 433 */ T { name: "PagingAttemptCount", kind: K::Alias { field: F::Unsigned(1, 16, true), form: None, secret: false } },
/* 434 */ T { name: "PagingAttemptInformation", kind: K::Sequence { names: &["pagingAttemptCount", "intendedNumberOfPagingAttempts", "nextPagingAreaScope", "iE-Extensions"], members: &[M { field: "paging_attempt_count", ty: F::Named(433), optional: false }, M { field: "intended_number_of_paging_attempts", ty: F::Named(292), optional: false }, M { field: "next_paging_area_scope", ty: F::Named(404), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 435 */ T { name: "PagingCause", kind: K::Enumerated { names: &["voice"], root: 1, extensible: true } },
/* 436 */ T { name: "PagingDRX", kind: K::Enumerated { names: &["v32", "v64", "v128", "v256"], root: 4, extensible: true } },
/* 437 */ T { name: "PagingPriority", kind: K::Enumerated { names: &["priolevel1", "priolevel2", "priolevel3", "priolevel4", "priolevel5", "priolevel6", "priolevel7", "priolevel8"], root: 8, extensible: true } },
/* 438 */ T { name: "PagingProbabilityInformation", kind: K::Enumerated { names: &["p00", "p05", "p10", "p15", "p20", "p25", "p30", "p35", "p40", "p45", "p50", "p55", "p60", "p65", "p70", "p75", "p80", "p85", "p90", "p95", "p100"], root: 21, extensible: true } },
/* 439 */ T { name: "PagingTimeWindow", kind: K::Enumerated { names: &["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13", "s14", "s15", "s16"], root: 16, extensible: true } },
/* 440 */ T { name: "PathSwitchRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 441 */ T { name: "PathSwitchRequestAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 442 */ T { name: "PathSwitchRequestFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 443 */ T { name: "PedestrianUE", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 444 */ T { name: "PendingDataIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 445 */ T { name: "Port-Number", kind: K::Alias { field: F::OctetArray(2), form: Some(Form::Number(16)), secret: false } },
/* 446 */ T { name: "Pre-emptionCapability", kind: K::Enumerated { names: &["shall-not-trigger-pre-emption", "may-trigger-pre-emption"], root: 2, extensible: false } },
/* 447 */ T { name: "Pre-emptionVulnerability", kind: K::Enumerated { names: &["not-pre-emptable", "pre-emptable"], root: 2, extensible: false } },
/* 448 */ T { name: "Presence", kind: K::Enumerated { names: &["optional", "conditional", "mandatory"], root: 3, extensible: false } },
/* 449 */ T { name: "PriorityLevel", kind: K::Alias { field: F::Unsigned(0, 15, false), form: None, secret: false } },
/* 450 */ T { name: "PrivacyIndicator", kind: K::Enumerated { names: &["immediate-MDT", "logged-MDT"], root: 2, extensible: true } },
/* 451 */ T { name: "PrivateIE-ID", kind: K::Choice { names: &["local", "global"], alternatives: &[F::Unsigned(0, 65535, false), F::ObjectIdentifier], root: 2, extensible: false } },
/* 452 */ T { name: "PrivateMessage", kind: K::Sequence { names: &["privateIEs"], members: &[M { field: "private_ies", ty: F::Named(453), optional: false }], extensible: true } },
/* 453 */ T { name: "PrivateMessagePrivateIEs", kind: K::Alias { field: F::List(&F::Named(513), 1, 65535), form: None, secret: false } },
/* 454 */ T { name: "ProSeAuthorized", kind: K::Sequence { names: &["proSeDirectDiscovery", "proSeDirectCommunication", "iE-Extensions"], members: &[M { field: "prose_direct_discovery", ty: F::Named(456), optional: true }, M { field: "prose_direct_communication", ty: F::Named(455), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 455 */ T { name: "ProSeDirectCommunication", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 456 */ T { name: "ProSeDirectDiscovery", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 457 */ T { name: "ProSeUEtoNetworkRelaying", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 458 */ T { name: "ProcedureCode", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 459 */ T { name: "ProtocolExtensionContainer", kind: K::Alias { field: F::List(&F::Named(460), 1, 65535), form: None, secret: false } },
/* 460 */ T { name: "ProtocolExtensionField", kind: K::Sequence { names: &["id", "criticality", "extensionValue"], members: &[M { field: "id", ty: F::Named(461), optional: false }, M { field: "criticality", ty: F::Named(91), optional: false }, M { field: "extension_value", ty: F::Open, optional: false }], extensible: false } },
/* 461 */ T { name: "ProtocolExtensionID", kind: K::Alias { field: F::Unsigned(0, 65535, false), form: None, secret: false } },
/* 462 */ T { name: "ProtocolIE-Container", kind: K::Alias { field: F::List(&F::Named(463), 0, 65535), form: None, secret: false } },
/* 463 */ T { name: "ProtocolIE-Field", kind: K::Sequence { names: &["id", "criticality", "value"], members: &[M { field: "id", ty: F::Named(464), optional: false }, M { field: "criticality", ty: F::Named(91), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 464 */ T { name: "ProtocolIE-ID", kind: K::Alias { field: F::Unsigned(0, 65535, false), form: None, secret: false } },
/* 465 */ T { name: "QCI", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 466 */ T { name: "RAC", kind: K::Alias { field: F::OctetArray(1), form: Some(Form::Number(8)), secret: false } },
/* 467 */ T { name: "RACSIndication", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 468 */ T { name: "RAN-UE-NGAP-ID", kind: K::Alias { field: F::Unsigned(0, 4294967295, false), form: None, secret: false } },
/* 469 */ T { name: "RAT-Restrictions", kind: K::Alias { field: F::List(&F::Named(470), 1, 16), form: None, secret: false } },
/* 470 */ T { name: "RAT-RestrictionsItem", kind: K::Sequence { names: &["pLMNidentity", "rAT-RestrictionInformation", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "rat_restriction_information", ty: F::BitString(8, 8, true), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 471 */ T { name: "RAT-Type", kind: K::Enumerated { names: &["nbiot", "nbiot-leo", "nbiot-meo", "nbiot-geo", "nbiot-othersat", "eutran-leo", "eutran-meo", "eutran-geo", "eutran-othersat"], root: 1, extensible: true } },
/* 472 */ T { name: "RIMInformation", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 473 */ T { name: "RIMRoutingAddress", kind: K::Choice { names: &["gERAN-Cell-ID", "targetRNC-ID", "eHRPD-Sector-ID"], alternatives: &[F::Named(245), F::Named(593), F::OctetArray(16)], root: 1, extensible: true } },
/* 474 */ T { name: "RIMTransfer", kind: K::Sequence { names: &["rIMInformation", "rIMRoutingAddress", "iE-Extensions"], members: &[M { field: "rim_information", ty: F::Named(472), optional: false }, M { field: "rim_routing_address", ty: F::Named(473), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 475 */ T { name: "RLFReportInformation", kind: K::Sequence { names: &["uE-RLF-Report-Container", "uE-RLF-Report-Container-for-extended-bands", "iE-Extensions"], members: &[M { field: "ue_rlf_report_container", ty: F::Named(620), optional: false }, M { field: "ue_rlf_report_container_for_extended_bands", ty: F::Named(621), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 476 */ T { name: "RNC-ID", kind: K::Alias { field: F::Unsigned(0, 4095, false), form: None, secret: false } },
/* 477 */ T { name: "RRC-Container", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 478 */ T { name: "RRC-Establishment-Cause", kind: K::Enumerated { names: &["emergency", "highPriorityAccess", "mt-Access", "mo-Signalling", "mo-Data", "delay-TolerantAccess", "mo-VoiceCall", "mo-ExceptionData"], root: 5, extensible: true } },
/* 479 */ T { name: "Range", kind: K::Enumerated { names: &["m50", "m80", "m180", "m200", "m350", "m400", "m500", "m700", "m1000"], root: 9, extensible: true } },
/* 480 */ T { name: "ReceiveStatusOfULPDCPSDUsExtended", kind: K::Alias { field: F::BitString(1, 16384, false), form: None, secret: false } },
/* 481 */ T { name: "ReceiveStatusOfULPDCPSDUsPDCP-SNlength18", kind: K::Alias { field: F::BitString(1, 131072, false), form: None, secret: false } },
/* 482 */ T { name: "ReceiveStatusofULPDCPSDUs", kind: K::Alias { field: F::BitString(4096, 4096, false), form: None, secret: false } },
/* 483 */ T { name: "RecommendedCellItem", kind: K::Sequence { names: &["eUTRAN-CGI", "timeStayedInCell", "iE-Extensions"], members: &[M { field: "eutran_cgi", ty: F::Named(205), optional: false }, M { field: "time_stayed_in_cell", ty: F::Unsigned(0, 4095, false), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 484 */ T { name: "RecommendedCellList", kind: K::Alias { field: F::List(&F::Named(463), 1, 16), form: None, secret: false } },
/* 485 */ T { name: "RecommendedCellsForPaging", kind: K::Sequence { names: &["recommendedCellList", "iE-Extensions"], members: &[M { field: "recommended_cell_list", ty: F::Named(484), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 486 */ T { name: "RecommendedENBItem", kind: K::Sequence { names: &["mMEPagingTarget", "iE-Extensions"], members: &[M { field: "mme_paging_target", ty: F::Named(366), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 487 */ T { name: "RecommendedENBList", kind: K::Alias { field: F::List(&F::Named(463), 1, 16), form: None, secret: false } },
/* 488 */ T { name: "RecommendedENBsForPaging", kind: K::Sequence { names: &["recommendedENBList", "iE-Extensions"], members: &[M { field: "recommended_enb_list", ty: F::Named(487), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 489 */ T { name: "RelativeMMECapacity", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 490 */ T { name: "RelayNode-Indicator", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 491 */ T { name: "RepetitionPeriod", kind: K::Alias { field: F::Unsigned(0, 4095, false), form: None, secret: false } },
/* 492 */ T { name: "ReportAmountMDT", kind: K::Enumerated { names: &["r1", "r2", "r4", "r8", "r16", "r32", "r64", "rinfinity"], root: 8, extensible: false } },
/* 493 */ T { name: "ReportArea", kind: K::Enumerated { names: &["ecgi"], root: 1, extensible: true } },
/* 494 */ T { name: "ReportIntervalMDT", kind: K::Enumerated { names: &["ms120", "ms240", "ms480", "ms640", "ms1024", "ms2048", "ms5120", "ms10240", "min1", "min6", "min12", "min30", "min60"], root: 13, extensible: false } },
/* 495 */ T { name: "RequestType", kind: K::Sequence { names: &["eventType", "reportArea", "iE-Extensions"], members: &[M { field: "event_type", ty: F::Named(224), optional: false }, M { field: "report_area", ty: F::Named(493), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 496 */ T { name: "RequestTypeAdditionalInfo", kind: K::Enumerated { names: &["includePSCell"], root: 1, extensible: true } },
/* 497 */ T { name: "RequestedTNLInfo", kind: K::Sequence { names: &["pLMNidentity", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 498 */ T { name: "RerouteNASRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 499 */ T { name: "Reset", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 500 */ T { name: "ResetAcknowledge", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 501 */ T { name: "ResetAll", kind: K::Enumerated { names: &["reset-all"], root: 1, extensible: true } },
/* 502 */ T { name: "ResetType", kind: K::Choice { names: &["s1-Interface", "partOfS1-Interface"], alternatives: &[F::Named(501), F::Named(627)], root: 2, extensible: true } },
/* 503 */ T { name: "RetrieveUEInformation", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 504 */ T { name: "Routing-ID", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 505 */ T { name: "S-TMSI", kind: K::Sequence { names: &["mMEC", "m-TMSI", "iE-Extensions"], members: &[M { field: "mmec", ty: F::Named(356), optional: false }, M { field: "m_tmsi", ty: F::Named(329), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 506 */ T { name: "S1AP-PDU", kind: K::Choice { names: &["initiatingMessage", "successfulOutcome", "unsuccessfulOutcome"], alternatives: &[F::Named(288), F::Named(563), F::Named(664)], root: 3, extensible: true } },
/* 507 */ T { name: "S1RemovalFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 508 */ T { name: "S1RemovalRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 509 */ T { name: "S1RemovalResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 510 */ T { name: "S1SetupFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 511 */ T { name: "S1SetupRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 512 */ T { name: "S1SetupResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 513 */ T { name: "SEQUENCE", kind: K::Sequence { names: &["id", "criticality", "value"], members: &[M { field: "id", ty: F::Named(451), optional: false }, M { field: "criticality", ty: F::Named(91), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 514 */ T { name: "SONConfigurationTransfer", kind: K::Sequence { names: &["targeteNB-ID", "sourceeNB-ID", "sONInformation", "iE-Extensions"], members: &[M { field: "target_enb_id", ty: F::Named(595), optional: false }, M { field: "source_enb_id", ty: F::Named(554), optional: false }, M { field: "son_information", ty: F::Named(515), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 515 */ T { name: "SONInformation", kind: K::Choice { names: &["sONInformationRequest", "sONInformationReply", "sONInformation-Extension"], alternatives: &[F::Named(518), F::Named(516), F::Named(463)], root: 2, extensible: true } },
/* 516 */ T { name: "SONInformationReply", kind: K::Sequence { names: &["x2TNLConfigurationInfo", "iE-Extensions"], members: &[M { field: "x2_tnl_configuration_info", ty: F::Named(687), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 517 */ T { name: "SONInformationReport", kind: K::Choice { names: &["rLFReportInformation"], alternatives: &[F::Named(475)], root: 1, extensible: true } },
/* 518 */ T { name: "SONInformationRequest", kind: K::Enumerated { names: &["x2TNL-Configuration-Info", "time-Synchronisation-Info", "activate-Muting", "deactivate-Muting"], root: 1, extensible: true } },
/* 519 */ T { name: "SRVCCHOIndication", kind: K::Enumerated { names: &["pSandCS", "cSonly"], root: 2, extensible: true } },
/* 520 */ T { name: "SRVCCOperationNotPossible", kind: K::Enumerated { names: &["notPossible"], root: 1, extensible: true } },
/* 521 */ T { name: "SRVCCOperationPossible", kind: K::Enumerated { names: &["possible"], root: 1, extensible: true } },
/* 522 */ T { name: "ScheduledCommunicationTime", kind: K::Sequence { names: &["dayofWeek", "timeofDayStart", "timeofDayEnd", "iE-Extensions"], members: &[M { field: "dayof_week", ty: F::Bits(7, 7), optional: true }, M { field: "timeof_day_start", ty: F::Unsigned(0, 86399, true), optional: true }, M { field: "timeof_day_end", ty: F::Unsigned(0, 86399, true), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 523 */ T { name: "SecondaryRATDataUsageReport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 524 */ T { name: "SecondaryRATDataUsageReportItem", kind: K::Sequence { names: &["e-RAB-ID", "secondaryRATType", "e-RABUsageReportList", "iE-Extensions"], members: &[M { field: "e_rab_id", ty: F::Named(117), optional: false }, M { field: "secondary_rat_type", ty: F::Named(527), optional: false }, M { field: "e_rab_usage_report_list", ty: F::Named(173), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 525 */ T { name: "SecondaryRATDataUsageReportList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 526 */ T { name: "SecondaryRATDataUsageRequest", kind: K::Enumerated { names: &["requested"], root: 1, extensible: true } },
/* 527 */ T { name: "SecondaryRATType", kind: K::Enumerated { names: &["nR", "unlicensed"], root: 1, extensible: true } },
/* 528 */ T { name: "SecurityContext", kind: K::Sequence { names: &["nextHopChainingCount", "nextHopParameter", "iE-Extensions"], members: &[M { field: "next_hop_chaining_count", ty: F::Unsigned(0, 7, false), optional: false }, M { field: "next_hop_parameter", ty: F::Named(530), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 529 */ T { name: "SecurityIndication", kind: K::Sequence { names: &["integrityProtectionIndication", "iE-Extensions"], members: &[M { field: "integrity_protection_indication", ty: F::Named(290), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 530 */ T { name: "SecurityKey", kind: K::Alias { field: F::BitString(256, 256, false), form: None, secret: true } },
/* 531 */ T { name: "SecurityResult", kind: K::Sequence { names: &["integrityProtectionResult", "iE-Extensions"], members: &[M { field: "integrity_protection_result", ty: F::Named(291), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 532 */ T { name: "SensorMeasConfig", kind: K::Enumerated { names: &["setup"], root: 1, extensible: true } },
/* 533 */ T { name: "SensorMeasConfigNameItem", kind: K::Sequence { names: &["sensorNameConfig", "iE-Extensions"], members: &[M { field: "sensor_name_config", ty: F::Named(536), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 534 */ T { name: "SensorMeasConfigNameList", kind: K::Alias { field: F::List(&F::Named(533), 1, 3), form: None, secret: false } },
/* 535 */ T { name: "SensorMeasurementConfiguration", kind: K::Sequence { names: &["sensorMeasConfig", "sensorMeasConfigNameList", "iE-Extensions"], members: &[M { field: "sensor_meas_config", ty: F::Named(532), optional: false }, M { field: "sensor_meas_config_name_list", ty: F::Named(534), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 536 */ T { name: "SensorNameConfig", kind: K::Choice { names: &["uncompensatedBarometricConfig", "choice-Extensions"], alternatives: &[F::Named(537), F::Named(463)], root: 2, extensible: false } },
/* 537 */ T { name: "SensorNameConfigUncompensatedBarometricConfig", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 538 */ T { name: "SerialNumber", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 539 */ T { name: "ServedDCNs", kind: K::Alias { field: F::List(&F::Named(540), 0, 32), form: None, secret: false } },
/* 540 */ T { name: "ServedDCNsItem", kind: K::Sequence { names: &["dCN-ID", "relativeDCNCapacity", "iE-Extensions"], members: &[M { field: "dcn_id", ty: F::Named(101), optional: false }, M { field: "relative_dcn_capacity", ty: F::Named(489), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 541 */ T { name: "ServedGUMMEIs", kind: K::Alias { field: F::List(&F::Named(542), 1, 8), form: None, secret: false } },
/* 542 */ T { name: "ServedGUMMEIsItem", kind: K::Sequence { names: &["servedPLMNs", "servedGroupIDs", "servedMMECs", "iE-Extensions"], members: &[M { field: "served_plmns", ty: F::Named(545), optional: false }, M { field: "served_group_ids", ty: F::Named(543), optional: false }, M { field: "served_mmecs", ty: F::Named(544), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 543 */ T { name: "ServedGroupIDs", kind: K::Alias { field: F::List(&F::Named(357), 1, 65535), form: None, secret: false } },
/* 544 */ T { name: "ServedMMECs", kind: K::Alias { field: F::List(&F::Named(356), 1, 256), form: None, secret: false } },
/* 545 */ T { name: "ServedPLMNs", kind: K::Alias { field: F::List(&F::Named(423), 1, 32), form: None, secret: false } },
/* 546 */ T { name: "ServiceType", kind: K::Enumerated { names: &["qMC-for-streaming-service", "qMC-for-MTSI-service"], root: 2, extensible: true } },
/* 547 */ T { name: "Source-ToTarget-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 548 */ T { name: "SourceBSS-ToTargetBSS-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 549 */ T { name: "SourceNgRanNode-ID", kind: K::Sequence { names: &["global-RAN-NODE-ID", "selected-TAI", "iE-Extensions"], members: &[M { field: "global_ran_node_id", ty: F::Named(256), optional: false }, M { field: "selected_tai", ty: F::Named(235), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 550 */ T { name: "SourceNgRanNode-ToTargetNgRanNode-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 551 */ T { name: "SourceNodeID", kind: K::Choice { names: &["sourceNgRanNode-ID", "sourceNodeID-Extension"], alternatives: &[F::Named(549), F::Named(463)], root: 2, extensible: false } },
/* 552 */ T { name: "SourceOfUEActivityBehaviourInformation", kind: K::Enumerated { names: &["subscription-information", "statistics"], root: 2, extensible: true } },
/* 553 */ T { name: "SourceRNC-ToTargetRNC-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 554 */ T { name: "SourceeNB-ID", kind: K::Sequence { names: &["global-ENB-ID", "selected-TAI", "iE-Extensions"], members: &[M { field: "global_enb_id", ty: F::Named(254), optional: false }, M { field: "selected_tai", ty: F::Named(572), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: false } },
/* 555 */ T { name: "SourceeNB-ToTargeteNB-TransparentContainer", kind: K::Sequence { names: &["rRC-Container", "e-RABInformationList", "targetCell-ID", "subscriberProfileIDforRFP", "uE-HistoryInformation", "iE-Extensions"], members: &[M { field: "rrc_container", ty: F::Named(477), optional: false }, M { field: "e_rab_information_list", ty: F::Named(127), optional: true }, M { field: "target_cell_id", ty: F::Named(205), optional: false }, M { field: "subscriber_profile_id_for_rfp", ty: F::Named(557), optional: true }, M { field: "ue_history_information", ty: F::Named(618), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 556 */ T { name: "StratumLevel", kind: K::Alias { field: F::Unsigned(0, 3, true), form: None, secret: false } },
/* 557 */ T { name: "SubscriberProfileIDforRFP", kind: K::Alias { field: F::Unsigned(1, 256, false), form: None, secret: false } },
/* 558 */ T { name: "Subscription-Based-UE-DifferentiationInfo", kind: K::Sequence { names: &["periodicCommunicationIndicator", "periodicTime", "scheduledCommunicationTime", "stationaryIndication", "trafficProfile", "batteryIndication", "iE-Extensions"], members: &[M { field: "periodic_communication_indicator", ty: F::Named(560), optional: true }, M { field: "periodic_time", ty: F::Unsigned(1, 3600, true), optional: true }, M { field: "scheduled_communication_time", ty: F::Named(522), optional: true }, M { field: "stationary_indication", ty: F::Named(561), optional: true }, M { field: "traffic_profile", ty: F::Named(562), optional: true }, M { field: "battery_indication", ty: F::Named(559), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 559 */ T { name: "SubscriptionBasedUEDifferentiationInfoBatteryIndication", kind: K::Enumerated { names: &["battery-powered", "battery-powered-not-rechargeable-or-replaceable", "not-battery-powered"], root: 3, extensible: true } },
/* 560 */ T { name: "SubscriptionBasedUEDifferentiationInfoPeriodicCommunicationIndicator", kind: K::Enumerated { names: &["periodically", "ondemand"], root: 2, extensible: true } },
/* 561 */ T { name: "SubscriptionBasedUEDifferentiationInfoStationaryIndication", kind: K::Enumerated { names: &["stationary", "mobile"], root: 2, extensible: true } },
/* 562 */ T { name: "SubscriptionBasedUEDifferentiationInfoTrafficProfile", kind: K::Enumerated { names: &["single-packet", "dual-packets", "multiple-packets"], root: 3, extensible: true } },
/* 563 */ T { name: "SuccessfulOutcome", kind: K::Sequence { names: &["procedureCode", "criticality", "value"], members: &[M { field: "procedure_code", ty: F::Named(458), optional: false }, M { field: "criticality", ty: F::Named(91), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 564 */ T { name: "SupportedTAs", kind: K::Alias { field: F::List(&F::Named(565), 1, 256), form: None, secret: false } },
/* 565 */ T { name: "SupportedTAs-Item", kind: K::Sequence { names: &["tAC", "broadcastPLMNs", "iE-Extensions"], members: &[M { field: "tac", ty: F::Named(570), optional: false }, M { field: "broadcast_plmns", ty: F::Named(10), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 566 */ T { name: "SynchronisationInformation", kind: K::Sequence { names: &["sourceStratumLevel", "listeningSubframePattern", "aggressoreCGI-List", "iE-Extensions"], members: &[M { field: "source_stratum_level", ty: F::Named(556), optional: true }, M { field: "listening_subframe_pattern", ty: F::Named(319), optional: true }, M { field: "aggressor_ecgi_list", ty: F::Named(176), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 567 */ T { name: "SynchronisationStatus", kind: K::Enumerated { names: &["synchronous", "asynchronous"], root: 2, extensible: true } },
/* 568 */ T { name: "TABasedMDT", kind: K::Sequence { names: &["tAListforMDT", "iE-Extensions"], members: &[M { field: "ta_listfor_mdt", ty: F::Named(585), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 569 */ T { name: "TABasedQMC", kind: K::Sequence { names: &["tAListforQMC", "iE-Extensions"], members: &[M { field: "ta_listfor_qmc", ty: F::Named(586), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 570 */ T { name: "TAC", kind: K::Alias { field: F::OctetArray(2), form: Some(Form::Number(16)), secret: false } },
/* 571 */ T { name: "TACList-In-LTE-NTN", kind: K::Alias { field: F::List(&F::Named(570), 1, 12), form: None, secret: false } },
/* 572 */ T { name: "TAI", kind: K::Sequence { names: &["pLMNidentity", "tAC", "iE-Extensions"], members: &[M { field: "plmn_identity", ty: F::Named(423), optional: false }, M { field: "tac", ty: F::Named(570), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 573 */ T { name: "TAI-Broadcast", kind: K::Alias { field: F::List(&F::Named(574), 1, 65535), form: None, secret: false } },
/* 574 */ T { name: "TAI-Broadcast-Item", kind: K::Sequence { names: &["tAI", "completedCellinTAI", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(572), optional: false }, M { field: "completed_cellin_tai", ty: F::Named(82), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 575 */ T { name: "TAI-Cancelled", kind: K::Alias { field: F::List(&F::Named(576), 1, 65535), form: None, secret: false } },
/* 576 */ T { name: "TAI-Cancelled-Item", kind: K::Sequence { names: &["tAI", "cancelledCellinTAI", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(572), optional: false }, M { field: "cancelled_cellin_tai", ty: F::Named(46), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 577 */ T { name: "TAIBasedMDT", kind: K::Sequence { names: &["tAIListforMDT", "iE-Extensions"], members: &[M { field: "tai_listfor_mdt", ty: F::Named(582), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 578 */ T { name: "TAIBasedQMC", kind: K::Sequence { names: &["tAIListforQMC", "iE-Extensions"], members: &[M { field: "tai_listfor_qmc", ty: F::Named(583), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 579 */ T { name: "TAIItem", kind: K::Sequence { names: &["tAI", "iE-Extensions"], members: &[M { field: "tai", ty: F::Named(572), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 580 */ T { name: "TAIList", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 581 */ T { name: "TAIListForRestart", kind: K::Alias { field: F::List(&F::Named(572), 1, 2048), form: None, secret: false } },
/* 582 */ T { name: "TAIListforMDT", kind: K::Alias { field: F::List(&F::Named(572), 1, 8), form: None, secret: false } },
/* 583 */ T { name: "TAIListforQMC", kind: K::Alias { field: F::List(&F::Named(572), 1, 8), form: None, secret: false } },
/* 584 */ T { name: "TAIListforWarning", kind: K::Alias { field: F::List(&F::Named(572), 1, 65535), form: None, secret: false } },
/* 585 */ T { name: "TAListforMDT", kind: K::Alias { field: F::List(&F::Named(570), 1, 8), form: None, secret: false } },
/* 586 */ T { name: "TAListforQMC", kind: K::Alias { field: F::List(&F::Named(570), 1, 8), form: None, secret: false } },
/* 587 */ T { name: "TBCD-STRING", kind: K::Alias { field: F::OctetArray(3), form: None, secret: false } },
/* 588 */ T { name: "Target-ToSource-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 589 */ T { name: "TargetBSS-ToSourceBSS-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 590 */ T { name: "TargetID", kind: K::Choice { names: &["targeteNB-ID", "targetRNC-ID", "cGI", "targetgNgRanNode-ID"], alternatives: &[F::Named(595), F::Named(593), F::Named(29), F::Named(591)], root: 3, extensible: true } },
/* 591 */ T { name: "TargetNgRanNode-ID", kind: K::Sequence { names: &["global-RAN-NODE-ID", "selected-TAI", "iE-Extensions"], members: &[M { field: "global_ran_node_id", ty: F::Named(256), optional: false }, M { field: "selected_tai", ty: F::Named(235), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 592 */ T { name: "TargetNgRanNode-ToSourceNgRanNode-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 593 */ T { name: "TargetRNC-ID", kind: K::Sequence { names: &["lAI", "rAC", "rNC-ID", "extendedRNC-ID", "iE-Extensions"], members: &[M { field: "lai", ty: F::Named(306), optional: false }, M { field: "rac", ty: F::Named(466), optional: true }, M { field: "rnc_id", ty: F::Named(476), optional: false }, M { field: "extended_rnc_id", ty: F::Named(232), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 594 */ T { name: "TargetRNC-ToSourceRNC-TransparentContainer", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 595 */ T { name: "TargeteNB-ID", kind: K::Sequence { names: &["global-ENB-ID", "selected-TAI", "iE-Extensions"], members: &[M { field: "global_enb_id", ty: F::Named(254), optional: false }, M { field: "selected_tai", ty: F::Named(572), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 596 */ T { name: "TargeteNB-ToSourceeNB-TransparentContainer", kind: K::Sequence { names: &["rRC-Container", "iE-Extensions"], members: &[M { field: "rrc_container", ty: F::Named(477), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 597 */ T { name: "Threshold-RSRP", kind: K::Alias { field: F::Unsigned(0, 97, false), form: None, secret: false } },
/* 598 */ T { name: "Threshold-RSRQ", kind: K::Alias { field: F::Unsigned(0, 34, false), form: None, secret: false } },
/* 599 */ T { name: "Time-UE-StayedInCell", kind: K::Alias { field: F::Unsigned(0, 4095, false), form: None, secret: false } },
/* 600 */ T { name: "Time-UE-StayedInCell-EnhancedGranularity", kind: K::Alias { field: F::Unsigned(0, 40950, false), form: None, secret: false } },
/* 601 */ T { name: "TimeBasedHandoverInformation", kind: K::Sequence { names: &["hOWindowStart", "hOWindowDuration", "iE-Extensions"], members: &[M { field: "ho_window_start", ty: F::Named(275), optional: false }, M { field: "ho_window_duration", ty: F::Named(274), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 602 */ T { name: "TimeRefDistribution", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 603 */ T { name: "TimeSinceSecondaryNodeRelease", kind: K::Alias { field: F::OctetArray(4), form: None, secret: false } },
/* 604 */ T { name: "TimeSynchronisationInfo", kind: K::Sequence { names: &["stratumLevel", "synchronisationStatus", "iE-Extensions"], members: &[M { field: "stratum_level", ty: F::Named(556), optional: false }, M { field: "synchronisation_status", ty: F::Named(567), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 605 */ T { name: "TimeToTrigger", kind: K::Enumerated { names: &["ms0", "ms40", "ms64", "ms80", "ms100", "ms128", "ms160", "ms256", "ms320", "ms480", "ms512", "ms640", "ms1024", "ms1280", "ms2560", "ms5120"], root: 16, extensible: false } },
/* 606 */ T { name: "TimeToWait", kind: K::Enumerated { names: &["v1s", "v2s", "v5s", "v10s", "v20s", "v60s"], root: 6, extensible: true } },
/* 607 */ T { name: "TraceActivation", kind: K::Sequence { names: &["e-UTRAN-Trace-ID", "interfacesToTrace", "traceDepth", "traceCollectionEntityIPAddress", "iE-Extensions"], members: &[M { field: "e_utran_trace_id", ty: F::Named(174), optional: false }, M { field: "interfaces_to_trace", ty: F::Named(298), optional: false }, M { field: "trace_depth", ty: F::Named(608), optional: false }, M { field: "trace_collection_entity_ip_address", ty: F::Named(613), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 608 */ T { name: "TraceDepth", kind: K::Enumerated { names: &["minimum", "medium", "maximum", "minimumWithoutVendorSpecificExtension", "mediumWithoutVendorSpecificExtension", "maximumWithoutVendorSpecificExtension"], root: 6, extensible: true } },
/* 609 */ T { name: "TraceFailureIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 610 */ T { name: "TraceStart", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 611 */ T { name: "TrafficLoadReductionIndication", kind: K::Alias { field: F::Unsigned(1, 99, false), form: None, secret: false } },
/* 612 */ T { name: "TransportInformation", kind: K::Sequence { names: &["transportLayerAddress", "uL-GTP-TEID"], members: &[M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "ul_gtp_teid", ty: F::Named(249), optional: false }], extensible: true } },
/* 613 */ T { name: "TransportLayerAddress", kind: K::Alias { field: F::BitString(1, 160, true), form: Some(Form::Address), secret: false } },
/* 614 */ T { name: "TriggeringMessage", kind: K::Enumerated { names: &["initiating-message", "successful-outcome", "unsuccessfull-outcome"], root: 3, extensible: false } },
/* 615 */ T { name: "TunnelInformation", kind: K::Sequence { names: &["transportLayerAddress", "uDP-Port-Number", "iE-Extensions"], members: &[M { field: "transport_layer_address", ty: F::Named(613), optional: false }, M { field: "udp_port_number", ty: F::Named(445), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 616 */ T { name: "TypeOfError", kind: K::Enumerated { names: &["not-understood", "missing"], root: 2, extensible: true } },
/* 617 */ T { name: "UE-Application-Layer-Measurement-Capability", kind: K::Alias { field: F::Bits(8, 8), form: None, secret: false } },
/* 618 */ T { name: "UE-HistoryInformation", kind: K::Alias { field: F::List(&F::Named(311), 1, 16), form: None, secret: false } },
/* 619 */ T { name: "UE-HistoryInformationFromTheUE", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 620 */ T { name: "UE-RLF-Report-Container", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 621 */ T { name: "UE-RLF-Report-Container-for-extended-bands", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 622 */ T { name: "UE-RetentionInformation", kind: K::Enumerated { names: &["ues-retained"], root: 1, extensible: true } },
/* 623 */ T { name: "UE-S1AP-ID-pair", kind: K::Sequence { names: &["mME-UE-S1AP-ID", "eNB-UE-S1AP-ID", "iE-Extensions"], members: &[M { field: "mme_ue_s1ap_id", ty: F::Named(358), optional: false }, M { field: "enb_ue_s1ap_id", ty: F::Named(189), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 624 */ T { name: "UE-S1AP-IDs", kind: K::Choice { names: &["uE-S1AP-ID-pair", "mME-UE-S1AP-ID"], alternatives: &[F::Named(623), F::Named(358)], root: 2, extensible: true } },
/* 625 */ T { name: "UE-Usage-Type", kind: K::Alias { field: F::Unsigned(0, 255, false), form: None, secret: false } },
/* 626 */ T { name: "UE-associatedLogicalS1-ConnectionItem", kind: K::Sequence { names: &["mME-UE-S1AP-ID", "eNB-UE-S1AP-ID", "iE-Extensions"], members: &[M { field: "mme_ue_s1ap_id", ty: F::Named(358), optional: true }, M { field: "enb_ue_s1ap_id", ty: F::Named(189), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 627 */ T { name: "UE-associatedLogicalS1-ConnectionListRes", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 628 */ T { name: "UE-associatedLogicalS1-ConnectionListResAck", kind: K::Alias { field: F::List(&F::Named(463), 1, 256), form: None, secret: false } },
/* 629 */ T { name: "UEAggregateMaximumBitrate", kind: K::Sequence { names: &["uEaggregateMaximumBitRateDL", "uEaggregateMaximumBitRateUL", "iE-Extensions"], members: &[M { field: "ue_aggregate_maximum_bit_rate_dl", ty: F::Named(18), optional: false }, M { field: "ue_aggregate_maximum_bit_rate_ul", ty: F::Named(18), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 630 */ T { name: "UEAppLayerMeasConfig", kind: K::Sequence { names: &["containerForAppLayerMeasConfig", "areaScopeOfQMC", "iE-Extensions"], members: &[M { field: "container_for_app_layer_meas_config", ty: F::Octets(1, 1000, false), optional: false }, M { field: "area_scope_of_qmc", ty: F::Named(6), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 631 */ T { name: "UECapabilityInfoIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 632 */ T { name: "UECapabilityInfoRequest", kind: K::Enumerated { names: &["requested"], root: 1, extensible: true } },
/* 633 */ T { name: "UEContextModificationConfirm", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 634 */ T { name: "UEContextModificationFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 635 */ T { name: "UEContextModificationIndication", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 636 */ T { name: "UEContextModificationRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 637 */ T { name: "UEContextModificationResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 638 */ T { name: "UEContextReleaseCommand", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 639 */ T { name: "UEContextReleaseComplete", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 640 */ T { name: "UEContextReleaseRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 641 */ T { name: "UEContextResumeFailure", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 642 */ T { name: "UEContextResumeRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 643 */ T { name: "UEContextResumeResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 644 */ T { name: "UEContextSuspendRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 645 */ T { name: "UEContextSuspendResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 646 */ T { name: "UEIdentityIndexValue", kind: K::Alias { field: F::Bits(10, 10), form: None, secret: false } },
/* 647 */ T { name: "UEInformationTransfer", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 648 */ T { name: "UEPagingID", kind: K::Choice { names: &["s-TMSI", "iMSI"], alternatives: &[F::Named(505), F::Named(280)], root: 2, extensible: true } },
/* 649 */ T { name: "UERadioCapability", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 650 */ T { name: "UERadioCapabilityForPaging", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 651 */ T { name: "UERadioCapabilityID", kind: K::Alias { field: F::Octets(0, UNBOUNDED, false), form: None, secret: false } },
/* 652 */ T { name: "UERadioCapabilityIDMappingRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 653 */ T { name: "UERadioCapabilityIDMappingResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 654 */ T { name: "UERadioCapabilityMatchRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 655 */ T { name: "UERadioCapabilityMatchResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 656 */ T { name: "UESecurityCapabilities", kind: K::Sequence { names: &["encryptionAlgorithms", "integrityProtectionAlgorithms", "iE-Extensions"], members: &[M { field: "encryption_algorithms", ty: F::Named(216), optional: false }, M { field: "integrity_protection_algorithms", ty: F::Named(289), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 657 */ T { name: "UESidelinkAggregateMaximumBitrate", kind: K::Sequence { names: &["uESidelinkAggregateMaximumBitRate", "iE-Extensions"], members: &[M { field: "ue_sidelink_aggregate_maximum_bit_rate", ty: F::Named(18), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 658 */ T { name: "UEUserPlaneCIoTSupportIndicator", kind: K::Enumerated { names: &["supported"], root: 1, extensible: true } },
/* 659 */ T { name: "UL-CP-SecurityInformation", kind: K::Sequence { names: &["ul-NAS-MAC", "ul-NAS-Count", "iE-Extensions"], members: &[M { field: "ul_nas_mac", ty: F::Named(661), optional: false }, M { field: "ul_nas_count", ty: F::Named(660), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 660 */ T { name: "UL-NAS-Count", kind: K::Alias { field: F::Bits(5, 5), form: Some(Form::Number(5)), secret: false } },
/* 661 */ T { name: "UL-NAS-MAC", kind: K::Alias { field: F::Bits(16, 16), form: None, secret: false } },
/* 662 */ T { name: "URI-Address", kind: K::Alias { field: F::Text(Text::Visible, 0, UNBOUNDED, false), form: None, secret: false } },
/* 663 */ T { name: "UnlicensedSpectrumRestriction", kind: K::Enumerated { names: &["unlicensed-restricted"], root: 1, extensible: true } },
/* 664 */ T { name: "UnsuccessfulOutcome", kind: K::Sequence { names: &["procedureCode", "criticality", "value"], members: &[M { field: "procedure_code", ty: F::Named(458), optional: false }, M { field: "criticality", ty: F::Named(91), optional: false }, M { field: "value", ty: F::Open, optional: false }], extensible: false } },
/* 665 */ T { name: "UplinkNASTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 666 */ T { name: "UplinkNonUEAssociatedLPPaTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 667 */ T { name: "UplinkS1cdma2000tunnelling", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 668 */ T { name: "UplinkUEAssociatedLPPaTransport", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 669 */ T { name: "UserLocationInformation", kind: K::Sequence { names: &["eutran-cgi", "tai", "iE-Extensions"], members: &[M { field: "eutran_cgi", ty: F::Named(205), optional: false }, M { field: "tai", ty: F::Named(572), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 670 */ T { name: "V2XServicesAuthorized", kind: K::Sequence { names: &["vehicleUE", "pedestrianUE", "iE-Extensions"], members: &[M { field: "vehicle_ue", ty: F::Named(671), optional: true }, M { field: "pedestrian_ue", ty: F::Named(443), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 671 */ T { name: "VehicleUE", kind: K::Enumerated { names: &["authorized", "not-authorized"], root: 2, extensible: true } },
/* 672 */ T { name: "VoiceSupportMatchIndicator", kind: K::Enumerated { names: &["supported", "not-supported"], root: 2, extensible: true } },
/* 673 */ T { name: "WLANMeasConfig", kind: K::Enumerated { names: &["setup"], root: 1, extensible: true } },
/* 674 */ T { name: "WLANMeasConfigNameList", kind: K::Alias { field: F::List(&F::Named(678), 1, 4), form: None, secret: false } },
/* 675 */ T { name: "WLANMeasurementConfiguration", kind: K::Sequence { names: &["wlanMeasConfig", "wlanMeasConfigNameList", "wlan-rssi", "wlan-rtt", "iE-Extensions"], members: &[M { field: "wlan_meas_config", ty: F::Named(673), optional: false }, M { field: "wlan_meas_config_name_list", ty: F::Named(674), optional: true }, M { field: "wlan_rssi", ty: F::Named(676), optional: true }, M { field: "wlan_rtt", ty: F::Named(677), optional: true }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 676 */ T { name: "WLANMeasurementConfigurationWlanRssi", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 677 */ T { name: "WLANMeasurementConfigurationWlanRtt", kind: K::Enumerated { names: &["true"], root: 1, extensible: true } },
/* 678 */ T { name: "WLANName", kind: K::Alias { field: F::Octets(1, 32, false), form: None, secret: false } },
/* 679 */ T { name: "WUS-Assistance-Information", kind: K::Sequence { names: &["pagingProbabilityInformation", "iE-Extensions"], members: &[M { field: "paging_probability_information", ty: F::Named(438), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
/* 680 */ T { name: "WarningAreaCoordinates", kind: K::Alias { field: F::Octets(1, 1024, false), form: None, secret: false } },
/* 681 */ T { name: "WarningAreaList", kind: K::Choice { names: &["cellIDList", "trackingAreaListforWarning", "emergencyAreaIDList"], alternatives: &[F::Named(177), F::Named(584), F::Named(212)], root: 3, extensible: true } },
/* 682 */ T { name: "WarningMessageContents", kind: K::Alias { field: F::Octets(1, 9600, false), form: None, secret: false } },
/* 683 */ T { name: "WarningSecurityInfo", kind: K::Alias { field: F::OctetArray(50), form: None, secret: false } },
/* 684 */ T { name: "WarningType", kind: K::Alias { field: F::OctetArray(2), form: None, secret: false } },
/* 685 */ T { name: "WriteReplaceWarningRequest", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 686 */ T { name: "WriteReplaceWarningResponse", kind: K::Sequence { names: &["protocolIEs"], members: &[M { field: "protocol_ies", ty: F::Named(462), optional: false }], extensible: true } },
/* 687 */ T { name: "X2TNLConfigurationInfo", kind: K::Sequence { names: &["eNBX2TransportLayerAddresses", "iE-Extensions"], members: &[M { field: "enb_x2_transport_layer_addresses", ty: F::Named(202), optional: false }, M { field: "ie_extensions", ty: F::Named(459), optional: true }], extensible: true } },
];
