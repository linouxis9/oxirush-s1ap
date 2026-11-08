// Auto-generated from ASN.1 by the generator of oxirush-aper; do not edit.
oxirush_aper::__protocol! { protocol S1ap "S1AP", bindings s1ap, pdu S1APPDU, triggering initiating_message successful_outcome unsuccessfull_outcome }
oxirush_aper::__procedures! {
    protocol S1ap "S1AP", bindings s1ap, pdu S1APPDU, kind S1apPduKind, hooks oxirush_s1ap_hooks;
    0 HandoverPreparation reject { InitiatingMessage HandoverRequired "HandoverRequired" [0 reject mandatory, 8 reject mandatory, 1 reject mandatory, 2 ignore mandatory, 4 reject mandatory, 79 ignore optional, 125 reject optional, 104 reject mandatory, 138 reject optional, 132 reject conditional, 133 ignore conditional, 127 reject optional, 145 reject optional, 150 ignore optional]; SuccessfulOutcome HandoverCommand "HandoverCommand" [0 reject mandatory, 8 reject mandatory, 1 reject mandatory, 135 reject conditional, 12 ignore optional, 13 ignore optional, 123 reject mandatory, 139 reject optional, 58 ignore optional]; UnsuccessfulOutcome HandoverPreparationFailure "HandoverPreparationFailure" [0 ignore mandatory, 8 ignore mandatory, 2 ignore mandatory, 58 ignore optional]; }
    1 HandoverResourceAllocation reject { InitiatingMessage HandoverRequest "HandoverRequest" [0 reject mandatory, 1 reject mandatory, 2 ignore mandatory, 66 reject mandatory, 53 reject mandatory, 104 reject mandatory, 107 reject mandatory, 41 ignore optional, 25 ignore optional, 98 ignore optional, 124 ignore optional, 40 reject mandatory, 136 reject conditional, 127 reject optional, 146 ignore optional, 75 ignore optional, 158 ignore optional, 165 ignore optional, 177 ignore optional, 192 ignore optional, 196 ignore optional, 195 ignore optional, 241 ignore optional, 240 ignore optional, 248 ignore optional, 251 ignore optional, 269 ignore optional, 271 ignore optional, 277 ignore optional, 283 ignore optional, 278 ignore optional, 299 ignore optional, 301 reject optional, 306 ignore optional, 307 ignore optional, 308 ignore optional, 314 reject optional, 355 ignore optional]; SuccessfulOutcome HandoverRequestAcknowledge "HandoverRequestAcknowledge" [0 ignore mandatory, 8 ignore mandatory, 18 ignore mandatory, 19 ignore optional, 123 reject mandatory, 127 ignore optional, 58 ignore optional, 145 ignore optional, 242 ignore optional]; UnsuccessfulOutcome HandoverFailure "HandoverFailure" [0 ignore mandatory, 2 ignore mandatory, 58 ignore optional]; }
    2 HandoverNotification ignore { InitiatingMessage HandoverNotify "HandoverNotify" [0 reject mandatory, 8 reject mandatory, 100 ignore mandatory, 67 ignore mandatory, 176 ignore optional, 186 ignore optional, 288 ignore optional, 320 ignore optional, 339 ignore optional]; }
    3 PathSwitchRequest reject { InitiatingMessage PathSwitchRequest "PathSwitchRequest" [8 reject mandatory, 22 reject mandatory, 88 reject mandatory, 100 ignore mandatory, 67 ignore mandatory, 107 ignore mandatory, 127 ignore optional, 145 ignore optional, 157 ignore optional, 146 ignore optional, 176 ignore optional, 186 ignore optional, 245 ignore optional, 269 ignore optional, 288 ignore optional, 339 ignore optional]; SuccessfulOutcome PathSwitchRequestAcknowledge "PathSwitchRequestAcknowledge" [0 ignore mandatory, 8 ignore mandatory, 66 ignore optional, 95 ignore optional, 33 ignore optional, 40 reject mandatory, 58 ignore optional, 158 ignore optional, 146 ignore optional, 195 ignore optional, 241 ignore optional, 240 ignore optional, 248 ignore optional, 251 ignore optional, 269 ignore optional, 271 ignore optional, 277 ignore optional, 283 ignore optional, 278 ignore optional, 41 ignore optional, 299 ignore optional, 306 ignore optional, 307 ignore optional, 308 ignore optional, 314 reject optional, 107 ignore optional, 341 ignore optional, 355 ignore optional]; UnsuccessfulOutcome PathSwitchRequestFailure "PathSwitchRequestFailure" [0 ignore mandatory, 8 ignore mandatory, 2 ignore mandatory, 58 ignore optional]; }
    4 HandoverCancel reject { InitiatingMessage HandoverCancel "HandoverCancel" [0 reject mandatory, 8 reject mandatory, 2 ignore mandatory]; SuccessfulOutcome HandoverCancelAcknowledge "HandoverCancelAcknowledge" [0 ignore mandatory, 8 ignore mandatory, 58 ignore optional]; }
    5 E_RABSetup reject { InitiatingMessage ERABSetupRequest "E-RABSetupRequest" [0 reject mandatory, 8 reject mandatory, 66 reject optional, 16 reject mandatory]; SuccessfulOutcome ERABSetupResponse "E-RABSetupResponse" [0 ignore mandatory, 8 ignore mandatory, 28 ignore optional, 29 ignore optional, 58 ignore optional, 189 ignore optional]; }
    6 E_RABModify reject { InitiatingMessage ERABModifyRequest "E-RABModifyRequest" [0 reject mandatory, 8 reject mandatory, 66 reject optional, 30 reject mandatory, 268 ignore optional]; SuccessfulOutcome ERABModifyResponse "E-RABModifyResponse" [0 ignore mandatory, 8 ignore mandatory, 31 ignore optional, 32 ignore optional, 58 ignore optional, 264 ignore optional, 189 ignore optional]; }
    7 E_RABRelease reject { InitiatingMessage ERABReleaseCommand "E-RABReleaseCommand" [0 reject mandatory, 8 reject mandatory, 66 reject optional, 33 ignore mandatory, 26 ignore optional]; SuccessfulOutcome ERABReleaseResponse "E-RABReleaseResponse" [0 ignore mandatory, 8 ignore mandatory, 69 ignore optional, 34 ignore optional, 58 ignore optional, 189 ignore optional, 264 ignore optional]; }
    8 E_RABReleaseIndication ignore { InitiatingMessage ERABReleaseIndication "E-RABReleaseIndication" [0 reject mandatory, 8 reject mandatory, 110 ignore mandatory, 189 ignore optional, 264 ignore optional]; }
    9 InitialContextSetup reject { InitiatingMessage InitialContextSetupRequest "InitialContextSetupRequest" [0 reject mandatory, 8 reject mandatory, 66 reject mandatory, 24 reject mandatory, 107 reject mandatory, 73 reject mandatory, 25 ignore optional, 41 ignore optional, 74 ignore optional, 106 ignore optional, 108 reject optional, 124 ignore optional, 146 ignore optional, 159 ignore optional, 75 ignore optional, 158 ignore optional, 165 ignore optional, 177 ignore optional, 187 ignore conditional, 192 ignore optional, 196 ignore optional, 195 ignore optional, 241 ignore optional, 240 ignore optional, 248 ignore optional, 251 ignore optional, 269 ignore optional, 271 ignore optional, 277 ignore optional, 283 ignore optional, 278 ignore optional, 299 ignore optional, 301 ignore optional, 306 ignore optional, 307 ignore optional, 308 ignore optional, 314 reject optional, 354 ignore optional, 355 ignore optional]; SuccessfulOutcome InitialContextSetupResponse "InitialContextSetupResponse" [0 ignore mandatory, 8 ignore mandatory, 51 ignore mandatory, 48 ignore optional, 58 ignore optional]; UnsuccessfulOutcome InitialContextSetupFailure "InitialContextSetupFailure" [0 ignore mandatory, 8 ignore mandatory, 2 ignore mandatory, 58 ignore optional]; }
    10 Paging ignore { InitiatingMessage Paging "Paging" [80 ignore mandatory, 43 ignore mandatory, 44 ignore optional, 109 ignore mandatory, 46 ignore mandatory, 128 ignore optional, 151 ignore optional, 198 ignore optional, 211 ignore optional, 227 ignore optional, 231 ignore optional, 239 ignore optional, 244 ignore optional, 251 ignore optional, 271 ignore optional, 304 ignore optional, 323 ignore optional, 324 ignore optional, 331 ignore optional]; }
    11 downlinkNASTransport ignore { InitiatingMessage DownlinkNASTransport "DownlinkNASTransport" [0 reject mandatory, 8 reject mandatory, 26 reject mandatory, 41 ignore optional, 106 ignore optional, 124 ignore optional, 74 ignore optional, 249 ignore optional, 251 ignore optional, 269 ignore optional, 271 ignore optional, 275 ignore optional, 280 ignore optional, 283 ignore optional, 278 ignore optional, 299 ignore optional, 314 reject optional, 192 ignore optional, 354 ignore optional]; }
    12 initialUEMessage ignore { InitiatingMessage InitialUEMessage "InitialUEMessage" [8 reject mandatory, 26 reject mandatory, 67 reject mandatory, 100 ignore mandatory, 134 ignore mandatory, 96 reject optional, 127 reject optional, 75 reject optional, 145 reject optional, 155 ignore optional, 160 reject optional, 170 ignore optional, 176 ignore optional, 184 ignore optional, 186 ignore optional, 223 ignore optional, 230 ignore optional, 242 ignore optional, 246 ignore optional, 250 ignore optional, 263 ignore optional, 281 ignore optional, 302 reject optional, 339 ignore optional, 353 ignore optional]; }
    13 uplinkNASTransport ignore { InitiatingMessage UplinkNASTransport "UplinkNASTransport" [0 reject mandatory, 8 reject mandatory, 26 reject mandatory, 100 ignore mandatory, 67 ignore mandatory, 155 ignore optional, 184 ignore optional, 186 ignore optional, 288 ignore optional, 339 ignore optional]; }
    14 Reset reject { InitiatingMessage Reset "Reset" [2 ignore mandatory, 92 reject mandatory]; SuccessfulOutcome ResetAcknowledge "ResetAcknowledge" [93 ignore optional, 58 ignore optional]; }
    15 ErrorIndication ignore { InitiatingMessage ErrorIndication "ErrorIndication" [0 ignore optional, 8 ignore optional, 2 ignore optional, 58 ignore optional, 96 ignore optional]; }
    16 NASNonDeliveryIndication ignore { InitiatingMessage NASNonDeliveryIndication "NASNonDeliveryIndication" [0 reject mandatory, 8 reject mandatory, 26 ignore mandatory, 2 ignore mandatory]; }
    17 S1Setup reject { InitiatingMessage S1SetupRequest "S1SetupRequest" [59 reject mandatory, 60 ignore optional, 64 reject mandatory, 137 ignore mandatory, 128 reject optional, 228 ignore optional, 234 ignore optional, 291 ignore optional]; SuccessfulOutcome S1SetupResponse "S1SetupResponse" [61 ignore optional, 105 reject mandatory, 87 ignore mandatory, 163 ignore optional, 58 ignore optional, 228 ignore optional, 247 ignore optional, 303 ignore optional]; UnsuccessfulOutcome S1SetupFailure "S1SetupFailure" [2 ignore mandatory, 65 ignore optional, 58 ignore optional]; }
    18 UEContextReleaseRequest ignore { InitiatingMessage UEContextReleaseRequest "UEContextReleaseRequest" [0 reject mandatory, 8 reject mandatory, 2 ignore mandatory, 164 reject optional, 264 ignore optional]; }
    19 DownlinkS1cdma2000tunnelling ignore { InitiatingMessage DownlinkS1cdma2000tunnelling "DownlinkS1cdma2000tunnelling" [0 reject mandatory, 8 reject mandatory, 12 ignore optional, 83 ignore optional, 71 reject mandatory, 70 reject mandatory]; }
    20 UplinkS1cdma2000tunnelling ignore { InitiatingMessage UplinkS1cdma2000tunnelling "UplinkS1cdma2000tunnelling" [0 reject mandatory, 8 reject mandatory, 71 reject mandatory, 72 reject mandatory, 84 ignore optional, 102 reject optional, 97 reject optional, 70 reject mandatory, 140 ignore optional]; }
    21 UEContextModification reject { InitiatingMessage UEContextModificationRequest "UEContextModificationRequest" [0 reject mandatory, 8 reject mandatory, 73 reject optional, 106 ignore optional, 66 ignore optional, 108 reject optional, 107 reject optional, 146 ignore optional, 159 ignore optional, 187 ignore conditional, 195 ignore optional, 124 ignore optional, 243 ignore optional, 240 ignore optional, 248 ignore optional, 269 ignore optional, 277 ignore optional, 299 ignore optional, 301 ignore optional, 306 ignore optional, 307 ignore optional, 308 ignore optional, 314 reject optional, 355 ignore optional]; SuccessfulOutcome UEContextModificationResponse "UEContextModificationResponse" [0 ignore mandatory, 8 ignore mandatory, 58 ignore optional]; UnsuccessfulOutcome UEContextModificationFailure "UEContextModificationFailure" [0 ignore mandatory, 8 ignore mandatory, 2 ignore mandatory, 58 ignore optional]; }
    22 UECapabilityInfoIndication ignore { InitiatingMessage UECapabilityInfoIndication "UECapabilityInfoIndication" [0 reject mandatory, 8 reject mandatory, 74 ignore mandatory, 198 ignore optional, 263 ignore optional, 272 ignore optional, 315 ignore optional, 327 ignore optional]; }
    23 UEContextRelease reject { InitiatingMessage UEContextReleaseCommand "UEContextReleaseCommand" [99 reject mandatory, 2 ignore mandatory]; SuccessfulOutcome UEContextReleaseComplete "UEContextReleaseComplete" [0 ignore mandatory, 8 ignore mandatory, 58 ignore optional, 189 ignore optional, 213 ignore optional, 212 ignore optional, 264 ignore optional, 297 ignore optional]; }
    24 eNBStatusTransfer ignore { InitiatingMessage ENBStatusTransfer "ENBStatusTransfer" [0 reject mandatory, 8 reject mandatory, 90 reject mandatory]; }
    25 MMEStatusTransfer ignore { InitiatingMessage MMEStatusTransfer "MMEStatusTransfer" [0 reject mandatory, 8 reject mandatory, 90 reject mandatory]; }
    26 DeactivateTrace ignore { InitiatingMessage DeactivateTrace "DeactivateTrace" [0 reject mandatory, 8 reject mandatory, 86 ignore mandatory]; }
    27 TraceStart ignore { InitiatingMessage TraceStart "TraceStart" [0 reject mandatory, 8 reject mandatory, 25 ignore mandatory]; }
    28 TraceFailureIndication ignore { InitiatingMessage TraceFailureIndication "TraceFailureIndication" [0 reject mandatory, 8 reject mandatory, 86 ignore mandatory, 2 ignore mandatory]; }
    29 ENBConfigurationUpdate reject { InitiatingMessage ENBConfigurationUpdate "ENBConfigurationUpdate" [60 ignore optional, 64 reject optional, 128 reject optional, 137 ignore optional, 234 ignore optional, 292 ignore optional, 293 ignore optional]; SuccessfulOutcome ENBConfigurationUpdateAcknowledge "ENBConfigurationUpdateAcknowledge" [58 ignore optional]; UnsuccessfulOutcome ENBConfigurationUpdateFailure "ENBConfigurationUpdateFailure" [2 ignore mandatory, 65 ignore optional, 58 ignore optional]; }
    30 MMEConfigurationUpdate reject { InitiatingMessage MMEConfigurationUpdate "MMEConfigurationUpdate" [61 ignore optional, 105 reject optional, 87 reject optional, 247 ignore optional]; SuccessfulOutcome MMEConfigurationUpdateAcknowledge "MMEConfigurationUpdateAcknowledge" [58 ignore optional]; UnsuccessfulOutcome MMEConfigurationUpdateFailure "MMEConfigurationUpdateFailure" [2 ignore mandatory, 65 ignore optional, 58 ignore optional]; }
    31 LocationReportingControl ignore { InitiatingMessage LocationReportingControl "LocationReportingControl" [0 reject mandatory, 8 reject mandatory, 98 ignore mandatory]; }
    32 LocationReportingFailureIndication ignore { InitiatingMessage LocationReportingFailureIndication "LocationReportingFailureIndication" [0 reject mandatory, 8 reject mandatory, 2 ignore mandatory]; }
    33 LocationReport ignore { InitiatingMessage LocationReport "LocationReport" [0 reject mandatory, 8 reject mandatory, 100 ignore mandatory, 67 ignore mandatory, 98 ignore mandatory, 288 ignore optional, 339 ignore optional]; }
    34 OverloadStart ignore { InitiatingMessage OverloadStart "OverloadStart" [101 reject mandatory, 154 ignore optional, 161 ignore optional]; }
    35 OverloadStop reject { InitiatingMessage OverloadStop "OverloadStop" [154 ignore optional]; }
    36 WriteReplaceWarning reject { InitiatingMessage WriteReplaceWarningRequest "WriteReplaceWarningRequest" [111 reject mandatory, 112 reject mandatory, 113 ignore optional, 114 reject mandatory, 144 reject optional, 115 reject mandatory, 116 ignore optional, 117 ignore optional, 118 ignore optional, 119 ignore optional, 142 reject optional, 286 ignore optional]; SuccessfulOutcome WriteReplaceWarningResponse "WriteReplaceWarningResponse" [111 reject mandatory, 112 reject mandatory, 120 ignore optional, 58 ignore optional]; }
    37 eNBDirectInformationTransfer ignore { InitiatingMessage ENBDirectInformationTransfer "ENBDirectInformationTransfer" [121 reject mandatory]; }
    38 MMEDirectInformationTransfer ignore { InitiatingMessage MMEDirectInformationTransfer "MMEDirectInformationTransfer" [122 reject mandatory]; }
    39 PrivateMessage ignore { InitiatingMessage PrivateMessage "PrivateMessage"; }
    40 eNBConfigurationTransfer ignore { InitiatingMessage ENBConfigurationTransfer "ENBConfigurationTransfer" [129 ignore optional, 294 ignore optional, 310 ignore optional]; }
    41 MMEConfigurationTransfer ignore { InitiatingMessage MMEConfigurationTransfer "MMEConfigurationTransfer" [130 ignore optional, 295 ignore optional, 309 ignore optional]; }
    42 CellTrafficTrace ignore { InitiatingMessage CellTrafficTrace "CellTrafficTrace" [0 reject mandatory, 8 reject mandatory, 86 ignore mandatory, 100 ignore mandatory, 131 ignore mandatory, 166 ignore optional]; }
    43 Kill reject { InitiatingMessage KillRequest "KillRequest" [111 reject mandatory, 112 reject mandatory, 113 ignore optional, 191 reject optional]; SuccessfulOutcome KillResponse "KillResponse" [111 reject mandatory, 112 reject mandatory, 141 ignore optional, 58 ignore optional]; }
    44 downlinkUEAssociatedLPPaTransport ignore { InitiatingMessage DownlinkUEAssociatedLPPaTransport "DownlinkUEAssociatedLPPaTransport" [0 reject mandatory, 8 reject mandatory, 148 reject mandatory, 147 reject mandatory]; }
    45 uplinkUEAssociatedLPPaTransport ignore { InitiatingMessage UplinkUEAssociatedLPPaTransport "UplinkUEAssociatedLPPaTransport" [0 reject mandatory, 8 reject mandatory, 148 reject mandatory, 147 reject mandatory]; }
    46 downlinkNonUEAssociatedLPPaTransport ignore { InitiatingMessage DownlinkNonUEAssociatedLPPaTransport "DownlinkNonUEAssociatedLPPaTransport" [148 reject mandatory, 147 reject mandatory]; }
    47 uplinkNonUEAssociatedLPPaTransport ignore { InitiatingMessage UplinkNonUEAssociatedLPPaTransport "UplinkNonUEAssociatedLPPaTransport" [148 reject mandatory, 147 reject mandatory]; }
    48 UERadioCapabilityMatch reject { InitiatingMessage UERadioCapabilityMatchRequest "UERadioCapabilityMatchRequest" [0 reject mandatory, 8 reject mandatory, 74 ignore optional, 314 reject optional]; SuccessfulOutcome UERadioCapabilityMatchResponse "UERadioCapabilityMatchResponse" [0 ignore mandatory, 8 ignore mandatory, 169 reject mandatory, 58 ignore optional]; }
    49 PWSRestartIndication ignore { InitiatingMessage PWSRestartIndication "PWSRestartIndication" [182 reject mandatory, 59 reject mandatory, 188 reject mandatory, 190 reject optional]; }
    50 E_RABModificationIndication reject { InitiatingMessage ERABModificationIndication "E-RABModificationIndication" [0 reject mandatory, 8 reject mandatory, 199 reject mandatory, 201 reject optional, 226 reject optional, 176 ignore optional, 264 ignore optional, 189 ignore optional]; SuccessfulOutcome ERABModificationConfirm "E-RABModificationConfirm" [0 ignore mandatory, 8 ignore mandatory, 203 ignore optional, 205 ignore optional, 210 ignore optional, 58 ignore optional, 146 ignore optional]; }
    51 PWSFailureIndication ignore { InitiatingMessage PWSFailureIndication "PWSFailureIndication" [222 reject mandatory, 59 reject mandatory]; }
    52 RerouteNASRequest reject { InitiatingMessage RerouteNASRequest "RerouteNASRequest" [8 reject mandatory, 0 ignore optional, 225 reject mandatory, 223 reject mandatory, 224 ignore optional, 230 ignore optional]; }
    53 UEContextModificationIndication reject { InitiatingMessage UEContextModificationIndication "UEContextModificationIndication" [0 reject mandatory, 8 reject mandatory, 226 reject optional]; SuccessfulOutcome UEContextModificationConfirm "UEContextModificationConfirm" [0 ignore mandatory, 8 ignore mandatory, 146 ignore optional, 58 ignore optional]; }
    54 ConnectionEstablishmentIndication reject { InitiatingMessage ConnectionEstablishmentIndication "ConnectionEstablishmentIndication" [0 ignore mandatory, 8 ignore mandatory, 74 ignore optional, 251 ignore optional, 253 ignore optional, 271 ignore optional, 280 ignore optional, 278 ignore optional, 252 ignore optional, 314 reject optional, 192 ignore optional, 354 ignore optional]; }
    55 UEContextSuspend reject { InitiatingMessage UEContextSuspendRequest "UEContextSuspendRequest" [0 reject mandatory, 8 reject mandatory, 213 ignore optional, 212 ignore optional, 264 ignore optional, 189 ignore optional, 297 ignore optional]; SuccessfulOutcome UEContextSuspendResponse "UEContextSuspendResponse" [0 ignore mandatory, 8 ignore mandatory, 58 ignore optional, 40 reject optional]; }
    56 UEContextResume reject { InitiatingMessage UEContextResumeRequest "UEContextResumeRequest" [0 reject mandatory, 8 reject mandatory, 235 reject optional, 245 ignore optional]; SuccessfulOutcome UEContextResumeResponse "UEContextResumeResponse" [0 ignore mandatory, 8 ignore mandatory, 237 reject optional, 58 ignore optional, 40 reject optional, 283 ignore optional]; UnsuccessfulOutcome UEContextResumeFailure "UEContextResumeFailure" [0 ignore mandatory, 8 ignore mandatory, 2 ignore mandatory, 58 ignore optional]; }
    57 NASDeliveryIndication ignore { InitiatingMessage NASDeliveryIndication "NASDeliveryIndication" [0 reject mandatory, 8 reject mandatory]; }
    58 RetrieveUEInformation reject { InitiatingMessage RetrieveUEInformation "RetrieveUEInformation" [96 reject mandatory]; }
    59 UEInformationTransfer reject { InitiatingMessage UEInformationTransfer "UEInformationTransfer" [96 reject mandatory, 252 ignore optional, 74 ignore optional, 278 ignore optional, 283 ignore optional, 192 ignore optional]; }
    60 eNBCPRelocationIndication reject { InitiatingMessage ENBCPRelocationIndication "ENBCPRelocationIndication" [8 reject mandatory, 96 reject mandatory, 100 ignore mandatory, 67 ignore mandatory, 254 reject mandatory, 339 ignore optional]; }
    61 MMECPRelocationIndication reject { InitiatingMessage MMECPRelocationIndication "MMECPRelocationIndication" [0 reject mandatory, 8 reject mandatory]; }
    62 SecondaryRATDataUsageReport ignore { InitiatingMessage SecondaryRATDataUsageReport "SecondaryRATDataUsageReport" [0 ignore mandatory, 8 ignore mandatory, 264 ignore mandatory, 266 ignore optional, 189 ignore optional, 297 ignore optional]; }
    63 UERadioCapabilityIDMapping reject { InitiatingMessage UERadioCapabilityIDMappingRequest "UERadioCapabilityIDMappingRequest" [314 reject mandatory]; SuccessfulOutcome UERadioCapabilityIDMappingResponse "UERadioCapabilityIDMappingResponse" [314 reject mandatory, 74 ignore mandatory, 58 ignore optional]; }
    64 HandoverSuccess ignore { InitiatingMessage HandoverSuccess "HandoverSuccess" [0 reject mandatory, 8 reject mandatory]; }
    65 eNBEarlyStatusTransfer reject { InitiatingMessage ENBEarlyStatusTransfer "ENBEarlyStatusTransfer" [0 reject mandatory, 8 reject mandatory, 321 reject mandatory]; }
    66 MMEEarlyStatusTransfer ignore { InitiatingMessage MMEEarlyStatusTransfer "MMEEarlyStatusTransfer" [0 reject mandatory, 8 reject mandatory, 321 reject mandatory]; }
    67 S1Removal reject { InitiatingMessage S1RemovalRequest "S1RemovalRequest" [59 reject mandatory]; SuccessfulOutcome S1RemovalResponse "S1RemovalResponse" [61 ignore optional, 58 ignore optional]; UnsuccessfulOutcome S1RemovalFailure "S1RemovalFailure" [2 ignore mandatory, 58 ignore optional]; }
}
oxirush_aper::__ies! {
    protocol S1ap "S1AP";
    0 "MME-UE-S1AP-ID" crate::s1ap::MMEUES1APID => MME_UE_S1AP_ID;
    1 "HandoverType" crate::s1ap::HandoverType => HandoverType;
    2 "Cause" crate::s1ap::Cause => Cause;
    4 "TargetID" crate::s1ap::TargetID => TargetID;
    8 "eNB-UE-S1AP-ID" crate::s1ap::ENBUES1APID => eNB_UE_S1AP_ID;
    12 "E-RABSubjecttoDataForwardingList" crate::s1ap::ERABSubjecttoDataForwardingList<'a> => E_RABSubjecttoDataForwardingList;
    13 "E-RABtoReleaseListHOCmd" crate::s1ap::ERABList<'a> => E_RABtoReleaseListHOCmd;
    14 ignore "E-RABDataForwardingItem" crate::s1ap::ERABDataForwardingItem<'a> => E_RABDataForwardingItem;
    15 ignore "E-RABReleaseItemBearerRelComp" crate::s1ap::ERABReleaseItemBearerRelComp => E_RABReleaseItemBearerRelComp;
    16 "E-RABToBeSetupListBearerSUReq" crate::s1ap::ERABToBeSetupListBearerSUReq<'a> => E_RABToBeSetupListBearerSUReq;
    17 reject "E-RABToBeSetupItemBearerSUReq" crate::s1ap::ERABToBeSetupItemBearerSUReq<'a> => E_RABToBeSetupItemBearerSUReq;
    18 "E-RABAdmittedList" crate::s1ap::ERABAdmittedList<'a> => E_RABAdmittedList;
    19 "E-RABFailedToSetupListHOReqAck" crate::s1ap::ERABFailedtoSetupListHOReqAck<'a> => E_RABFailedToSetupListHOReqAck;
    20 ignore "E-RABAdmittedItem" crate::s1ap::ERABAdmittedItem<'a> => E_RABAdmittedItem;
    21 ignore "E-RABFailedtoSetupItemHOReqAck" crate::s1ap::ERABFailedToSetupItemHOReqAck => E_RABFailedtoSetupItemHOReqAck;
    22 "E-RABToBeSwitchedDLList" crate::s1ap::ERABToBeSwitchedDLList<'a> => E_RABToBeSwitchedDLList;
    23 reject "E-RABToBeSwitchedDLItem" crate::s1ap::ERABToBeSwitchedDLItem<'a> => E_RABToBeSwitchedDLItem;
    24 "E-RABToBeSetupListCtxtSUReq" crate::s1ap::ERABToBeSetupListCtxtSUReq<'a> => E_RABToBeSetupListCtxtSUReq;
    25 "TraceActivation" crate::s1ap::TraceActivation<'a> => TraceActivation;
    26 "NAS-PDU" crate::s1ap::NASPDU<'a> => NAS_PDU;
    27 reject "E-RABToBeSetupItemHOReq" crate::s1ap::ERABToBeSetupItemHOReq<'a> => E_RABToBeSetupItemHOReq;
    28 "E-RABSetupListBearerSURes" crate::s1ap::ERABSetupListBearerSURes<'a> => E_RABSetupListBearerSURes;
    29 "E-RABFailedToSetupListBearerSURes" crate::s1ap::ERABList<'a> => E_RABFailedToSetupListBearerSURes;
    30 "E-RABToBeModifiedListBearerModReq" crate::s1ap::ERABToBeModifiedListBearerModReq<'a> => E_RABToBeModifiedListBearerModReq;
    31 "E-RABModifyListBearerModRes" crate::s1ap::ERABModifyListBearerModRes<'a> => E_RABModifyListBearerModRes;
    32 "E-RABFailedToModifyList" crate::s1ap::ERABList<'a> => E_RABFailedToModifyList;
    33 "E-RABToBeReleasedList" crate::s1ap::ERABList<'a> => E_RABToBeReleasedList;
    34 "E-RABFailedToReleaseList" crate::s1ap::ERABList<'a> => E_RABFailedToReleaseList;
    35 ignore "E-RABItem" crate::s1ap::ERABItem => E_RABItem;
    36 reject "E-RABToBeModifiedItemBearerModReq" crate::s1ap::ERABToBeModifiedItemBearerModReq<'a> => E_RABToBeModifiedItemBearerModReq;
    37 ignore "E-RABModifyItemBearerModRes" crate::s1ap::ERABModifyItemBearerModRes => E_RABModifyItemBearerModRes;
    39 ignore "E-RABSetupItemBearerSURes" crate::s1ap::ERABSetupItemBearerSURes<'a> => E_RABSetupItemBearerSURes;
    40 "SecurityContext" @key crate::s1ap::SecurityContext<'a> => SecurityContext;
    41 "HandoverRestrictionList" crate::s1ap::HandoverRestrictionList => HandoverRestrictionList;
    43 "UEPagingID" crate::s1ap::UEPagingID<'a> => UEPagingID;
    44 "pagingDRX" crate::s1ap::PagingDRX => pagingDRX;
    46 "TAIList" crate::s1ap::TAIList<'a> => TAIList;
    47 ignore "TAIItem" crate::s1ap::TAIItem => TAIItem;
    48 "E-RABFailedToSetupListCtxtSURes" crate::s1ap::ERABList<'a> => E_RABFailedToSetupListCtxtSURes;
    50 ignore "E-RABSetupItemCtxtSURes" crate::s1ap::ERABSetupItemCtxtSURes<'a> => E_RABSetupItemCtxtSURes;
    51 "E-RABSetupListCtxtSURes" crate::s1ap::ERABSetupListCtxtSURes<'a> => E_RABSetupListCtxtSURes;
    52 reject "E-RABToBeSetupItemCtxtSUReq" crate::s1ap::ERABToBeSetupItemCtxtSUReq<'a> => E_RABToBeSetupItemCtxtSUReq;
    53 "E-RABToBeSetupListHOReq" crate::s1ap::ERABToBeSetupListHOReq<'a> => E_RABToBeSetupListHOReq;
    58 "CriticalityDiagnostics" crate::s1ap::CriticalityDiagnostics => CriticalityDiagnostics;
    59 "Global-ENB-ID" crate::s1ap::GlobalENBID => Global_ENB_ID;
    60 "eNBname" crate::s1ap::ENBname<'a> => eNBname;
    61 "MMEname" crate::s1ap::MMEname<'a> => MMEname;
    64 "SupportedTAs" crate::s1ap::SupportedTAs => SupportedTAs;
    65 "TimeToWait" crate::s1ap::TimeToWait => TimeToWait;
    66 "uEaggregateMaximumBitrate" crate::s1ap::UEAggregateMaximumBitrate => uEaggregateMaximumBitrate;
    67 "TAI" crate::s1ap::TAI => TAI;
    69 "E-RABReleaseListBearerRelComp" crate::s1ap::ERABReleaseListBearerRelComp<'a> => E_RABReleaseListBearerRelComp;
    70 "cdma2000PDU" crate::s1ap::Cdma2000PDU<'a> => cdma2000PDU;
    71 "cdma2000RATType" crate::s1ap::Cdma2000RATType => cdma2000RATType;
    72 "cdma2000SectorID" crate::s1ap::Cdma2000SectorID<'a> => cdma2000SectorID;
    73 "SecurityKey" @key crate::s1ap::SecurityKey<'a> => SecurityKey;
    74 "UERadioCapability" crate::s1ap::UERadioCapability<'a> => UERadioCapability;
    75 "GUMMEI-ID" crate::s1ap::GUMMEI => GUMMEI_ID;
    78 ignore "E-RABInformationListItem" crate::s1ap::ERABInformationListItem => E_RABInformationListItem;
    79 "Direct-Forwarding-Path-Availability" crate::s1ap::DirectForwardingPathAvailability => Direct_Forwarding_Path_Availability;
    80 "UEIdentityIndexValue" crate::s1ap::UEIdentityIndexValue => UEIdentityIndexValue;
    83 "cdma2000HOStatus" crate::s1ap::Cdma2000HOStatus => cdma2000HOStatus;
    84 "cdma2000HORequiredIndication" crate::s1ap::Cdma2000HORequiredIndication => cdma2000HORequiredIndication;
    86 "E-UTRAN-Trace-ID" crate::s1ap::EUTRANTraceID => E_UTRAN_Trace_ID;
    87 "RelativeMMECapacity" crate::s1ap::RelativeMMECapacity => RelativeMMECapacity;
    88 "SourceMME-UE-S1AP-ID" crate::s1ap::MMEUES1APID => SourceMME_UE_S1AP_ID;
    89 ignore "Bearers-SubjectToStatusTransfer-Item" crate::s1ap::BearersSubjectToStatusTransferItem<'a> => Bearers_SubjectToStatusTransfer_Item;
    90 "eNB-StatusTransfer-TransparentContainer" crate::s1ap::ENBStatusTransferTransparentContainer<'a> => eNB_StatusTransfer_TransparentContainer;
    91 "UE-associatedLogicalS1-ConnectionItem" crate::s1ap::UEAssociatedLogicalS1ConnectionItem => UE_associatedLogicalS1_ConnectionItem;
    92 "ResetType" crate::s1ap::ResetType<'a> => ResetType;
    93 "UE-associatedLogicalS1-ConnectionListResAck" crate::s1ap::UEAssociatedLogicalS1ConnectionListResAck<'a> => UE_associatedLogicalS1_ConnectionListResAck;
    94 ignore "E-RABToBeSwitchedULItem" crate::s1ap::ERABToBeSwitchedULItem<'a> => E_RABToBeSwitchedULItem;
    95 "E-RABToBeSwitchedULList" crate::s1ap::ERABToBeSwitchedULList<'a> => E_RABToBeSwitchedULList;
    96 "S-TMSI" crate::s1ap::STMSI => S_TMSI;
    97 "cdma2000OneXRAND" crate::s1ap::Cdma2000OneXRAND<'a> => cdma2000OneXRAND;
    98 "RequestType" crate::s1ap::RequestType => RequestType;
    99 "UE-S1AP-IDs" crate::s1ap::UES1APIDs => UE_S1AP_IDs;
    100 "EUTRAN-CGI" crate::s1ap::EUTRANCGI => EUTRAN_CGI;
    101 "OverloadResponse" crate::s1ap::OverloadResponse => OverloadResponse;
    102 "cdma2000OneXSRVCCInfo" crate::s1ap::Cdma2000OneXSRVCCInfo<'a> => cdma2000OneXSRVCCInfo;
    104 "Source-ToTarget-TransparentContainer" crate::s1ap::SourceToTargetTransparentContainer<'a> => Source_ToTarget_TransparentContainer;
    105 "ServedGUMMEIs" crate::s1ap::ServedGUMMEIs => ServedGUMMEIs;
    106 "SubscriberProfileIDforRFP" crate::s1ap::SubscriberProfileIDforRFP => SubscriberProfileIDforRFP;
    107 "UESecurityCapabilities" crate::s1ap::UESecurityCapabilities<'a> => UESecurityCapabilities;
    108 "CSFallbackIndicator" crate::s1ap::CSFallbackIndicator => CSFallbackIndicator;
    109 "CNDomain" crate::s1ap::CNDomain => CNDomain;
    110 "E-RABReleasedList" crate::s1ap::ERABList<'a> => E_RABReleasedList;
    111 "MessageIdentifier" crate::s1ap::MessageIdentifier => MessageIdentifier;
    112 "SerialNumber" crate::s1ap::SerialNumber => SerialNumber;
    113 "WarningAreaList" crate::s1ap::WarningAreaList => WarningAreaList;
    114 "RepetitionPeriod" crate::s1ap::RepetitionPeriod => RepetitionPeriod;
    115 "NumberofBroadcastRequest" crate::s1ap::NumberofBroadcastRequest => NumberofBroadcastRequest;
    116 "WarningType" crate::s1ap::WarningType => WarningType;
    117 "WarningSecurityInfo" crate::s1ap::WarningSecurityInfo => WarningSecurityInfo;
    118 "DataCodingScheme" crate::s1ap::DataCodingScheme => DataCodingScheme;
    119 "WarningMessageContents" crate::s1ap::WarningMessageContents<'a> => WarningMessageContents;
    120 "BroadcastCompletedAreaList" crate::s1ap::BroadcastCompletedAreaList => BroadcastCompletedAreaList;
    121 "Inter-SystemInformationTransferTypeEDT" crate::s1ap::InterSystemInformationTransferType<'a> => Inter_SystemInformationTransferTypeEDT;
    122 "Inter-SystemInformationTransferTypeMDT" crate::s1ap::InterSystemInformationTransferType<'a> => Inter_SystemInformationTransferTypeMDT;
    123 "Target-ToSource-TransparentContainer" crate::s1ap::TargetToSourceTransparentContainer<'a> => Target_ToSource_TransparentContainer;
    124 "SRVCCOperationPossible" crate::s1ap::SRVCCOperationPossible => SRVCCOperationPossible;
    125 "SRVCCHOIndication" crate::s1ap::SRVCCHOIndication => SRVCCHOIndication;
    127 "CSG-Id" crate::s1ap::CSGId => CSG_Id;
    128 "CSG-IdList" crate::s1ap::CSGIdList => CSG_IdList;
    129 "SONConfigurationTransferECT" crate::s1ap::SONConfigurationTransfer<'a> => SONConfigurationTransferECT;
    130 "SONConfigurationTransferMCT" crate::s1ap::SONConfigurationTransfer<'a> => SONConfigurationTransferMCT;
    131 "TraceCollectionEntityIPAddress" crate::s1ap::TransportLayerAddress<'a> => TraceCollectionEntityIPAddress;
    132 "MSClassmark2" crate::s1ap::MSClassmark2<'a> => MSClassmark2;
    133 "MSClassmark3" crate::s1ap::MSClassmark3<'a> => MSClassmark3;
    134 "RRC-Establishment-Cause" crate::s1ap::RRCEstablishmentCause => RRC_Establishment_Cause;
    135 "NASSecurityParametersfromE-UTRAN" crate::s1ap::NASSecurityParametersfromEUTRAN<'a> => NASSecurityParametersfromE_UTRAN;
    136 "NASSecurityParameterstoE-UTRAN" crate::s1ap::NASSecurityParameterstoEUTRAN<'a> => NASSecurityParameterstoE_UTRAN;
    137 "DefaultPagingDRX" crate::s1ap::PagingDRX => DefaultPagingDRX;
    138 "Source-ToTarget-TransparentContainer-Secondary" crate::s1ap::SourceToTargetTransparentContainer<'a> => Source_ToTarget_TransparentContainer_Secondary;
    139 "Target-ToSource-TransparentContainer-Secondary" crate::s1ap::TargetToSourceTransparentContainer<'a> => Target_ToSource_TransparentContainer_Secondary;
    140 "EUTRANRoundTripDelayEstimationInfo" crate::s1ap::EUTRANRoundTripDelayEstimationInfo => EUTRANRoundTripDelayEstimationInfo;
    141 "BroadcastCancelledAreaList" crate::s1ap::BroadcastCancelledAreaList => BroadcastCancelledAreaList;
    142 "ConcurrentWarningMessageIndicator" crate::s1ap::ConcurrentWarningMessageIndicator => ConcurrentWarningMessageIndicator;
    143 "Data-Forwarding-Not-Possible" crate::s1ap::DataForwardingNotPossible;
    144 "ExtendedRepetitionPeriod" crate::s1ap::ExtendedRepetitionPeriod => ExtendedRepetitionPeriod;
    145 "CellAccessMode" crate::s1ap::CellAccessMode => CellAccessMode;
    146 "CSGMembershipStatus" crate::s1ap::CSGMembershipStatus => CSGMembershipStatus;
    147 "LPPa-PDU" crate::s1ap::LPPaPDU<'a> => LPPa_PDU;
    148 "Routing-ID" crate::s1ap::RoutingID => Routing_ID;
    149 "Time-Synchronisation-Info" crate::s1ap::TimeSynchronisationInfo;
    150 "PS-ServiceNotAvailable" crate::s1ap::PSServiceNotAvailable => PS_ServiceNotAvailable;
    151 "PagingPriority" crate::s1ap::PagingPriority => PagingPriority;
    152 "x2TNLConfigurationInfo" crate::s1ap::X2TNLConfigurationInfo<'a>;
    153 "eNBX2ExtendedTransportLayerAddresses" crate::s1ap::ENBX2ExtTLAs<'a>;
    154 "GUMMEIList" crate::s1ap::GUMMEIList => GUMMEIList;
    155 "GW-TransportLayerAddress" crate::s1ap::TransportLayerAddress<'a> => GW_TransportLayerAddress;
    156 "Correlation-ID" crate::s1ap::CorrelationID;
    157 "SourceMME-GUMMEI" crate::s1ap::GUMMEI => SourceMME_GUMMEI;
    158 "MME-UE-S1AP-ID-2" crate::s1ap::MMEUES1APID => MME_UE_S1AP_ID_2;
    159 "RegisteredLAI" crate::s1ap::LAI => RegisteredLAI;
    160 "RelayNode-Indicator" crate::s1ap::RelayNodeIndicator => RelayNode_Indicator;
    161 "TrafficLoadReductionIndication" crate::s1ap::TrafficLoadReductionIndication => TrafficLoadReductionIndication;
    162 "MDTConfiguration" crate::s1ap::MDTConfiguration<'a>;
    163 "MMERelaySupportIndicator" crate::s1ap::MMERelaySupportIndicator => MMERelaySupportIndicator;
    164 "GWContextReleaseIndication" crate::s1ap::GWContextReleaseIndication => GWContextReleaseIndication;
    165 "ManagementBasedMDTAllowed" crate::s1ap::ManagementBasedMDTAllowed => ManagementBasedMDTAllowed;
    166 "PrivacyIndicator" crate::s1ap::PrivacyIndicator => PrivacyIndicator;
    167 "Time-UE-StayedInCell-EnhancedGranularity" crate::s1ap::TimeUEStayedInCellEnhancedGranularity;
    168 "HO-Cause" crate::s1ap::Cause;
    169 "VoiceSupportMatchIndicator" crate::s1ap::VoiceSupportMatchIndicator => VoiceSupportMatchIndicator;
    170 "GUMMEIType" crate::s1ap::GUMMEIType => GUMMEIType;
    171 "M3Configuration" crate::s1ap::M3Configuration;
    172 "M4Configuration" crate::s1ap::M4Configuration;
    173 "M5Configuration" crate::s1ap::M5Configuration;
    174 "MDT-Location-Info" crate::s1ap::MDTLocationInfo;
    175 "MobilityInformation" crate::s1ap::MobilityInformation;
    176 "Tunnel-Information-for-BBF" crate::s1ap::TunnelInformation<'a> => Tunnel_Information_for_BBF;
    177 "ManagementBasedMDTPLMNList" crate::s1ap::MDTPLMNList => ManagementBasedMDTPLMNList;
    178 "SignallingBasedMDTPLMNList" crate::s1ap::MDTPLMNList;
    179 "ULCOUNTValueExtended" crate::s1ap::COUNTValueExtended;
    180 "DLCOUNTValueExtended" crate::s1ap::COUNTValueExtended;
    181 "ReceiveStatusOfULPDCPSDUsExtended" crate::s1ap::ReceiveStatusOfULPDCPSDUsExtended<'a>;
    182 "ECGIListForRestart" crate::s1ap::ECGIListForRestart => ECGIListForRestart;
    183 "SIPTO-Correlation-ID" crate::s1ap::CorrelationID;
    184 "SIPTO-L-GW-TransportLayerAddress" crate::s1ap::TransportLayerAddress<'a> => SIPTO_L_GW_TransportLayerAddress;
    185 "TransportInformation" crate::s1ap::TransportInformation<'a>;
    186 "LHN-ID" crate::s1ap::LHNID<'a> => LHN_ID;
    187 "AdditionalCSFallbackIndicator" crate::s1ap::AdditionalCSFallbackIndicator => AdditionalCSFallbackIndicator;
    188 "TAIListForRestart" crate::s1ap::TAIListForRestart => TAIListForRestart;
    189 "UserLocationInformation" crate::s1ap::UserLocationInformation => UserLocationInformation;
    190 "EmergencyAreaIDListForRestart" crate::s1ap::EmergencyAreaIDListForRestart => EmergencyAreaIDListForRestart;
    191 "KillAllWarningMessages" crate::s1ap::KillAllWarningMessages => KillAllWarningMessages;
    192 "Masked-IMEISV" crate::s1ap::MaskedIMEISV => Masked_IMEISV;
    193 "eNBIndirectX2TransportLayerAddresses" crate::s1ap::ENBIndirectX2TransportLayerAddresses<'a>;
    194 "uE-HistoryInformationFromTheUE" crate::s1ap::UEHistoryInformationFromTheUE<'a>;
    195 "ProSeAuthorized" crate::s1ap::ProSeAuthorized => ProSeAuthorized;
    196 "ExpectedUEBehaviour" crate::s1ap::ExpectedUEBehaviour => ExpectedUEBehaviour;
    197 ignore "LoggedMBSFNMDT" crate::s1ap::LoggedMBSFNMDT => LoggedMBSFNMDT;
    198 "UERadioCapabilityForPaging" crate::s1ap::UERadioCapabilityForPaging<'a> => UERadioCapabilityForPaging;
    199 "E-RABToBeModifiedListBearerModInd" crate::s1ap::ERABToBeModifiedListBearerModInd<'a> => E_RABToBeModifiedListBearerModInd;
    200 reject "E-RABToBeModifiedItemBearerModInd" crate::s1ap::ERABToBeModifiedItemBearerModInd<'a> => E_RABToBeModifiedItemBearerModInd;
    201 "E-RABNotToBeModifiedListBearerModInd" crate::s1ap::ERABNotToBeModifiedListBearerModInd<'a> => E_RABNotToBeModifiedListBearerModInd;
    202 reject "E-RABNotToBeModifiedItemBearerModInd" crate::s1ap::ERABNotToBeModifiedItemBearerModInd<'a> => E_RABNotToBeModifiedItemBearerModInd;
    203 "E-RABModifyListBearerModConf" crate::s1ap::ERABModifyListBearerModConf<'a> => E_RABModifyListBearerModConf;
    204 ignore "E-RABModifyItemBearerModConf" crate::s1ap::ERABModifyItemBearerModConf => E_RABModifyItemBearerModConf;
    205 "E-RABFailedToModifyListBearerModConf" crate::s1ap::ERABList<'a> => E_RABFailedToModifyListBearerModConf;
    206 ignore "SON-Information-Report" crate::s1ap::SONInformationReport<'a> => SON_Information_Report;
    207 "Muting-Availability-Indication" crate::s1ap::MutingAvailabilityIndication;
    208 "Muting-Pattern-Information" crate::s1ap::MutingPatternInformation;
    209 "Synchronisation-Information" crate::s1ap::SynchronisationInformation;
    210 "E-RABToBeReleasedListBearerModConf" crate::s1ap::ERABList<'a> => E_RABToBeReleasedListBearerModConf;
    211 "AssistanceDataForPaging" crate::s1ap::AssistanceDataForPaging<'a> => AssistanceDataForPaging;
    212 "CellIdentifierAndCELevelForCECapableUEs" crate::s1ap::CellIdentifierAndCELevelForCECapableUEs<'a> => CellIdentifierAndCELevelForCECapableUEs;
    213 "InformationOnRecommendedCellsAndENBsForPaging" crate::s1ap::InformationOnRecommendedCellsAndENBsForPaging<'a> => InformationOnRecommendedCellsAndENBsForPaging;
    214 ignore "RecommendedCellItem" crate::s1ap::RecommendedCellItem => RecommendedCellItem;
    215 ignore "RecommendedENBItem" crate::s1ap::RecommendedENBItem => RecommendedENBItem;
    216 "ProSeUEtoNetworkRelaying" crate::s1ap::ProSeUEtoNetworkRelaying;
    217 "ULCOUNTValuePDCP-SNlength18" crate::s1ap::COUNTvaluePDCPSNlength18;
    218 "DLCOUNTValuePDCP-SNlength18" crate::s1ap::COUNTvaluePDCPSNlength18;
    219 "ReceiveStatusOfULPDCPSDUsPDCP-SNlength18" crate::s1ap::ReceiveStatusOfULPDCPSDUsPDCPSNlength18<'a>;
    220 "M6Configuration" crate::s1ap::M6Configuration;
    221 "M7Configuration" crate::s1ap::M7Configuration;
    222 "PWSfailedECGIList" crate::s1ap::PWSfailedECGIList => PWSfailedECGIList;
    223 "MME-Group-ID" crate::s1ap::MMEGroupID => MME_Group_ID;
    224 "Additional-GUTI" crate::s1ap::AdditionalGUTI => Additional_GUTI;
    225 "S1-Message" oxirush_aper::Octets<'a> => S1_Message;
    226 "CSGMembershipInfo" crate::s1ap::CSGMembershipInfo => CSGMembershipInfo;
    227 "Paging-eDRXInformation" crate::s1ap::PagingEDRXInformation => Paging_eDRXInformation;
    228 "UE-RetentionInformation" crate::s1ap::UERetentionInformation => UE_RetentionInformation;
    230 "UE-Usage-Type" crate::s1ap::UEUsageType => UE_Usage_Type;
    231 "extended-UEIdentityIndexValue" crate::s1ap::ExtendedUEIdentityIndexValue => extended_UEIdentityIndexValue;
    232 "RAT-Type" crate::s1ap::RATType;
    233 "BearerType" crate::s1ap::BearerType;
    234 "NB-IoT-DefaultPagingDRX" crate::s1ap::NBIoTDefaultPagingDRX => NB_IoT_DefaultPagingDRX;
    235 "E-RABFailedToResumeListResumeReq" crate::s1ap::ERABFailedToResumeListResumeReq<'a> => E_RABFailedToResumeListResumeReq;
    236 reject "E-RABFailedToResumeItemResumeReq" crate::s1ap::ERABFailedToResumeItemResumeReq => E_RABFailedToResumeItemResumeReq;
    237 "E-RABFailedToResumeListResumeRes" crate::s1ap::ERABFailedToResumeListResumeRes<'a> => E_RABFailedToResumeListResumeRes;
    238 reject "E-RABFailedToResumeItemResumeRes" crate::s1ap::ERABFailedToResumeItemResumeRes => E_RABFailedToResumeItemResumeRes;
    239 "NB-IoT-Paging-eDRXInformation" crate::s1ap::NBIoTPagingEDRXInformation => NB_IoT_Paging_eDRXInformation;
    240 "V2XServicesAuthorized" crate::s1ap::V2XServicesAuthorized => V2XServicesAuthorized;
    241 "UEUserPlaneCIoTSupportIndicator" crate::s1ap::UEUserPlaneCIoTSupportIndicator => UEUserPlaneCIoTSupportIndicator;
    242 "CE-mode-B-SupportIndicator" crate::s1ap::CEModeBSupportIndicator => CE_mode_B_SupportIndicator;
    243 "SRVCCOperationNotPossible" crate::s1ap::SRVCCOperationNotPossible => SRVCCOperationNotPossible;
    244 "NB-IoT-UEIdentityIndexValue" crate::s1ap::NBIoTUEIdentityIndexValue => NB_IoT_UEIdentityIndexValue;
    245 "RRC-Resume-Cause" crate::s1ap::RRCEstablishmentCause => RRC_Resume_Cause;
    246 "DCN-ID" crate::s1ap::DCNID => DCN_ID;
    247 "ServedDCNs" crate::s1ap::ServedDCNs => ServedDCNs;
    248 "UESidelinkAggregateMaximumBitrate" crate::s1ap::UESidelinkAggregateMaximumBitrate => UESidelinkAggregateMaximumBitrate;
    249 "DLNASPDUDeliveryAckRequest" crate::s1ap::DLNASPDUDeliveryAckRequest => DLNASPDUDeliveryAckRequest;
    250 "Coverage-Level" crate::s1ap::CoverageLevel => Coverage_Level;
    251 "EnhancedCoverageRestricted" crate::s1ap::EnhancedCoverageRestricted => EnhancedCoverageRestricted;
    252 "UE-Level-QoS-Parameters" crate::s1ap::ERABLevelQoSParameters => UE_Level_QoS_Parameters;
    253 "DL-CP-SecurityInformation" crate::s1ap::DLCPSecurityInformation => DL_CP_SecurityInformation;
    254 "UL-CP-SecurityInformation" crate::s1ap::ULCPSecurityInformation => UL_CP_SecurityInformation;
    255 "extended-e-RAB-MaximumBitrateDL" crate::s1ap::ExtendedBitRate;
    256 "extended-e-RAB-MaximumBitrateUL" crate::s1ap::ExtendedBitRate;
    257 "extended-e-RAB-GuaranteedBitrateDL" crate::s1ap::ExtendedBitRate;
    258 "extended-e-RAB-GuaranteedBitrateUL" crate::s1ap::ExtendedBitRate;
    259 "extended-uEaggregateMaximumBitRateDL" crate::s1ap::ExtendedBitRate;
    260 "extended-uEaggregateMaximumBitRateUL" crate::s1ap::ExtendedBitRate;
    261 "NRrestrictioninEPSasSecondaryRAT" crate::s1ap::NRrestrictioninEPSasSecondaryRAT;
    262 "UEAppLayerMeasConfig" crate::s1ap::UEAppLayerMeasConfig<'a>;
    263 "UE-Application-Layer-Measurement-Capability" crate::s1ap::UEApplicationLayerMeasurementCapability => UE_Application_Layer_Measurement_Capability;
    264 "SecondaryRATDataUsageReportList" crate::s1ap::SecondaryRATDataUsageReportList<'a> => SecondaryRATDataUsageReportList;
    265 ignore "SecondaryRATDataUsageReportItem" crate::s1ap::SecondaryRATDataUsageReportItem<'a> => SecondaryRATDataUsageReportItem;
    266 "HandoverFlag" crate::s1ap::HandoverFlag => HandoverFlag;
    267 ignore "E-RABUsageReportItem" crate::s1ap::ERABUsageReportItem => E_RABUsageReportItem;
    268 "SecondaryRATDataUsageRequest" crate::s1ap::SecondaryRATDataUsageRequest => SecondaryRATDataUsageRequest;
    269 "NRUESecurityCapabilities" crate::s1ap::NRUESecurityCapabilities<'a> => NRUESecurityCapabilities;
    270 "UnlicensedSpectrumRestriction" crate::s1ap::UnlicensedSpectrumRestriction;
    271 "CE-ModeBRestricted" crate::s1ap::CEModeBRestricted => CE_ModeBRestricted;
    272 "LTE-M-Indication" crate::s1ap::LTEMIndication => LTE_M_Indication;
    273 "DownlinkPacketLossRate" crate::s1ap::PacketLossRate;
    274 "UplinkPacketLossRate" crate::s1ap::PacketLossRate;
    275 "UECapabilityInfoRequest" crate::s1ap::UECapabilityInfoRequest => UECapabilityInfoRequest;
    276 "serviceType" crate::s1ap::ServiceType;
    277 "AerialUEsubscriptionInformation" crate::s1ap::AerialUEsubscriptionInformation => AerialUEsubscriptionInformation;
    278 "Subscription-Based-UE-DifferentiationInfo" crate::s1ap::SubscriptionBasedUEDifferentiationInfo => Subscription_Based_UE_DifferentiationInfo;
    280 "EndIndication" crate::s1ap::EndIndication => EndIndication;
    281 "EDT-Session" crate::s1ap::EDTSession => EDT_Session;
    282 "CNTypeRestrictions" crate::s1ap::CNTypeRestrictions;
    283 "PendingDataIndication" crate::s1ap::PendingDataIndication => PendingDataIndication;
    284 "BluetoothMeasurementConfiguration" crate::s1ap::BluetoothMeasurementConfiguration<'a>;
    285 "WLANMeasurementConfiguration" crate::s1ap::WLANMeasurementConfiguration<'a>;
    286 "WarningAreaCoordinates" crate::s1ap::WarningAreaCoordinates<'a> => WarningAreaCoordinates;
    287 "NRrestrictionin5GS" crate::s1ap::NRrestrictionin5GS;
    288 "PSCellInformation" crate::s1ap::PSCellInformation => PSCellInformation;
    290 "LastNG-RANPLMNIdentity" crate::s1ap::PLMNidentity;
    291 "ConnectedengNBList" crate::s1ap::ConnectedengNBList<'a> => ConnectedengNBList;
    292 "ConnectedengNBToAddList" crate::s1ap::ConnectedengNBList<'a> => ConnectedengNBToAddList;
    293 "ConnectedengNBToRemoveList" crate::s1ap::ConnectedengNBList<'a> => ConnectedengNBToRemoveList;
    294 "EN-DCSONConfigurationTransfer-ECT" crate::s1ap::ENDCSONConfigurationTransfer<'a> => EN_DCSONConfigurationTransfer_ECT;
    295 "EN-DCSONConfigurationTransfer-MCT" crate::s1ap::ENDCSONConfigurationTransfer<'a> => EN_DCSONConfigurationTransfer_MCT;
    296 "IMSvoiceEPSfallbackfrom5G" crate::s1ap::IMSvoiceEPSfallbackfrom5G;
    297 "TimeSinceSecondaryNodeRelease" crate::s1ap::TimeSinceSecondaryNodeRelease => TimeSinceSecondaryNodeRelease;
    298 "RequestTypeAdditionalInfo" crate::s1ap::RequestTypeAdditionalInfo;
    299 "AdditionalRRMPriorityIndex" crate::s1ap::AdditionalRRMPriorityIndex => AdditionalRRMPriorityIndex;
    300 "ContextatSource" crate::s1ap::ContextatSource;
    301 "IAB-Authorized" crate::s1ap::IABAuthorized => IAB_Authorized;
    302 "IAB-Node-Indication" crate::s1ap::IABNodeIndication => IAB_Node_Indication;
    303 "IAB-Supported" crate::s1ap::IABSupported => IAB_Supported;
    304 "DataSize" crate::s1ap::DataSize => DataSize;
    305 "Ethernet-Type" crate::s1ap::EthernetType;
    306 "NRV2XServicesAuthorized" crate::s1ap::NRV2XServicesAuthorized => NRV2XServicesAuthorized;
    307 "NRUESidelinkAggregateMaximumBitrate" crate::s1ap::NRUESidelinkAggregateMaximumBitrate => NRUESidelinkAggregateMaximumBitrate;
    308 "PC5QoSParameters" crate::s1ap::PC5QoSParameters => PC5QoSParameters;
    309 "IntersystemSONConfigurationTransferMCT" crate::s1ap::IntersystemSONConfigurationTransfer<'a> => IntersystemSONConfigurationTransferMCT;
    310 "IntersystemSONConfigurationTransferECT" crate::s1ap::IntersystemSONConfigurationTransfer<'a> => IntersystemSONConfigurationTransferECT;
    311 "IntersystemMeasurementConfiguration" crate::s1ap::IntersystemMeasurementConfiguration<'a>;
    312 "SourceNodeID" crate::s1ap::SourceNodeID<'a>;
    313 "NB-IoT-RLF-Report-Container" crate::s1ap::NBIoTRLFReportContainer<'a>;
    314 "UERadioCapabilityID" crate::s1ap::UERadioCapabilityID<'a> => UERadioCapabilityID;
    315 "UERadioCapability-NR-Format" crate::s1ap::UERadioCapability<'a> => UERadioCapability_NR_Format;
    316 "MDTConfigurationNR" crate::s1ap::MDTConfigurationNR<'a>;
    317 "DAPSRequestInfo" crate::s1ap::DAPSRequestInfo;
    318 "DAPSResponseInfoList" crate::s1ap::DAPSResponseInfoList<'a>;
    319 ignore "DAPSResponseInfoItem" crate::s1ap::DAPSResponseInfoItem => DAPSResponseInfoItem;
    320 "NotifySourceeNB" crate::s1ap::NotifySourceeNB => NotifySourceeNB;
    321 "eNB-EarlyStatusTransfer-TransparentContainer" crate::s1ap::ENBEarlyStatusTransferTransparentContainer<'a> => eNB_EarlyStatusTransfer_TransparentContainer;
    322 ignore "Bearers-SubjectToEarlyStatusTransfer-Item" crate::s1ap::BearersSubjectToEarlyStatusTransferItem => Bearers_SubjectToEarlyStatusTransfer_Item;
    323 "WUS-Assistance-Information" crate::s1ap::WUSAssistanceInformation => WUS_Assistance_Information;
    324 "NB-IoT-PagingDRX" crate::s1ap::NBIoTPagingDRX => NB_IoT_PagingDRX;
    325 "TraceCollectionEntityURI" crate::s1ap::URIAddress<'a>;
    326 "EmergencyIndicator" crate::s1ap::EmergencyIndicator;
    327 "UERadioCapabilityForPaging-NR-Format" crate::s1ap::UERadioCapabilityForPaging<'a> => UERadioCapabilityForPaging_NR_Format;
    328 "SourceTransportLayerAddress" crate::s1ap::TransportLayerAddress<'a>;
    329 "lastVisitedPSCellList" crate::s1ap::LastVisitedPSCellList;
    330 "RACSIndication" crate::s1ap::RACSIndication;
    331 "PagingCause" crate::s1ap::PagingCause => PagingCause;
    332 "SecurityIndication" crate::s1ap::SecurityIndication;
    334 ignore "E-RABSecurityResultItem" crate::s1ap::ERABSecurityResultItem => E_RABSecurityResultItem;
    335 "E-RABSecurityResultList" crate::s1ap::ERABSecurityResultList<'a>;
    336 "RAT-Restrictions" crate::s1ap::RATRestrictions<'a>;
    337 "UEContextReferenceatSourceeNB" crate::s1ap::ENBUES1APID;
    339 "LTE-NTN-TAI-Information" crate::s1ap::LTENTNTAIInformation => LTE_NTN_TAI_Information;
    340 "SourceNodeTransportLayerAddress" crate::s1ap::TransportLayerAddress<'a>;
    341 "E-RABToBeUpdatedList" crate::s1ap::ERABToBeUpdatedList<'a> => E_RABToBeUpdatedList;
    342 ignore "E-RABToBeUpdatedItem" crate::s1ap::ERABToBeUpdatedItem => E_RABToBeUpdatedItem;
    343 "SourceSNID" crate::s1ap::GlobalRANNODEID;
    344 "LoggedMDTTrigger" crate::s1ap::LoggedMDTTrigger;
    345 "SensorMeasurementConfiguration" crate::s1ap::SensorMeasurementConfiguration;
    346 "M4ReportAmount" crate::s1ap::M4ReportAmountMDT;
    347 "M5ReportAmount" crate::s1ap::M5ReportAmountMDT;
    348 "M6ReportAmount" crate::s1ap::M6ReportAmountMDT;
    349 "M7ReportAmount" crate::s1ap::M7ReportAmountMDT;
    350 "TimeBasedHandoverInformation" crate::s1ap::TimeBasedHandoverInformation;
    351 ignore "Bearers-SubjectToDLDiscarding-Item" crate::s1ap::BearersSubjectToDLDiscardingItem => Bearers_SubjectToDLDiscarding_Item;
    352 "Bearers-SubjectToDLDiscardingList" crate::s1ap::BearersSubjectToDLDiscardingList<'a>;
    353 "CoarseUELocationRequested" crate::s1ap::CoarseUELocationRequested => CoarseUELocationRequested;
    354 "CoarseUELocation" crate::s1ap::CoarseUELocation<'a> => CoarseUELocation;
    355 "TimeRefDistribution" crate::s1ap::TimeRefDistribution => TimeRefDistribution;
    356 "RequestedTNLInfo" crate::s1ap::RequestedTNLInfo;
}
oxirush_aper::__lists! {
    E_RABSubjecttoDataForwardingList E_RABDataForwardingItem 1 256;
    E_RABtoReleaseListHOCmd E_RABItem 1 256;
    E_RABToBeSetupListBearerSUReq E_RABToBeSetupItemBearerSUReq 1 256;
    E_RABAdmittedList E_RABAdmittedItem 1 256;
    E_RABFailedToSetupListHOReqAck E_RABFailedtoSetupItemHOReqAck 1 256;
    E_RABToBeSwitchedDLList E_RABToBeSwitchedDLItem 1 256;
    E_RABToBeSetupListCtxtSUReq E_RABToBeSetupItemCtxtSUReq 1 256;
    E_RABSetupListBearerSURes E_RABSetupItemBearerSURes 1 256;
    E_RABFailedToSetupListBearerSURes E_RABItem 1 256;
    E_RABToBeModifiedListBearerModReq E_RABToBeModifiedItemBearerModReq 1 256;
    E_RABModifyListBearerModRes E_RABModifyItemBearerModRes 1 256;
    E_RABFailedToModifyList E_RABItem 1 256;
    E_RABToBeReleasedList E_RABItem 1 256;
    E_RABFailedToReleaseList E_RABItem 1 256;
    TAIList TAIItem 1 256;
    E_RABFailedToSetupListCtxtSURes E_RABItem 1 256;
    E_RABSetupListCtxtSURes E_RABSetupItemCtxtSURes 1 256;
    E_RABToBeSetupListHOReq E_RABToBeSetupItemHOReq 1 256;
    E_RABReleaseListBearerRelComp E_RABReleaseItemBearerRelComp 1 256;
    UE_associatedLogicalS1_ConnectionListResAck UE_associatedLogicalS1_ConnectionItem 1 256;
    E_RABToBeSwitchedULList E_RABToBeSwitchedULItem 1 256;
    E_RABReleasedList E_RABItem 1 256;
    E_RABToBeModifiedListBearerModInd E_RABToBeModifiedItemBearerModInd 1 256;
    E_RABNotToBeModifiedListBearerModInd E_RABNotToBeModifiedItemBearerModInd 1 256;
    E_RABModifyListBearerModConf E_RABModifyItemBearerModConf 1 256;
    E_RABFailedToModifyListBearerModConf E_RABItem 1 256;
    E_RABToBeReleasedListBearerModConf E_RABItem 1 256;
    E_RABFailedToResumeListResumeReq E_RABFailedToResumeItemResumeReq 1 256;
    E_RABFailedToResumeListResumeRes E_RABFailedToResumeItemResumeRes 1 256;
    SecondaryRATDataUsageReportList SecondaryRATDataUsageReportItem 1 256;
    E_RABToBeUpdatedList E_RABToBeUpdatedItem 1 256;
}
oxirush_aper::__transfer_sets! {
    protocol S1ap, bindings s1ap;
}
