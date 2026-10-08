#![allow(clippy::large_enum_variant)]

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod s1_ap_common_data_types {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum Criticality {
        reject = 0,
        ignore = 1,
        notify = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum Presence {
        optional = 0,
        conditional = 1,
        mandatory = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "PrivateIE-ID")]
    pub enum PrivateIEID {
        #[rasn(value("0..=65535"))]
        local(u16),
        global(ObjectIdentifier),
    }
    impl From<u16> for PrivateIEID {
        fn from(value: u16) -> Self {
            Self::local(value)
        }
    }
    impl From<ObjectIdentifier> for PrivateIEID {
        fn from(value: ObjectIdentifier) -> Self {
            Self::global(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct ProcedureCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct ProtocolExtensionID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "ProtocolIE-ID", value("0..=65535"))]
    pub struct ProtocolIEID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TriggeringMessage {
        #[rasn(identifier = "initiating-message")]
        initiating_message = 0,
        #[rasn(identifier = "successful-outcome")]
        successful_outcome = 1,
        #[rasn(identifier = "unsuccessfull-outcome")]
        unsuccessfull_outcome = 2,
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod s1_ap_constants {
    extern crate alloc;
    use super::s1_ap_common_data_types::{ProcedureCode, ProtocolIEID};
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    pub const ID_ADDITIONAL_GUTI: ProtocolIEID = ProtocolIEID(224);
    pub const ID_ADDITIONAL_CSFALLBACK_INDICATOR: ProtocolIEID = ProtocolIEID(187);
    pub const ID_ADDITIONAL_RRMPRIORITY_INDEX: ProtocolIEID = ProtocolIEID(299);
    pub const ID_AERIAL_UESUBSCRIPTION_INFORMATION: ProtocolIEID = ProtocolIEID(277);
    pub const ID_ASSISTANCE_DATA_FOR_PAGING: ProtocolIEID = ProtocolIEID(211);
    pub const ID_BEARER_TYPE: ProtocolIEID = ProtocolIEID(233);
    pub const ID_BEARERS_SUBJECT_TO_DLDISCARDING_ITEM: ProtocolIEID = ProtocolIEID(351);
    pub const ID_BEARERS_SUBJECT_TO_DLDISCARDING_LIST: ProtocolIEID = ProtocolIEID(352);
    pub const ID_BEARERS_SUBJECT_TO_EARLY_STATUS_TRANSFER_ITEM: ProtocolIEID = ProtocolIEID(322);
    pub const ID_BEARERS_SUBJECT_TO_STATUS_TRANSFER_ITEM: ProtocolIEID = ProtocolIEID(89);
    pub const ID_BLUETOOTH_MEASUREMENT_CONFIGURATION: ProtocolIEID = ProtocolIEID(284);
    pub const ID_BROADCAST_CANCELLED_AREA_LIST: ProtocolIEID = ProtocolIEID(141);
    pub const ID_BROADCAST_COMPLETED_AREA_LIST: ProtocolIEID = ProtocolIEID(120);
    pub const ID_CE_MODE_BRESTRICTED: ProtocolIEID = ProtocolIEID(271);
    pub const ID_CE_MODE_B_SUPPORT_INDICATOR: ProtocolIEID = ProtocolIEID(242);
    pub const ID_CNDOMAIN: ProtocolIEID = ProtocolIEID(109);
    pub const ID_CNTYPE_RESTRICTIONS: ProtocolIEID = ProtocolIEID(282);
    pub const ID_CSFALLBACK_INDICATOR: ProtocolIEID = ProtocolIEID(108);
    pub const ID_CSG_ID: ProtocolIEID = ProtocolIEID(127);
    pub const ID_CSG_ID_LIST: ProtocolIEID = ProtocolIEID(128);
    pub const ID_CSGMEMBERSHIP_INFO: ProtocolIEID = ProtocolIEID(226);
    pub const ID_CSGMEMBERSHIP_STATUS: ProtocolIEID = ProtocolIEID(146);
    pub const ID_CAUSE: ProtocolIEID = ProtocolIEID(2);
    pub const ID_CELL_ACCESS_MODE: ProtocolIEID = ProtocolIEID(145);
    pub const ID_CELL_IDENTIFIER_AND_CELEVEL_FOR_CECAPABLE_UES: ProtocolIEID = ProtocolIEID(212);
    pub const ID_CELL_TRAFFIC_TRACE: ProcedureCode = ProcedureCode(42);
    pub const ID_COARSE_UELOCATION: ProtocolIEID = ProtocolIEID(354);
    pub const ID_COARSE_UELOCATION_REQUESTED: ProtocolIEID = ProtocolIEID(353);
    pub const ID_CONCURRENT_WARNING_MESSAGE_INDICATOR: ProtocolIEID = ProtocolIEID(142);
    pub const ID_CONNECTEDENG_NBLIST: ProtocolIEID = ProtocolIEID(291);
    pub const ID_CONNECTEDENG_NBTO_ADD_LIST: ProtocolIEID = ProtocolIEID(292);
    pub const ID_CONNECTEDENG_NBTO_REMOVE_LIST: ProtocolIEID = ProtocolIEID(293);
    pub const ID_CONNECTION_ESTABLISHMENT_INDICATION: ProcedureCode = ProcedureCode(54);
    pub const ID_CONTEXTAT_SOURCE: ProtocolIEID = ProtocolIEID(300);
    pub const ID_CORRELATION_ID: ProtocolIEID = ProtocolIEID(156);
    pub const ID_COVERAGE_LEVEL: ProtocolIEID = ProtocolIEID(250);
    pub const ID_CRITICALITY_DIAGNOSTICS: ProtocolIEID = ProtocolIEID(58);
    pub const ID_DAPSREQUEST_INFO: ProtocolIEID = ProtocolIEID(317);
    pub const ID_DAPSRESPONSE_INFO_ITEM: ProtocolIEID = ProtocolIEID(319);
    pub const ID_DAPSRESPONSE_INFO_LIST: ProtocolIEID = ProtocolIEID(318);
    pub const ID_DCN_ID: ProtocolIEID = ProtocolIEID(246);
    pub const ID_DL_CP_SECURITY_INFORMATION: ProtocolIEID = ProtocolIEID(253);
    pub const ID_DLCOUNTVALUE_EXTENDED: ProtocolIEID = ProtocolIEID(180);
    pub const ID_DLCOUNTVALUE_PDCP_SNLENGTH18: ProtocolIEID = ProtocolIEID(218);
    pub const ID_DLNASPDUDELIVERY_ACK_REQUEST: ProtocolIEID = ProtocolIEID(249);
    pub const ID_DATA_FORWARDING_NOT_POSSIBLE: ProtocolIEID = ProtocolIEID(143);
    pub const ID_DATA_CODING_SCHEME: ProtocolIEID = ProtocolIEID(118);
    pub const ID_DATA_SIZE: ProtocolIEID = ProtocolIEID(304);
    pub const ID_DEACTIVATE_TRACE: ProcedureCode = ProcedureCode(26);
    pub const ID_DEFAULT_PAGING_DRX: ProtocolIEID = ProtocolIEID(137);
    pub const ID_DIRECT_FORWARDING_PATH_AVAILABILITY: ProtocolIEID = ProtocolIEID(79);
    pub const ID_DOWNLINK_PACKET_LOSS_RATE: ProtocolIEID = ProtocolIEID(273);
    pub const ID_DOWNLINK_S1CDMA2000TUNNELLING: ProcedureCode = ProcedureCode(19);
    pub const ID_E_RABADMITTED_ITEM: ProtocolIEID = ProtocolIEID(20);
    pub const ID_E_RABADMITTED_LIST: ProtocolIEID = ProtocolIEID(18);
    pub const ID_E_RABDATA_FORWARDING_ITEM: ProtocolIEID = ProtocolIEID(14);
    pub const ID_E_RABFAILED_TO_BE_RELEASED_LIST: ProtocolIEID = ProtocolIEID(103);
    pub const ID_E_RABFAILED_TO_MODIFY_LIST: ProtocolIEID = ProtocolIEID(32);
    pub const ID_E_RABFAILED_TO_MODIFY_LIST_BEARER_MOD_CONF: ProtocolIEID = ProtocolIEID(205);
    pub const ID_E_RABFAILED_TO_RELEASE_LIST: ProtocolIEID = ProtocolIEID(34);
    pub const ID_E_RABFAILED_TO_RESUME_ITEM_RESUME_REQ: ProtocolIEID = ProtocolIEID(236);
    pub const ID_E_RABFAILED_TO_RESUME_ITEM_RESUME_RES: ProtocolIEID = ProtocolIEID(238);
    pub const ID_E_RABFAILED_TO_RESUME_LIST_RESUME_REQ: ProtocolIEID = ProtocolIEID(235);
    pub const ID_E_RABFAILED_TO_RESUME_LIST_RESUME_RES: ProtocolIEID = ProtocolIEID(237);
    pub const ID_E_RABFAILED_TO_SETUP_LIST_BEARER_SURES: ProtocolIEID = ProtocolIEID(29);
    pub const ID_E_RABFAILED_TO_SETUP_LIST_CTXT_SURES: ProtocolIEID = ProtocolIEID(48);
    pub const ID_E_RABFAILED_TO_SETUP_LIST_HOREQ_ACK: ProtocolIEID = ProtocolIEID(19);
    pub const ID_E_RABFAILEDTO_SETUP_ITEM_HOREQ_ACK: ProtocolIEID = ProtocolIEID(21);
    pub const ID_E_RABINFORMATION_LIST_ITEM: ProtocolIEID = ProtocolIEID(78);
    pub const ID_E_RABITEM: ProtocolIEID = ProtocolIEID(35);
    pub const ID_E_RABMODIFICATION_INDICATION: ProcedureCode = ProcedureCode(50);
    pub const ID_E_RABMODIFY: ProcedureCode = ProcedureCode(6);
    pub const ID_E_RABMODIFY_ITEM_BEARER_MOD_CONF: ProtocolIEID = ProtocolIEID(204);
    pub const ID_E_RABMODIFY_ITEM_BEARER_MOD_RES: ProtocolIEID = ProtocolIEID(37);
    pub const ID_E_RABMODIFY_LIST_BEARER_MOD_CONF: ProtocolIEID = ProtocolIEID(203);
    pub const ID_E_RABMODIFY_LIST_BEARER_MOD_RES: ProtocolIEID = ProtocolIEID(31);
    pub const ID_E_RABNOT_TO_BE_MODIFIED_ITEM_BEARER_MOD_IND: ProtocolIEID = ProtocolIEID(202);
    pub const ID_E_RABNOT_TO_BE_MODIFIED_LIST_BEARER_MOD_IND: ProtocolIEID = ProtocolIEID(201);
    pub const ID_E_RABRELEASE: ProcedureCode = ProcedureCode(7);
    pub const ID_E_RABRELEASE_INDICATION: ProcedureCode = ProcedureCode(8);
    pub const ID_E_RABRELEASE_ITEM: ProtocolIEID = ProtocolIEID(38);
    pub const ID_E_RABRELEASE_ITEM_BEARER_REL_COMP: ProtocolIEID = ProtocolIEID(15);
    pub const ID_E_RABRELEASE_ITEM_HOCMD: ProtocolIEID = ProtocolIEID(49);
    pub const ID_E_RABRELEASE_LIST_BEARER_REL_COMP: ProtocolIEID = ProtocolIEID(69);
    pub const ID_E_RABRELEASED_LIST: ProtocolIEID = ProtocolIEID(110);
    pub const ID_E_RABSECURITY_RESULT_ITEM: ProtocolIEID = ProtocolIEID(334);
    pub const ID_E_RABSECURITY_RESULT_LIST: ProtocolIEID = ProtocolIEID(335);
    pub const ID_E_RABSETUP: ProcedureCode = ProcedureCode(5);
    pub const ID_E_RABSETUP_ITEM_BEARER_SURES: ProtocolIEID = ProtocolIEID(39);
    pub const ID_E_RABSETUP_ITEM_CTXT_SURES: ProtocolIEID = ProtocolIEID(50);
    pub const ID_E_RABSETUP_LIST_BEARER_SURES: ProtocolIEID = ProtocolIEID(28);
    pub const ID_E_RABSETUP_LIST_CTXT_SURES: ProtocolIEID = ProtocolIEID(51);
    pub const ID_E_RABSUBJECTTO_DATA_FORWARDING_LIST: ProtocolIEID = ProtocolIEID(12);
    pub const ID_E_RABTO_BE_MODIFIED_ITEM_BEARER_MOD_IND: ProtocolIEID = ProtocolIEID(200);
    pub const ID_E_RABTO_BE_MODIFIED_ITEM_BEARER_MOD_REQ: ProtocolIEID = ProtocolIEID(36);
    pub const ID_E_RABTO_BE_MODIFIED_LIST_BEARER_MOD_IND: ProtocolIEID = ProtocolIEID(199);
    pub const ID_E_RABTO_BE_MODIFIED_LIST_BEARER_MOD_REQ: ProtocolIEID = ProtocolIEID(30);
    pub const ID_E_RABTO_BE_RELEASED_LIST: ProtocolIEID = ProtocolIEID(33);
    pub const ID_E_RABTO_BE_RELEASED_LIST_BEARER_MOD_CONF: ProtocolIEID = ProtocolIEID(210);
    pub const ID_E_RABTO_BE_SETUP_ITEM_BEARER_SUREQ: ProtocolIEID = ProtocolIEID(17);
    pub const ID_E_RABTO_BE_SETUP_ITEM_CTXT_SUREQ: ProtocolIEID = ProtocolIEID(52);
    pub const ID_E_RABTO_BE_SETUP_ITEM_HOREQ: ProtocolIEID = ProtocolIEID(27);
    pub const ID_E_RABTO_BE_SETUP_LIST_BEARER_SUREQ: ProtocolIEID = ProtocolIEID(16);
    pub const ID_E_RABTO_BE_SETUP_LIST_CTXT_SUREQ: ProtocolIEID = ProtocolIEID(24);
    pub const ID_E_RABTO_BE_SETUP_LIST_HOREQ: ProtocolIEID = ProtocolIEID(53);
    pub const ID_E_RABTO_BE_SWITCHED_DLITEM: ProtocolIEID = ProtocolIEID(23);
    pub const ID_E_RABTO_BE_SWITCHED_DLLIST: ProtocolIEID = ProtocolIEID(22);
    pub const ID_E_RABTO_BE_SWITCHED_ULITEM: ProtocolIEID = ProtocolIEID(94);
    pub const ID_E_RABTO_BE_SWITCHED_ULLIST: ProtocolIEID = ProtocolIEID(95);
    pub const ID_E_RABTO_BE_UPDATED_ITEM: ProtocolIEID = ProtocolIEID(342);
    pub const ID_E_RABTO_BE_UPDATED_LIST: ProtocolIEID = ProtocolIEID(341);
    pub const ID_E_RABUSAGE_REPORT_ITEM: ProtocolIEID = ProtocolIEID(267);
    pub const ID_E_RABTO_RELEASE_LIST_HOCMD: ProtocolIEID = ProtocolIEID(13);
    pub const ID_E_UTRAN_TRACE_ID: ProtocolIEID = ProtocolIEID(86);
    pub const ID_ECGILIST_FOR_RESTART: ProtocolIEID = ProtocolIEID(182);
    pub const ID_EDT_SESSION: ProtocolIEID = ProtocolIEID(281);
    pub const ID_EN_DCSONCONFIGURATION_TRANSFER_ECT: ProtocolIEID = ProtocolIEID(294);
    pub const ID_EN_DCSONCONFIGURATION_TRANSFER_MCT: ProtocolIEID = ProtocolIEID(295);
    pub const ID_ENBCONFIGURATION_UPDATE: ProcedureCode = ProcedureCode(29);
    pub const ID_EUTRAN_CGI: ProtocolIEID = ProtocolIEID(100);
    pub const ID_EUTRANROUND_TRIP_DELAY_ESTIMATION_INFO: ProtocolIEID = ProtocolIEID(140);
    pub const ID_EMERGENCY_AREA_IDLIST_FOR_RESTART: ProtocolIEID = ProtocolIEID(190);
    pub const ID_EMERGENCY_INDICATOR: ProtocolIEID = ProtocolIEID(326);
    pub const ID_END_INDICATION: ProtocolIEID = ProtocolIEID(280);
    pub const ID_ENHANCED_COVERAGE_RESTRICTED: ProtocolIEID = ProtocolIEID(251);
    pub const ID_ERROR_INDICATION: ProcedureCode = ProcedureCode(15);
    pub const ID_ETHERNET_TYPE: ProtocolIEID = ProtocolIEID(305);
    pub const ID_EXPECTED_UEBEHAVIOUR: ProtocolIEID = ProtocolIEID(196);
    pub const ID_EXTENDED_REPETITION_PERIOD: ProtocolIEID = ProtocolIEID(144);
    pub const ID_GERANTO_LTEHOINFORMATION_RES: ProtocolIEID = ProtocolIEID(55);
    pub const ID_GUMMEI_ID: ProtocolIEID = ProtocolIEID(75);
    pub const ID_GUMMEILIST: ProtocolIEID = ProtocolIEID(154);
    pub const ID_GUMMEITYPE: ProtocolIEID = ProtocolIEID(170);
    pub const ID_GW_TRANSPORT_LAYER_ADDRESS: ProtocolIEID = ProtocolIEID(155);
    pub const ID_GWCONTEXT_RELEASE_INDICATION: ProtocolIEID = ProtocolIEID(164);
    pub const ID_GLOBAL_ENB_ID: ProtocolIEID = ProtocolIEID(59);
    pub const ID_HO_CAUSE: ProtocolIEID = ProtocolIEID(168);
    pub const ID_HANDOVER_CANCEL: ProcedureCode = ProcedureCode(4);
    pub const ID_HANDOVER_FLAG: ProtocolIEID = ProtocolIEID(266);
    pub const ID_HANDOVER_NOTIFICATION: ProcedureCode = ProcedureCode(2);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Elementary Procedures"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub const ID_HANDOVER_PREPARATION: ProcedureCode = ProcedureCode(0);
    pub const ID_HANDOVER_RESOURCE_ALLOCATION: ProcedureCode = ProcedureCode(1);
    pub const ID_HANDOVER_RESTRICTION_LIST: ProtocolIEID = ProtocolIEID(41);
    pub const ID_HANDOVER_SUCCESS: ProcedureCode = ProcedureCode(64);
    pub const ID_HANDOVER_TYPE: ProtocolIEID = ProtocolIEID(1);
    pub const ID_IAB_AUTHORIZED: ProtocolIEID = ProtocolIEID(301);
    pub const ID_IAB_NODE_INDICATION: ProtocolIEID = ProtocolIEID(302);
    pub const ID_IAB_SUPPORTED: ProtocolIEID = ProtocolIEID(303);
    pub const ID_IMSVOICE_EPSFALLBACKFROM5_G: ProtocolIEID = ProtocolIEID(296);
    pub const ID_INFORMATION_ON_RECOMMENDED_CELLS_AND_ENBS_FOR_PAGING: ProtocolIEID =
        ProtocolIEID(213);
    pub const ID_INITIAL_CONTEXT_SETUP: ProcedureCode = ProcedureCode(9);
    pub const ID_INTER_SYSTEM_INFORMATION_TRANSFER_TYPE_EDT: ProtocolIEID = ProtocolIEID(121);
    pub const ID_INTER_SYSTEM_INFORMATION_TRANSFER_TYPE_MDT: ProtocolIEID = ProtocolIEID(122);
    pub const ID_INTERSYSTEM_MEASUREMENT_CONFIGURATION: ProtocolIEID = ProtocolIEID(311);
    pub const ID_INTERSYSTEM_SONCONFIGURATION_TRANSFER_ECT: ProtocolIEID = ProtocolIEID(310);
    pub const ID_INTERSYSTEM_SONCONFIGURATION_TRANSFER_MCT: ProtocolIEID = ProtocolIEID(309);
    pub const ID_KILL: ProcedureCode = ProcedureCode(43);
    pub const ID_KILL_ALL_WARNING_MESSAGES: ProtocolIEID = ProtocolIEID(191);
    pub const ID_LHN_ID: ProtocolIEID = ProtocolIEID(186);
    pub const ID_LPPA_PDU: ProtocolIEID = ProtocolIEID(147);
    pub const ID_LTE_M_INDICATION: ProtocolIEID = ProtocolIEID(272);
    pub const ID_LTE_NTN_TAI_INFORMATION: ProtocolIEID = ProtocolIEID(339);
    pub const ID_LAST_NG_RANPLMNIDENTITY: ProtocolIEID = ProtocolIEID(290);
    pub const ID_LOCATION_REPORT: ProcedureCode = ProcedureCode(33);
    pub const ID_LOCATION_REPORTING_CONTROL: ProcedureCode = ProcedureCode(31);
    pub const ID_LOCATION_REPORTING_FAILURE_INDICATION: ProcedureCode = ProcedureCode(32);
    pub const ID_LOGGED_MBSFNMDT: ProtocolIEID = ProtocolIEID(197);
    pub const ID_LOGGED_MDTTRIGGER: ProtocolIEID = ProtocolIEID(344);
    pub const ID_M3_CONFIGURATION: ProtocolIEID = ProtocolIEID(171);
    pub const ID_M4_CONFIGURATION: ProtocolIEID = ProtocolIEID(172);
    pub const ID_M4_REPORT_AMOUNT: ProtocolIEID = ProtocolIEID(346);
    pub const ID_M5_CONFIGURATION: ProtocolIEID = ProtocolIEID(173);
    pub const ID_M5_REPORT_AMOUNT: ProtocolIEID = ProtocolIEID(347);
    pub const ID_M6_CONFIGURATION: ProtocolIEID = ProtocolIEID(220);
    pub const ID_M6_REPORT_AMOUNT: ProtocolIEID = ProtocolIEID(348);
    pub const ID_M7_CONFIGURATION: ProtocolIEID = ProtocolIEID(221);
    pub const ID_M7_REPORT_AMOUNT: ProtocolIEID = ProtocolIEID(349);
    pub const ID_MDT_LOCATION_INFO: ProtocolIEID = ProtocolIEID(174);
    pub const ID_MDTCONFIGURATION: ProtocolIEID = ProtocolIEID(162);
    pub const ID_MDTCONFIGURATION_NR: ProtocolIEID = ProtocolIEID(316);
    pub const ID_MME_GROUP_ID: ProtocolIEID = ProtocolIEID(223);
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " IEs"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub const ID_MME_UE_S1_AP_ID: ProtocolIEID = ProtocolIEID(0);
    pub const ID_MME_UE_S1_AP_ID_2: ProtocolIEID = ProtocolIEID(158);
    pub const ID_MMECPRELOCATION_INDICATION: ProcedureCode = ProcedureCode(61);
    pub const ID_MMECONFIGURATION_TRANSFER: ProcedureCode = ProcedureCode(41);
    pub const ID_MMECONFIGURATION_UPDATE: ProcedureCode = ProcedureCode(30);
    pub const ID_MMEDIRECT_INFORMATION_TRANSFER: ProcedureCode = ProcedureCode(38);
    pub const ID_MMEEARLY_STATUS_TRANSFER: ProcedureCode = ProcedureCode(66);
    pub const ID_MMERELAY_SUPPORT_INDICATOR: ProtocolIEID = ProtocolIEID(163);
    pub const ID_MMESTATUS_TRANSFER: ProcedureCode = ProcedureCode(25);
    pub const ID_MMENAME: ProtocolIEID = ProtocolIEID(61);
    pub const ID_MSCLASSMARK2: ProtocolIEID = ProtocolIEID(132);
    pub const ID_MSCLASSMARK3: ProtocolIEID = ProtocolIEID(133);
    pub const ID_MANAGEMENT_BASED_MDTALLOWED: ProtocolIEID = ProtocolIEID(165);
    pub const ID_MANAGEMENT_BASED_MDTPLMNLIST: ProtocolIEID = ProtocolIEID(177);
    pub const ID_MASKED_IMEISV: ProtocolIEID = ProtocolIEID(192);
    pub const ID_MESSAGE_IDENTIFIER: ProtocolIEID = ProtocolIEID(111);
    pub const ID_MOBILITY_INFORMATION: ProtocolIEID = ProtocolIEID(175);
    pub const ID_MUTING_AVAILABILITY_INDICATION: ProtocolIEID = ProtocolIEID(207);
    pub const ID_MUTING_PATTERN_INFORMATION: ProtocolIEID = ProtocolIEID(208);
    pub const ID_NAS_DOWNLINK_COUNT: ProtocolIEID = ProtocolIEID(126);
    pub const ID_NAS_PDU: ProtocolIEID = ProtocolIEID(26);
    pub const ID_NASDELIVERY_INDICATION: ProcedureCode = ProcedureCode(57);
    pub const ID_NASNON_DELIVERY_INDICATION: ProcedureCode = ProcedureCode(16);
    pub const ID_NASSECURITY_PARAMETERSFROM_E_UTRAN: ProtocolIEID = ProtocolIEID(135);
    pub const ID_NASSECURITY_PARAMETERSTO_E_UTRAN: ProtocolIEID = ProtocolIEID(136);
    pub const ID_NB_IO_T_DEFAULT_PAGING_DRX: ProtocolIEID = ProtocolIEID(234);
    pub const ID_NB_IO_T_PAGING_E_DRXINFORMATION: ProtocolIEID = ProtocolIEID(239);
    pub const ID_NB_IO_T_PAGING_DRX: ProtocolIEID = ProtocolIEID(324);
    pub const ID_NB_IO_T_RLF_REPORT_CONTAINER: ProtocolIEID = ProtocolIEID(313);
    pub const ID_NB_IO_T_UEIDENTITY_INDEX_VALUE: ProtocolIEID = ProtocolIEID(244);
    pub const ID_NRUESECURITY_CAPABILITIES: ProtocolIEID = ProtocolIEID(269);
    pub const ID_NRUESIDELINK_AGGREGATE_MAXIMUM_BITRATE: ProtocolIEID = ProtocolIEID(307);
    pub const ID_NRV2_XSERVICES_AUTHORIZED: ProtocolIEID = ProtocolIEID(306);
    pub const ID_NRRESTRICTIONIN5_GS: ProtocolIEID = ProtocolIEID(287);
    pub const ID_NRRESTRICTIONIN_EPSAS_SECONDARY_RAT: ProtocolIEID = ProtocolIEID(261);
    pub const ID_NOTIFY_SOURCEE_NB: ProtocolIEID = ProtocolIEID(320);
    pub const ID_NUMBEROF_BROADCAST_REQUEST: ProtocolIEID = ProtocolIEID(115);
    pub const ID_OVERLOAD_RESPONSE: ProtocolIEID = ProtocolIEID(101);
    pub const ID_OVERLOAD_START: ProcedureCode = ProcedureCode(34);
    pub const ID_OVERLOAD_STOP: ProcedureCode = ProcedureCode(35);
    pub const ID_PC5_QO_SPARAMETERS: ProtocolIEID = ProtocolIEID(308);
    pub const ID_PS_SERVICE_NOT_AVAILABLE: ProtocolIEID = ProtocolIEID(150);
    pub const ID_PSCELL_INFORMATION: ProtocolIEID = ProtocolIEID(288);
    pub const ID_PWSFAILURE_INDICATION: ProcedureCode = ProcedureCode(51);
    pub const ID_PWSRESTART_INDICATION: ProcedureCode = ProcedureCode(49);
    pub const ID_PWSFAILED_ECGILIST: ProtocolIEID = ProtocolIEID(222);
    pub const ID_PAGING: ProcedureCode = ProcedureCode(10);
    pub const ID_PAGING_E_DRXINFORMATION: ProtocolIEID = ProtocolIEID(227);
    pub const ID_PAGING_CAUSE: ProtocolIEID = ProtocolIEID(331);
    pub const ID_PAGING_PRIORITY: ProtocolIEID = ProtocolIEID(151);
    pub const ID_PATH_SWITCH_REQUEST: ProcedureCode = ProcedureCode(3);
    pub const ID_PENDING_DATA_INDICATION: ProtocolIEID = ProtocolIEID(283);
    pub const ID_PRIVACY_INDICATOR: ProtocolIEID = ProtocolIEID(166);
    pub const ID_PRIVATE_MESSAGE: ProcedureCode = ProcedureCode(39);
    pub const ID_PRO_SE_AUTHORIZED: ProtocolIEID = ProtocolIEID(195);
    pub const ID_PRO_SE_UETO_NETWORK_RELAYING: ProtocolIEID = ProtocolIEID(216);
    pub const ID_RACSINDICATION: ProtocolIEID = ProtocolIEID(330);
    pub const ID_RAT_RESTRICTIONS: ProtocolIEID = ProtocolIEID(336);
    pub const ID_RAT_TYPE: ProtocolIEID = ProtocolIEID(232);
    pub const ID_RRC_ESTABLISHMENT_CAUSE: ProtocolIEID = ProtocolIEID(134);
    pub const ID_RRC_RESUME_CAUSE: ProtocolIEID = ProtocolIEID(245);
    pub const ID_RECEIVE_STATUS_OF_ULPDCPSDUS_EXTENDED: ProtocolIEID = ProtocolIEID(181);
    pub const ID_RECEIVE_STATUS_OF_ULPDCPSDUS_PDCP_SNLENGTH18: ProtocolIEID = ProtocolIEID(219);
    pub const ID_RECOMMENDED_CELL_ITEM: ProtocolIEID = ProtocolIEID(214);
    pub const ID_RECOMMENDED_ENBITEM: ProtocolIEID = ProtocolIEID(215);
    pub const ID_REGISTERED_LAI: ProtocolIEID = ProtocolIEID(159);
    pub const ID_RELATIVE_MMECAPACITY: ProtocolIEID = ProtocolIEID(87);
    pub const ID_RELAY_NODE_INDICATOR: ProtocolIEID = ProtocolIEID(160);
    pub const ID_REPETITION_PERIOD: ProtocolIEID = ProtocolIEID(114);
    pub const ID_REQUEST_TYPE: ProtocolIEID = ProtocolIEID(98);
    pub const ID_REQUEST_TYPE_ADDITIONAL_INFO: ProtocolIEID = ProtocolIEID(298);
    pub const ID_REQUESTED_TNLINFO: ProtocolIEID = ProtocolIEID(356);
    pub const ID_REROUTE_NASREQUEST: ProcedureCode = ProcedureCode(52);
    pub const ID_RESET: ProcedureCode = ProcedureCode(14);
    pub const ID_RESET_TYPE: ProtocolIEID = ProtocolIEID(92);
    pub const ID_RETRIEVE_UEINFORMATION: ProcedureCode = ProcedureCode(58);
    pub const ID_ROUTING_ID: ProtocolIEID = ProtocolIEID(148);
    pub const ID_S_TMSI: ProtocolIEID = ProtocolIEID(96);
    pub const ID_S1_MESSAGE: ProtocolIEID = ProtocolIEID(225);
    pub const ID_S1_REMOVAL: ProcedureCode = ProcedureCode(67);
    pub const ID_S1_SETUP: ProcedureCode = ProcedureCode(17);
    pub const ID_SIPTO_CORRELATION_ID: ProtocolIEID = ProtocolIEID(183);
    pub const ID_SIPTO_L_GW_TRANSPORT_LAYER_ADDRESS: ProtocolIEID = ProtocolIEID(184);
    pub const ID_SON_INFORMATION_REPORT: ProtocolIEID = ProtocolIEID(206);
    pub const ID_SONCONFIGURATION_TRANSFER_ECT: ProtocolIEID = ProtocolIEID(129);
    pub const ID_SONCONFIGURATION_TRANSFER_MCT: ProtocolIEID = ProtocolIEID(130);
    pub const ID_SRVCCHOINDICATION: ProtocolIEID = ProtocolIEID(125);
    pub const ID_SRVCCOPERATION_NOT_POSSIBLE: ProtocolIEID = ProtocolIEID(243);
    pub const ID_SRVCCOPERATION_POSSIBLE: ProtocolIEID = ProtocolIEID(124);
    pub const ID_SECONDARY_RATDATA_USAGE_REPORT: ProcedureCode = ProcedureCode(62);
    pub const ID_SECONDARY_RATDATA_USAGE_REPORT_ITEM: ProtocolIEID = ProtocolIEID(265);
    pub const ID_SECONDARY_RATDATA_USAGE_REPORT_LIST: ProtocolIEID = ProtocolIEID(264);
    pub const ID_SECONDARY_RATDATA_USAGE_REQUEST: ProtocolIEID = ProtocolIEID(268);
    pub const ID_SECURITY_CONTEXT: ProtocolIEID = ProtocolIEID(40);
    pub const ID_SECURITY_INDICATION: ProtocolIEID = ProtocolIEID(332);
    pub const ID_SECURITY_KEY: ProtocolIEID = ProtocolIEID(73);
    pub const ID_SECURITY_RESULT: ProtocolIEID = ProtocolIEID(333);
    pub const ID_SENSOR_MEASUREMENT_CONFIGURATION: ProtocolIEID = ProtocolIEID(345);
    pub const ID_SERIAL_NUMBER: ProtocolIEID = ProtocolIEID(112);
    pub const ID_SERVED_DCNS: ProtocolIEID = ProtocolIEID(247);
    pub const ID_SERVED_GUMMEIS: ProtocolIEID = ProtocolIEID(105);
    pub const ID_SERVED_PLMNS: ProtocolIEID = ProtocolIEID(63);
    pub const ID_SIGNALLING_BASED_MDTPLMNLIST: ProtocolIEID = ProtocolIEID(178);
    pub const ID_SOURCE_TO_TARGET_TRANSPARENT_CONTAINER: ProtocolIEID = ProtocolIEID(104);
    pub const ID_SOURCE_TO_TARGET_TRANSPARENT_CONTAINER_SECONDARY: ProtocolIEID = ProtocolIEID(138);
    pub const ID_SOURCE_ID: ProtocolIEID = ProtocolIEID(3);
    pub const ID_SOURCE_MME_GUMMEI: ProtocolIEID = ProtocolIEID(157);
    pub const ID_SOURCE_MME_UE_S1_AP_ID: ProtocolIEID = ProtocolIEID(88);
    pub const ID_SOURCE_NODE_ID: ProtocolIEID = ProtocolIEID(312);
    pub const ID_SOURCE_NODE_TRANSPORT_LAYER_ADDRESS: ProtocolIEID = ProtocolIEID(340);
    pub const ID_SOURCE_SNID: ProtocolIEID = ProtocolIEID(343);
    pub const ID_SOURCE_TRANSPORT_LAYER_ADDRESS: ProtocolIEID = ProtocolIEID(328);
    pub const ID_SUBSCRIBER_PROFILE_IDFOR_RFP: ProtocolIEID = ProtocolIEID(106);
    pub const ID_SUBSCRIPTION_BASED_UE_DIFFERENTIATION_INFO: ProtocolIEID = ProtocolIEID(278);
    pub const ID_SUPPORTED_TAS: ProtocolIEID = ProtocolIEID(64);
    pub const ID_SYNCHRONISATION_INFORMATION: ProtocolIEID = ProtocolIEID(209);
    pub const ID_TAI: ProtocolIEID = ProtocolIEID(67);
    pub const ID_TAIITEM: ProtocolIEID = ProtocolIEID(47);
    pub const ID_TAILIST: ProtocolIEID = ProtocolIEID(46);
    pub const ID_TAILIST_FOR_RESTART: ProtocolIEID = ProtocolIEID(188);
    pub const ID_TARGET_TO_SOURCE_TRANSPARENT_CONTAINER: ProtocolIEID = ProtocolIEID(123);
    pub const ID_TARGET_TO_SOURCE_TRANSPARENT_CONTAINER_SECONDARY: ProtocolIEID = ProtocolIEID(139);
    pub const ID_TARGET_ID: ProtocolIEID = ProtocolIEID(4);
    pub const ID_TIME_SYNCHRONISATION_INFO: ProtocolIEID = ProtocolIEID(149);
    pub const ID_TIME_UE_STAYED_IN_CELL_ENHANCED_GRANULARITY: ProtocolIEID = ProtocolIEID(167);
    pub const ID_TIME_BASED_HANDOVER_INFORMATION: ProtocolIEID = ProtocolIEID(350);
    pub const ID_TIME_REF_DISTRIBUTION: ProtocolIEID = ProtocolIEID(355);
    pub const ID_TIME_SINCE_SECONDARY_NODE_RELEASE: ProtocolIEID = ProtocolIEID(297);
    pub const ID_TIME_TO_WAIT: ProtocolIEID = ProtocolIEID(65);
    pub const ID_TRACE_ACTIVATION: ProtocolIEID = ProtocolIEID(25);
    pub const ID_TRACE_COLLECTION_ENTITY_IPADDRESS: ProtocolIEID = ProtocolIEID(131);
    pub const ID_TRACE_COLLECTION_ENTITY_URI: ProtocolIEID = ProtocolIEID(325);
    pub const ID_TRACE_FAILURE_INDICATION: ProcedureCode = ProcedureCode(28);
    pub const ID_TRACE_START: ProcedureCode = ProcedureCode(27);
    pub const ID_TRAFFIC_LOAD_REDUCTION_INDICATION: ProtocolIEID = ProtocolIEID(161);
    pub const ID_TRANSPORT_INFORMATION: ProtocolIEID = ProtocolIEID(185);
    pub const ID_TUNNEL_INFORMATION_FOR_BBF: ProtocolIEID = ProtocolIEID(176);
    pub const ID_UE_APPLICATION_LAYER_MEASUREMENT_CAPABILITY: ProtocolIEID = ProtocolIEID(263);
    pub const ID_UE_LEVEL_QO_S_PARAMETERS: ProtocolIEID = ProtocolIEID(252);
    pub const ID_UE_RETENTION_INFORMATION: ProtocolIEID = ProtocolIEID(228);
    pub const ID_UE_S1_AP_IDS: ProtocolIEID = ProtocolIEID(99);
    pub const ID_UE_USAGE_TYPE: ProtocolIEID = ProtocolIEID(230);
    pub const ID_UE_ASSOCIATED_LOGICAL_S1_CONNECTION_ITEM: ProtocolIEID = ProtocolIEID(91);
    pub const ID_UE_ASSOCIATED_LOGICAL_S1_CONNECTION_LIST_RES_ACK: ProtocolIEID = ProtocolIEID(93);
    pub const ID_UEAPP_LAYER_MEAS_CONFIG: ProtocolIEID = ProtocolIEID(262);
    pub const ID_UECAPABILITY_INFO_INDICATION: ProcedureCode = ProcedureCode(22);
    pub const ID_UECAPABILITY_INFO_REQUEST: ProtocolIEID = ProtocolIEID(275);
    pub const ID_UECONTEXT_MODIFICATION: ProcedureCode = ProcedureCode(21);
    pub const ID_UECONTEXT_MODIFICATION_INDICATION: ProcedureCode = ProcedureCode(53);
    pub const ID_UECONTEXT_REFERENCEAT_SOURCEE_NB: ProtocolIEID = ProtocolIEID(337);
    pub const ID_UECONTEXT_RELEASE: ProcedureCode = ProcedureCode(23);
    pub const ID_UECONTEXT_RELEASE_REQUEST: ProcedureCode = ProcedureCode(18);
    pub const ID_UECONTEXT_RESUME: ProcedureCode = ProcedureCode(56);
    pub const ID_UECONTEXT_SUSPEND: ProcedureCode = ProcedureCode(55);
    pub const ID_UEIDENTITY_INDEX_VALUE: ProtocolIEID = ProtocolIEID(80);
    pub const ID_UEINFORMATION_TRANSFER: ProcedureCode = ProcedureCode(59);
    pub const ID_UEPAGING_ID: ProtocolIEID = ProtocolIEID(43);
    pub const ID_UERADIO_CAPABILITY: ProtocolIEID = ProtocolIEID(74);
    pub const ID_UERADIO_CAPABILITY_NR_FORMAT: ProtocolIEID = ProtocolIEID(315);
    pub const ID_UERADIO_CAPABILITY_FOR_PAGING: ProtocolIEID = ProtocolIEID(198);
    pub const ID_UERADIO_CAPABILITY_FOR_PAGING_NR_FORMAT: ProtocolIEID = ProtocolIEID(327);
    pub const ID_UERADIO_CAPABILITY_ID: ProtocolIEID = ProtocolIEID(314);
    pub const ID_UERADIO_CAPABILITY_IDMAPPING: ProcedureCode = ProcedureCode(63);
    pub const ID_UERADIO_CAPABILITY_MATCH: ProcedureCode = ProcedureCode(48);
    pub const ID_UESECURITY_CAPABILITIES: ProtocolIEID = ProtocolIEID(107);
    pub const ID_UESIDELINK_AGGREGATE_MAXIMUM_BITRATE: ProtocolIEID = ProtocolIEID(248);
    pub const ID_UEUSER_PLANE_CIO_TSUPPORT_INDICATOR: ProtocolIEID = ProtocolIEID(241);
    pub const ID_UL_CP_SECURITY_INFORMATION: ProtocolIEID = ProtocolIEID(254);
    pub const ID_ULCOUNTVALUE_EXTENDED: ProtocolIEID = ProtocolIEID(179);
    pub const ID_ULCOUNTVALUE_PDCP_SNLENGTH18: ProtocolIEID = ProtocolIEID(217);
    pub const ID_UTRANTO_LTEHOINFORMATION_RES: ProtocolIEID = ProtocolIEID(57);
    pub const ID_UNLICENSED_SPECTRUM_RESTRICTION: ProtocolIEID = ProtocolIEID(270);
    pub const ID_UPLINK_PACKET_LOSS_RATE: ProtocolIEID = ProtocolIEID(274);
    pub const ID_UPLINK_S1CDMA2000TUNNELLING: ProcedureCode = ProcedureCode(20);
    pub const ID_USER_LOCATION_INFORMATION: ProtocolIEID = ProtocolIEID(189);
    pub const ID_V2_XSERVICES_AUTHORIZED: ProtocolIEID = ProtocolIEID(240);
    pub const ID_VOICE_SUPPORT_MATCH_INDICATOR: ProtocolIEID = ProtocolIEID(169);
    pub const ID_WLANMEASUREMENT_CONFIGURATION: ProtocolIEID = ProtocolIEID(285);
    pub const ID_WUS_ASSISTANCE_INFORMATION: ProtocolIEID = ProtocolIEID(323);
    pub const ID_WARNING_AREA_COORDINATES: ProtocolIEID = ProtocolIEID(286);
    pub const ID_WARNING_AREA_LIST: ProtocolIEID = ProtocolIEID(113);
    pub const ID_WARNING_MESSAGE_CONTENTS: ProtocolIEID = ProtocolIEID(119);
    pub const ID_WARNING_SECURITY_INFO: ProtocolIEID = ProtocolIEID(117);
    pub const ID_WARNING_TYPE: ProtocolIEID = ProtocolIEID(116);
    pub const ID_WRITE_REPLACE_WARNING: ProcedureCode = ProcedureCode(36);
    pub const ID_CDMA2000_HOREQUIRED_INDICATION: ProtocolIEID = ProtocolIEID(84);
    pub const ID_CDMA2000_HOSTATUS: ProtocolIEID = ProtocolIEID(83);
    pub const ID_CDMA2000_ONE_XRAND: ProtocolIEID = ProtocolIEID(97);
    pub const ID_CDMA2000_ONE_XSRVCCINFO: ProtocolIEID = ProtocolIEID(102);
    pub const ID_CDMA2000_PDU: ProtocolIEID = ProtocolIEID(70);
    pub const ID_CDMA2000_RATTYPE: ProtocolIEID = ProtocolIEID(71);
    pub const ID_CDMA2000_SECTOR_ID: ProtocolIEID = ProtocolIEID(72);
    pub const ID_DOWNLINK_NASTRANSPORT: ProcedureCode = ProcedureCode(11);
    pub const ID_DOWNLINK_NON_UEASSOCIATED_LPPA_TRANSPORT: ProcedureCode = ProcedureCode(46);
    pub const ID_DOWNLINK_UEASSOCIATED_LPPA_TRANSPORT: ProcedureCode = ProcedureCode(44);
    pub const ID_E_NB_EARLY_STATUS_TRANSFER_TRANSPARENT_CONTAINER: ProtocolIEID = ProtocolIEID(321);
    pub const ID_E_NB_STATUS_TRANSFER_TRANSPARENT_CONTAINER: ProtocolIEID = ProtocolIEID(90);
    pub const ID_E_NB_UE_S1_AP_ID: ProtocolIEID = ProtocolIEID(8);
    pub const ID_E_NBCPRELOCATION_INDICATION: ProcedureCode = ProcedureCode(60);
    pub const ID_E_NBCONFIGURATION_TRANSFER: ProcedureCode = ProcedureCode(40);
    pub const ID_E_NBDIRECT_INFORMATION_TRANSFER: ProcedureCode = ProcedureCode(37);
    pub const ID_E_NBEARLY_STATUS_TRANSFER: ProcedureCode = ProcedureCode(65);
    pub const ID_E_NBINDIRECT_X2_TRANSPORT_LAYER_ADDRESSES: ProtocolIEID = ProtocolIEID(193);
    pub const ID_E_NBSTATUS_TRANSFER: ProcedureCode = ProcedureCode(24);
    pub const ID_E_NBX2_EXTENDED_TRANSPORT_LAYER_ADDRESSES: ProtocolIEID = ProtocolIEID(153);
    pub const ID_E_NBNAME: ProtocolIEID = ProtocolIEID(60);
    pub const ID_EXTENDED_UEIDENTITY_INDEX_VALUE: ProtocolIEID = ProtocolIEID(231);
    pub const ID_EXTENDED_E_RAB_GUARANTEED_BITRATE_DL: ProtocolIEID = ProtocolIEID(257);
    pub const ID_EXTENDED_E_RAB_GUARANTEED_BITRATE_UL: ProtocolIEID = ProtocolIEID(258);
    pub const ID_EXTENDED_E_RAB_MAXIMUM_BITRATE_DL: ProtocolIEID = ProtocolIEID(255);
    pub const ID_EXTENDED_E_RAB_MAXIMUM_BITRATE_UL: ProtocolIEID = ProtocolIEID(256);
    pub const ID_EXTENDED_U_EAGGREGATE_MAXIMUM_BIT_RATE_DL: ProtocolIEID = ProtocolIEID(259);
    pub const ID_EXTENDED_U_EAGGREGATE_MAXIMUM_BIT_RATE_UL: ProtocolIEID = ProtocolIEID(260);
    pub const ID_INITIAL_UEMESSAGE: ProcedureCode = ProcedureCode(12);
    pub const ID_LAST_VISITED_PSCELL_LIST: ProtocolIEID = ProtocolIEID(329);
    pub const ID_PAGING_DRX: ProtocolIEID = ProtocolIEID(44);
    pub const ID_SERVICE_TYPE: ProtocolIEID = ProtocolIEID(276);
    pub const ID_U_E_HISTORY_INFORMATION_FROM_THE_UE: ProtocolIEID = ProtocolIEID(194);
    pub const ID_U_EAGGREGATE_MAXIMUM_BITRATE: ProtocolIEID = ProtocolIEID(66);
    pub const ID_UPLINK_NASTRANSPORT: ProcedureCode = ProcedureCode(13);
    pub const ID_UPLINK_NON_UEASSOCIATED_LPPA_TRANSPORT: ProcedureCode = ProcedureCode(47);
    pub const ID_UPLINK_UEASSOCIATED_LPPA_TRANSPORT: ProcedureCode = ProcedureCode(45);
    pub const ID_X2_TNLCONFIGURATION_INFO: ProtocolIEID = ProtocolIEID(152);
    pub static MAX_EARFCN: LazyLock<Integer> = LazyLock::new(|| Integer::from(262143i128));
    pub static MAX_NARFCN: LazyLock<Integer> = LazyLock::new(|| Integer::from(3279165i128));
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Extension constants"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub static MAX_PRIVATE_IES: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535i128));
    pub static MAX_PROTOCOL_EXTENSIONS: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(65535i128));
    pub static MAX_PROTOCOL_IES: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535i128));
    pub static MAX_RS_INDEX_CELL_QUAL: LazyLock<Integer> = LazyLock::new(|| Integer::from(16i128));
    pub static MAXNOOF_BPLMNS: LazyLock<Integer> = LazyLock::new(|| Integer::from(6i128));
    pub static MAXNOOF_BLUETOOTH_NAME: LazyLock<Integer> = LazyLock::new(|| Integer::from(4i128));
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Lists"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    pub static MAXNOOF_CSGS: LazyLock<Integer> = LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_CELL_ID: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535i128));
    pub static MAXNOOF_CELL_IDFOR_MDT: LazyLock<Integer> = LazyLock::new(|| Integer::from(32i128));
    pub static MAXNOOF_CELL_IDFOR_QMC: LazyLock<Integer> = LazyLock::new(|| Integer::from(32i128));
    pub static MAXNOOF_CELLIN_EAI: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535i128));
    pub static MAXNOOF_CELLIN_TAI: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535i128));
    pub static MAXNOOF_CELLSFOR_RESTART: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_CELLSIN_UEHISTORY_INFO: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(16i128));
    pub static MAXNOOF_CELLSINE_NB: LazyLock<Integer> = LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_CONNECTEDENG_NBS: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_DCNS: LazyLock<Integer> = LazyLock::new(|| Integer::from(32i128));
    pub static MAXNOOF_E_RABS: LazyLock<Integer> = LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_EPLMNS: LazyLock<Integer> = LazyLock::new(|| Integer::from(15i128));
    pub static MAXNOOF_EPLMNS_PLUS_ONE: LazyLock<Integer> = LazyLock::new(|| Integer::from(16i128));
    pub static MAXNOOF_EMERGENCY_AREA_ID: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(65535i128));
    pub static MAXNOOF_ERRORS: LazyLock<Integer> = LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_FORB_LACS: LazyLock<Integer> = LazyLock::new(|| Integer::from(4096i128));
    pub static MAXNOOF_FORB_TACS: LazyLock<Integer> = LazyLock::new(|| Integer::from(4096i128));
    pub static MAXNOOF_GROUP_IDS: LazyLock<Integer> = LazyLock::new(|| Integer::from(65535i128));
    pub static MAXNOOF_INDIVIDUAL_S1_CONNECTIONS_TO_RESET: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_MBSFNAREA_MDT: LazyLock<Integer> = LazyLock::new(|| Integer::from(8i128));
    pub static MAXNOOF_MDTPLMNS: LazyLock<Integer> = LazyLock::new(|| Integer::from(16i128));
    pub static MAXNOOF_MMECS: LazyLock<Integer> = LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_PC5_QO_SFLOWS: LazyLock<Integer> = LazyLock::new(|| Integer::from(2048i128));
    pub static MAXNOOF_PLMNFOR_QMC: LazyLock<Integer> = LazyLock::new(|| Integer::from(16i128));
    pub static MAXNOOF_PLMNS_PER_MME: LazyLock<Integer> = LazyLock::new(|| Integer::from(32i128));
    pub static MAXNOOF_PSCELLS_PER_PRIMARY_CELLIN_UEHISTORY_INFO: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(8i128));
    pub static MAXNOOF_RATS: LazyLock<Integer> = LazyLock::new(|| Integer::from(8i128));
    pub static MAXNOOF_RECOMMENDED_CELLS: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(16i128));
    pub static MAXNOOF_RECOMMENDED_ENBS: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(16i128));
    pub static MAXNOOF_RESTART_EMERGENCY_AREA_IDS: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_RESTART_TAIS: LazyLock<Integer> = LazyLock::new(|| Integer::from(2048i128));
    pub static MAXNOOF_SENSOR_NAME: LazyLock<Integer> = LazyLock::new(|| Integer::from(3i128));
    pub static MAXNOOF_TACS: LazyLock<Integer> = LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_TACS_IN_NTN: LazyLock<Integer> = LazyLock::new(|| Integer::from(12i128));
    pub static MAXNOOF_TAIFOR_WARNING: LazyLock<Integer> =
        LazyLock::new(|| Integer::from(65535i128));
    pub static MAXNOOF_TAIS: LazyLock<Integer> = LazyLock::new(|| Integer::from(256i128));
    pub static MAXNOOF_TAFOR_MDT: LazyLock<Integer> = LazyLock::new(|| Integer::from(8i128));
    pub static MAXNOOF_TAFOR_QMC: LazyLock<Integer> = LazyLock::new(|| Integer::from(8i128));
    pub static MAXNOOF_WLANNAME: LazyLock<Integer> = LazyLock::new(|| Integer::from(4i128));
    pub static MAXNOOFE_NBX2_EXT_TLAS: LazyLock<Integer> = LazyLock::new(|| Integer::from(16i128));
    pub static MAXNOOFE_NBX2_GTPTLAS: LazyLock<Integer> = LazyLock::new(|| Integer::from(16i128));
    pub static MAXNOOFE_NBX2_TLAS: LazyLock<Integer> = LazyLock::new(|| Integer::from(2i128));
    pub static MAXNOOFFREQUENCIES: LazyLock<Integer> = LazyLock::new(|| Integer::from(64i128));
    pub static MAXNOOFTIMEPERIODS: LazyLock<Integer> = LazyLock::new(|| Integer::from(2i128));
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod s1_ap_containers {
    extern crate alloc;
    use super::s1_ap_common_data_types::{
        Criticality, Presence, PrivateIEID, ProtocolExtensionID, ProtocolIEID,
    };
    use super::s1_ap_constants::{MAX_PRIVATE_IES, MAX_PROTOCOL_EXTENSIONS, MAX_PROTOCOL_IES};
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " `ProtocolIE-Field`: an IE of a protocol IE container, and a `ProtocolIE-SingleContainer`."]
    #[doc = " The object set of a container does not show in its type: `value` is an open type."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ProtocolIE-Field")]
    pub struct ProtocolIEField {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl ProtocolIEField {
        pub fn new(id: impl Into<ProtocolIEID>, criticality: Criticality, value: Any) -> Self {
            Self {
                id: id.into(),
                criticality,
                value,
            }
        }
    }
    #[doc = " `ProtocolIE-Container`: the IEs of a message."]
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"), identifier = "ProtocolIE-Container")]
    pub struct ProtocolIEContainer(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ProtocolIEContainer, 0, 65535 }
    #[doc = " `ProtocolExtensionField`: an IE of a protocol extension container."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ProtocolExtensionField")]
    pub struct ProtocolExtensionField {
        pub id: ProtocolExtensionID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl ProtocolExtensionField {
        pub fn new(
            id: impl Into<ProtocolExtensionID>,
            criticality: Criticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id: id.into(),
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " `ProtocolExtensionContainer`: the `iE-Extensions` of a type."]
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"), identifier = "ProtocolExtensionContainer")]
    pub struct ProtocolExtensionContainer(pub SequenceOf<ProtocolExtensionField>);
    crate::per::sequence_of! { ProtocolExtensionContainer, 1, 65535 }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod s1_ap_ies {
    extern crate alloc;
    use super::s1_ap_common_data_types::{
        Criticality, ProcedureCode, ProtocolIEID, TriggeringMessage,
    };
    use super::s1_ap_common_data_types::{Presence, ProtocolExtensionID};
    use super::s1_ap_constants::{
        ID_ADDITIONAL_RRMPRIORITY_INDEX, ID_BEARERS_SUBJECT_TO_DLDISCARDING_ITEM,
        ID_BEARERS_SUBJECT_TO_DLDISCARDING_LIST, ID_BEARERS_SUBJECT_TO_EARLY_STATUS_TRANSFER_ITEM,
        ID_BEARERS_SUBJECT_TO_STATUS_TRANSFER_ITEM, ID_BLUETOOTH_MEASUREMENT_CONFIGURATION,
        ID_CNTYPE_RESTRICTIONS, ID_CONTEXTAT_SOURCE, ID_DAPSREQUEST_INFO,
        ID_DAPSRESPONSE_INFO_ITEM, ID_DAPSRESPONSE_INFO_LIST,
        ID_DIRECT_FORWARDING_PATH_AVAILABILITY, ID_DLCOUNTVALUE_EXTENDED,
        ID_DLCOUNTVALUE_PDCP_SNLENGTH18, ID_DOWNLINK_PACKET_LOSS_RATE,
        ID_E_NBINDIRECT_X2_TRANSPORT_LAYER_ADDRESSES, ID_E_NBX2_EXTENDED_TRANSPORT_LAYER_ADDRESSES,
        ID_E_RABINFORMATION_LIST_ITEM, ID_E_RABITEM, ID_E_RABSECURITY_RESULT_ITEM,
        ID_E_RABSECURITY_RESULT_LIST, ID_E_RABUSAGE_REPORT_ITEM, ID_EMERGENCY_INDICATOR,
        ID_EXTENDED_E_RAB_GUARANTEED_BITRATE_DL, ID_EXTENDED_E_RAB_GUARANTEED_BITRATE_UL,
        ID_EXTENDED_E_RAB_MAXIMUM_BITRATE_DL, ID_EXTENDED_E_RAB_MAXIMUM_BITRATE_UL,
        ID_EXTENDED_U_EAGGREGATE_MAXIMUM_BIT_RATE_DL, ID_EXTENDED_U_EAGGREGATE_MAXIMUM_BIT_RATE_UL,
        ID_GUMMEITYPE, ID_HO_CAUSE, ID_IMSVOICE_EPSFALLBACKFROM5_G,
        ID_INTERSYSTEM_MEASUREMENT_CONFIGURATION, ID_LAST_NG_RANPLMNIDENTITY,
        ID_LAST_VISITED_PSCELL_LIST, ID_LOGGED_MBSFNMDT, ID_LOGGED_MDTTRIGGER,
        ID_LTE_NTN_TAI_INFORMATION, ID_M3_CONFIGURATION, ID_M4_CONFIGURATION, ID_M4_REPORT_AMOUNT,
        ID_M5_CONFIGURATION, ID_M5_REPORT_AMOUNT, ID_M6_CONFIGURATION, ID_M6_REPORT_AMOUNT,
        ID_M7_CONFIGURATION, ID_M7_REPORT_AMOUNT, ID_MDT_LOCATION_INFO, ID_MDTCONFIGURATION,
        ID_MDTCONFIGURATION_NR, ID_MOBILITY_INFORMATION, ID_MUTING_AVAILABILITY_INDICATION,
        ID_MUTING_PATTERN_INFORMATION, ID_NB_IO_T_RLF_REPORT_CONTAINER,
        ID_NRRESTRICTIONIN_EPSAS_SECONDARY_RAT, ID_NRRESTRICTIONIN5_GS,
        ID_PRO_SE_UETO_NETWORK_RELAYING, ID_PSCELL_INFORMATION, ID_RACSINDICATION,
        ID_RAT_RESTRICTIONS, ID_RAT_TYPE, ID_RECEIVE_STATUS_OF_ULPDCPSDUS_EXTENDED,
        ID_RECEIVE_STATUS_OF_ULPDCPSDUS_PDCP_SNLENGTH18, ID_RECOMMENDED_CELL_ITEM,
        ID_RECOMMENDED_ENBITEM, ID_REQUEST_TYPE_ADDITIONAL_INFO, ID_REQUESTED_TNLINFO,
        ID_SECONDARY_RATDATA_USAGE_REPORT_ITEM, ID_SECURITY_INDICATION,
        ID_SENSOR_MEASUREMENT_CONFIGURATION, ID_SERVICE_TYPE, ID_SIGNALLING_BASED_MDTPLMNLIST,
        ID_SON_INFORMATION_REPORT, ID_SOURCE_NODE_ID, ID_SOURCE_NODE_TRANSPORT_LAYER_ADDRESS,
        ID_SOURCE_SNID, ID_SOURCE_TRANSPORT_LAYER_ADDRESS, ID_SYNCHRONISATION_INFORMATION,
        ID_TIME_BASED_HANDOVER_INFORMATION, ID_TIME_REF_DISTRIBUTION, ID_TIME_SYNCHRONISATION_INFO,
        ID_TIME_UE_STAYED_IN_CELL_ENHANCED_GRANULARITY, ID_TRACE_COLLECTION_ENTITY_URI,
        ID_U_E_HISTORY_INFORMATION_FROM_THE_UE, ID_UEAPP_LAYER_MEAS_CONFIG,
        ID_UECONTEXT_REFERENCEAT_SOURCEE_NB, ID_ULCOUNTVALUE_EXTENDED,
        ID_ULCOUNTVALUE_PDCP_SNLENGTH18, ID_UNLICENSED_SPECTRUM_RESTRICTION,
        ID_UPLINK_PACKET_LOSS_RATE, ID_WLANMEASUREMENT_CONFIGURATION, ID_X2_TNLCONFIGURATION_INFO,
        MAX_EARFCN, MAX_NARFCN, MAX_RS_INDEX_CELL_QUAL, MAXNOOF_BLUETOOTH_NAME, MAXNOOF_BPLMNS,
        MAXNOOF_CELL_ID, MAXNOOF_CELL_IDFOR_MDT, MAXNOOF_CELL_IDFOR_QMC, MAXNOOF_CELLIN_EAI,
        MAXNOOF_CELLIN_TAI, MAXNOOF_CELLSFOR_RESTART, MAXNOOF_CELLSIN_UEHISTORY_INFO,
        MAXNOOF_CELLSINE_NB, MAXNOOF_CONNECTEDENG_NBS, MAXNOOF_CSGS, MAXNOOF_DCNS, MAXNOOF_E_RABS,
        MAXNOOF_EMERGENCY_AREA_ID, MAXNOOF_EPLMNS, MAXNOOF_EPLMNS_PLUS_ONE, MAXNOOF_ERRORS,
        MAXNOOF_FORB_LACS, MAXNOOF_FORB_TACS, MAXNOOF_GROUP_IDS, MAXNOOF_MBSFNAREA_MDT,
        MAXNOOF_MDTPLMNS, MAXNOOF_MMECS, MAXNOOF_PC5_QO_SFLOWS, MAXNOOF_PLMNFOR_QMC,
        MAXNOOF_PLMNS_PER_MME, MAXNOOF_PSCELLS_PER_PRIMARY_CELLIN_UEHISTORY_INFO, MAXNOOF_RATS,
        MAXNOOF_RECOMMENDED_CELLS, MAXNOOF_RECOMMENDED_ENBS, MAXNOOF_RESTART_EMERGENCY_AREA_IDS,
        MAXNOOF_RESTART_TAIS, MAXNOOF_SENSOR_NAME, MAXNOOF_TACS, MAXNOOF_TACS_IN_NTN,
        MAXNOOF_TAFOR_MDT, MAXNOOF_TAFOR_QMC, MAXNOOF_TAIFOR_WARNING, MAXNOOF_WLANNAME,
        MAXNOOFE_NBX2_EXT_TLAS, MAXNOOFE_NBX2_GTPTLAS, MAXNOOFE_NBX2_TLAS, MAXNOOFFREQUENCIES,
        MAXNOOFTIMEPERIODS,
    };
    use super::s1_ap_containers::*;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " A"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Additional-GUTI")]
    #[non_exhaustive]
    pub struct AdditionalGUTI {
        #[rasn(identifier = "gUMMEI")]
        pub g_ummei: GUMMEI,
        #[rasn(identifier = "m-TMSI")]
        pub m_tmsi: MTMSI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { AdditionalGUTI {
        #[rasn(identifier = "gUMMEI")]
        g_ummei: [GUMMEI],
        #[rasn(identifier = "m-TMSI")]
        m_tmsi: [MTMSI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl AdditionalGUTI {
        pub fn new(
            g_ummei: GUMMEI,
            m_tmsi: MTMSI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                g_ummei,
                m_tmsi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AdditionalCSFallbackIndicator {
        #[rasn(identifier = "no-restriction")]
        no_restriction = 0,
        restriction = 1,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct AdditionalRRMPriorityIndex(pub FixedBitString<32usize>);
    impl Decode for AdditionalRRMPriorityIndex {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {
            if decoder.codec() != rasn::Codec::Aper {
                return FixedBitString::<32usize>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self);
            }
            const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
            const REST: Constraints = rasn::constraints!(rasn::size_constraint!(24));
            let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
            let mut bits = BitString::from_element(first);
            bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
            let mut value = FixedBitString::<32usize>::ZERO;
            value[..32].copy_from_bitslice(&bits);
            Ok(Self(value))
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AerialUEsubscriptionInformation {
        allowed = 0,
        #[rasn(identifier = "not-allowed")]
        not_allowed = 1,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AllocationAndRetentionPriority {
        #[rasn(identifier = "priorityLevel")]
        pub priority_level: PriorityLevel,
        #[rasn(identifier = "pre-emptionCapability")]
        pub pre_emption_capability: PreEmptionCapability,
        #[rasn(identifier = "pre-emptionVulnerability")]
        pub pre_emption_vulnerability: PreEmptionVulnerability,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { AllocationAndRetentionPriority {
        #[rasn(identifier = "priorityLevel")]
        priority_level: [PriorityLevel],
        #[rasn(identifier = "pre-emptionCapability")]
        pre_emption_capability: [PreEmptionCapability],
        #[rasn(identifier = "pre-emptionVulnerability")]
        pre_emption_vulnerability: [PreEmptionVulnerability],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl AllocationAndRetentionPriority {
        pub fn new(
            priority_level: PriorityLevel,
            pre_emption_capability: PreEmptionCapability,
            pre_emption_vulnerability: PreEmptionVulnerability,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                priority_level,
                pre_emption_capability,
                pre_emption_vulnerability,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum AreaScopeOfMDT {
        cellBased(CellBasedMDT),
        tABased(TABasedMDT),
        pLMNWide(()),
        #[rasn(extension_addition)]
        tAIBased(TAIBasedMDT),
    }
    impl From<CellBasedMDT> for AreaScopeOfMDT {
        fn from(value: CellBasedMDT) -> Self {
            Self::cellBased(value)
        }
    }
    impl From<TABasedMDT> for AreaScopeOfMDT {
        fn from(value: TABasedMDT) -> Self {
            Self::tABased(value)
        }
    }
    impl From<()> for AreaScopeOfMDT {
        fn from(value: ()) -> Self {
            Self::pLMNWide(value)
        }
    }
    impl From<TAIBasedMDT> for AreaScopeOfMDT {
        fn from(value: TAIBasedMDT) -> Self {
            Self::tAIBased(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum AreaScopeOfQMC {
        cellBased(CellBasedQMC),
        tABased(TABasedQMC),
        tAIBased(TAIBasedQMC),
        pLMNAreaBased(PLMNAreaBasedQMC),
    }
    impl From<CellBasedQMC> for AreaScopeOfQMC {
        fn from(value: CellBasedQMC) -> Self {
            Self::cellBased(value)
        }
    }
    impl From<TABasedQMC> for AreaScopeOfQMC {
        fn from(value: TABasedQMC) -> Self {
            Self::tABased(value)
        }
    }
    impl From<TAIBasedQMC> for AreaScopeOfQMC {
        fn from(value: TAIBasedQMC) -> Self {
            Self::tAIBased(value)
        }
    }
    impl From<PLMNAreaBasedQMC> for AreaScopeOfQMC {
        fn from(value: PLMNAreaBasedQMC) -> Self {
            Self::pLMNAreaBased(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AssistanceDataForCECapableUEs {
        #[rasn(identifier = "cellIdentifierAndCELevelForCECapableUEs")]
        pub cell_identifier_and_celevel_for_cecapable_ues: CellIdentifierAndCELevelForCECapableUEs,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { AssistanceDataForCECapableUEs {
        #[rasn(identifier = "cellIdentifierAndCELevelForCECapableUEs")]
        cell_identifier_and_celevel_for_cecapable_ues: [CellIdentifierAndCELevelForCECapableUEs],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl AssistanceDataForCECapableUEs {
        pub fn new(
            cell_identifier_and_celevel_for_cecapable_ues: CellIdentifierAndCELevelForCECapableUEs,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                cell_identifier_and_celevel_for_cecapable_ues,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AssistanceDataForPaging {
        #[rasn(identifier = "assistanceDataForRecommendedCells")]
        pub assistance_data_for_recommended_cells: Option<AssistanceDataForRecommendedCells>,
        #[rasn(identifier = "assistanceDataForCECapableUEs")]
        pub assistance_data_for_cecapable_ues: Option<AssistanceDataForCECapableUEs>,
        #[rasn(identifier = "pagingAttemptInformation")]
        pub paging_attempt_information: Option<PagingAttemptInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { AssistanceDataForPaging {
        #[rasn(identifier = "assistanceDataForRecommendedCells")]
        assistance_data_for_recommended_cells: [Option<AssistanceDataForRecommendedCells>],
        #[rasn(identifier = "assistanceDataForCECapableUEs")]
        assistance_data_for_cecapable_ues: [Option<AssistanceDataForCECapableUEs>],
        #[rasn(identifier = "pagingAttemptInformation")]
        paging_attempt_information: [Option<PagingAttemptInformation>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl AssistanceDataForPaging {
        pub fn new(
            assistance_data_for_recommended_cells: Option<AssistanceDataForRecommendedCells>,
            assistance_data_for_cecapable_ues: Option<AssistanceDataForCECapableUEs>,
            paging_attempt_information: Option<PagingAttemptInformation>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                assistance_data_for_recommended_cells,
                assistance_data_for_cecapable_ues,
                paging_attempt_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AssistanceDataForRecommendedCells {
        #[rasn(identifier = "recommendedCellsForPaging")]
        pub recommended_cells_for_paging: RecommendedCellsForPaging,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { AssistanceDataForRecommendedCells {
        #[rasn(identifier = "recommendedCellsForPaging")]
        recommended_cells_for_paging: [RecommendedCellsForPaging],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl AssistanceDataForRecommendedCells {
        pub fn new(
            recommended_cells_for_paging: RecommendedCellsForPaging,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                recommended_cells_for_paging,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=6"))]
    pub struct BPLMNs(pub SequenceOf<PLMNidentity>);
    crate::per::sequence_of! { BPLMNs, 1, 6 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BearerType {
        #[rasn(identifier = "non-IP")]
        non_IP = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Bearers-SubjectToDLDiscarding-Item")]
    #[non_exhaustive]
    pub struct BearersSubjectToDLDiscardingItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "dL-Discarding")]
        pub d_l_discarding: DLDiscarding,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { BearersSubjectToDLDiscardingItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "dL-Discarding")]
        d_l_discarding: [DLDiscarding],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl BearersSubjectToDLDiscardingItem {
        pub fn new(
            e_rab_id: ERABID,
            d_l_discarding: DLDiscarding,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                d_l_discarding,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "Bearers-SubjectToDLDiscardingList"
    )]
    pub struct BearersSubjectToDLDiscardingList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { BearersSubjectToDLDiscardingList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "Bearers-SubjectToEarlyStatusTransfer-Item"
    )]
    #[non_exhaustive]
    pub struct BearersSubjectToEarlyStatusTransferItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "dLCOUNT-PDCP-SNlength")]
        pub d_lcount_pdcp_snlength: DLCOUNTPDCPSNlength,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { BearersSubjectToEarlyStatusTransferItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "dLCOUNT-PDCP-SNlength")]
        d_lcount_pdcp_snlength: [DLCOUNTPDCPSNlength],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl BearersSubjectToEarlyStatusTransferItem {
        pub fn new(
            e_rab_id: ERABID,
            d_lcount_pdcp_snlength: DLCOUNTPDCPSNlength,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                d_lcount_pdcp_snlength,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "Bearers-SubjectToEarlyStatusTransferList"
    )]
    pub struct BearersSubjectToEarlyStatusTransferList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { BearersSubjectToEarlyStatusTransferList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Bearers-SubjectToStatusTransfer-Item")]
    #[non_exhaustive]
    pub struct BearersSubjectToStatusTransferItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "uL-COUNTvalue")]
        pub u_l_countvalue: COUNTvalue,
        #[rasn(identifier = "dL-COUNTvalue")]
        pub d_l_countvalue: COUNTvalue,
        #[rasn(identifier = "receiveStatusofULPDCPSDUs")]
        pub receive_statusof_ulpdcpsdus: Option<ReceiveStatusofULPDCPSDUs>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { BearersSubjectToStatusTransferItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "uL-COUNTvalue")]
        u_l_countvalue: [COUNTvalue],
        #[rasn(identifier = "dL-COUNTvalue")]
        d_l_countvalue: [COUNTvalue],
        #[rasn(identifier = "receiveStatusofULPDCPSDUs")]
        receive_statusof_ulpdcpsdus: [Option<ReceiveStatusofULPDCPSDUs>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl BearersSubjectToStatusTransferItem {
        pub fn new(
            e_rab_id: ERABID,
            u_l_countvalue: COUNTvalue,
            d_l_countvalue: COUNTvalue,
            receive_statusof_ulpdcpsdus: Option<ReceiveStatusofULPDCPSDUs>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                u_l_countvalue,
                d_l_countvalue,
                receive_statusof_ulpdcpsdus,
                i_e_extensions,
            }
        }
    }
    #[doc = " B"]
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "Bearers-SubjectToStatusTransferList"
    )]
    pub struct BearersSubjectToStatusTransferList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { BearersSubjectToStatusTransferList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=10000000000"))]
    pub struct BitRate(pub u64);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BluetoothMeasConfig {
        setup = 0,
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct BluetoothMeasConfigNameList(pub SequenceOf<BluetoothName>);
    crate::per::sequence_of! { BluetoothMeasConfigNameList, 1, 4 }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BluetoothMeasurementConfigurationBtRssi {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BluetoothMeasurementConfiguration {
        #[rasn(identifier = "bluetoothMeasConfig")]
        pub bluetooth_meas_config: BluetoothMeasConfig,
        #[rasn(identifier = "bluetoothMeasConfigNameList")]
        pub bluetooth_meas_config_name_list: Option<BluetoothMeasConfigNameList>,
        #[rasn(identifier = "bt-rssi")]
        pub bt_rssi: Option<BluetoothMeasurementConfigurationBtRssi>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { BluetoothMeasurementConfiguration {
        #[rasn(identifier = "bluetoothMeasConfig")]
        bluetooth_meas_config: [BluetoothMeasConfig],
        #[rasn(identifier = "bluetoothMeasConfigNameList")]
        bluetooth_meas_config_name_list: [Option<BluetoothMeasConfigNameList>],
        #[rasn(identifier = "bt-rssi")]
        bt_rssi: [Option<BluetoothMeasurementConfigurationBtRssi>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl BluetoothMeasurementConfiguration {
        pub fn new(
            bluetooth_meas_config: BluetoothMeasConfig,
            bluetooth_meas_config_name_list: Option<BluetoothMeasConfigNameList>,
            bt_rssi: Option<BluetoothMeasurementConfigurationBtRssi>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                bluetooth_meas_config,
                bluetooth_meas_config_name_list,
                bt_rssi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=248"))]
    pub struct BluetoothName(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum BroadcastCancelledAreaList {
        #[rasn(identifier = "cellID-Cancelled")]
        cellID_Cancelled(CellIDCancelled),
        #[rasn(identifier = "tAI-Cancelled")]
        tAI_Cancelled(TAICancelled),
        #[rasn(identifier = "emergencyAreaID-Cancelled")]
        emergencyAreaID_Cancelled(EmergencyAreaIDCancelled),
    }
    impl From<CellIDCancelled> for BroadcastCancelledAreaList {
        fn from(value: CellIDCancelled) -> Self {
            Self::cellID_Cancelled(value)
        }
    }
    impl From<TAICancelled> for BroadcastCancelledAreaList {
        fn from(value: TAICancelled) -> Self {
            Self::tAI_Cancelled(value)
        }
    }
    impl From<EmergencyAreaIDCancelled> for BroadcastCancelledAreaList {
        fn from(value: EmergencyAreaIDCancelled) -> Self {
            Self::emergencyAreaID_Cancelled(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum BroadcastCompletedAreaList {
        #[rasn(identifier = "cellID-Broadcast")]
        cellID_Broadcast(CellIDBroadcast),
        #[rasn(identifier = "tAI-Broadcast")]
        tAI_Broadcast(TAIBroadcast),
        #[rasn(identifier = "emergencyAreaID-Broadcast")]
        emergencyAreaID_Broadcast(EmergencyAreaIDBroadcast),
    }
    impl From<CellIDBroadcast> for BroadcastCompletedAreaList {
        fn from(value: CellIDBroadcast) -> Self {
            Self::cellID_Broadcast(value)
        }
    }
    impl From<TAIBroadcast> for BroadcastCompletedAreaList {
        fn from(value: TAIBroadcast) -> Self {
            Self::tAI_Broadcast(value)
        }
    }
    impl From<EmergencyAreaIDBroadcast> for BroadcastCompletedAreaList {
        fn from(value: EmergencyAreaIDBroadcast) -> Self {
            Self::emergencyAreaID_Broadcast(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "CE-ModeBRestricted")]
    #[non_exhaustive]
    pub enum CEModeBRestricted {
        restricted = 0,
        #[rasn(identifier = "not-restricted")]
        not_restricted = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "CE-mode-B-SupportIndicator")]
    #[non_exhaustive]
    pub enum CEModeBSupportIndicator {
        supported = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CELevel(pub OctetString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CGI {
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(identifier = "lAC")]
        pub l_ac: LAC,
        #[rasn(identifier = "cI")]
        pub c_i: CI,
        #[rasn(identifier = "rAC")]
        pub r_ac: Option<RAC>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CGI {
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(identifier = "lAC")]
        l_ac: [LAC],
        #[rasn(identifier = "cI")]
        c_i: [CI],
        #[rasn(identifier = "rAC")]
        r_ac: [Option<RAC>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CGI {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            l_ac: LAC,
            c_i: CI,
            r_ac: Option<RAC>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                l_ac,
                c_i,
                r_ac,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CI(pub FixedOctetString<2usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum CNDomain {
        ps = 0,
        cs = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CNType {
        fiveGCForbidden = 0,
        #[rasn(extension_addition, identifier = "epc-Forbiddden")]
        epc_Forbiddden = 1,
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct CNTypeRestrictions(pub SequenceOf<CNTypeRestrictionsItem>);
    crate::per::sequence_of! { CNTypeRestrictions, 1, 16 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "CNTypeRestrictions-Item")]
    #[non_exhaustive]
    pub struct CNTypeRestrictionsItem {
        #[rasn(identifier = "pLMN-Identity")]
        pub p_lmn_identity: PLMNidentity,
        #[rasn(identifier = "cNType")]
        pub c_ntype: CNType,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CNTypeRestrictionsItem {
        #[rasn(identifier = "pLMN-Identity")]
        p_lmn_identity: [PLMNidentity],
        #[rasn(identifier = "cNType")]
        c_ntype: [CNType],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CNTypeRestrictionsItem {
        pub fn new(
            p_lmn_identity: PLMNidentity,
            c_ntype: CNType,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmn_identity,
                c_ntype,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct COUNTValueExtended {
        #[rasn(identifier = "pDCP-SNExtended")]
        pub p_dcp_snextended: PDCPSNExtended,
        #[rasn(identifier = "hFNModified")]
        pub h_fnmodified: HFNModified,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { COUNTValueExtended {
        #[rasn(identifier = "pDCP-SNExtended")]
        p_dcp_snextended: [PDCPSNExtended],
        #[rasn(identifier = "hFNModified")]
        h_fnmodified: [HFNModified],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl COUNTValueExtended {
        pub fn new(
            p_dcp_snextended: PDCPSNExtended,
            h_fnmodified: HFNModified,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_dcp_snextended,
                h_fnmodified,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct COUNTvalue {
        #[rasn(identifier = "pDCP-SN")]
        pub p_dcp_sn: PDCPSN,
        #[rasn(identifier = "hFN")]
        pub h_fn: HFN,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { COUNTvalue {
        #[rasn(identifier = "pDCP-SN")]
        p_dcp_sn: [PDCPSN],
        #[rasn(identifier = "hFN")]
        h_fn: [HFN],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl COUNTvalue {
        pub fn new(
            p_dcp_sn: PDCPSN,
            h_fn: HFN,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_dcp_sn,
                h_fn,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "COUNTvaluePDCP-SNlength18")]
    #[non_exhaustive]
    pub struct COUNTvaluePDCPSNlength18 {
        #[rasn(identifier = "pDCP-SNlength18")]
        pub p_dcp_snlength18: PDCPSNlength18,
        #[rasn(identifier = "hFNforPDCP-SNlength18")]
        pub h_fnfor_pdcp_snlength18: HFNforPDCPSNlength18,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { COUNTvaluePDCPSNlength18 {
        #[rasn(identifier = "pDCP-SNlength18")]
        p_dcp_snlength18: [PDCPSNlength18],
        #[rasn(identifier = "hFNforPDCP-SNlength18")]
        h_fnfor_pdcp_snlength18: [HFNforPDCPSNlength18],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl COUNTvaluePDCPSNlength18 {
        pub fn new(
            p_dcp_snlength18: PDCPSNlength18,
            h_fnfor_pdcp_snlength18: HFNforPDCPSNlength18,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_dcp_snlength18,
                h_fnfor_pdcp_snlength18,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CSFallbackIndicator {
        #[rasn(identifier = "cs-fallback-required")]
        cs_fallback_required = 0,
        #[rasn(extension_addition, identifier = "cs-fallback-high-priority")]
        cs_fallback_high_priority = 1,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "CSG-Id")]
    pub struct CSGId(pub FixedBitString<27usize>);
    impl Decode for CSGId {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {
            if decoder.codec() != rasn::Codec::Aper {
                return FixedBitString::<27usize>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self);
            }
            const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
            const REST: Constraints = rasn::constraints!(rasn::size_constraint!(19));
            let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
            let mut bits = BitString::from_element(first);
            bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
            let mut value = FixedBitString::<27usize>::ZERO;
            value[..27].copy_from_bitslice(&bits);
            Ok(Self(value))
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "CSG-IdList")]
    pub struct CSGIdList(pub SequenceOf<CSGIdListItem>);
    crate::per::sequence_of! { CSGIdList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "CSG-IdList-Item")]
    #[non_exhaustive]
    pub struct CSGIdListItem {
        #[rasn(identifier = "cSG-Id")]
        pub c_sg_id: CSGId,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CSGIdListItem {
        #[rasn(identifier = "cSG-Id")]
        c_sg_id: [CSGId],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CSGIdListItem {
        pub fn new(c_sg_id: CSGId, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                c_sg_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum CSGMembershipStatus {
        member = 0,
        #[rasn(identifier = "not-member")]
        not_member = 1,
    }
    #[doc = " C"]
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct CancelledCellinEAI(pub SequenceOf<CancelledCellinEAIItem>);
    crate::per::sequence_of! { CancelledCellinEAI, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "CancelledCellinEAI-Item")]
    #[non_exhaustive]
    pub struct CancelledCellinEAIItem {
        #[rasn(identifier = "eCGI")]
        pub e_cgi: EUTRANCGI,
        #[rasn(identifier = "numberOfBroadcasts")]
        pub number_of_broadcasts: NumberOfBroadcasts,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CancelledCellinEAIItem {
        #[rasn(identifier = "eCGI")]
        e_cgi: [EUTRANCGI],
        #[rasn(identifier = "numberOfBroadcasts")]
        number_of_broadcasts: [NumberOfBroadcasts],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CancelledCellinEAIItem {
        pub fn new(
            e_cgi: EUTRANCGI,
            number_of_broadcasts: NumberOfBroadcasts,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_cgi,
                number_of_broadcasts,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct CancelledCellinTAI(pub SequenceOf<CancelledCellinTAIItem>);
    crate::per::sequence_of! { CancelledCellinTAI, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "CancelledCellinTAI-Item")]
    #[non_exhaustive]
    pub struct CancelledCellinTAIItem {
        #[rasn(identifier = "eCGI")]
        pub e_cgi: EUTRANCGI,
        #[rasn(identifier = "numberOfBroadcasts")]
        pub number_of_broadcasts: NumberOfBroadcasts,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CancelledCellinTAIItem {
        #[rasn(identifier = "eCGI")]
        e_cgi: [EUTRANCGI],
        #[rasn(identifier = "numberOfBroadcasts")]
        number_of_broadcasts: [NumberOfBroadcasts],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CancelledCellinTAIItem {
        pub fn new(
            e_cgi: EUTRANCGI,
            number_of_broadcasts: NumberOfBroadcasts,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_cgi,
                number_of_broadcasts,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum Cause {
        radioNetwork(CauseRadioNetwork),
        transport(CauseTransport),
        nas(CauseNas),
        protocol(CauseProtocol),
        misc(CauseMisc),
    }
    impl From<CauseRadioNetwork> for Cause {
        fn from(value: CauseRadioNetwork) -> Self {
            Self::radioNetwork(value)
        }
    }
    impl From<CauseTransport> for Cause {
        fn from(value: CauseTransport) -> Self {
            Self::transport(value)
        }
    }
    impl From<CauseNas> for Cause {
        fn from(value: CauseNas) -> Self {
            Self::nas(value)
        }
    }
    impl From<CauseProtocol> for Cause {
        fn from(value: CauseProtocol) -> Self {
            Self::protocol(value)
        }
    }
    impl From<CauseMisc> for Cause {
        fn from(value: CauseMisc) -> Self {
            Self::misc(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CauseMisc {
        #[rasn(identifier = "control-processing-overload")]
        control_processing_overload = 0,
        #[rasn(identifier = "not-enough-user-plane-processing-resources")]
        not_enough_user_plane_processing_resources = 1,
        #[rasn(identifier = "hardware-failure")]
        hardware_failure = 2,
        #[rasn(identifier = "om-intervention")]
        om_intervention = 3,
        unspecified = 4,
        #[rasn(identifier = "unknown-PLMN")]
        unknown_PLMN = 5,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CauseNas {
        #[rasn(identifier = "normal-release")]
        normal_release = 0,
        #[rasn(identifier = "authentication-failure")]
        authentication_failure = 1,
        detach = 2,
        unspecified = 3,
        #[rasn(extension_addition, identifier = "csg-subscription-expiry")]
        csg_subscription_expiry = 4,
        #[rasn(extension_addition, identifier = "uE-not-in-PLMN-serving-area")]
        uE_not_in_PLMN_serving_area = 5,
        #[rasn(extension_addition, identifier = "iab-not-authorized")]
        iab_not_authorized = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CauseProtocol {
        #[rasn(identifier = "transfer-syntax-error")]
        transfer_syntax_error = 0,
        #[rasn(identifier = "abstract-syntax-error-reject")]
        abstract_syntax_error_reject = 1,
        #[rasn(identifier = "abstract-syntax-error-ignore-and-notify")]
        abstract_syntax_error_ignore_and_notify = 2,
        #[rasn(identifier = "message-not-compatible-with-receiver-state")]
        message_not_compatible_with_receiver_state = 3,
        #[rasn(identifier = "semantic-error")]
        semantic_error = 4,
        #[rasn(identifier = "abstract-syntax-error-falsely-constructed-message")]
        abstract_syntax_error_falsely_constructed_message = 5,
        unspecified = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CauseRadioNetwork {
        unspecified = 0,
        #[rasn(identifier = "tx2relocoverall-expiry")]
        tx2relocoverall_expiry = 1,
        #[rasn(identifier = "successful-handover")]
        successful_handover = 2,
        #[rasn(identifier = "release-due-to-eutran-generated-reason")]
        release_due_to_eutran_generated_reason = 3,
        #[rasn(identifier = "handover-cancelled")]
        handover_cancelled = 4,
        #[rasn(identifier = "partial-handover")]
        partial_handover = 5,
        #[rasn(identifier = "ho-failure-in-target-EPC-eNB-or-target-system")]
        ho_failure_in_target_EPC_eNB_or_target_system = 6,
        #[rasn(identifier = "ho-target-not-allowed")]
        ho_target_not_allowed = 7,
        #[rasn(identifier = "tS1relocoverall-expiry")]
        tS1relocoverall_expiry = 8,
        #[rasn(identifier = "tS1relocprep-expiry")]
        tS1relocprep_expiry = 9,
        #[rasn(identifier = "cell-not-available")]
        cell_not_available = 10,
        #[rasn(identifier = "unknown-targetID")]
        unknown_targetID = 11,
        #[rasn(identifier = "no-radio-resources-available-in-target-cell")]
        no_radio_resources_available_in_target_cell = 12,
        #[rasn(identifier = "unknown-mme-ue-s1ap-id")]
        unknown_mme_ue_s1ap_id = 13,
        #[rasn(identifier = "unknown-enb-ue-s1ap-id")]
        unknown_enb_ue_s1ap_id = 14,
        #[rasn(identifier = "unknown-pair-ue-s1ap-id")]
        unknown_pair_ue_s1ap_id = 15,
        #[rasn(identifier = "handover-desirable-for-radio-reason")]
        handover_desirable_for_radio_reason = 16,
        #[rasn(identifier = "time-critical-handover")]
        time_critical_handover = 17,
        #[rasn(identifier = "resource-optimisation-handover")]
        resource_optimisation_handover = 18,
        #[rasn(identifier = "reduce-load-in-serving-cell")]
        reduce_load_in_serving_cell = 19,
        #[rasn(identifier = "user-inactivity")]
        user_inactivity = 20,
        #[rasn(identifier = "radio-connection-with-ue-lost")]
        radio_connection_with_ue_lost = 21,
        #[rasn(identifier = "load-balancing-tau-required")]
        load_balancing_tau_required = 22,
        #[rasn(identifier = "cs-fallback-triggered")]
        cs_fallback_triggered = 23,
        #[rasn(identifier = "ue-not-available-for-ps-service")]
        ue_not_available_for_ps_service = 24,
        #[rasn(identifier = "radio-resources-not-available")]
        radio_resources_not_available = 25,
        #[rasn(identifier = "failure-in-radio-interface-procedure")]
        failure_in_radio_interface_procedure = 26,
        #[rasn(identifier = "invalid-qos-combination")]
        invalid_qos_combination = 27,
        #[rasn(identifier = "interrat-redirection")]
        interrat_redirection = 28,
        #[rasn(identifier = "interaction-with-other-procedure")]
        interaction_with_other_procedure = 29,
        #[rasn(identifier = "unknown-E-RAB-ID")]
        unknown_E_RAB_ID = 30,
        #[rasn(identifier = "multiple-E-RAB-ID-instances")]
        multiple_E_RAB_ID_instances = 31,
        #[rasn(identifier = "encryption-and-or-integrity-protection-algorithms-not-supported")]
        encryption_and_or_integrity_protection_algorithms_not_supported = 32,
        #[rasn(identifier = "s1-intra-system-handover-triggered")]
        s1_intra_system_handover_triggered = 33,
        #[rasn(identifier = "s1-inter-system-handover-triggered")]
        s1_inter_system_handover_triggered = 34,
        #[rasn(identifier = "x2-handover-triggered")]
        x2_handover_triggered = 35,
        #[rasn(extension_addition, identifier = "redirection-towards-1xRTT")]
        redirection_towards_1xRTT = 36,
        #[rasn(extension_addition, identifier = "not-supported-QCI-value")]
        not_supported_QCI_value = 37,
        #[rasn(extension_addition, identifier = "invalid-CSG-Id")]
        invalid_CSG_Id = 38,
        #[rasn(extension_addition, identifier = "release-due-to-pre-emption")]
        release_due_to_pre_emption = 39,
        #[rasn(extension_addition, identifier = "n26-interface-not-available")]
        n26_interface_not_available = 40,
        #[rasn(extension_addition, identifier = "insufficient-ue-capabilities")]
        insufficient_ue_capabilities = 41,
        #[rasn(
            extension_addition,
            identifier = "maximum-bearer-pre-emption-rate-exceeded"
        )]
        maximum_bearer_pre_emption_rate_exceeded = 42,
        #[rasn(
            extension_addition,
            identifier = "up-integrity-protection-not-possible"
        )]
        up_integrity_protection_not_possible = 43,
        #[rasn(
            extension_addition,
            identifier = "release-due-to-discontinuous-coverage"
        )]
        release_due_to_discontinuous_coverage = 44,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CauseTransport {
        #[rasn(identifier = "transport-resource-unavailable")]
        transport_resource_unavailable = 0,
        unspecified = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum Cdma2000HORequiredIndication {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum Cdma2000HOStatus {
        hOSuccess = 0,
        hOFailure = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct Cdma2000OneXMEID(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct Cdma2000OneXMSI(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct Cdma2000OneXPilot(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct Cdma2000OneXRAND(pub OctetString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Cdma2000OneXSRVCCInfo {
        #[rasn(identifier = "cdma2000OneXMEID")]
        pub cdma2000_one_xmeid: Cdma2000OneXMEID,
        #[rasn(identifier = "cdma2000OneXMSI")]
        pub cdma2000_one_xmsi: Cdma2000OneXMSI,
        #[rasn(identifier = "cdma2000OneXPilot")]
        pub cdma2000_one_xpilot: Cdma2000OneXPilot,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { Cdma2000OneXSRVCCInfo {
        #[rasn(identifier = "cdma2000OneXMEID")]
        cdma2000_one_xmeid: [Cdma2000OneXMEID],
        #[rasn(identifier = "cdma2000OneXMSI")]
        cdma2000_one_xmsi: [Cdma2000OneXMSI],
        #[rasn(identifier = "cdma2000OneXPilot")]
        cdma2000_one_xpilot: [Cdma2000OneXPilot],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl Cdma2000OneXSRVCCInfo {
        pub fn new(
            cdma2000_one_xmeid: Cdma2000OneXMEID,
            cdma2000_one_xmsi: Cdma2000OneXMSI,
            cdma2000_one_xpilot: Cdma2000OneXPilot,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                cdma2000_one_xmeid,
                cdma2000_one_xmsi,
                cdma2000_one_xpilot,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct Cdma2000PDU(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum Cdma2000RATType {
        hRPD = 0,
        onexRTT = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct Cdma2000SectorID(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Cell-Size")]
    #[non_exhaustive]
    pub enum CellSize {
        verysmall = 0,
        small = 1,
        medium = 2,
        large = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CellAccessMode {
        hybrid = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CellBasedMDT {
        #[rasn(identifier = "cellIdListforMDT")]
        pub cell_id_listfor_mdt: CellIdListforMDT,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CellBasedMDT {
        #[rasn(identifier = "cellIdListforMDT")]
        cell_id_listfor_mdt: [CellIdListforMDT],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CellBasedMDT {
        pub fn new(
            cell_id_listfor_mdt: CellIdListforMDT,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                cell_id_listfor_mdt,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CellBasedQMC {
        #[rasn(identifier = "cellIdListforQMC")]
        pub cell_id_listfor_qmc: CellIdListforQMC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CellBasedQMC {
        #[rasn(identifier = "cellIdListforQMC")]
        cell_id_listfor_qmc: [CellIdListforQMC],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CellBasedQMC {
        pub fn new(
            cell_id_listfor_qmc: CellIdListforQMC,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                cell_id_listfor_qmc,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"), identifier = "CellID-Broadcast")]
    pub struct CellIDBroadcast(pub SequenceOf<CellIDBroadcastItem>);
    crate::per::sequence_of! { CellIDBroadcast, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "CellID-Broadcast-Item")]
    #[non_exhaustive]
    pub struct CellIDBroadcastItem {
        #[rasn(identifier = "eCGI")]
        pub e_cgi: EUTRANCGI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CellIDBroadcastItem {
        #[rasn(identifier = "eCGI")]
        e_cgi: [EUTRANCGI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CellIDBroadcastItem {
        pub fn new(e_cgi: EUTRANCGI, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                e_cgi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"), identifier = "CellID-Cancelled")]
    pub struct CellIDCancelled(pub SequenceOf<CellIDCancelledItem>);
    crate::per::sequence_of! { CellIDCancelled, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "CellID-Cancelled-Item")]
    #[non_exhaustive]
    pub struct CellIDCancelledItem {
        #[rasn(identifier = "eCGI")]
        pub e_cgi: EUTRANCGI,
        #[rasn(identifier = "numberOfBroadcasts")]
        pub number_of_broadcasts: NumberOfBroadcasts,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CellIDCancelledItem {
        #[rasn(identifier = "eCGI")]
        e_cgi: [EUTRANCGI],
        #[rasn(identifier = "numberOfBroadcasts")]
        number_of_broadcasts: [NumberOfBroadcasts],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CellIDCancelledItem {
        pub fn new(
            e_cgi: EUTRANCGI,
            number_of_broadcasts: NumberOfBroadcasts,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_cgi,
                number_of_broadcasts,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct CellIdListforMDT(pub SequenceOf<EUTRANCGI>);
    crate::per::sequence_of! { CellIdListforMDT, 1, 32 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct CellIdListforQMC(pub SequenceOf<EUTRANCGI>);
    crate::per::sequence_of! { CellIdListforQMC, 1, 32 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CellIdentifierAndCELevelForCECapableUEs {
        #[rasn(identifier = "global-Cell-ID")]
        pub global_cell_id: EUTRANCGI,
        #[rasn(identifier = "cELevel")]
        pub c_elevel: CELevel,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CellIdentifierAndCELevelForCECapableUEs {
        #[rasn(identifier = "global-Cell-ID")]
        global_cell_id: [EUTRANCGI],
        #[rasn(identifier = "cELevel")]
        c_elevel: [CELevel],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CellIdentifierAndCELevelForCECapableUEs {
        pub fn new(
            global_cell_id: EUTRANCGI,
            c_elevel: CELevel,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                global_cell_id,
                c_elevel,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CellIdentity(pub FixedBitString<28usize>);
    impl Decode for CellIdentity {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {
            if decoder.codec() != rasn::Codec::Aper {
                return FixedBitString::<28usize>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self);
            }
            const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
            const REST: Constraints = rasn::constraints!(rasn::size_constraint!(20));
            let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
            let mut bits = BitString::from_element(first);
            bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
            let mut value = FixedBitString::<28usize>::ZERO;
            value[..28].copy_from_bitslice(&bits);
            Ok(Self(value))
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CellType {
        #[rasn(identifier = "cell-Size")]
        pub cell_size: CellSize,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CellType {
        #[rasn(identifier = "cell-Size")]
        cell_size: [CellSize],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CellType {
        pub fn new(
            cell_size: CellSize,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                cell_size,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CoarseUELocation(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CoarseUELocationRequested {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct CompletedCellinEAI(pub SequenceOf<CompletedCellinEAIItem>);
    crate::per::sequence_of! { CompletedCellinEAI, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "CompletedCellinEAI-Item")]
    #[non_exhaustive]
    pub struct CompletedCellinEAIItem {
        #[rasn(identifier = "eCGI")]
        pub e_cgi: EUTRANCGI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CompletedCellinEAIItem {
        #[rasn(identifier = "eCGI")]
        e_cgi: [EUTRANCGI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CompletedCellinEAIItem {
        pub fn new(e_cgi: EUTRANCGI, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                e_cgi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct CompletedCellinTAI(pub SequenceOf<CompletedCellinTAIItem>);
    crate::per::sequence_of! { CompletedCellinTAI, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "CompletedCellinTAI-Item")]
    #[non_exhaustive]
    pub struct CompletedCellinTAIItem {
        #[rasn(identifier = "eCGI")]
        pub e_cgi: EUTRANCGI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CompletedCellinTAIItem {
        #[rasn(identifier = "eCGI")]
        e_cgi: [EUTRANCGI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CompletedCellinTAIItem {
        pub fn new(e_cgi: EUTRANCGI, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                e_cgi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum ConcurrentWarningMessageIndicator {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ConnectedengNBItem {
        #[rasn(identifier = "en-gNB-ID")]
        pub en_g_nb_id: EnGNBID,
        #[rasn(identifier = "supportedTAs")]
        pub supported_tas: SupportedTAs,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ConnectedengNBItem {
        #[rasn(identifier = "en-gNB-ID")]
        en_g_nb_id: [EnGNBID],
        #[rasn(identifier = "supportedTAs")]
        supported_tas: [SupportedTAs],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ConnectedengNBItem {
        pub fn new(
            en_g_nb_id: EnGNBID,
            supported_tas: SupportedTAs,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                en_g_nb_id,
                supported_tas,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct ConnectedengNBList(pub SequenceOf<ConnectedengNBItem>);
    crate::per::sequence_of! { ConnectedengNBList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ContextatSource {
        #[rasn(identifier = "sourceNG-RAN-node-ID")]
        pub source_ng_ran_node_id: GlobalRANNODEID,
        #[rasn(identifier = "rAN-UE-NGAP-ID")]
        pub r_an_ue_ngap_id: RANUENGAPID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ContextatSource {
        #[rasn(identifier = "sourceNG-RAN-node-ID")]
        source_ng_ran_node_id: [GlobalRANNODEID],
        #[rasn(identifier = "rAN-UE-NGAP-ID")]
        r_an_ue_ngap_id: [RANUENGAPID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ContextatSource {
        pub fn new(
            source_ng_ran_node_id: GlobalRANNODEID,
            r_an_ue_ngap_id: RANUENGAPID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                source_ng_ran_node_id,
                r_an_ue_ngap_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Correlation-ID")]
    pub struct CorrelationID(pub FixedOctetString<4usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Coverage-Level")]
    #[non_exhaustive]
    pub enum CoverageLevel {
        extendedcoverage = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CriticalityDiagnostics {
        #[rasn(identifier = "procedureCode")]
        pub procedure_code: Option<ProcedureCode>,
        #[rasn(identifier = "triggeringMessage")]
        pub triggering_message: Option<TriggeringMessage>,
        #[rasn(identifier = "procedureCriticality")]
        pub procedure_criticality: Option<Criticality>,
        #[rasn(identifier = "iEsCriticalityDiagnostics")]
        pub i_es_criticality_diagnostics: Option<CriticalityDiagnosticsIEList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CriticalityDiagnostics {
        #[rasn(identifier = "procedureCode")]
        procedure_code: [Option<ProcedureCode>],
        #[rasn(identifier = "triggeringMessage")]
        triggering_message: [Option<TriggeringMessage>],
        #[rasn(identifier = "procedureCriticality")]
        procedure_criticality: [Option<Criticality>],
        #[rasn(identifier = "iEsCriticalityDiagnostics")]
        i_es_criticality_diagnostics: [Option<CriticalityDiagnosticsIEList>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CriticalityDiagnostics {
        pub fn new(
            procedure_code: Option<ProcedureCode>,
            triggering_message: Option<TriggeringMessage>,
            procedure_criticality: Option<Criticality>,
            i_es_criticality_diagnostics: Option<CriticalityDiagnosticsIEList>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                procedure_code,
                triggering_message,
                procedure_criticality,
                i_es_criticality_diagnostics,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "CriticalityDiagnostics-IE-Item")]
    #[non_exhaustive]
    pub struct CriticalityDiagnosticsIEItem {
        #[rasn(identifier = "iECriticality")]
        pub i_ecriticality: Criticality,
        #[rasn(identifier = "iE-ID")]
        pub i_e_id: ProtocolIEID,
        #[rasn(identifier = "typeOfError")]
        pub type_of_error: TypeOfError,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CriticalityDiagnosticsIEItem {
        #[rasn(identifier = "iECriticality")]
        i_ecriticality: [Criticality],
        #[rasn(identifier = "iE-ID")]
        i_e_id: [ProtocolIEID],
        #[rasn(identifier = "typeOfError")]
        type_of_error: [TypeOfError],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CriticalityDiagnosticsIEItem {
        pub fn new(
            i_ecriticality: Criticality,
            i_e_id: ProtocolIEID,
            type_of_error: TypeOfError,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                i_ecriticality,
                i_e_id,
                type_of_error,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "CriticalityDiagnostics-IE-List"
    )]
    pub struct CriticalityDiagnosticsIEList(pub SequenceOf<CriticalityDiagnosticsIEItem>);
    crate::per::sequence_of! { CriticalityDiagnosticsIEList, 1, 256 }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DAPSRequestInfoDAPSIndicator {
        #[rasn(identifier = "dAPS-HO-required")]
        dAPS_HO_required = 0,
    }
    #[doc = " D"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DAPSRequestInfo {
        #[rasn(identifier = "dAPSIndicator")]
        pub d_apsindicator: DAPSRequestInfoDAPSIndicator,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { DAPSRequestInfo {
        #[rasn(identifier = "dAPSIndicator")]
        d_apsindicator: [DAPSRequestInfoDAPSIndicator],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl DAPSRequestInfo {
        pub fn new(
            d_apsindicator: DAPSRequestInfoDAPSIndicator,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                d_apsindicator,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DAPSResponseInfoDapsresponseindicator {
        #[rasn(identifier = "dAPS-HO-accepted")]
        dAPS_HO_accepted = 0,
        #[rasn(identifier = "dAPS-HO-not-accepted")]
        dAPS_HO_not_accepted = 1,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DAPSResponseInfo {
        pub dapsresponseindicator: DAPSResponseInfoDapsresponseindicator,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { DAPSResponseInfo {
        dapsresponseindicator: [DAPSResponseInfoDapsresponseindicator],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl DAPSResponseInfo {
        pub fn new(
            dapsresponseindicator: DAPSResponseInfoDapsresponseindicator,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                dapsresponseindicator,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DAPSResponseInfoItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "dAPSResponseInfo")]
        pub d_apsresponse_info: DAPSResponseInfo,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { DAPSResponseInfoItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "dAPSResponseInfo")]
        d_apsresponse_info: [DAPSResponseInfo],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl DAPSResponseInfoItem {
        pub fn new(
            e_rab_id: ERABID,
            d_apsresponse_info: DAPSResponseInfo,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                d_apsresponse_info,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct DAPSResponseInfoList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { DAPSResponseInfoList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "DCN-ID", value("0..=65535"))]
    pub struct DCNID(pub u16);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DL-CP-SecurityInformation")]
    #[non_exhaustive]
    pub struct DLCPSecurityInformation {
        #[rasn(identifier = "dl-NAS-MAC")]
        pub dl_nas_mac: DLNASMAC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { DLCPSecurityInformation {
        #[rasn(identifier = "dl-NAS-MAC")]
        dl_nas_mac: [DLNASMAC],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl DLCPSecurityInformation {
        pub fn new(
            dl_nas_mac: DLNASMAC,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                dl_nas_mac,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "DL-Forwarding")]
    #[non_exhaustive]
    pub enum DLForwarding {
        #[rasn(identifier = "dL-Forwarding-proposed")]
        dL_Forwarding_proposed = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "DL-NAS-MAC")]
    pub struct DLNASMAC(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "DLCOUNT-PDCP-SNlength")]
    #[non_exhaustive]
    pub enum DLCOUNTPDCPSNlength {
        #[rasn(identifier = "dLCOUNTValuePDCP-SNlength12")]
        dLCOUNTValuePDCP_SNlength12(COUNTvalue),
        #[rasn(identifier = "dLCOUNTValuePDCP-SNlength15")]
        dLCOUNTValuePDCP_SNlength15(COUNTValueExtended),
        #[rasn(identifier = "dLCOUNTValuePDCP-SNlength18")]
        dLCOUNTValuePDCP_SNlength18(COUNTvaluePDCPSNlength18),
    }
    impl From<COUNTvalue> for DLCOUNTPDCPSNlength {
        fn from(value: COUNTvalue) -> Self {
            Self::dLCOUNTValuePDCP_SNlength12(value)
        }
    }
    impl From<COUNTValueExtended> for DLCOUNTPDCPSNlength {
        fn from(value: COUNTValueExtended) -> Self {
            Self::dLCOUNTValuePDCP_SNlength15(value)
        }
    }
    impl From<COUNTvaluePDCPSNlength18> for DLCOUNTPDCPSNlength {
        fn from(value: COUNTvaluePDCPSNlength18) -> Self {
            Self::dLCOUNTValuePDCP_SNlength18(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum DLDiscarding {
        #[rasn(identifier = "discardDLCOUNTValuePDCP-SNlength12")]
        discardDLCOUNTValuePDCP_SNlength12(COUNTvalue),
        #[rasn(identifier = "discardDLCOUNTValuePDCP-SNlength15")]
        discardDLCOUNTValuePDCP_SNlength15(COUNTValueExtended),
        #[rasn(identifier = "discardDLCOUNTValuePDCP-SNlength18")]
        discardDLCOUNTValuePDCP_SNlength18(COUNTvaluePDCPSNlength18),
    }
    impl From<COUNTvalue> for DLDiscarding {
        fn from(value: COUNTvalue) -> Self {
            Self::discardDLCOUNTValuePDCP_SNlength12(value)
        }
    }
    impl From<COUNTValueExtended> for DLDiscarding {
        fn from(value: COUNTValueExtended) -> Self {
            Self::discardDLCOUNTValuePDCP_SNlength15(value)
        }
    }
    impl From<COUNTvaluePDCPSNlength18> for DLDiscarding {
        fn from(value: COUNTvaluePDCPSNlength18) -> Self {
            Self::discardDLCOUNTValuePDCP_SNlength18(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum DLNASPDUDeliveryAckRequest {
        requested = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Data-Forwarding-Not-Possible")]
    #[non_exhaustive]
    pub enum DataForwardingNotPossible {
        #[rasn(identifier = "data-Forwarding-not-Possible")]
        data_Forwarding_not_Possible = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct DataCodingScheme(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=4095", extensible))]
    pub struct DataSize(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Direct-Forwarding-Path-Availability")]
    #[non_exhaustive]
    pub enum DirectForwardingPathAvailability {
        directPathAvailable = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "E-RAB-ID", value("0..=15", extensible))]
    pub struct ERABID(pub Integer);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABInformationList")]
    pub struct ERABInformationList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABInformationList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABInformationListItem")]
    #[non_exhaustive]
    pub struct ERABInformationListItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "dL-Forwarding")]
        pub d_l_forwarding: Option<DLForwarding>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABInformationListItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "dL-Forwarding")]
        d_l_forwarding: [Option<DLForwarding>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABInformationListItem {
        pub fn new(
            e_rab_id: ERABID,
            d_l_forwarding: Option<DLForwarding>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                d_l_forwarding,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABItem")]
    #[non_exhaustive]
    pub struct ERABItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        cause: [Cause],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABItem {
        pub fn new(
            e_rab_id: ERABID,
            cause: Cause,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABLevelQoSParameters")]
    #[non_exhaustive]
    pub struct ERABLevelQoSParameters {
        #[rasn(identifier = "qCI")]
        pub q_ci: QCI,
        #[rasn(identifier = "allocationRetentionPriority")]
        pub allocation_retention_priority: AllocationAndRetentionPriority,
        #[rasn(identifier = "gbrQosInformation")]
        pub gbr_qos_information: Option<GBRQosInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABLevelQoSParameters {
        #[rasn(identifier = "qCI")]
        q_ci: [QCI],
        #[rasn(identifier = "allocationRetentionPriority")]
        allocation_retention_priority: [AllocationAndRetentionPriority],
        #[rasn(identifier = "gbrQosInformation")]
        gbr_qos_information: [Option<GBRQosInformation>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABLevelQoSParameters {
        pub fn new(
            q_ci: QCI,
            allocation_retention_priority: AllocationAndRetentionPriority,
            gbr_qos_information: Option<GBRQosInformation>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                q_ci,
                allocation_retention_priority,
                gbr_qos_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABList")]
    pub struct ERABList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABSecurityResultItem")]
    #[non_exhaustive]
    pub struct ERABSecurityResultItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "securityResult")]
        pub security_result: SecurityResult,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABSecurityResultItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "securityResult")]
        security_result: [SecurityResult],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABSecurityResultItem {
        pub fn new(
            e_rab_id: ERABID,
            security_result: SecurityResult,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                security_result,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABSecurityResultList")]
    pub struct ERABSecurityResultList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABSecurityResultList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABUsageReportItem")]
    #[non_exhaustive]
    pub struct ERABUsageReportItem {
        #[rasn(size("4"), identifier = "startTimestamp")]
        pub start_timestamp: OctetString,
        #[rasn(size("4"), identifier = "endTimestamp")]
        pub end_timestamp: OctetString,
        #[rasn(value("0..=18446744073709551615"), identifier = "usageCountUL")]
        pub usage_count_ul: u64,
        #[rasn(value("0..=18446744073709551615"), identifier = "usageCountDL")]
        pub usage_count_dl: u64,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABUsageReportItem {
        #[rasn(size("4"), identifier = "startTimestamp")]
        start_timestamp: [OctetString],
        #[rasn(size("4"), identifier = "endTimestamp")]
        end_timestamp: [OctetString],
        #[rasn(value("0..=18446744073709551615"), identifier = "usageCountUL")]
        usage_count_ul: [u64],
        #[rasn(value("0..=18446744073709551615"), identifier = "usageCountDL")]
        usage_count_dl: [u64],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABUsageReportItem {
        pub fn new(
            start_timestamp: OctetString,
            end_timestamp: OctetString,
            usage_count_ul: u64,
            usage_count_dl: u64,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                start_timestamp,
                end_timestamp,
                usage_count_ul,
                usage_count_dl,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=2"), identifier = "E-RABUsageReportList")]
    pub struct ERABUsageReportList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABUsageReportList, 1, 2 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "E-UTRAN-Trace-ID")]
    pub struct EUTRANTraceID(pub FixedOctetString<8usize>);
    #[doc = " E"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=262143", extensible))]
    pub struct EARFCN(pub Integer);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "ECGI-List")]
    pub struct ECGI_List(pub SequenceOf<EUTRANCGI>);
    crate::per::sequence_of! { ECGI_List, 1, 256 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ECGIList(pub SequenceOf<EUTRANCGI>);
    crate::per::sequence_of! { ECGIList, 1, 65535 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct ECGIListForRestart(pub SequenceOf<EUTRANCGI>);
    crate::per::sequence_of! { ECGIListForRestart, 1, 256 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "EDT-Session")]
    #[non_exhaustive]
    pub enum EDTSession {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EN-DCSONConfigurationTransfer")]
    #[non_exhaustive]
    pub struct ENDCSONConfigurationTransfer {
        pub transfertype: ENDCSONTransferType,
        #[rasn(identifier = "sONInformation")]
        pub s_oninformation: SONInformation,
        #[rasn(identifier = "x2TNLConfigInfo")]
        pub x2_tnlconfig_info: Option<X2TNLConfigurationInfo>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ENDCSONConfigurationTransfer {
        transfertype: [ENDCSONTransferType],
        #[rasn(identifier = "sONInformation")]
        s_oninformation: [SONInformation],
        #[rasn(identifier = "x2TNLConfigInfo")]
        x2_tnlconfig_info: [Option<X2TNLConfigurationInfo>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ENDCSONConfigurationTransfer {
        pub fn new(
            transfertype: ENDCSONTransferType,
            s_oninformation: SONInformation,
            x2_tnlconfig_info: Option<X2TNLConfigurationInfo>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                transfertype,
                s_oninformation,
                x2_tnlconfig_info,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "EN-DCSONTransferType")]
    #[non_exhaustive]
    pub enum ENDCSONTransferType {
        request(ENDCTransferTypeRequest),
        reply(ENDCTransferTypeReply),
    }
    impl From<ENDCTransferTypeRequest> for ENDCSONTransferType {
        fn from(value: ENDCTransferTypeRequest) -> Self {
            Self::request(value)
        }
    }
    impl From<ENDCTransferTypeReply> for ENDCSONTransferType {
        fn from(value: ENDCTransferTypeReply) -> Self {
            Self::reply(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EN-DCSONeNBIdentification")]
    #[non_exhaustive]
    pub struct ENDCSONeNBIdentification {
        #[rasn(identifier = "globaleNBID")]
        pub globale_nbid: GlobalENBID,
        #[rasn(identifier = "selectedTAI")]
        pub selected_tai: TAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ENDCSONeNBIdentification {
        #[rasn(identifier = "globaleNBID")]
        globale_nbid: [GlobalENBID],
        #[rasn(identifier = "selectedTAI")]
        selected_tai: [TAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ENDCSONeNBIdentification {
        pub fn new(
            globale_nbid: GlobalENBID,
            selected_tai: TAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                globale_nbid,
                selected_tai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EN-DCSONengNBIdentification")]
    #[non_exhaustive]
    pub struct ENDCSONengNBIdentification {
        #[rasn(identifier = "globalengNBID")]
        pub globaleng_nbid: GlobalEnGNBID,
        #[rasn(identifier = "selectedTAI")]
        pub selected_tai: TAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ENDCSONengNBIdentification {
        #[rasn(identifier = "globalengNBID")]
        globaleng_nbid: [GlobalEnGNBID],
        #[rasn(identifier = "selectedTAI")]
        selected_tai: [TAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ENDCSONengNBIdentification {
        pub fn new(
            globaleng_nbid: GlobalEnGNBID,
            selected_tai: TAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                globaleng_nbid,
                selected_tai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EN-DCTransferTypeReply")]
    #[non_exhaustive]
    pub struct ENDCTransferTypeReply {
        #[rasn(identifier = "sourceengNB")]
        pub sourceeng_nb: ENDCSONengNBIdentification,
        #[rasn(identifier = "targeteNB")]
        pub targete_nb: ENDCSONeNBIdentification,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ENDCTransferTypeReply {
        #[rasn(identifier = "sourceengNB")]
        sourceeng_nb: [ENDCSONengNBIdentification],
        #[rasn(identifier = "targeteNB")]
        targete_nb: [ENDCSONeNBIdentification],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ENDCTransferTypeReply {
        pub fn new(
            sourceeng_nb: ENDCSONengNBIdentification,
            targete_nb: ENDCSONeNBIdentification,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                sourceeng_nb,
                targete_nb,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EN-DCTransferTypeRequest")]
    #[non_exhaustive]
    pub struct ENDCTransferTypeRequest {
        #[rasn(identifier = "sourceeNB")]
        pub sourcee_nb: ENDCSONeNBIdentification,
        #[rasn(identifier = "targetengNB")]
        pub targeteng_nb: ENDCSONengNBIdentification,
        #[rasn(identifier = "targeteNB")]
        pub targete_nb: Option<ENDCSONeNBIdentification>,
        #[rasn(identifier = "associatedTAI")]
        pub associated_tai: Option<TAI>,
        #[rasn(identifier = "broadcast5GSTAI")]
        pub broadcast5_gstai: Option<FiveGSTAI>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ENDCTransferTypeRequest {
        #[rasn(identifier = "sourceeNB")]
        sourcee_nb: [ENDCSONeNBIdentification],
        #[rasn(identifier = "targetengNB")]
        targeteng_nb: [ENDCSONengNBIdentification],
        #[rasn(identifier = "targeteNB")]
        targete_nb: [Option<ENDCSONeNBIdentification>],
        #[rasn(identifier = "associatedTAI")]
        associated_tai: [Option<TAI>],
        #[rasn(identifier = "broadcast5GSTAI")]
        broadcast5_gstai: [Option<FiveGSTAI>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ENDCTransferTypeRequest {
        pub fn new(
            sourcee_nb: ENDCSONeNBIdentification,
            targeteng_nb: ENDCSONengNBIdentification,
            targete_nb: Option<ENDCSONeNBIdentification>,
            associated_tai: Option<TAI>,
            broadcast5_gstai: Option<FiveGSTAI>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                sourcee_nb,
                targeteng_nb,
                targete_nb,
                associated_tai,
                broadcast5_gstai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "ENB-EarlyStatusTransfer-TransparentContainer"
    )]
    #[non_exhaustive]
    pub struct ENBEarlyStatusTransferTransparentContainer {
        #[rasn(identifier = "bearers-SubjectToEarlyStatusTransferList")]
        pub bearers_subject_to_early_status_transfer_list: BearersSubjectToEarlyStatusTransferList,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ENBEarlyStatusTransferTransparentContainer {
        #[rasn(identifier = "bearers-SubjectToEarlyStatusTransferList")]
        bearers_subject_to_early_status_transfer_list: [BearersSubjectToEarlyStatusTransferList],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ENBEarlyStatusTransferTransparentContainer {
        pub fn new(
            bearers_subject_to_early_status_transfer_list: BearersSubjectToEarlyStatusTransferList,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                bearers_subject_to_early_status_transfer_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "ENB-ID")]
    #[non_exhaustive]
    pub enum ENBID {
        #[rasn(size("20"), identifier = "macroENB-ID")]
        macroENB_ID(BitString),
        #[rasn(size("28"), identifier = "homeENB-ID")]
        homeENB_ID(BitString),
        #[rasn(extension_addition, size("18"), identifier = "short-macroENB-ID")]
        short_macroENB_ID(BitString),
        #[rasn(extension_addition, size("21"), identifier = "long-macroENB-ID")]
        long_macroENB_ID(BitString),
    }
    impl rasn::types::DecodeChoice for ENBID {
        fn from_tag<D: Decoder>(decoder: &mut D, tag: Tag) -> Result<Self, D::Error> {
            if tag == Tag::new(Class::Context, 0) {
                const SIZE: Constraints = rasn::constraints!(rasn::size_constraint!(20));
                if decoder.codec() != rasn::Codec::Aper {
                    return BitString::decode_with_tag_and_constraints(decoder, tag, SIZE)
                        .map(Self::macroENB_ID);
                }
                const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
                const REST: Constraints = rasn::constraints!(rasn::size_constraint!(12));
                let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
                let mut bits = BitString::from_element(first);
                bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
                return Ok(Self::macroENB_ID(bits));
            }
            if tag == Tag::new(Class::Context, 1) {
                const SIZE: Constraints = rasn::constraints!(rasn::size_constraint!(28));
                if decoder.codec() != rasn::Codec::Aper {
                    return BitString::decode_with_tag_and_constraints(decoder, tag, SIZE)
                        .map(Self::homeENB_ID);
                }
                const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
                const REST: Constraints = rasn::constraints!(rasn::size_constraint!(20));
                let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
                let mut bits = BitString::from_element(first);
                bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
                return Ok(Self::homeENB_ID(bits));
            }
            if tag == Tag::new(Class::Context, 2) {
                const SIZE: Constraints = rasn::constraints!(rasn::size_constraint!(18));
                if decoder.codec() != rasn::Codec::Aper {
                    return BitString::decode_with_tag_and_constraints(decoder, tag, SIZE)
                        .map(Self::short_macroENB_ID);
                }
                const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
                const REST: Constraints = rasn::constraints!(rasn::size_constraint!(10));
                let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
                let mut bits = BitString::from_element(first);
                bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
                return Ok(Self::short_macroENB_ID(bits));
            }
            if tag == Tag::new(Class::Context, 3) {
                const SIZE: Constraints = rasn::constraints!(rasn::size_constraint!(21));
                if decoder.codec() != rasn::Codec::Aper {
                    return BitString::decode_with_tag_and_constraints(decoder, tag, SIZE)
                        .map(Self::long_macroENB_ID);
                }
                const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
                const REST: Constraints = rasn::constraints!(rasn::size_constraint!(13));
                let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
                let mut bits = BitString::from_element(first);
                bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
                return Ok(Self::long_macroENB_ID(bits));
            }
            Err(rasn::de::Error::no_valid_choice("ENBID", decoder.codec()))
        }
    }
    impl Decode for ENBID {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            _: Constraints,
        ) -> Result<Self, D::Error> {
            decoder.decode_explicit_prefix(tag)
        }
        fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
            decoder.decode_choice(Self::CONSTRAINTS)
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ENB-StatusTransfer-TransparentContainer")]
    #[non_exhaustive]
    pub struct ENBStatusTransferTransparentContainer {
        #[rasn(identifier = "bearers-SubjectToStatusTransferList")]
        pub bearers_subject_to_status_transfer_list: BearersSubjectToStatusTransferList,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ENBStatusTransferTransparentContainer {
        #[rasn(identifier = "bearers-SubjectToStatusTransferList")]
        bearers_subject_to_status_transfer_list: [BearersSubjectToStatusTransferList],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ENBStatusTransferTransparentContainer {
        pub fn new(
            bearers_subject_to_status_transfer_list: BearersSubjectToStatusTransferList,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                bearers_subject_to_status_transfer_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "ENB-UE-S1AP-ID", value("0..=16777215"))]
    pub struct ENBUES1APID(pub u32);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=2"))]
    pub struct ENBIndirectX2TransportLayerAddresses(pub SequenceOf<TransportLayerAddress>);
    crate::per::sequence_of! { ENBIndirectX2TransportLayerAddresses, 1, 2 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ENBX2ExtTLA {
        #[rasn(identifier = "iPsecTLA")]
        pub i_psec_tla: Option<TransportLayerAddress>,
        #[rasn(identifier = "gTPTLAa")]
        pub g_tptlaa: Option<ENBX2GTPTLAs>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ENBX2ExtTLA {
        #[rasn(identifier = "iPsecTLA")]
        i_psec_tla: [Option<TransportLayerAddress>],
        #[rasn(identifier = "gTPTLAa")]
        g_tptlaa: [Option<ENBX2GTPTLAs>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ENBX2ExtTLA {
        pub fn new(
            i_psec_tla: Option<TransportLayerAddress>,
            g_tptlaa: Option<ENBX2GTPTLAs>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                i_psec_tla,
                g_tptlaa,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ENBX2ExtTLAs(pub SequenceOf<ENBX2ExtTLA>);
    crate::per::sequence_of! { ENBX2ExtTLAs, 1, 16 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ENBX2GTPTLAs(pub SequenceOf<TransportLayerAddress>);
    crate::per::sequence_of! { ENBX2GTPTLAs, 1, 16 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=2"))]
    pub struct ENBX2TLAs(pub SequenceOf<TransportLayerAddress>);
    crate::per::sequence_of! { ENBX2TLAs, 1, 2 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=150", extensible))]
    pub struct ENBname(pub PrintableString);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=15"))]
    pub struct EPLMNs(pub SequenceOf<PLMNidentity>);
    crate::per::sequence_of! { EPLMNs, 1, 15 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EUTRAN-CGI")]
    #[non_exhaustive]
    pub struct EUTRANCGI {
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(identifier = "cell-ID")]
        pub cell_id: CellIdentity,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { EUTRANCGI {
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(identifier = "cell-ID")]
        cell_id: [CellIdentity],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl EUTRANCGI {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            cell_id: CellIdentity,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                cell_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=2047"))]
    pub struct EUTRANRoundTripDelayEstimationInfo(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct EmergencyAreaID(pub FixedOctetString<3usize>);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"), identifier = "EmergencyAreaID-Broadcast")]
    pub struct EmergencyAreaIDBroadcast(pub SequenceOf<EmergencyAreaIDBroadcastItem>);
    crate::per::sequence_of! { EmergencyAreaIDBroadcast, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EmergencyAreaID-Broadcast-Item")]
    #[non_exhaustive]
    pub struct EmergencyAreaIDBroadcastItem {
        #[rasn(identifier = "emergencyAreaID")]
        pub emergency_area_id: EmergencyAreaID,
        #[rasn(identifier = "completedCellinEAI")]
        pub completed_cellin_eai: CompletedCellinEAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { EmergencyAreaIDBroadcastItem {
        #[rasn(identifier = "emergencyAreaID")]
        emergency_area_id: [EmergencyAreaID],
        #[rasn(identifier = "completedCellinEAI")]
        completed_cellin_eai: [CompletedCellinEAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl EmergencyAreaIDBroadcastItem {
        pub fn new(
            emergency_area_id: EmergencyAreaID,
            completed_cellin_eai: CompletedCellinEAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                emergency_area_id,
                completed_cellin_eai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"), identifier = "EmergencyAreaID-Cancelled")]
    pub struct EmergencyAreaIDCancelled(pub SequenceOf<EmergencyAreaIDCancelledItem>);
    crate::per::sequence_of! { EmergencyAreaIDCancelled, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "EmergencyAreaID-Cancelled-Item")]
    #[non_exhaustive]
    pub struct EmergencyAreaIDCancelledItem {
        #[rasn(identifier = "emergencyAreaID")]
        pub emergency_area_id: EmergencyAreaID,
        #[rasn(identifier = "cancelledCellinEAI")]
        pub cancelled_cellin_eai: CancelledCellinEAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { EmergencyAreaIDCancelledItem {
        #[rasn(identifier = "emergencyAreaID")]
        emergency_area_id: [EmergencyAreaID],
        #[rasn(identifier = "cancelledCellinEAI")]
        cancelled_cellin_eai: [CancelledCellinEAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl EmergencyAreaIDCancelledItem {
        pub fn new(
            emergency_area_id: EmergencyAreaID,
            cancelled_cellin_eai: CancelledCellinEAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                emergency_area_id,
                cancelled_cellin_eai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct EmergencyAreaIDList(pub SequenceOf<EmergencyAreaID>);
    crate::per::sequence_of! { EmergencyAreaIDList, 1, 65535 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct EmergencyAreaIDListForRestart(pub SequenceOf<EmergencyAreaID>);
    crate::per::sequence_of! { EmergencyAreaIDListForRestart, 1, 256 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EmergencyIndicator {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "En-gNB-ID", size("22..=32", extensible))]
    pub struct EnGNBID(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("16", extensible))]
    pub struct EncryptionAlgorithms(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EndIndication {
        #[rasn(identifier = "no-further-data")]
        no_further_data = 0,
        #[rasn(identifier = "further-data-exists")]
        further_data_exists = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EnhancedCoverageRestricted {
        restricted = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Ethernet-Type")]
    #[non_exhaustive]
    pub enum EthernetType {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct EventL1LoggedMDTConfig {
        #[rasn(identifier = "l1Threshold")]
        pub l1_threshold: MeasurementThresholdL1LoggedMDT,
        pub hysteresis: Hysteresis,
        #[rasn(identifier = "timeToTrigger")]
        pub time_to_trigger: TimeToTrigger,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { EventL1LoggedMDTConfig {
        #[rasn(identifier = "l1Threshold")]
        l1_threshold: [MeasurementThresholdL1LoggedMDT],
        hysteresis: [Hysteresis],
        #[rasn(identifier = "timeToTrigger")]
        time_to_trigger: [TimeToTrigger],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl EventL1LoggedMDTConfig {
        pub fn new(
            l1_threshold: MeasurementThresholdL1LoggedMDT,
            hysteresis: Hysteresis,
            time_to_trigger: TimeToTrigger,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                l1_threshold,
                hysteresis,
                time_to_trigger,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EventTriggerOutOfCoverage {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum EventTrigger {
        outOfCoverage(EventTriggerOutOfCoverage),
        eventL1LoggedMDTConfig(EventL1LoggedMDTConfig),
        #[rasn(identifier = "choice-Extensions")]
        choice_Extensions(ProtocolIEField),
    }
    impl From<EventTriggerOutOfCoverage> for EventTrigger {
        fn from(value: EventTriggerOutOfCoverage) -> Self {
            Self::outOfCoverage(value)
        }
    }
    impl From<EventL1LoggedMDTConfig> for EventTrigger {
        fn from(value: EventL1LoggedMDTConfig) -> Self {
            Self::eventL1LoggedMDTConfig(value)
        }
    }
    impl From<ProtocolIEField> for EventTrigger {
        fn from(value: ProtocolIEField) -> Self {
            Self::choice_Extensions(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EventType {
        direct = 0,
        #[rasn(identifier = "change-of-serve-cell")]
        change_of_serve_cell = 1,
        #[rasn(identifier = "stop-change-of-serve-cell")]
        stop_change_of_serve_cell = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=181", extensible))]
    pub struct ExpectedActivityPeriod(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ExpectedHOInterval {
        sec15 = 0,
        sec30 = 1,
        sec60 = 2,
        sec90 = 3,
        sec120 = 4,
        sec180 = 5,
        #[rasn(identifier = "long-time")]
        long_time = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=181", extensible))]
    pub struct ExpectedIdlePeriod(pub Integer);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ExpectedUEActivityBehaviour {
        #[rasn(identifier = "expectedActivityPeriod")]
        pub expected_activity_period: Option<ExpectedActivityPeriod>,
        #[rasn(identifier = "expectedIdlePeriod")]
        pub expected_idle_period: Option<ExpectedIdlePeriod>,
        #[rasn(identifier = "sourceofUEActivityBehaviourInformation")]
        pub sourceof_ueactivity_behaviour_information:
            Option<SourceOfUEActivityBehaviourInformation>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ExpectedUEActivityBehaviour {
        #[rasn(identifier = "expectedActivityPeriod")]
        expected_activity_period: [Option<ExpectedActivityPeriod>],
        #[rasn(identifier = "expectedIdlePeriod")]
        expected_idle_period: [Option<ExpectedIdlePeriod>],
        #[rasn(identifier = "sourceofUEActivityBehaviourInformation")]
        sourceof_ueactivity_behaviour_information: [Option<SourceOfUEActivityBehaviourInformation>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ExpectedUEActivityBehaviour {
        pub fn new(
            expected_activity_period: Option<ExpectedActivityPeriod>,
            expected_idle_period: Option<ExpectedIdlePeriod>,
            sourceof_ueactivity_behaviour_information: Option<
                SourceOfUEActivityBehaviourInformation,
            >,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                expected_activity_period,
                expected_idle_period,
                sourceof_ueactivity_behaviour_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ExpectedUEBehaviour {
        #[rasn(identifier = "expectedActivity")]
        pub expected_activity: Option<ExpectedUEActivityBehaviour>,
        #[rasn(identifier = "expectedHOInterval")]
        pub expected_hointerval: Option<ExpectedHOInterval>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ExpectedUEBehaviour {
        #[rasn(identifier = "expectedActivity")]
        expected_activity: [Option<ExpectedUEActivityBehaviour>],
        #[rasn(identifier = "expectedHOInterval")]
        expected_hointerval: [Option<ExpectedHOInterval>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ExpectedUEBehaviour {
        pub fn new(
            expected_activity: Option<ExpectedUEActivityBehaviour>,
            expected_hointerval: Option<ExpectedHOInterval>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                expected_activity,
                expected_hointerval,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Extended-UEIdentityIndexValue")]
    pub struct ExtendedUEIdentityIndexValue(pub FixedBitString<14usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("10000000001..=4000000000000", extensible))]
    pub struct ExtendedBitRate(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "ExtendedRNC-ID", value("4096..=65535"))]
    pub struct ExtendedRNCID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("4096..=131071"))]
    pub struct ExtendedRepetitionPeriod(pub u32);
    #[doc = " F"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct FiveGSTAC(pub FixedOctetString<3usize>);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct FiveGSTAI {
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(identifier = "fiveGSTAC")]
        pub five_gstac: FiveGSTAC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { FiveGSTAI {
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(identifier = "fiveGSTAC")]
        five_gstac: [FiveGSTAC],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl FiveGSTAI {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            five_gstac: FiveGSTAC,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                five_gstac,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255", extensible))]
    pub struct FiveQI(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ForbiddenInterRATs {
        all = 0,
        geran = 1,
        utran = 2,
        cdma2000 = 3,
        #[rasn(extension_addition)]
        geranandutran = 4,
        #[rasn(extension_addition)]
        cdma2000andutran = 5,
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4096"))]
    pub struct ForbiddenLACs(pub SequenceOf<LAC>);
    crate::per::sequence_of! { ForbiddenLACs, 1, 4096 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ForbiddenLAs(pub SequenceOf<ForbiddenLAsItem>);
    crate::per::sequence_of! { ForbiddenLAs, 1, 16 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ForbiddenLAs-Item")]
    #[non_exhaustive]
    pub struct ForbiddenLAsItem {
        #[rasn(identifier = "pLMN-Identity")]
        pub p_lmn_identity: PLMNidentity,
        #[rasn(identifier = "forbiddenLACs")]
        pub forbidden_lacs: ForbiddenLACs,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ForbiddenLAsItem {
        #[rasn(identifier = "pLMN-Identity")]
        p_lmn_identity: [PLMNidentity],
        #[rasn(identifier = "forbiddenLACs")]
        forbidden_lacs: [ForbiddenLACs],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ForbiddenLAsItem {
        pub fn new(
            p_lmn_identity: PLMNidentity,
            forbidden_lacs: ForbiddenLACs,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmn_identity,
                forbidden_lacs,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4096"))]
    pub struct ForbiddenTACs(pub SequenceOf<TAC>);
    crate::per::sequence_of! { ForbiddenTACs, 1, 4096 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ForbiddenTAs(pub SequenceOf<ForbiddenTAsItem>);
    crate::per::sequence_of! { ForbiddenTAs, 1, 16 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ForbiddenTAs-Item")]
    #[non_exhaustive]
    pub struct ForbiddenTAsItem {
        #[rasn(identifier = "pLMN-Identity")]
        pub p_lmn_identity: PLMNidentity,
        #[rasn(identifier = "forbiddenTACs")]
        pub forbidden_tacs: ForbiddenTACs,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ForbiddenTAsItem {
        #[rasn(identifier = "pLMN-Identity")]
        p_lmn_identity: [PLMNidentity],
        #[rasn(identifier = "forbiddenTACs")]
        forbidden_tacs: [ForbiddenTACs],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ForbiddenTAsItem {
        pub fn new(
            p_lmn_identity: PLMNidentity,
            forbidden_tacs: ForbiddenTACs,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmn_identity,
                forbidden_tacs,
                i_e_extensions,
            }
        }
    }
    #[doc = " G"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GBR-QosInformation")]
    #[non_exhaustive]
    pub struct GBRQosInformation {
        #[rasn(identifier = "e-RAB-MaximumBitrateDL")]
        pub e_rab_maximum_bitrate_dl: BitRate,
        #[rasn(identifier = "e-RAB-MaximumBitrateUL")]
        pub e_rab_maximum_bitrate_ul: BitRate,
        #[rasn(identifier = "e-RAB-GuaranteedBitrateDL")]
        pub e_rab_guaranteed_bitrate_dl: BitRate,
        #[rasn(identifier = "e-RAB-GuaranteedBitrateUL")]
        pub e_rab_guaranteed_bitrate_ul: BitRate,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { GBRQosInformation {
        #[rasn(identifier = "e-RAB-MaximumBitrateDL")]
        e_rab_maximum_bitrate_dl: [BitRate],
        #[rasn(identifier = "e-RAB-MaximumBitrateUL")]
        e_rab_maximum_bitrate_ul: [BitRate],
        #[rasn(identifier = "e-RAB-GuaranteedBitrateDL")]
        e_rab_guaranteed_bitrate_dl: [BitRate],
        #[rasn(identifier = "e-RAB-GuaranteedBitrateUL")]
        e_rab_guaranteed_bitrate_ul: [BitRate],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl GBRQosInformation {
        pub fn new(
            e_rab_maximum_bitrate_dl: BitRate,
            e_rab_maximum_bitrate_ul: BitRate,
            e_rab_guaranteed_bitrate_dl: BitRate,
            e_rab_guaranteed_bitrate_ul: BitRate,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_maximum_bitrate_dl,
                e_rab_maximum_bitrate_ul,
                e_rab_guaranteed_bitrate_dl,
                e_rab_guaranteed_bitrate_ul,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "GERAN-Cell-ID")]
    #[non_exhaustive]
    pub struct GERANCellID {
        #[rasn(identifier = "lAI")]
        pub l_ai: LAI,
        #[rasn(identifier = "rAC")]
        pub r_ac: RAC,
        #[rasn(identifier = "cI")]
        pub c_i: CI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { GERANCellID {
        #[rasn(identifier = "lAI")]
        l_ai: [LAI],
        #[rasn(identifier = "rAC")]
        r_ac: [RAC],
        #[rasn(identifier = "cI")]
        c_i: [CI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl GERANCellID {
        pub fn new(
            l_ai: LAI,
            r_ac: RAC,
            c_i: CI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                l_ai,
                r_ac,
                c_i,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GNB {
        #[rasn(identifier = "global-gNB-ID")]
        pub global_g_nb_id: GlobalGNBID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { GNB {
        #[rasn(identifier = "global-gNB-ID")]
        global_g_nb_id: [GlobalGNBID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl GNB {
        pub fn new(
            global_g_nb_id: GlobalGNBID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                global_g_nb_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GNB-ID", size("22..=32"))]
    pub struct GNBID(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "GNB-Identity")]
    #[non_exhaustive]
    pub enum GNBIdentity {
        #[rasn(identifier = "gNB-ID")]
        gNB_ID(GNBID),
    }
    impl From<GNBID> for GNBIdentity {
        fn from(value: GNBID) -> Self {
            Self::gNB_ID(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "GTP-TEID")]
    pub struct GTPTEID(pub FixedOctetString<4usize>);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GUMMEI {
        #[rasn(identifier = "pLMN-Identity")]
        pub p_lmn_identity: PLMNidentity,
        #[rasn(identifier = "mME-Group-ID")]
        pub m_me_group_id: MMEGroupID,
        #[rasn(identifier = "mME-Code")]
        pub m_me_code: MMECode,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { GUMMEI {
        #[rasn(identifier = "pLMN-Identity")]
        p_lmn_identity: [PLMNidentity],
        #[rasn(identifier = "mME-Group-ID")]
        m_me_group_id: [MMEGroupID],
        #[rasn(identifier = "mME-Code")]
        m_me_code: [MMECode],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl GUMMEI {
        pub fn new(
            p_lmn_identity: PLMNidentity,
            m_me_group_id: MMEGroupID,
            m_me_code: MMECode,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmn_identity,
                m_me_group_id,
                m_me_code,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct GUMMEIList(pub SequenceOf<GUMMEI>);
    crate::per::sequence_of! { GUMMEIList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum GUMMEIType {
        native = 0,
        mapped = 1,
        #[rasn(extension_addition)]
        mappedFrom5G = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum GWContextReleaseIndication {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Global-ENB-ID")]
    #[non_exhaustive]
    pub struct GlobalENBID {
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(identifier = "eNB-ID")]
        pub e_nb_id: ENBID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { GlobalENBID {
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(identifier = "eNB-ID")]
        e_nb_id: [ENBID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl GlobalENBID {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            e_nb_id: ENBID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                e_nb_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Global-GNB-ID")]
    #[non_exhaustive]
    pub struct GlobalGNBID {
        #[rasn(identifier = "pLMN-Identity")]
        pub p_lmn_identity: PLMNidentity,
        #[rasn(identifier = "gNB-ID")]
        pub g_nb_id: GNBIdentity,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { GlobalGNBID {
        #[rasn(identifier = "pLMN-Identity")]
        p_lmn_identity: [PLMNidentity],
        #[rasn(identifier = "gNB-ID")]
        g_nb_id: [GNBIdentity],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl GlobalGNBID {
        pub fn new(
            p_lmn_identity: PLMNidentity,
            g_nb_id: GNBIdentity,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmn_identity,
                g_nb_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "Global-RAN-NODE-ID")]
    #[non_exhaustive]
    pub enum GlobalRANNODEID {
        gNB(GNB),
        #[rasn(identifier = "ng-eNB")]
        ng_eNB(NGENB),
    }
    impl From<GNB> for GlobalRANNODEID {
        fn from(value: GNB) -> Self {
            Self::gNB(value)
        }
    }
    impl From<NGENB> for GlobalRANNODEID {
        fn from(value: NGENB) -> Self {
            Self::ng_eNB(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Global-en-gNB-ID")]
    #[non_exhaustive]
    pub struct GlobalEnGNBID {
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(identifier = "en-gNB-ID")]
        pub en_g_nb_id: EnGNBID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { GlobalEnGNBID {
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(identifier = "en-gNB-ID")]
        en_g_nb_id: [EnGNBID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl GlobalEnGNBID {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            en_g_nb_id: EnGNBID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                en_g_nb_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1048575"))]
    pub struct HFN(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=131071"))]
    pub struct HFNModified(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "HFNforPDCP-SNlength18", value("0..=16383"))]
    pub struct HFNforPDCPSNlength18(pub u16);
    #[doc = " H"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum HandoverFlag {
        handoverPreparation = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverRestrictionList {
        #[rasn(identifier = "servingPLMN")]
        pub serving_plmn: PLMNidentity,
        #[rasn(identifier = "equivalentPLMNs")]
        pub equivalent_plmns: Option<EPLMNs>,
        #[rasn(identifier = "forbiddenTAs")]
        pub forbidden_tas: Option<ForbiddenTAs>,
        #[rasn(identifier = "forbiddenLAs")]
        pub forbidden_las: Option<ForbiddenLAs>,
        #[rasn(identifier = "forbiddenInterRATs")]
        pub forbidden_inter_rats: Option<ForbiddenInterRATs>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { HandoverRestrictionList {
        #[rasn(identifier = "servingPLMN")]
        serving_plmn: [PLMNidentity],
        #[rasn(identifier = "equivalentPLMNs")]
        equivalent_plmns: [Option<EPLMNs>],
        #[rasn(identifier = "forbiddenTAs")]
        forbidden_tas: [Option<ForbiddenTAs>],
        #[rasn(identifier = "forbiddenLAs")]
        forbidden_las: [Option<ForbiddenLAs>],
        #[rasn(identifier = "forbiddenInterRATs")]
        forbidden_inter_rats: [Option<ForbiddenInterRATs>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl HandoverRestrictionList {
        pub fn new(
            serving_plmn: PLMNidentity,
            equivalent_plmns: Option<EPLMNs>,
            forbidden_tas: Option<ForbiddenTAs>,
            forbidden_las: Option<ForbiddenLAs>,
            forbidden_inter_rats: Option<ForbiddenInterRATs>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                serving_plmn,
                equivalent_plmns,
                forbidden_tas,
                forbidden_las,
                forbidden_inter_rats,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum HandoverType {
        intralte = 0,
        ltetoutran = 1,
        ltetogeran = 2,
        utrantolte = 3,
        gerantolte = 4,
        #[rasn(extension_addition, identifier = "eps-to-5gs")]
        eps_to_5gs = 5,
        #[rasn(extension_addition, identifier = "fivegs-to-eps")]
        fivegs_to_eps = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=6000"))]
    pub struct HandoverWindowDuration(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1048575"))]
    pub struct HandoverWindowStart(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=30"))]
    pub struct Hysteresis(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "IAB-Authorized")]
    #[non_exhaustive]
    pub enum IABAuthorized {
        authorized = 0,
        #[rasn(identifier = "not-authorized")]
        not_authorized = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "IAB-Node-Indication")]
    #[non_exhaustive]
    pub enum IABNodeIndication {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "IAB-Supported")]
    #[non_exhaustive]
    pub enum IABSupported {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("3..=8"))]
    pub struct IMSI(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum IMSvoiceEPSfallbackfrom5G {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ImmediateMDT {
        #[rasn(identifier = "measurementsToActivate")]
        pub measurements_to_activate: MeasurementsToActivate,
        #[rasn(identifier = "m1reportingTrigger")]
        pub m1reporting_trigger: M1ReportingTrigger,
        #[rasn(identifier = "m1thresholdeventA2")]
        pub m1thresholdevent_a2: Option<M1ThresholdEventA2>,
        #[rasn(identifier = "m1periodicReporting")]
        pub m1periodic_reporting: Option<M1PeriodicReporting>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ImmediateMDT {
        #[rasn(identifier = "measurementsToActivate")]
        measurements_to_activate: [MeasurementsToActivate],
        #[rasn(identifier = "m1reportingTrigger")]
        m1reporting_trigger: [M1ReportingTrigger],
        #[rasn(identifier = "m1thresholdeventA2")]
        m1thresholdevent_a2: [Option<M1ThresholdEventA2>],
        #[rasn(identifier = "m1periodicReporting")]
        m1periodic_reporting: [Option<M1PeriodicReporting>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ImmediateMDT {
        pub fn new(
            measurements_to_activate: MeasurementsToActivate,
            m1reporting_trigger: M1ReportingTrigger,
            m1thresholdevent_a2: Option<M1ThresholdEventA2>,
            m1periodic_reporting: Option<M1PeriodicReporting>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                measurements_to_activate,
                m1reporting_trigger,
                m1thresholdevent_a2,
                m1periodic_reporting,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct InformationOnRecommendedCellsAndENBsForPaging {
        #[rasn(identifier = "recommendedCellsForPaging")]
        pub recommended_cells_for_paging: RecommendedCellsForPaging,
        #[rasn(identifier = "recommendENBsForPaging")]
        pub recommend_enbs_for_paging: RecommendedENBsForPaging,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { InformationOnRecommendedCellsAndENBsForPaging {
        #[rasn(identifier = "recommendedCellsForPaging")]
        recommended_cells_for_paging: [RecommendedCellsForPaging],
        #[rasn(identifier = "recommendENBsForPaging")]
        recommend_enbs_for_paging: [RecommendedENBsForPaging],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl InformationOnRecommendedCellsAndENBsForPaging {
        pub fn new(
            recommended_cells_for_paging: RecommendedCellsForPaging,
            recommend_enbs_for_paging: RecommendedENBsForPaging,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                recommended_cells_for_paging,
                recommend_enbs_for_paging,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("16", extensible))]
    pub struct IntegrityProtectionAlgorithms(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum IntegrityProtectionIndication {
        required = 0,
        preferred = 1,
        #[rasn(identifier = "not-needed")]
        not_needed = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum IntegrityProtectionResult {
        performed = 0,
        #[rasn(identifier = "not-performed")]
        not_performed = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=16", extensible))]
    pub struct IntendedNumberOfPagingAttempts(pub Integer);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum InterSystemMeasurementItemSubcarrierSpacingSSB {
        kHz15 = 0,
        kHz30 = 1,
        kHz60 = 2,
        kHz120 = 3,
        kHz240 = 4,
        #[rasn(extension_addition)]
        kHz480 = 5,
        #[rasn(extension_addition)]
        kHz960 = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct InterSystemMeasurementItem {
        #[rasn(value("1..=1024"), identifier = "freqBandIndicatorNR")]
        pub freq_band_indicator_nr: u16,
        #[rasn(value("0..=3279165"), identifier = "sSBfrequencies")]
        pub s_sbfrequencies: u32,
        #[rasn(identifier = "subcarrierSpacingSSB")]
        pub subcarrier_spacing_ssb: InterSystemMeasurementItemSubcarrierSpacingSSB,
        #[rasn(value("1..=16"), identifier = "maxRSIndexCellQual")]
        pub max_rsindex_cell_qual: Option<u8>,
        #[rasn(identifier = "sMTC")]
        pub s_mtc: Option<OctetString>,
        #[rasn(identifier = "threshRS-Index-r15")]
        pub thresh_rs_index_r15: Option<OctetString>,
        #[rasn(identifier = "sSBToMeasure")]
        pub s_sbto_measure: Option<OctetString>,
        #[rasn(identifier = "sSRSSIMeasurement")]
        pub s_srssimeasurement: Option<OctetString>,
        #[rasn(identifier = "quantityConfigNR-R15")]
        pub quantity_config_nr_r15: Option<OctetString>,
        #[rasn(identifier = "excludedCellsToAddModList")]
        pub excluded_cells_to_add_mod_list: Option<OctetString>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    impl InterSystemMeasurementItem {
        pub fn new(
            freq_band_indicator_nr: u16,
            s_sbfrequencies: u32,
            subcarrier_spacing_ssb: InterSystemMeasurementItemSubcarrierSpacingSSB,
            max_rsindex_cell_qual: Option<u8>,
            s_mtc: Option<OctetString>,
            thresh_rs_index_r15: Option<OctetString>,
            s_sbto_measure: Option<OctetString>,
            s_srssimeasurement: Option<OctetString>,
            quantity_config_nr_r15: Option<OctetString>,
            excluded_cells_to_add_mod_list: Option<OctetString>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                freq_band_indicator_nr,
                s_sbfrequencies,
                subcarrier_spacing_ssb,
                max_rsindex_cell_qual,
                s_mtc,
                thresh_rs_index_r15,
                s_sbto_measure,
                s_srssimeasurement,
                quantity_config_nr_r15,
                excluded_cells_to_add_mod_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"))]
    pub struct InterSystemMeasurementList(pub SequenceOf<InterSystemMeasurementItem>);
    crate::per::sequence_of! { InterSystemMeasurementList, 1, 64 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct InterSystemMeasurementParameters {
        #[rasn(value("1..=100"), identifier = "measurementDuration")]
        pub measurement_duration: u8,
        #[rasn(identifier = "interSystemMeasurementList")]
        pub inter_system_measurement_list: Option<InterSystemMeasurementList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { InterSystemMeasurementParameters {
        #[rasn(value("1..=100"), identifier = "measurementDuration")]
        measurement_duration: [u8],
        #[rasn(identifier = "interSystemMeasurementList")]
        inter_system_measurement_list: [Option<InterSystemMeasurementList>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl InterSystemMeasurementParameters {
        pub fn new(
            measurement_duration: u8,
            inter_system_measurement_list: Option<InterSystemMeasurementList>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                measurement_duration,
                inter_system_measurement_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct InterfacesToTrace(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct IntersystemMeasurementConfiguration {
        #[rasn(value("0..=127"), identifier = "rSRP")]
        pub r_srp: Option<u8>,
        #[rasn(value("0..=127"), identifier = "rSRQ")]
        pub r_srq: Option<u8>,
        #[rasn(value("0..=127"), identifier = "sINR")]
        pub s_inr: Option<u8>,
        #[rasn(identifier = "interSystemMeasurementParameters")]
        pub inter_system_measurement_parameters: InterSystemMeasurementParameters,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { IntersystemMeasurementConfiguration {
        #[rasn(value("0..=127"), identifier = "rSRP")]
        r_srp: [Option<u8>],
        #[rasn(value("0..=127"), identifier = "rSRQ")]
        r_srq: [Option<u8>],
        #[rasn(value("0..=127"), identifier = "sINR")]
        s_inr: [Option<u8>],
        #[rasn(identifier = "interSystemMeasurementParameters")]
        inter_system_measurement_parameters: [InterSystemMeasurementParameters],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl IntersystemMeasurementConfiguration {
        pub fn new(
            r_srp: Option<u8>,
            r_srq: Option<u8>,
            s_inr: Option<u8>,
            inter_system_measurement_parameters: InterSystemMeasurementParameters,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                r_srp,
                r_srq,
                s_inr,
                inter_system_measurement_parameters,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct IntersystemSONConfigurationTransfer(pub OctetString);
    #[doc = " J"]
    #[doc = " K"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum KillAllWarningMessages {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "L3-Information")]
    pub struct L3Information(pub OctetString);
    #[doc = " L"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LAC(pub FixedOctetString<2usize>);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LAI {
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(identifier = "lAC")]
        pub l_ac: LAC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { LAI {
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(identifier = "lAC")]
        l_ac: [LAC],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl LAI {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            l_ac: LAC,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                l_ac,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LHN-ID", size("32..=256"))]
    pub struct LHNID(pub OctetString);
    #[doc = " This is a dummy IE used only as a reference to the actual definition in relevant specification."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LPPa-PDU")]
    pub struct LPPaPDU(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "LTE-M-Indication")]
    #[non_exhaustive]
    pub enum LTEMIndication {
        #[rasn(identifier = "lte-m")]
        lte_m = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "LTE-NTN-TAI-Information")]
    #[non_exhaustive]
    pub struct LTENTNTAIInformation {
        #[rasn(identifier = "servingPLMN")]
        pub serving_plmn: PLMNidentity,
        #[rasn(identifier = "tACList-In-LTE-NTN")]
        pub t_aclist_in_lte_ntn: TACListInLTENTN,
        #[rasn(identifier = "uE-Location-Derived-TAC")]
        pub u_e_location_derived_tac: Option<TAC>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { LTENTNTAIInformation {
        #[rasn(identifier = "servingPLMN")]
        serving_plmn: [PLMNidentity],
        #[rasn(identifier = "tACList-In-LTE-NTN")]
        t_aclist_in_lte_ntn: [TACListInLTENTN],
        #[rasn(identifier = "uE-Location-Derived-TAC")]
        u_e_location_derived_tac: [Option<TAC>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl LTENTNTAIInformation {
        pub fn new(
            serving_plmn: PLMNidentity,
            t_aclist_in_lte_ntn: TACListInLTENTN,
            u_e_location_derived_tac: Option<TAC>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                serving_plmn,
                t_aclist_in_lte_ntn,
                u_e_location_derived_tac,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "LastVisitedCell-Item")]
    #[non_exhaustive]
    pub enum LastVisitedCellItem {
        #[rasn(identifier = "e-UTRAN-Cell")]
        e_UTRAN_Cell(LastVisitedEUTRANCellInformation),
        #[rasn(identifier = "uTRAN-Cell")]
        uTRAN_Cell(LastVisitedUTRANCellInformation),
        #[rasn(identifier = "gERAN-Cell")]
        gERAN_Cell(LastVisitedGERANCellInformation),
        #[rasn(extension_addition, identifier = "nG-RAN-Cell")]
        nG_RAN_Cell(LastVisitedNGRANCellInformation),
    }
    impl From<LastVisitedEUTRANCellInformation> for LastVisitedCellItem {
        fn from(value: LastVisitedEUTRANCellInformation) -> Self {
            Self::e_UTRAN_Cell(value)
        }
    }
    impl From<LastVisitedUTRANCellInformation> for LastVisitedCellItem {
        fn from(value: LastVisitedUTRANCellInformation) -> Self {
            Self::uTRAN_Cell(value)
        }
    }
    impl From<LastVisitedGERANCellInformation> for LastVisitedCellItem {
        fn from(value: LastVisitedGERANCellInformation) -> Self {
            Self::gERAN_Cell(value)
        }
    }
    impl From<LastVisitedNGRANCellInformation> for LastVisitedCellItem {
        fn from(value: LastVisitedNGRANCellInformation) -> Self {
            Self::nG_RAN_Cell(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LastVisitedEUTRANCellInformation {
        #[rasn(identifier = "global-Cell-ID")]
        pub global_cell_id: EUTRANCGI,
        #[rasn(identifier = "cellType")]
        pub cell_type: CellType,
        #[rasn(identifier = "time-UE-StayedInCell")]
        pub time_ue_stayed_in_cell: TimeUEStayedInCell,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { LastVisitedEUTRANCellInformation {
        #[rasn(identifier = "global-Cell-ID")]
        global_cell_id: [EUTRANCGI],
        #[rasn(identifier = "cellType")]
        cell_type: [CellType],
        #[rasn(identifier = "time-UE-StayedInCell")]
        time_ue_stayed_in_cell: [TimeUEStayedInCell],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl LastVisitedEUTRANCellInformation {
        pub fn new(
            global_cell_id: EUTRANCGI,
            cell_type: CellType,
            time_ue_stayed_in_cell: TimeUEStayedInCell,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                global_cell_id,
                cell_type,
                time_ue_stayed_in_cell,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LastVisitedGERANCellInformation {
        undefined(()),
    }
    impl From<()> for LastVisitedGERANCellInformation {
        fn from(value: ()) -> Self {
            Self::undefined(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LastVisitedNGRANCellInformation(pub OctetString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LastVisitedPSCellInformation {
        #[rasn(identifier = "pSCellID")]
        pub p_scell_id: Option<PSCellInformation>,
        #[rasn(value("0..=40950"), identifier = "timeStay")]
        pub time_stay: u16,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { LastVisitedPSCellInformation {
        #[rasn(identifier = "pSCellID")]
        p_scell_id: [Option<PSCellInformation>],
        #[rasn(value("0..=40950"), identifier = "timeStay")]
        time_stay: [u16],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl LastVisitedPSCellInformation {
        pub fn new(
            p_scell_id: Option<PSCellInformation>,
            time_stay: u16,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_scell_id,
                time_stay,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct LastVisitedPSCellList(pub SequenceOf<LastVisitedPSCellInformation>);
    crate::per::sequence_of! { LastVisitedPSCellList, 1, 8 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LastVisitedUTRANCellInformation(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Links-to-log")]
    #[non_exhaustive]
    pub enum LinksToLog {
        uplink = 0,
        downlink = 1,
        #[rasn(identifier = "both-uplink-and-downlink")]
        both_uplink_and_downlink = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ListeningSubframePatternPatternPeriod {
        ms1280 = 0,
        ms2560 = 1,
        ms5120 = 2,
        ms10240 = 3,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ListeningSubframePattern {
        #[rasn(identifier = "pattern-period")]
        pub pattern_period: ListeningSubframePatternPatternPeriod,
        #[rasn(value("0..=10239", extensible), identifier = "pattern-offset")]
        pub pattern_offset: Integer,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ListeningSubframePattern {
        #[rasn(identifier = "pattern-period")]
        pattern_period: [ListeningSubframePatternPatternPeriod],
        #[rasn(value("0..=10239", extensible), identifier = "pattern-offset")]
        pattern_offset: [Integer],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ListeningSubframePattern {
        pub fn new(
            pattern_period: ListeningSubframePatternPatternPeriod,
            pattern_offset: Integer,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                pattern_period,
                pattern_offset,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LoggedMBSFNMDT {
        #[rasn(identifier = "loggingInterval")]
        pub logging_interval: LoggingInterval,
        #[rasn(identifier = "loggingDuration")]
        pub logging_duration: LoggingDuration,
        #[rasn(identifier = "mBSFN-ResultToLog")]
        pub m_bsfn_result_to_log: Option<MBSFNResultToLog>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { LoggedMBSFNMDT {
        #[rasn(identifier = "loggingInterval")]
        logging_interval: [LoggingInterval],
        #[rasn(identifier = "loggingDuration")]
        logging_duration: [LoggingDuration],
        #[rasn(identifier = "mBSFN-ResultToLog")]
        m_bsfn_result_to_log: [Option<MBSFNResultToLog>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl LoggedMBSFNMDT {
        pub fn new(
            logging_interval: LoggingInterval,
            logging_duration: LoggingDuration,
            m_bsfn_result_to_log: Option<MBSFNResultToLog>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                logging_interval,
                logging_duration,
                m_bsfn_result_to_log,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LoggedMDT {
        #[rasn(identifier = "loggingInterval")]
        pub logging_interval: LoggingInterval,
        #[rasn(identifier = "loggingDuration")]
        pub logging_duration: LoggingDuration,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { LoggedMDT {
        #[rasn(identifier = "loggingInterval")]
        logging_interval: [LoggingInterval],
        #[rasn(identifier = "loggingDuration")]
        logging_duration: [LoggingDuration],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl LoggedMDT {
        pub fn new(
            logging_interval: LoggingInterval,
            logging_duration: LoggingDuration,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                logging_interval,
                logging_duration,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LoggedMDTTrigger {
        periodical(()),
        eventTrigger(EventTrigger),
    }
    impl From<()> for LoggedMDTTrigger {
        fn from(value: ()) -> Self {
            Self::periodical(value)
        }
    }
    impl From<EventTrigger> for LoggedMDTTrigger {
        fn from(value: EventTrigger) -> Self {
            Self::eventTrigger(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum LoggingDuration {
        m10 = 0,
        m20 = 1,
        m40 = 2,
        m60 = 3,
        m90 = 4,
        m120 = 5,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum LoggingInterval {
        ms1280 = 0,
        ms2560 = 1,
        ms5120 = 2,
        ms10240 = 3,
        ms20480 = 4,
        ms30720 = 5,
        ms40960 = 6,
        ms61440 = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "M-TMSI")]
    pub struct MTMSI(pub FixedOctetString<4usize>);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M1PeriodicReporting {
        #[rasn(identifier = "reportInterval")]
        pub report_interval: ReportIntervalMDT,
        #[rasn(identifier = "reportAmount")]
        pub report_amount: ReportAmountMDT,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { M1PeriodicReporting {
        #[rasn(identifier = "reportInterval")]
        report_interval: [ReportIntervalMDT],
        #[rasn(identifier = "reportAmount")]
        report_amount: [ReportAmountMDT],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl M1PeriodicReporting {
        pub fn new(
            report_interval: ReportIntervalMDT,
            report_amount: ReportAmountMDT,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                report_interval,
                report_amount,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M1ReportingTrigger {
        periodic = 0,
        a2eventtriggered = 1,
        #[rasn(extension_addition, identifier = "a2eventtriggered-periodic")]
        a2eventtriggered_periodic = 2,
    }
    #[doc = " This is a dummy IE used only as a reference to the actual definition in relevant specification."]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M1ThresholdEventA2 {
        #[rasn(identifier = "measurementThreshold")]
        pub measurement_threshold: MeasurementThresholdA2,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { M1ThresholdEventA2 {
        #[rasn(identifier = "measurementThreshold")]
        measurement_threshold: [MeasurementThresholdA2],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl M1ThresholdEventA2 {
        pub fn new(
            measurement_threshold: MeasurementThresholdA2,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                measurement_threshold,
                i_e_extensions,
            }
        }
    }
    #[doc = " M"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M3Configuration {
        pub m3period: M3period,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { M3Configuration {
        m3period: [M3period],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl M3Configuration {
        pub fn new(m3period: M3period, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                m3period,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M3period {
        ms100 = 0,
        ms1000 = 1,
        ms10000 = 2,
        #[rasn(extension_addition)]
        ms1024 = 3,
        #[rasn(extension_addition)]
        ms1280 = 4,
        #[rasn(extension_addition)]
        ms2048 = 5,
        #[rasn(extension_addition)]
        ms2560 = 6,
        #[rasn(extension_addition)]
        ms5120 = 7,
        #[rasn(extension_addition)]
        ms10240 = 8,
        #[rasn(extension_addition)]
        min1 = 9,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M4Configuration {
        pub m4period: M4period,
        #[rasn(identifier = "m4-links-to-log")]
        pub m4_links_to_log: LinksToLog,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { M4Configuration {
        m4period: [M4period],
        #[rasn(identifier = "m4-links-to-log")]
        m4_links_to_log: [LinksToLog],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl M4Configuration {
        pub fn new(
            m4period: M4period,
            m4_links_to_log: LinksToLog,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                m4period,
                m4_links_to_log,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M4ReportAmountMDT {
        r1 = 0,
        r2 = 1,
        r4 = 2,
        r8 = 3,
        r16 = 4,
        r32 = 5,
        r64 = 6,
        infinity = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M4period {
        ms1024 = 0,
        ms2048 = 1,
        ms5120 = 2,
        ms10240 = 3,
        min1 = 4,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M5Configuration {
        pub m5period: M5period,
        #[rasn(identifier = "m5-links-to-log")]
        pub m5_links_to_log: LinksToLog,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { M5Configuration {
        m5period: [M5period],
        #[rasn(identifier = "m5-links-to-log")]
        m5_links_to_log: [LinksToLog],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl M5Configuration {
        pub fn new(
            m5period: M5period,
            m5_links_to_log: LinksToLog,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                m5period,
                m5_links_to_log,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M5ReportAmountMDT {
        r1 = 0,
        r2 = 1,
        r4 = 2,
        r8 = 3,
        r16 = 4,
        r32 = 5,
        r64 = 6,
        infinity = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M5period {
        ms1024 = 0,
        ms2048 = 1,
        ms5120 = 2,
        ms10240 = 3,
        min1 = 4,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M6Configuration {
        #[rasn(identifier = "m6report-Interval")]
        pub m6report_interval: M6reportInterval,
        #[rasn(identifier = "m6delay-threshold")]
        pub m6delay_threshold: Option<M6delayThreshold>,
        #[rasn(identifier = "m6-links-to-log")]
        pub m6_links_to_log: LinksToLog,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { M6Configuration {
        #[rasn(identifier = "m6report-Interval")]
        m6report_interval: [M6reportInterval],
        #[rasn(identifier = "m6delay-threshold")]
        m6delay_threshold: [Option<M6delayThreshold>],
        #[rasn(identifier = "m6-links-to-log")]
        m6_links_to_log: [LinksToLog],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl M6Configuration {
        pub fn new(
            m6report_interval: M6reportInterval,
            m6delay_threshold: Option<M6delayThreshold>,
            m6_links_to_log: LinksToLog,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                m6report_interval,
                m6delay_threshold,
                m6_links_to_log,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M6ReportAmountMDT {
        r1 = 0,
        r2 = 1,
        r4 = 2,
        r8 = 3,
        r16 = 4,
        r32 = 5,
        r64 = 6,
        infinity = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "M6delay-threshold")]
    #[non_exhaustive]
    pub enum M6delayThreshold {
        ms30 = 0,
        ms40 = 1,
        ms50 = 2,
        ms60 = 3,
        ms70 = 4,
        ms80 = 5,
        ms90 = 6,
        ms100 = 7,
        ms150 = 8,
        ms300 = 9,
        ms500 = 10,
        ms750 = 11,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "M6report-Interval")]
    #[non_exhaustive]
    pub enum M6reportInterval {
        ms1024 = 0,
        ms2048 = 1,
        ms5120 = 2,
        ms10240 = 3,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct M7Configuration {
        pub m7period: M7period,
        #[rasn(identifier = "m7-links-to-log")]
        pub m7_links_to_log: LinksToLog,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { M7Configuration {
        m7period: [M7period],
        #[rasn(identifier = "m7-links-to-log")]
        m7_links_to_log: [LinksToLog],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl M7Configuration {
        pub fn new(
            m7period: M7period,
            m7_links_to_log: LinksToLog,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                m7period,
                m7_links_to_log,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum M7ReportAmountMDT {
        r1 = 0,
        r2 = 1,
        r4 = 2,
        r8 = 3,
        r16 = 4,
        r32 = 5,
        r64 = 6,
        infinity = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=60", extensible))]
    pub struct M7period(pub Integer);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"), identifier = "MBSFN-ResultToLog")]
    pub struct MBSFNResultToLog(pub SequenceOf<MBSFNResultToLogInfo>);
    crate::per::sequence_of! { MBSFNResultToLog, 1, 8 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MBSFN-ResultToLogInfo")]
    #[non_exhaustive]
    pub struct MBSFNResultToLogInfo {
        #[rasn(value("0..=255"), identifier = "mBSFN-AreaId")]
        pub m_bsfn_area_id: Option<u8>,
        #[rasn(identifier = "carrierFreq")]
        pub carrier_freq: EARFCN,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { MBSFNResultToLogInfo {
        #[rasn(value("0..=255"), identifier = "mBSFN-AreaId")]
        m_bsfn_area_id: [Option<u8>],
        #[rasn(identifier = "carrierFreq")]
        carrier_freq: [EARFCN],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl MBSFNResultToLogInfo {
        pub fn new(
            m_bsfn_area_id: Option<u8>,
            carrier_freq: EARFCN,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                m_bsfn_area_id,
                carrier_freq,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "MDT-Activation")]
    #[non_exhaustive]
    pub enum MDTActivation {
        #[rasn(identifier = "immediate-MDT-only")]
        immediate_MDT_only = 0,
        #[rasn(identifier = "immediate-MDT-and-Trace")]
        immediate_MDT_and_Trace = 1,
        #[rasn(identifier = "logged-MDT-only")]
        logged_MDT_only = 2,
        #[rasn(extension_addition, identifier = "logged-MBSFN-MDT")]
        logged_MBSFN_MDT = 3,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MDT-Configuration")]
    #[non_exhaustive]
    pub struct MDTConfiguration {
        #[rasn(identifier = "mdt-Activation")]
        pub mdt_activation: MDTActivation,
        #[rasn(identifier = "areaScopeOfMDT")]
        pub area_scope_of_mdt: AreaScopeOfMDT,
        #[rasn(identifier = "mDTMode")]
        pub m_dtmode: MDTMode,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { MDTConfiguration {
        #[rasn(identifier = "mdt-Activation")]
        mdt_activation: [MDTActivation],
        #[rasn(identifier = "areaScopeOfMDT")]
        area_scope_of_mdt: [AreaScopeOfMDT],
        #[rasn(identifier = "mDTMode")]
        m_dtmode: [MDTMode],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl MDTConfiguration {
        pub fn new(
            mdt_activation: MDTActivation,
            area_scope_of_mdt: AreaScopeOfMDT,
            m_dtmode: MDTMode,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                mdt_activation,
                area_scope_of_mdt,
                m_dtmode,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "MDT-ConfigurationNR")]
    pub struct MDTConfigurationNR(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "MDT-Location-Info")]
    pub struct MDTLocationInfo(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum MDTMode {
        immediateMDT(ImmediateMDT),
        loggedMDT(LoggedMDT),
        #[rasn(extension_addition, identifier = "mDTMode-Extension")]
        mDTMode_Extension(ProtocolIEField),
    }
    impl From<ImmediateMDT> for MDTMode {
        fn from(value: ImmediateMDT) -> Self {
            Self::immediateMDT(value)
        }
    }
    impl From<LoggedMDT> for MDTMode {
        fn from(value: LoggedMDT) -> Self {
            Self::loggedMDT(value)
        }
    }
    impl From<ProtocolIEField> for MDTMode {
        fn from(value: ProtocolIEField) -> Self {
            Self::mDTMode_Extension(value)
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct MDTPLMNList(pub SequenceOf<PLMNidentity>);
    crate::per::sequence_of! { MDTPLMNList, 1, 16 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "MME-Code")]
    pub struct MMECode(pub FixedOctetString<1usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "MME-Group-ID")]
    pub struct MMEGroupID(pub FixedOctetString<2usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "MME-UE-S1AP-ID", value("0..=4294967295"))]
    pub struct MMEUES1APID(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum MMEPagingTarget {
        #[rasn(identifier = "global-ENB-ID")]
        global_ENB_ID(GlobalENBID),
        tAI(TAI),
    }
    impl From<GlobalENBID> for MMEPagingTarget {
        fn from(value: GlobalENBID) -> Self {
            Self::global_ENB_ID(value)
        }
    }
    impl From<TAI> for MMEPagingTarget {
        fn from(value: TAI) -> Self {
            Self::tAI(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MMERelaySupportIndicator {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=150", extensible))]
    pub struct MMEname(pub PrintableString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct MSClassmark2(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct MSClassmark3(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ManagementBasedMDTAllowed {
        allowed = 0,
    }
    #[doc = " I"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Masked-IMEISV")]
    pub struct MaskedIMEISV(pub FixedBitString<64usize>);
    impl Decode for MaskedIMEISV {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {
            if decoder.codec() != rasn::Codec::Aper {
                return FixedBitString::<64usize>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self);
            }
            const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
            const REST: Constraints = rasn::constraints!(rasn::size_constraint!(56));
            let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
            let mut bits = BitString::from_element(first);
            bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
            let mut value = FixedBitString::<64usize>::ZERO;
            value[..64].copy_from_bitslice(&bits);
            Ok(Self(value))
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum MeasurementThresholdA2 {
        #[rasn(identifier = "threshold-RSRP")]
        threshold_RSRP(ThresholdRSRP),
        #[rasn(identifier = "threshold-RSRQ")]
        threshold_RSRQ(ThresholdRSRQ),
    }
    impl From<ThresholdRSRP> for MeasurementThresholdA2 {
        fn from(value: ThresholdRSRP) -> Self {
            Self::threshold_RSRP(value)
        }
    }
    impl From<ThresholdRSRQ> for MeasurementThresholdA2 {
        fn from(value: ThresholdRSRQ) -> Self {
            Self::threshold_RSRQ(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum MeasurementThresholdL1LoggedMDT {
        #[rasn(identifier = "threshold-RSRP")]
        threshold_RSRP(ThresholdRSRP),
        #[rasn(identifier = "threshold-RSRQ")]
        threshold_RSRQ(ThresholdRSRQ),
        #[rasn(identifier = "choice-Extensions")]
        choice_Extensions(ProtocolIEField),
    }
    impl From<ThresholdRSRP> for MeasurementThresholdL1LoggedMDT {
        fn from(value: ThresholdRSRP) -> Self {
            Self::threshold_RSRP(value)
        }
    }
    impl From<ThresholdRSRQ> for MeasurementThresholdL1LoggedMDT {
        fn from(value: ThresholdRSRQ) -> Self {
            Self::threshold_RSRQ(value)
        }
    }
    impl From<ProtocolIEField> for MeasurementThresholdL1LoggedMDT {
        fn from(value: ProtocolIEField) -> Self {
            Self::choice_Extensions(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct MeasurementsToActivate(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct MessageIdentifier(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct MobilityInformation(pub FixedBitString<32usize>);
    impl Decode for MobilityInformation {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {
            if decoder.codec() != rasn::Codec::Aper {
                return FixedBitString::<32usize>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self);
            }
            const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
            const REST: Constraints = rasn::constraints!(rasn::size_constraint!(24));
            let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
            let mut bits = BitString::from_element(first);
            bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
            let mut value = FixedBitString::<32usize>::ZERO;
            value[..32].copy_from_bitslice(&bits);
            Ok(Self(value))
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MutingAvailabilityIndication {
        available = 0,
        unavailable = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum MutingPatternInformationMutingPatternPeriod {
        ms0 = 0,
        ms1280 = 1,
        ms2560 = 2,
        ms5120 = 3,
        ms10240 = 4,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MutingPatternInformation {
        #[rasn(identifier = "muting-pattern-period")]
        pub muting_pattern_period: MutingPatternInformationMutingPatternPeriod,
        #[rasn(value("0..=10239", extensible), identifier = "muting-pattern-offset")]
        pub muting_pattern_offset: Option<Integer>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { MutingPatternInformation {
        #[rasn(identifier = "muting-pattern-period")]
        muting_pattern_period: [MutingPatternInformationMutingPatternPeriod],
        #[rasn(value("0..=10239", extensible), identifier = "muting-pattern-offset")]
        muting_pattern_offset: [Option<Integer>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl MutingPatternInformation {
        pub fn new(
            muting_pattern_period: MutingPatternInformationMutingPatternPeriod,
            muting_pattern_offset: Option<Integer>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                muting_pattern_period,
                muting_pattern_offset,
                i_e_extensions,
            }
        }
    }
    #[doc = " N"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "NAS-PDU")]
    pub struct NASPDU(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "NASSecurityParametersfromE-UTRAN")]
    pub struct NASSecurityParametersfromEUTRAN(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "NASSecurityParameterstoE-UTRAN")]
    pub struct NASSecurityParameterstoEUTRAN(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "NB-IoT-DefaultPagingDRX")]
    #[non_exhaustive]
    pub enum NBIoTDefaultPagingDRX {
        v128 = 0,
        v256 = 1,
        v512 = 2,
        v1024 = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "NB-IoT-Paging-eDRX-Cycle")]
    #[non_exhaustive]
    pub enum NBIoTPagingEDRXCycle {
        hf2 = 0,
        hf4 = 1,
        hf6 = 2,
        hf8 = 3,
        hf10 = 4,
        hf12 = 5,
        hf14 = 6,
        hf16 = 7,
        hf32 = 8,
        hf64 = 9,
        hf128 = 10,
        hf256 = 11,
        hf512 = 12,
        hf1024 = 13,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NB-IoT-Paging-eDRXInformation")]
    #[non_exhaustive]
    pub struct NBIoTPagingEDRXInformation {
        #[rasn(identifier = "nB-IoT-paging-eDRX-Cycle")]
        pub n_b_io_t_paging_e_drx_cycle: NBIoTPagingEDRXCycle,
        #[rasn(identifier = "nB-IoT-pagingTimeWindow")]
        pub n_b_io_t_paging_time_window: Option<NBIoTPagingTimeWindow>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { NBIoTPagingEDRXInformation {
        #[rasn(identifier = "nB-IoT-paging-eDRX-Cycle")]
        n_b_io_t_paging_e_drx_cycle: [NBIoTPagingEDRXCycle],
        #[rasn(identifier = "nB-IoT-pagingTimeWindow")]
        n_b_io_t_paging_time_window: [Option<NBIoTPagingTimeWindow>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl NBIoTPagingEDRXInformation {
        pub fn new(
            n_b_io_t_paging_e_drx_cycle: NBIoTPagingEDRXCycle,
            n_b_io_t_paging_time_window: Option<NBIoTPagingTimeWindow>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                n_b_io_t_paging_e_drx_cycle,
                n_b_io_t_paging_time_window,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "NB-IoT-PagingDRX")]
    #[non_exhaustive]
    pub enum NBIoTPagingDRX {
        v32 = 0,
        v64 = 1,
        v128 = 2,
        v256 = 3,
        v512 = 4,
        v1024 = 5,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "NB-IoT-PagingTimeWindow")]
    #[non_exhaustive]
    pub enum NBIoTPagingTimeWindow {
        s1 = 0,
        s2 = 1,
        s3 = 2,
        s4 = 3,
        s5 = 4,
        s6 = 5,
        s7 = 6,
        s8 = 7,
        s9 = 8,
        s10 = 9,
        s11 = 10,
        s12 = 11,
        s13 = 12,
        s14 = 13,
        s15 = 14,
        s16 = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "NB-IoT-RLF-Report-Container")]
    pub struct NBIoTRLFReportContainer(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "NB-IoT-UEIdentityIndexValue")]
    pub struct NBIoTUEIdentityIndexValue(pub FixedBitString<12usize>);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NG-eNB")]
    #[non_exhaustive]
    pub struct NGENB {
        #[rasn(identifier = "global-ng-eNB-ID")]
        pub global_ng_e_nb_id: GlobalENBID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { NGENB {
        #[rasn(identifier = "global-ng-eNB-ID")]
        global_ng_e_nb_id: [GlobalENBID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl NGENB {
        pub fn new(
            global_ng_e_nb_id: GlobalENBID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                global_ng_e_nb_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NR-CGI")]
    #[non_exhaustive]
    pub struct NRCGI {
        #[rasn(identifier = "pLMNIdentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(identifier = "nRCellIdentity")]
        pub n_rcell_identity: NRCellIdentity,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { NRCGI {
        #[rasn(identifier = "pLMNIdentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(identifier = "nRCellIdentity")]
        n_rcell_identity: [NRCellIdentity],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl NRCGI {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            n_rcell_identity: NRCellIdentity,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                n_rcell_identity,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct NRCellIdentity(pub FixedBitString<36usize>);
    impl Decode for NRCellIdentity {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {
            if decoder.codec() != rasn::Codec::Aper {
                return FixedBitString::<36usize>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self);
            }
            const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
            const REST: Constraints = rasn::constraints!(rasn::size_constraint!(28));
            let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
            let mut bits = BitString::from_element(first);
            bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
            let mut value = FixedBitString::<36usize>::ZERO;
            value[..36].copy_from_bitslice(&bits);
            Ok(Self(value))
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct NRUESecurityCapabilities {
        #[rasn(identifier = "nRencryptionAlgorithms")]
        pub n_rencryption_algorithms: NRencryptionAlgorithms,
        #[rasn(identifier = "nRintegrityProtectionAlgorithms")]
        pub n_rintegrity_protection_algorithms: NRintegrityProtectionAlgorithms,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { NRUESecurityCapabilities {
        #[rasn(identifier = "nRencryptionAlgorithms")]
        n_rencryption_algorithms: [NRencryptionAlgorithms],
        #[rasn(identifier = "nRintegrityProtectionAlgorithms")]
        n_rintegrity_protection_algorithms: [NRintegrityProtectionAlgorithms],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl NRUESecurityCapabilities {
        pub fn new(
            n_rencryption_algorithms: NRencryptionAlgorithms,
            n_rintegrity_protection_algorithms: NRintegrityProtectionAlgorithms,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                n_rencryption_algorithms,
                n_rintegrity_protection_algorithms,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct NRUESidelinkAggregateMaximumBitrate {
        #[rasn(identifier = "uEaggregateMaximumBitRate")]
        pub u_eaggregate_maximum_bit_rate: BitRate,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { NRUESidelinkAggregateMaximumBitrate {
        #[rasn(identifier = "uEaggregateMaximumBitRate")]
        u_eaggregate_maximum_bit_rate: [BitRate],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl NRUESidelinkAggregateMaximumBitrate {
        pub fn new(
            u_eaggregate_maximum_bit_rate: BitRate,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                u_eaggregate_maximum_bit_rate,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct NRV2XServicesAuthorized {
        #[rasn(identifier = "vehicleUE")]
        pub vehicle_ue: Option<VehicleUE>,
        #[rasn(identifier = "pedestrianUE")]
        pub pedestrian_ue: Option<PedestrianUE>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { NRV2XServicesAuthorized {
        #[rasn(identifier = "vehicleUE")]
        vehicle_ue: [Option<VehicleUE>],
        #[rasn(identifier = "pedestrianUE")]
        pedestrian_ue: [Option<PedestrianUE>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl NRV2XServicesAuthorized {
        pub fn new(
            vehicle_ue: Option<VehicleUE>,
            pedestrian_ue: Option<PedestrianUE>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                vehicle_ue,
                pedestrian_ue,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("16", extensible))]
    pub struct NRencryptionAlgorithms(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("16", extensible))]
    pub struct NRintegrityProtectionAlgorithms(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum NRrestrictionin5GS {
        nRrestrictedin5GS = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum NRrestrictioninEPSasSecondaryRAT {
        nRrestrictedinEPSasSecondaryRAT = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum NextPagingAreaScope {
        same = 0,
        changed = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum NotifySourceeNB {
        notifySource = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct NumberOfBroadcasts(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct NumberofBroadcastRequest(pub u16);
    #[doc = " O"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "OldBSS-ToNewBSS-Information")]
    pub struct OldBSSToNewBSSInformation(pub OctetString);
    #[doc = " This is a dummy IE used only as a reference to the actual definition in relevant specification."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum OverloadAction {
        #[rasn(identifier = "reject-non-emergency-mo-dt")]
        reject_non_emergency_mo_dt = 0,
        #[rasn(identifier = "reject-rrc-cr-signalling")]
        reject_rrc_cr_signalling = 1,
        #[rasn(identifier = "permit-emergency-sessions-and-mobile-terminated-services-only")]
        permit_emergency_sessions_and_mobile_terminated_services_only = 2,
        #[rasn(
            extension_addition,
            identifier = "permit-high-priority-sessions-and-mobile-terminated-services-only"
        )]
        permit_high_priority_sessions_and_mobile_terminated_services_only = 3,
        #[rasn(extension_addition, identifier = "reject-delay-tolerant-access")]
        reject_delay_tolerant_access = 4,
        #[rasn(
            extension_addition,
            identifier = "permit-high-priority-sessions-and-exception-reporting-and-mobile-terminated-services-only"
        )]
        permit_high_priority_sessions_and_exception_reporting_and_mobile_terminated_services_only =
            5,
        #[rasn(
            extension_addition,
            identifier = "not-accept-mo-data-or-delay-tolerant-access-from-CP-CIoT"
        )]
        not_accept_mo_data_or_delay_tolerant_access_from_CP_CIoT = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum OverloadResponse {
        overloadAction(OverloadAction),
    }
    impl From<OverloadAction> for OverloadResponse {
        fn from(value: OverloadAction) -> Self {
            Self::overloadAction(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PC5FlowBitRates {
        #[rasn(identifier = "guaranteedFlowBitRate")]
        pub guaranteed_flow_bit_rate: BitRate,
        #[rasn(identifier = "maximumFlowBitRate")]
        pub maximum_flow_bit_rate: BitRate,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { PC5FlowBitRates {
        #[rasn(identifier = "guaranteedFlowBitRate")]
        guaranteed_flow_bit_rate: [BitRate],
        #[rasn(identifier = "maximumFlowBitRate")]
        maximum_flow_bit_rate: [BitRate],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl PC5FlowBitRates {
        pub fn new(
            guaranteed_flow_bit_rate: BitRate,
            maximum_flow_bit_rate: BitRate,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                guaranteed_flow_bit_rate,
                maximum_flow_bit_rate,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PC5QoSFlowItem {
        #[rasn(identifier = "pQI")]
        pub p_qi: FiveQI,
        #[rasn(identifier = "pc5FlowBitRates")]
        pub pc5_flow_bit_rates: Option<PC5FlowBitRates>,
        pub range: Option<Range>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { PC5QoSFlowItem {
        #[rasn(identifier = "pQI")]
        p_qi: [FiveQI],
        #[rasn(identifier = "pc5FlowBitRates")]
        pc5_flow_bit_rates: [Option<PC5FlowBitRates>],
        range: [Option<Range>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl PC5QoSFlowItem {
        pub fn new(
            p_qi: FiveQI,
            pc5_flow_bit_rates: Option<PC5FlowBitRates>,
            range: Option<Range>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_qi,
                pc5_flow_bit_rates,
                range,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=2048"))]
    pub struct PC5QoSFlowList(pub SequenceOf<PC5QoSFlowItem>);
    crate::per::sequence_of! { PC5QoSFlowList, 1, 2048 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PC5QoSParameters {
        #[rasn(identifier = "pc5QoSFlowList")]
        pub pc5_qo_sflow_list: PC5QoSFlowList,
        #[rasn(identifier = "pc5LinkAggregatedBitRates")]
        pub pc5_link_aggregated_bit_rates: Option<BitRate>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { PC5QoSParameters {
        #[rasn(identifier = "pc5QoSFlowList")]
        pc5_qo_sflow_list: [PC5QoSFlowList],
        #[rasn(identifier = "pc5LinkAggregatedBitRates")]
        pc5_link_aggregated_bit_rates: [Option<BitRate>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl PC5QoSParameters {
        pub fn new(
            pc5_qo_sflow_list: PC5QoSFlowList,
            pc5_link_aggregated_bit_rates: Option<BitRate>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                pc5_qo_sflow_list,
                pc5_link_aggregated_bit_rates,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "PDCP-SN", value("0..=4095"))]
    pub struct PDCPSN(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "PDCP-SNExtended", value("0..=32767"))]
    pub struct PDCPSNExtended(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "PDCP-SNlength18", value("0..=262143"))]
    pub struct PDCPSNlength18(pub u32);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PLMNAreaBasedQMC {
        #[rasn(identifier = "plmnListforQMC")]
        pub plmn_listfor_qmc: PLMNListforQMC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { PLMNAreaBasedQMC {
        #[rasn(identifier = "plmnListforQMC")]
        plmn_listfor_qmc: [PLMNListforQMC],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl PLMNAreaBasedQMC {
        pub fn new(
            plmn_listfor_qmc: PLMNListforQMC,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                plmn_listfor_qmc,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct PLMNListforQMC(pub SequenceOf<PLMNidentity>);
    crate::per::sequence_of! { PLMNListforQMC, 1, 16 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct PLMNidentity(pub TBCDSTRING);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "PS-ServiceNotAvailable")]
    #[non_exhaustive]
    pub enum PSServiceNotAvailable {
        #[rasn(identifier = "ps-service-not-available")]
        ps_service_not_available = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PSCellInformation {
        #[rasn(identifier = "nCGI")]
        pub n_cgi: NRCGI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { PSCellInformation {
        #[rasn(identifier = "nCGI")]
        n_cgi: [NRCGI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl PSCellInformation {
        pub fn new(n_cgi: NRCGI, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                n_cgi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct PWSfailedECGIList(pub SequenceOf<EUTRANCGI>);
    crate::per::sequence_of! { PWSfailedECGIList, 1, 256 }
    #[doc = " P"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Packet-LossRate", value("0..=1000"))]
    pub struct PacketLossRate(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Paging-eDRX-Cycle")]
    #[non_exhaustive]
    pub enum PagingEDRXCycle {
        hfhalf = 0,
        hf1 = 1,
        hf2 = 2,
        hf4 = 3,
        hf6 = 4,
        hf8 = 5,
        hf10 = 6,
        hf12 = 7,
        hf14 = 8,
        hf16 = 9,
        hf32 = 10,
        hf64 = 11,
        hf128 = 12,
        hf256 = 13,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Paging-eDRXInformation")]
    #[non_exhaustive]
    pub struct PagingEDRXInformation {
        #[rasn(identifier = "paging-eDRX-Cycle")]
        pub paging_e_drx_cycle: PagingEDRXCycle,
        #[rasn(identifier = "pagingTimeWindow")]
        pub paging_time_window: Option<PagingTimeWindow>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { PagingEDRXInformation {
        #[rasn(identifier = "paging-eDRX-Cycle")]
        paging_e_drx_cycle: [PagingEDRXCycle],
        #[rasn(identifier = "pagingTimeWindow")]
        paging_time_window: [Option<PagingTimeWindow>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl PagingEDRXInformation {
        pub fn new(
            paging_e_drx_cycle: PagingEDRXCycle,
            paging_time_window: Option<PagingTimeWindow>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                paging_e_drx_cycle,
                paging_time_window,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=16", extensible))]
    pub struct PagingAttemptCount(pub Integer);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PagingAttemptInformation {
        #[rasn(identifier = "pagingAttemptCount")]
        pub paging_attempt_count: PagingAttemptCount,
        #[rasn(identifier = "intendedNumberOfPagingAttempts")]
        pub intended_number_of_paging_attempts: IntendedNumberOfPagingAttempts,
        #[rasn(identifier = "nextPagingAreaScope")]
        pub next_paging_area_scope: Option<NextPagingAreaScope>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { PagingAttemptInformation {
        #[rasn(identifier = "pagingAttemptCount")]
        paging_attempt_count: [PagingAttemptCount],
        #[rasn(identifier = "intendedNumberOfPagingAttempts")]
        intended_number_of_paging_attempts: [IntendedNumberOfPagingAttempts],
        #[rasn(identifier = "nextPagingAreaScope")]
        next_paging_area_scope: [Option<NextPagingAreaScope>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl PagingAttemptInformation {
        pub fn new(
            paging_attempt_count: PagingAttemptCount,
            intended_number_of_paging_attempts: IntendedNumberOfPagingAttempts,
            next_paging_area_scope: Option<NextPagingAreaScope>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                paging_attempt_count,
                intended_number_of_paging_attempts,
                next_paging_area_scope,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PagingCause {
        voice = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PagingDRX {
        v32 = 0,
        v64 = 1,
        v128 = 2,
        v256 = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PagingPriority {
        priolevel1 = 0,
        priolevel2 = 1,
        priolevel3 = 2,
        priolevel4 = 3,
        priolevel5 = 4,
        priolevel6 = 5,
        priolevel7 = 6,
        priolevel8 = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PagingProbabilityInformation {
        p00 = 0,
        p05 = 1,
        p10 = 2,
        p15 = 3,
        p20 = 4,
        p25 = 5,
        p30 = 6,
        p35 = 7,
        p40 = 8,
        p45 = 9,
        p50 = 10,
        p55 = 11,
        p60 = 12,
        p65 = 13,
        p70 = 14,
        p75 = 15,
        p80 = 16,
        p85 = 17,
        p90 = 18,
        p95 = 19,
        p100 = 20,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PagingTimeWindow {
        s1 = 0,
        s2 = 1,
        s3 = 2,
        s4 = 3,
        s5 = 4,
        s6 = 5,
        s7 = 6,
        s8 = 7,
        s9 = 8,
        s10 = 9,
        s11 = 10,
        s12 = 11,
        s13 = 12,
        s14 = 13,
        s15 = 14,
        s16 = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PedestrianUE {
        authorized = 0,
        #[rasn(identifier = "not-authorized")]
        not_authorized = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PendingDataIndication {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Port-Number")]
    pub struct PortNumber(pub FixedOctetString<2usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Pre-emptionCapability")]
    pub enum PreEmptionCapability {
        #[rasn(identifier = "shall-not-trigger-pre-emption")]
        shall_not_trigger_pre_emption = 0,
        #[rasn(identifier = "may-trigger-pre-emption")]
        may_trigger_pre_emption = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "Pre-emptionVulnerability")]
    pub enum PreEmptionVulnerability {
        #[rasn(identifier = "not-pre-emptable")]
        not_pre_emptable = 0,
        #[rasn(identifier = "pre-emptable")]
        pre_emptable = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct PriorityLevel(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PrivacyIndicator {
        #[rasn(identifier = "immediate-MDT")]
        immediate_MDT = 0,
        #[rasn(identifier = "logged-MDT")]
        logged_MDT = 1,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ProSeAuthorized {
        #[rasn(identifier = "proSeDirectDiscovery")]
        pub pro_se_direct_discovery: Option<ProSeDirectDiscovery>,
        #[rasn(identifier = "proSeDirectCommunication")]
        pub pro_se_direct_communication: Option<ProSeDirectCommunication>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ProSeAuthorized {
        #[rasn(identifier = "proSeDirectDiscovery")]
        pro_se_direct_discovery: [Option<ProSeDirectDiscovery>],
        #[rasn(identifier = "proSeDirectCommunication")]
        pro_se_direct_communication: [Option<ProSeDirectCommunication>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ProSeAuthorized {
        pub fn new(
            pro_se_direct_discovery: Option<ProSeDirectDiscovery>,
            pro_se_direct_communication: Option<ProSeDirectCommunication>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                pro_se_direct_discovery,
                pro_se_direct_communication,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ProSeDirectCommunication {
        authorized = 0,
        #[rasn(identifier = "not-authorized")]
        not_authorized = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ProSeDirectDiscovery {
        authorized = 0,
        #[rasn(identifier = "not-authorized")]
        not_authorized = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ProSeUEtoNetworkRelaying {
        authorized = 0,
        #[rasn(identifier = "not-authorized")]
        not_authorized = 1,
    }
    #[doc = " Q"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct QCI(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct RAC(pub FixedOctetString<1usize>);
    #[doc = " R"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum RACSIndication {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "RAN-UE-NGAP-ID", value("0..=4294967295"))]
    pub struct RANUENGAPID(pub u32);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"), identifier = "RAT-Restrictions")]
    pub struct RATRestrictions(pub SequenceOf<RATRestrictionsItem>);
    crate::per::sequence_of! { RATRestrictions, 1, 16 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "RAT-RestrictionsItem")]
    #[non_exhaustive]
    pub struct RATRestrictionsItem {
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(size("8", extensible), identifier = "rAT-RestrictionInformation")]
        pub r_at_restriction_information: BitString,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { RATRestrictionsItem {
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(size("8", extensible), identifier = "rAT-RestrictionInformation")]
        r_at_restriction_information: [BitString],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl RATRestrictionsItem {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            r_at_restriction_information: BitString,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                r_at_restriction_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "RAT-Type")]
    #[non_exhaustive]
    pub enum RATType {
        nbiot = 0,
        #[rasn(extension_addition, identifier = "nbiot-leo")]
        nbiot_leo = 1,
        #[rasn(extension_addition, identifier = "nbiot-meo")]
        nbiot_meo = 2,
        #[rasn(extension_addition, identifier = "nbiot-geo")]
        nbiot_geo = 3,
        #[rasn(extension_addition, identifier = "nbiot-othersat")]
        nbiot_othersat = 4,
        #[rasn(extension_addition, identifier = "eutran-leo")]
        eutran_leo = 5,
        #[rasn(extension_addition, identifier = "eutran-meo")]
        eutran_meo = 6,
        #[rasn(extension_addition, identifier = "eutran-geo")]
        eutran_geo = 7,
        #[rasn(extension_addition, identifier = "eutran-othersat")]
        eutran_othersat = 8,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct RIMInformation(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum RIMRoutingAddress {
        #[rasn(identifier = "gERAN-Cell-ID")]
        gERAN_Cell_ID(GERANCellID),
        #[rasn(extension_addition, identifier = "targetRNC-ID")]
        targetRNC_ID(TargetRNCID),
        #[rasn(extension_addition, size("16"), identifier = "eHRPD-Sector-ID")]
        eHRPD_Sector_ID(OctetString),
    }
    impl From<GERANCellID> for RIMRoutingAddress {
        fn from(value: GERANCellID) -> Self {
            Self::gERAN_Cell_ID(value)
        }
    }
    impl From<TargetRNCID> for RIMRoutingAddress {
        fn from(value: TargetRNCID) -> Self {
            Self::targetRNC_ID(value)
        }
    }
    impl From<OctetString> for RIMRoutingAddress {
        fn from(value: OctetString) -> Self {
            Self::eHRPD_Sector_ID(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RIMTransfer {
        #[rasn(identifier = "rIMInformation")]
        pub r_iminformation: RIMInformation,
        #[rasn(identifier = "rIMRoutingAddress")]
        pub r_imrouting_address: Option<RIMRoutingAddress>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { RIMTransfer {
        #[rasn(identifier = "rIMInformation")]
        r_iminformation: [RIMInformation],
        #[rasn(identifier = "rIMRoutingAddress")]
        r_imrouting_address: [Option<RIMRoutingAddress>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl RIMTransfer {
        pub fn new(
            r_iminformation: RIMInformation,
            r_imrouting_address: Option<RIMRoutingAddress>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                r_iminformation,
                r_imrouting_address,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RLFReportInformation {
        #[rasn(identifier = "uE-RLF-Report-Container")]
        pub u_e_rlf_report_container: UERLFReportContainer,
        #[rasn(identifier = "uE-RLF-Report-Container-for-extended-bands")]
        pub u_e_rlf_report_container_for_extended_bands:
            Option<UERLFReportContainerForExtendedBands>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { RLFReportInformation {
        #[rasn(identifier = "uE-RLF-Report-Container")]
        u_e_rlf_report_container: [UERLFReportContainer],
        #[rasn(identifier = "uE-RLF-Report-Container-for-extended-bands")]
        u_e_rlf_report_container_for_extended_bands: [Option<UERLFReportContainerForExtendedBands>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl RLFReportInformation {
        pub fn new(
            u_e_rlf_report_container: UERLFReportContainer,
            u_e_rlf_report_container_for_extended_bands: Option<
                UERLFReportContainerForExtendedBands,
            >,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                u_e_rlf_report_container,
                u_e_rlf_report_container_for_extended_bands,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "RNC-ID", value("0..=4095"))]
    pub struct RNCID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "RRC-Container")]
    pub struct RRCContainer(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "RRC-Establishment-Cause")]
    #[non_exhaustive]
    pub enum RRCEstablishmentCause {
        emergency = 0,
        highPriorityAccess = 1,
        #[rasn(identifier = "mt-Access")]
        mt_Access = 2,
        #[rasn(identifier = "mo-Signalling")]
        mo_Signalling = 3,
        #[rasn(identifier = "mo-Data")]
        mo_Data = 4,
        #[rasn(extension_addition, identifier = "delay-TolerantAccess")]
        delay_TolerantAccess = 5,
        #[rasn(extension_addition, identifier = "mo-VoiceCall")]
        mo_VoiceCall = 6,
        #[rasn(extension_addition, identifier = "mo-ExceptionData")]
        mo_ExceptionData = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum Range {
        m50 = 0,
        m80 = 1,
        m180 = 2,
        m200 = 3,
        m350 = 4,
        m400 = 5,
        m500 = 6,
        m700 = 7,
        m1000 = 8,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16384"))]
    pub struct ReceiveStatusOfULPDCPSDUsExtended(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "ReceiveStatusOfULPDCPSDUsPDCP-SNlength18",
        size("1..=131072")
    )]
    pub struct ReceiveStatusOfULPDCPSDUsPDCPSNlength18(pub BitString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct ReceiveStatusofULPDCPSDUs(pub FixedBitString<4096usize>);
    impl Decode for ReceiveStatusofULPDCPSDUs {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {
            if decoder.codec() != rasn::Codec::Aper {
                return FixedBitString::<4096usize>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self);
            }
            const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
            const REST: Constraints = rasn::constraints!(rasn::size_constraint!(4088));
            let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
            let mut bits = BitString::from_element(first);
            bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
            let mut value = FixedBitString::<4096usize>::ZERO;
            value[..4096].copy_from_bitslice(&bits);
            Ok(Self(value))
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RecommendedCellItem {
        #[rasn(identifier = "eUTRAN-CGI")]
        pub e_utran_cgi: EUTRANCGI,
        #[rasn(value("0..=4095"), identifier = "timeStayedInCell")]
        pub time_stayed_in_cell: Option<u16>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { RecommendedCellItem {
        #[rasn(identifier = "eUTRAN-CGI")]
        e_utran_cgi: [EUTRANCGI],
        #[rasn(value("0..=4095"), identifier = "timeStayedInCell")]
        time_stayed_in_cell: [Option<u16>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl RecommendedCellItem {
        pub fn new(
            e_utran_cgi: EUTRANCGI,
            time_stayed_in_cell: Option<u16>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_utran_cgi,
                time_stayed_in_cell,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct RecommendedCellList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { RecommendedCellList, 1, 16 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RecommendedCellsForPaging {
        #[rasn(identifier = "recommendedCellList")]
        pub recommended_cell_list: RecommendedCellList,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { RecommendedCellsForPaging {
        #[rasn(identifier = "recommendedCellList")]
        recommended_cell_list: [RecommendedCellList],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl RecommendedCellsForPaging {
        pub fn new(
            recommended_cell_list: RecommendedCellList,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                recommended_cell_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RecommendedENBItem {
        #[rasn(identifier = "mMEPagingTarget")]
        pub m_mepaging_target: MMEPagingTarget,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { RecommendedENBItem {
        #[rasn(identifier = "mMEPagingTarget")]
        m_mepaging_target: [MMEPagingTarget],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl RecommendedENBItem {
        pub fn new(
            m_mepaging_target: MMEPagingTarget,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                m_mepaging_target,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct RecommendedENBList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { RecommendedENBList, 1, 16 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RecommendedENBsForPaging {
        #[rasn(identifier = "recommendedENBList")]
        pub recommended_enblist: RecommendedENBList,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { RecommendedENBsForPaging {
        #[rasn(identifier = "recommendedENBList")]
        recommended_enblist: [RecommendedENBList],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl RecommendedENBsForPaging {
        pub fn new(
            recommended_enblist: RecommendedENBList,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                recommended_enblist,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RelativeMMECapacity(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "RelayNode-Indicator")]
    #[non_exhaustive]
    pub enum RelayNodeIndicator {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4095"))]
    pub struct RepetitionPeriod(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum ReportAmountMDT {
        r1 = 0,
        r2 = 1,
        r4 = 2,
        r8 = 3,
        r16 = 4,
        r32 = 5,
        r64 = 6,
        rinfinity = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ReportArea {
        ecgi = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum ReportIntervalMDT {
        ms120 = 0,
        ms240 = 1,
        ms480 = 2,
        ms640 = 3,
        ms1024 = 4,
        ms2048 = 5,
        ms5120 = 6,
        ms10240 = 7,
        min1 = 8,
        min6 = 9,
        min12 = 10,
        min30 = 11,
        min60 = 12,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RequestType {
        #[rasn(identifier = "eventType")]
        pub event_type: EventType,
        #[rasn(identifier = "reportArea")]
        pub report_area: ReportArea,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { RequestType {
        #[rasn(identifier = "eventType")]
        event_type: [EventType],
        #[rasn(identifier = "reportArea")]
        report_area: [ReportArea],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl RequestType {
        pub fn new(
            event_type: EventType,
            report_area: ReportArea,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                event_type,
                report_area,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum RequestTypeAdditionalInfo {
        includePSCell = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RequestedTNLInfo {
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { RequestedTNLInfo {
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl RequestedTNLInfo {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Routing-ID", value("0..=255"))]
    pub struct RoutingID(pub u8);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "S-TMSI")]
    #[non_exhaustive]
    pub struct STMSI {
        #[rasn(identifier = "mMEC")]
        pub m_mec: MMECode,
        #[rasn(identifier = "m-TMSI")]
        pub m_tmsi: MTMSI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { STMSI {
        #[rasn(identifier = "mMEC")]
        m_mec: [MMECode],
        #[rasn(identifier = "m-TMSI")]
        m_tmsi: [MTMSI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl STMSI {
        pub fn new(
            m_mec: MMECode,
            m_tmsi: MTMSI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                m_mec,
                m_tmsi,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SONConfigurationTransfer {
        #[rasn(identifier = "targeteNB-ID")]
        pub targete_nb_id: TargeteNBID,
        #[rasn(identifier = "sourceeNB-ID")]
        pub sourcee_nb_id: SourceeNBID,
        #[rasn(identifier = "sONInformation")]
        pub s_oninformation: SONInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SONConfigurationTransfer {
        #[rasn(identifier = "targeteNB-ID")]
        targete_nb_id: [TargeteNBID],
        #[rasn(identifier = "sourceeNB-ID")]
        sourcee_nb_id: [SourceeNBID],
        #[rasn(identifier = "sONInformation")]
        s_oninformation: [SONInformation],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SONConfigurationTransfer {
        pub fn new(
            targete_nb_id: TargeteNBID,
            sourcee_nb_id: SourceeNBID,
            s_oninformation: SONInformation,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                targete_nb_id,
                sourcee_nb_id,
                s_oninformation,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum SONInformation {
        sONInformationRequest(SONInformationRequest),
        sONInformationReply(SONInformationReply),
        #[rasn(extension_addition, identifier = "sONInformation-Extension")]
        sONInformation_Extension(ProtocolIEField),
    }
    impl From<SONInformationRequest> for SONInformation {
        fn from(value: SONInformationRequest) -> Self {
            Self::sONInformationRequest(value)
        }
    }
    impl From<SONInformationReply> for SONInformation {
        fn from(value: SONInformationReply) -> Self {
            Self::sONInformationReply(value)
        }
    }
    impl From<ProtocolIEField> for SONInformation {
        fn from(value: ProtocolIEField) -> Self {
            Self::sONInformation_Extension(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SONInformationReply {
        #[rasn(identifier = "x2TNLConfigurationInfo")]
        pub x2_tnlconfiguration_info: Option<X2TNLConfigurationInfo>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SONInformationReply {
        #[rasn(identifier = "x2TNLConfigurationInfo")]
        x2_tnlconfiguration_info: [Option<X2TNLConfigurationInfo>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SONInformationReply {
        pub fn new(
            x2_tnlconfiguration_info: Option<X2TNLConfigurationInfo>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                x2_tnlconfiguration_info,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum SONInformationReport {
        rLFReportInformation(RLFReportInformation),
    }
    impl From<RLFReportInformation> for SONInformationReport {
        fn from(value: RLFReportInformation) -> Self {
            Self::rLFReportInformation(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SONInformationRequest {
        #[rasn(identifier = "x2TNL-Configuration-Info")]
        x2TNL_Configuration_Info = 0,
        #[rasn(extension_addition, identifier = "time-Synchronisation-Info")]
        time_Synchronisation_Info = 1,
        #[rasn(extension_addition, identifier = "activate-Muting")]
        activate_Muting = 2,
        #[rasn(extension_addition, identifier = "deactivate-Muting")]
        deactivate_Muting = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SRVCCHOIndication {
        pSandCS = 0,
        cSonly = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SRVCCOperationNotPossible {
        notPossible = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SRVCCOperationPossible {
        possible = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ScheduledCommunicationTime {
        #[rasn(size("7"), identifier = "dayofWeek")]
        pub dayof_week: Option<BitString>,
        #[rasn(value("0..=86399", extensible), identifier = "timeofDayStart")]
        pub timeof_day_start: Option<Integer>,
        #[rasn(value("0..=86399", extensible), identifier = "timeofDayEnd")]
        pub timeof_day_end: Option<Integer>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ScheduledCommunicationTime {
        #[rasn(size("7"), identifier = "dayofWeek")]
        dayof_week: [Option<BitString>],
        #[rasn(value("0..=86399", extensible), identifier = "timeofDayStart")]
        timeof_day_start: [Option<Integer>],
        #[rasn(value("0..=86399", extensible), identifier = "timeofDayEnd")]
        timeof_day_end: [Option<Integer>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ScheduledCommunicationTime {
        pub fn new(
            dayof_week: Option<BitString>,
            timeof_day_start: Option<Integer>,
            timeof_day_end: Option<Integer>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                dayof_week,
                timeof_day_start,
                timeof_day_end,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SecondaryRATDataUsageReportItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "secondaryRATType")]
        pub secondary_rattype: SecondaryRATType,
        #[rasn(identifier = "e-RABUsageReportList")]
        pub e_rabusage_report_list: ERABUsageReportList,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SecondaryRATDataUsageReportItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "secondaryRATType")]
        secondary_rattype: [SecondaryRATType],
        #[rasn(identifier = "e-RABUsageReportList")]
        e_rabusage_report_list: [ERABUsageReportList],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SecondaryRATDataUsageReportItem {
        pub fn new(
            e_rab_id: ERABID,
            secondary_rattype: SecondaryRATType,
            e_rabusage_report_list: ERABUsageReportList,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                secondary_rattype,
                e_rabusage_report_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct SecondaryRATDataUsageReportList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { SecondaryRATDataUsageReportList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SecondaryRATDataUsageRequest {
        requested = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SecondaryRATType {
        nR = 0,
        #[rasn(extension_addition)]
        unlicensed = 1,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SecurityContext {
        #[rasn(value("0..=7"), identifier = "nextHopChainingCount")]
        pub next_hop_chaining_count: u8,
        #[rasn(identifier = "nextHopParameter")]
        pub next_hop_parameter: SecurityKey,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SecurityContext {
        #[rasn(value("0..=7"), identifier = "nextHopChainingCount")]
        next_hop_chaining_count: [u8],
        #[rasn(identifier = "nextHopParameter")]
        next_hop_parameter: [SecurityKey],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SecurityContext {
        pub fn new(
            next_hop_chaining_count: u8,
            next_hop_parameter: SecurityKey,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                next_hop_chaining_count,
                next_hop_parameter,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SecurityIndication {
        #[rasn(identifier = "integrityProtectionIndication")]
        pub integrity_protection_indication: IntegrityProtectionIndication,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SecurityIndication {
        #[rasn(identifier = "integrityProtectionIndication")]
        integrity_protection_indication: [IntegrityProtectionIndication],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SecurityIndication {
        pub fn new(
            integrity_protection_indication: IntegrityProtectionIndication,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                integrity_protection_indication,
                i_e_extensions,
            }
        }
    }
    #[doc = " S"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct SecurityKey(pub FixedBitString<256usize>);
    impl Decode for SecurityKey {
        fn decode_with_tag_and_constraints<D: Decoder>(
            decoder: &mut D,
            tag: Tag,
            constraints: Constraints,
        ) -> Result<Self, D::Error> {
            if decoder.codec() != rasn::Codec::Aper {
                return FixedBitString::<256usize>::decode_with_tag_and_constraints(
                    decoder,
                    tag,
                    constraints,
                )
                .map(Self);
            }
            const OCTET: Constraints = rasn::constraints!(rasn::value_constraint!(0, 255));
            const REST: Constraints = rasn::constraints!(rasn::size_constraint!(248));
            let first = decoder.decode_integer::<u8>(Tag::INTEGER, OCTET)?;
            let mut bits = BitString::from_element(first);
            bits.extend_from_bitslice(&decoder.decode_bit_string(Tag::BIT_STRING, REST)?);
            let mut value = FixedBitString::<256usize>::ZERO;
            value[..256].copy_from_bitslice(&bits);
            Ok(Self(value))
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SecurityResult {
        #[rasn(identifier = "integrityProtectionResult")]
        pub integrity_protection_result: IntegrityProtectionResult,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SecurityResult {
        #[rasn(identifier = "integrityProtectionResult")]
        integrity_protection_result: [IntegrityProtectionResult],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SecurityResult {
        pub fn new(
            integrity_protection_result: IntegrityProtectionResult,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                integrity_protection_result,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SensorMeasConfig {
        setup = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SensorMeasConfigNameItem {
        #[rasn(identifier = "sensorNameConfig")]
        pub sensor_name_config: SensorNameConfig,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SensorMeasConfigNameItem {
        #[rasn(identifier = "sensorNameConfig")]
        sensor_name_config: [SensorNameConfig],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SensorMeasConfigNameItem {
        pub fn new(
            sensor_name_config: SensorNameConfig,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                sensor_name_config,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=3"))]
    pub struct SensorMeasConfigNameList(pub SequenceOf<SensorMeasConfigNameItem>);
    crate::per::sequence_of! { SensorMeasConfigNameList, 1, 3 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SensorMeasurementConfiguration {
        #[rasn(identifier = "sensorMeasConfig")]
        pub sensor_meas_config: SensorMeasConfig,
        #[rasn(identifier = "sensorMeasConfigNameList")]
        pub sensor_meas_config_name_list: Option<SensorMeasConfigNameList>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SensorMeasurementConfiguration {
        #[rasn(identifier = "sensorMeasConfig")]
        sensor_meas_config: [SensorMeasConfig],
        #[rasn(identifier = "sensorMeasConfigNameList")]
        sensor_meas_config_name_list: [Option<SensorMeasConfigNameList>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SensorMeasurementConfiguration {
        pub fn new(
            sensor_meas_config: SensorMeasConfig,
            sensor_meas_config_name_list: Option<SensorMeasConfigNameList>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                sensor_meas_config,
                sensor_meas_config_name_list,
                i_e_extensions,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SensorNameConfigUncompensatedBarometricConfig {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum SensorNameConfig {
        uncompensatedBarometricConfig(SensorNameConfigUncompensatedBarometricConfig),
        #[rasn(identifier = "choice-Extensions")]
        choice_Extensions(ProtocolIEField),
    }
    impl From<SensorNameConfigUncompensatedBarometricConfig> for SensorNameConfig {
        fn from(value: SensorNameConfigUncompensatedBarometricConfig) -> Self {
            Self::uncompensatedBarometricConfig(value)
        }
    }
    impl From<ProtocolIEField> for SensorNameConfig {
        fn from(value: ProtocolIEField) -> Self {
            Self::choice_Extensions(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct SerialNumber(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=32"))]
    pub struct ServedDCNs(pub SequenceOf<ServedDCNsItem>);
    crate::per::sequence_of! { ServedDCNs, 0, 32 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ServedDCNsItem {
        #[rasn(identifier = "dCN-ID")]
        pub d_cn_id: DCNID,
        #[rasn(identifier = "relativeDCNCapacity")]
        pub relative_dcncapacity: RelativeMMECapacity,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ServedDCNsItem {
        #[rasn(identifier = "dCN-ID")]
        d_cn_id: [DCNID],
        #[rasn(identifier = "relativeDCNCapacity")]
        relative_dcncapacity: [RelativeMMECapacity],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ServedDCNsItem {
        pub fn new(
            d_cn_id: DCNID,
            relative_dcncapacity: RelativeMMECapacity,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                d_cn_id,
                relative_dcncapacity,
                i_e_extensions,
            }
        }
    }
    #[doc = " This is a dummy IE used only as a reference to the actual definition in relevant specification."]
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct ServedGUMMEIs(pub SequenceOf<ServedGUMMEIsItem>);
    crate::per::sequence_of! { ServedGUMMEIs, 1, 8 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ServedGUMMEIsItem {
        #[rasn(identifier = "servedPLMNs")]
        pub served_plmns: ServedPLMNs,
        #[rasn(identifier = "servedGroupIDs")]
        pub served_group_ids: ServedGroupIDs,
        #[rasn(identifier = "servedMMECs")]
        pub served_mmecs: ServedMMECs,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ServedGUMMEIsItem {
        #[rasn(identifier = "servedPLMNs")]
        served_plmns: [ServedPLMNs],
        #[rasn(identifier = "servedGroupIDs")]
        served_group_ids: [ServedGroupIDs],
        #[rasn(identifier = "servedMMECs")]
        served_mmecs: [ServedMMECs],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ServedGUMMEIsItem {
        pub fn new(
            served_plmns: ServedPLMNs,
            served_group_ids: ServedGroupIDs,
            served_mmecs: ServedMMECs,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                served_plmns,
                served_group_ids,
                served_mmecs,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct ServedGroupIDs(pub SequenceOf<MMEGroupID>);
    crate::per::sequence_of! { ServedGroupIDs, 1, 65535 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct ServedMMECs(pub SequenceOf<MMECode>);
    crate::per::sequence_of! { ServedMMECs, 1, 256 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct ServedPLMNs(pub SequenceOf<PLMNidentity>);
    crate::per::sequence_of! { ServedPLMNs, 1, 32 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ServiceType {
        #[rasn(identifier = "qMC-for-streaming-service")]
        qMC_for_streaming_service = 0,
        #[rasn(identifier = "qMC-for-MTSI-service")]
        qMC_for_MTSI_service = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Source-ToTarget-TransparentContainer")]
    pub struct SourceToTargetTransparentContainer(pub OctetString);
    #[doc = " This IE includes a transparent container from the source RAN node to the target RAN node."]
    #[doc = " The octets of the OCTET STRING are encoded according to the specifications of the target system."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "SourceBSS-ToTargetBSS-TransparentContainer")]
    pub struct SourceBSSToTargetBSSTransparentContainer(pub OctetString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SourceNgRanNode-ID")]
    #[non_exhaustive]
    pub struct SourceNgRanNodeID {
        #[rasn(identifier = "global-RAN-NODE-ID")]
        pub global_ran_node_id: GlobalRANNODEID,
        #[rasn(identifier = "selected-TAI")]
        pub selected_tai: FiveGSTAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SourceNgRanNodeID {
        #[rasn(identifier = "global-RAN-NODE-ID")]
        global_ran_node_id: [GlobalRANNODEID],
        #[rasn(identifier = "selected-TAI")]
        selected_tai: [FiveGSTAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SourceNgRanNodeID {
        pub fn new(
            global_ran_node_id: GlobalRANNODEID,
            selected_tai: FiveGSTAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                global_ran_node_id,
                selected_tai,
                i_e_extensions,
            }
        }
    }
    #[doc = " This is a dummy IE used only as a reference to the actual definition in relevant specification."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "SourceNgRanNode-ToTargetNgRanNode-TransparentContainer"
    )]
    pub struct SourceNgRanNodeToTargetNgRanNodeTransparentContainer(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum SourceNodeID {
        #[rasn(identifier = "sourceNgRanNode-ID")]
        sourceNgRanNode_ID(SourceNgRanNodeID),
        #[rasn(identifier = "sourceNodeID-Extension")]
        sourceNodeID_Extension(ProtocolIEField),
    }
    impl From<SourceNgRanNodeID> for SourceNodeID {
        fn from(value: SourceNgRanNodeID) -> Self {
            Self::sourceNgRanNode_ID(value)
        }
    }
    impl From<ProtocolIEField> for SourceNodeID {
        fn from(value: ProtocolIEField) -> Self {
            Self::sourceNodeID_Extension(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SourceOfUEActivityBehaviourInformation {
        #[rasn(identifier = "subscription-information")]
        subscription_information = 0,
        statistics = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "SourceRNC-ToTargetRNC-TransparentContainer")]
    pub struct SourceRNCToTargetRNCTransparentContainer(pub OctetString);
    #[doc = " This is a dummy IE used only as a reference to the actual definition in relevant specification."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SourceeNB-ID")]
    pub struct SourceeNBID {
        #[rasn(identifier = "global-ENB-ID")]
        pub global_enb_id: GlobalENBID,
        #[rasn(identifier = "selected-TAI")]
        pub selected_tai: TAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    impl SourceeNBID {
        pub fn new(
            global_enb_id: GlobalENBID,
            selected_tai: TAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                global_enb_id,
                selected_tai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "SourceeNB-ToTargeteNB-TransparentContainer"
    )]
    #[non_exhaustive]
    pub struct SourceeNBToTargeteNBTransparentContainer {
        #[rasn(identifier = "rRC-Container")]
        pub r_rc_container: RRCContainer,
        #[rasn(identifier = "e-RABInformationList")]
        pub e_rabinformation_list: Option<ERABInformationList>,
        #[rasn(identifier = "targetCell-ID")]
        pub target_cell_id: EUTRANCGI,
        #[rasn(identifier = "subscriberProfileIDforRFP")]
        pub subscriber_profile_idfor_rfp: Option<SubscriberProfileIDforRFP>,
        #[rasn(identifier = "uE-HistoryInformation")]
        pub u_e_history_information: UEHistoryInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SourceeNBToTargeteNBTransparentContainer {
        #[rasn(identifier = "rRC-Container")]
        r_rc_container: [RRCContainer],
        #[rasn(identifier = "e-RABInformationList")]
        e_rabinformation_list: [Option<ERABInformationList>],
        #[rasn(identifier = "targetCell-ID")]
        target_cell_id: [EUTRANCGI],
        #[rasn(identifier = "subscriberProfileIDforRFP")]
        subscriber_profile_idfor_rfp: [Option<SubscriberProfileIDforRFP>],
        #[rasn(identifier = "uE-HistoryInformation")]
        u_e_history_information: [UEHistoryInformation],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SourceeNBToTargeteNBTransparentContainer {
        pub fn new(
            r_rc_container: RRCContainer,
            e_rabinformation_list: Option<ERABInformationList>,
            target_cell_id: EUTRANCGI,
            subscriber_profile_idfor_rfp: Option<SubscriberProfileIDforRFP>,
            u_e_history_information: UEHistoryInformation,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                r_rc_container,
                e_rabinformation_list,
                target_cell_id,
                subscriber_profile_idfor_rfp,
                u_e_history_information,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3", extensible))]
    pub struct StratumLevel(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=256"))]
    pub struct SubscriberProfileIDforRFP(pub u16);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SubscriptionBasedUEDifferentiationInfoPeriodicCommunicationIndicator {
        periodically = 0,
        ondemand = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SubscriptionBasedUEDifferentiationInfoStationaryIndication {
        stationary = 0,
        mobile = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SubscriptionBasedUEDifferentiationInfoTrafficProfile {
        #[rasn(identifier = "single-packet")]
        single_packet = 0,
        #[rasn(identifier = "dual-packets")]
        dual_packets = 1,
        #[rasn(identifier = "multiple-packets")]
        multiple_packets = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SubscriptionBasedUEDifferentiationInfoBatteryIndication {
        #[rasn(identifier = "battery-powered")]
        battery_powered = 0,
        #[rasn(identifier = "battery-powered-not-rechargeable-or-replaceable")]
        battery_powered_not_rechargeable_or_replaceable = 1,
        #[rasn(identifier = "not-battery-powered")]
        not_battery_powered = 2,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "Subscription-Based-UE-DifferentiationInfo"
    )]
    #[non_exhaustive]
    pub struct SubscriptionBasedUEDifferentiationInfo {
        #[rasn(identifier = "periodicCommunicationIndicator")]
        pub periodic_communication_indicator:
            Option<SubscriptionBasedUEDifferentiationInfoPeriodicCommunicationIndicator>,
        #[rasn(value("1..=3600", extensible), identifier = "periodicTime")]
        pub periodic_time: Option<Integer>,
        #[rasn(identifier = "scheduledCommunicationTime")]
        pub scheduled_communication_time: Option<ScheduledCommunicationTime>,
        #[rasn(identifier = "stationaryIndication")]
        pub stationary_indication:
            Option<SubscriptionBasedUEDifferentiationInfoStationaryIndication>,
        #[rasn(identifier = "trafficProfile")]
        pub traffic_profile: Option<SubscriptionBasedUEDifferentiationInfoTrafficProfile>,
        #[rasn(identifier = "batteryIndication")]
        pub battery_indication: Option<SubscriptionBasedUEDifferentiationInfoBatteryIndication>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SubscriptionBasedUEDifferentiationInfo {
        #[rasn(identifier = "periodicCommunicationIndicator")]
        periodic_communication_indicator: [Option<SubscriptionBasedUEDifferentiationInfoPeriodicCommunicationIndicator>],
        #[rasn(value("1..=3600", extensible), identifier = "periodicTime")]
        periodic_time: [Option<Integer>],
        #[rasn(identifier = "scheduledCommunicationTime")]
        scheduled_communication_time: [Option<ScheduledCommunicationTime>],
        #[rasn(identifier = "stationaryIndication")]
        stationary_indication: [Option<SubscriptionBasedUEDifferentiationInfoStationaryIndication>],
        #[rasn(identifier = "trafficProfile")]
        traffic_profile: [Option<SubscriptionBasedUEDifferentiationInfoTrafficProfile>],
        #[rasn(identifier = "batteryIndication")]
        battery_indication: [Option<SubscriptionBasedUEDifferentiationInfoBatteryIndication>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SubscriptionBasedUEDifferentiationInfo {
        pub fn new(
            periodic_communication_indicator: Option<
                SubscriptionBasedUEDifferentiationInfoPeriodicCommunicationIndicator,
            >,
            periodic_time: Option<Integer>,
            scheduled_communication_time: Option<ScheduledCommunicationTime>,
            stationary_indication: Option<
                SubscriptionBasedUEDifferentiationInfoStationaryIndication,
            >,
            traffic_profile: Option<SubscriptionBasedUEDifferentiationInfoTrafficProfile>,
            battery_indication: Option<SubscriptionBasedUEDifferentiationInfoBatteryIndication>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                periodic_communication_indicator,
                periodic_time,
                scheduled_communication_time,
                stationary_indication,
                traffic_profile,
                battery_indication,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct SupportedTAs(pub SequenceOf<SupportedTAsItem>);
    crate::per::sequence_of! { SupportedTAs, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SupportedTAs-Item")]
    #[non_exhaustive]
    pub struct SupportedTAsItem {
        #[rasn(identifier = "tAC")]
        pub t_ac: TAC,
        #[rasn(identifier = "broadcastPLMNs")]
        pub broadcast_plmns: BPLMNs,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SupportedTAsItem {
        #[rasn(identifier = "tAC")]
        t_ac: [TAC],
        #[rasn(identifier = "broadcastPLMNs")]
        broadcast_plmns: [BPLMNs],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SupportedTAsItem {
        pub fn new(
            t_ac: TAC,
            broadcast_plmns: BPLMNs,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                t_ac,
                broadcast_plmns,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SynchronisationInformation {
        #[rasn(identifier = "sourceStratumLevel")]
        pub source_stratum_level: Option<StratumLevel>,
        #[rasn(identifier = "listeningSubframePattern")]
        pub listening_subframe_pattern: Option<ListeningSubframePattern>,
        #[rasn(identifier = "aggressoreCGI-List")]
        pub aggressore_cgi_list: Option<ECGI_List>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { SynchronisationInformation {
        #[rasn(identifier = "sourceStratumLevel")]
        source_stratum_level: [Option<StratumLevel>],
        #[rasn(identifier = "listeningSubframePattern")]
        listening_subframe_pattern: [Option<ListeningSubframePattern>],
        #[rasn(identifier = "aggressoreCGI-List")]
        aggressore_cgi_list: [Option<ECGI_List>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl SynchronisationInformation {
        pub fn new(
            source_stratum_level: Option<StratumLevel>,
            listening_subframe_pattern: Option<ListeningSubframePattern>,
            aggressore_cgi_list: Option<ECGI_List>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                source_stratum_level,
                listening_subframe_pattern,
                aggressore_cgi_list,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SynchronisationStatus {
        synchronous = 0,
        asynchronous = 1,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TABasedMDT {
        #[rasn(identifier = "tAListforMDT")]
        pub t_alistfor_mdt: TAListforMDT,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TABasedMDT {
        #[rasn(identifier = "tAListforMDT")]
        t_alistfor_mdt: [TAListforMDT],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TABasedMDT {
        pub fn new(
            t_alistfor_mdt: TAListforMDT,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                t_alistfor_mdt,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TABasedQMC {
        #[rasn(identifier = "tAListforQMC")]
        pub t_alistfor_qmc: TAListforQMC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TABasedQMC {
        #[rasn(identifier = "tAListforQMC")]
        t_alistfor_qmc: [TAListforQMC],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TABasedQMC {
        pub fn new(
            t_alistfor_qmc: TAListforQMC,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                t_alistfor_qmc,
                i_e_extensions,
            }
        }
    }
    #[doc = " T"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TAC(pub FixedOctetString<2usize>);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=12"), identifier = "TACList-In-LTE-NTN")]
    pub struct TACListInLTENTN(pub SequenceOf<TAC>);
    crate::per::sequence_of! { TACListInLTENTN, 1, 12 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TAI {
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: PLMNidentity,
        #[rasn(identifier = "tAC")]
        pub t_ac: TAC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TAI {
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [PLMNidentity],
        #[rasn(identifier = "tAC")]
        t_ac: [TAC],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TAI {
        pub fn new(
            p_lmnidentity: PLMNidentity,
            t_ac: TAC,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                p_lmnidentity,
                t_ac,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"), identifier = "TAI-Broadcast")]
    pub struct TAIBroadcast(pub SequenceOf<TAIBroadcastItem>);
    crate::per::sequence_of! { TAIBroadcast, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "TAI-Broadcast-Item")]
    #[non_exhaustive]
    pub struct TAIBroadcastItem {
        #[rasn(identifier = "tAI")]
        pub t_ai: TAI,
        #[rasn(identifier = "completedCellinTAI")]
        pub completed_cellin_tai: CompletedCellinTAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TAIBroadcastItem {
        #[rasn(identifier = "tAI")]
        t_ai: [TAI],
        #[rasn(identifier = "completedCellinTAI")]
        completed_cellin_tai: [CompletedCellinTAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TAIBroadcastItem {
        pub fn new(
            t_ai: TAI,
            completed_cellin_tai: CompletedCellinTAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                t_ai,
                completed_cellin_tai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"), identifier = "TAI-Cancelled")]
    pub struct TAICancelled(pub SequenceOf<TAICancelledItem>);
    crate::per::sequence_of! { TAICancelled, 1, 65535 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "TAI-Cancelled-Item")]
    #[non_exhaustive]
    pub struct TAICancelledItem {
        #[rasn(identifier = "tAI")]
        pub t_ai: TAI,
        #[rasn(identifier = "cancelledCellinTAI")]
        pub cancelled_cellin_tai: CancelledCellinTAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TAICancelledItem {
        #[rasn(identifier = "tAI")]
        t_ai: [TAI],
        #[rasn(identifier = "cancelledCellinTAI")]
        cancelled_cellin_tai: [CancelledCellinTAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TAICancelledItem {
        pub fn new(
            t_ai: TAI,
            cancelled_cellin_tai: CancelledCellinTAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                t_ai,
                cancelled_cellin_tai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TAIBasedMDT {
        #[rasn(identifier = "tAIListforMDT")]
        pub t_ailistfor_mdt: TAIListforMDT,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TAIBasedMDT {
        #[rasn(identifier = "tAIListforMDT")]
        t_ailistfor_mdt: [TAIListforMDT],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TAIBasedMDT {
        pub fn new(
            t_ailistfor_mdt: TAIListforMDT,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                t_ailistfor_mdt,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TAIBasedQMC {
        #[rasn(identifier = "tAIListforQMC")]
        pub t_ailistfor_qmc: TAIListforQMC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TAIBasedQMC {
        #[rasn(identifier = "tAIListforQMC")]
        t_ailistfor_qmc: [TAIListforQMC],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TAIBasedQMC {
        pub fn new(
            t_ailistfor_qmc: TAIListforQMC,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                t_ailistfor_qmc,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=2048"))]
    pub struct TAIListForRestart(pub SequenceOf<TAI>);
    crate::per::sequence_of! { TAIListForRestart, 1, 2048 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct TAIListforMDT(pub SequenceOf<TAI>);
    crate::per::sequence_of! { TAIListforMDT, 1, 8 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct TAIListforQMC(pub SequenceOf<TAI>);
    crate::per::sequence_of! { TAIListforQMC, 1, 8 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct TAIListforWarning(pub SequenceOf<TAI>);
    crate::per::sequence_of! { TAIListforWarning, 1, 65535 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct TAListforMDT(pub SequenceOf<TAC>);
    crate::per::sequence_of! { TAListforMDT, 1, 8 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct TAListforQMC(pub SequenceOf<TAC>);
    crate::per::sequence_of! { TAListforQMC, 1, 8 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "TBCD-STRING")]
    pub struct TBCDSTRING(pub FixedOctetString<3usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Target-ToSource-TransparentContainer")]
    pub struct TargetToSourceTransparentContainer(pub OctetString);
    #[doc = " This is a dummy IE used only as a reference to the actual definition in relevant specification."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "TargetBSS-ToSourceBSS-TransparentContainer")]
    pub struct TargetBSSToSourceBSSTransparentContainer(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum TargetID {
        #[rasn(identifier = "targeteNB-ID")]
        targeteNB_ID(TargeteNBID),
        #[rasn(identifier = "targetRNC-ID")]
        targetRNC_ID(TargetRNCID),
        cGI(CGI),
        #[rasn(extension_addition, identifier = "targetgNgRanNode-ID")]
        targetgNgRanNode_ID(TargetNgRanNodeID),
    }
    impl From<TargeteNBID> for TargetID {
        fn from(value: TargeteNBID) -> Self {
            Self::targeteNB_ID(value)
        }
    }
    impl From<TargetRNCID> for TargetID {
        fn from(value: TargetRNCID) -> Self {
            Self::targetRNC_ID(value)
        }
    }
    impl From<CGI> for TargetID {
        fn from(value: CGI) -> Self {
            Self::cGI(value)
        }
    }
    impl From<TargetNgRanNodeID> for TargetID {
        fn from(value: TargetNgRanNodeID) -> Self {
            Self::targetgNgRanNode_ID(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "TargetNgRanNode-ID")]
    #[non_exhaustive]
    pub struct TargetNgRanNodeID {
        #[rasn(identifier = "global-RAN-NODE-ID")]
        pub global_ran_node_id: GlobalRANNODEID,
        #[rasn(identifier = "selected-TAI")]
        pub selected_tai: FiveGSTAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TargetNgRanNodeID {
        #[rasn(identifier = "global-RAN-NODE-ID")]
        global_ran_node_id: [GlobalRANNODEID],
        #[rasn(identifier = "selected-TAI")]
        selected_tai: [FiveGSTAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TargetNgRanNodeID {
        pub fn new(
            global_ran_node_id: GlobalRANNODEID,
            selected_tai: FiveGSTAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                global_ran_node_id,
                selected_tai,
                i_e_extensions,
            }
        }
    }
    #[doc = " This is a dummy IE used only as a reference to the actual definition in relevant specification."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "TargetNgRanNode-ToSourceNgRanNode-TransparentContainer"
    )]
    pub struct TargetNgRanNodeToSourceNgRanNodeTransparentContainer(pub OctetString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "TargetRNC-ID")]
    #[non_exhaustive]
    pub struct TargetRNCID {
        #[rasn(identifier = "lAI")]
        pub l_ai: LAI,
        #[rasn(identifier = "rAC")]
        pub r_ac: Option<RAC>,
        #[rasn(identifier = "rNC-ID")]
        pub r_nc_id: RNCID,
        #[rasn(identifier = "extendedRNC-ID")]
        pub extended_rnc_id: Option<ExtendedRNCID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TargetRNCID {
        #[rasn(identifier = "lAI")]
        l_ai: [LAI],
        #[rasn(identifier = "rAC")]
        r_ac: [Option<RAC>],
        #[rasn(identifier = "rNC-ID")]
        r_nc_id: [RNCID],
        #[rasn(identifier = "extendedRNC-ID")]
        extended_rnc_id: [Option<ExtendedRNCID>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TargetRNCID {
        pub fn new(
            l_ai: LAI,
            r_ac: Option<RAC>,
            r_nc_id: RNCID,
            extended_rnc_id: Option<ExtendedRNCID>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                l_ai,
                r_ac,
                r_nc_id,
                extended_rnc_id,
                i_e_extensions,
            }
        }
    }
    #[doc = " This IE includes a transparent container from the target RAN node to the source RAN node."]
    #[doc = " The octets of the OCTET STRING are coded according to the specifications of the target system."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "TargetRNC-ToSourceRNC-TransparentContainer")]
    pub struct TargetRNCToSourceRNCTransparentContainer(pub OctetString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "TargeteNB-ID")]
    #[non_exhaustive]
    pub struct TargeteNBID {
        #[rasn(identifier = "global-ENB-ID")]
        pub global_enb_id: GlobalENBID,
        #[rasn(identifier = "selected-TAI")]
        pub selected_tai: TAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TargeteNBID {
        #[rasn(identifier = "global-ENB-ID")]
        global_enb_id: [GlobalENBID],
        #[rasn(identifier = "selected-TAI")]
        selected_tai: [TAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TargeteNBID {
        pub fn new(
            global_enb_id: GlobalENBID,
            selected_tai: TAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                global_enb_id,
                selected_tai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "TargeteNB-ToSourceeNB-TransparentContainer"
    )]
    #[non_exhaustive]
    pub struct TargeteNBToSourceeNBTransparentContainer {
        #[rasn(identifier = "rRC-Container")]
        pub r_rc_container: RRCContainer,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TargeteNBToSourceeNBTransparentContainer {
        #[rasn(identifier = "rRC-Container")]
        r_rc_container: [RRCContainer],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TargeteNBToSourceeNBTransparentContainer {
        pub fn new(
            r_rc_container: RRCContainer,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                r_rc_container,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Threshold-RSRP", value("0..=97"))]
    pub struct ThresholdRSRP(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Threshold-RSRQ", value("0..=34"))]
    pub struct ThresholdRSRQ(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Time-UE-StayedInCell", value("0..=4095"))]
    pub struct TimeUEStayedInCell(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "Time-UE-StayedInCell-EnhancedGranularity",
        value("0..=40950")
    )]
    pub struct TimeUEStayedInCellEnhancedGranularity(pub u16);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TimeBasedHandoverInformation {
        #[rasn(identifier = "hOWindowStart")]
        pub h_owindow_start: HandoverWindowStart,
        #[rasn(identifier = "hOWindowDuration")]
        pub h_owindow_duration: HandoverWindowDuration,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TimeBasedHandoverInformation {
        #[rasn(identifier = "hOWindowStart")]
        h_owindow_start: [HandoverWindowStart],
        #[rasn(identifier = "hOWindowDuration")]
        h_owindow_duration: [HandoverWindowDuration],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TimeBasedHandoverInformation {
        pub fn new(
            h_owindow_start: HandoverWindowStart,
            h_owindow_duration: HandoverWindowDuration,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                h_owindow_start,
                h_owindow_duration,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TimeRefDistribution {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TimeSinceSecondaryNodeRelease(pub FixedOctetString<4usize>);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TimeSynchronisationInfo {
        #[rasn(identifier = "stratumLevel")]
        pub stratum_level: StratumLevel,
        #[rasn(identifier = "synchronisationStatus")]
        pub synchronisation_status: SynchronisationStatus,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TimeSynchronisationInfo {
        #[rasn(identifier = "stratumLevel")]
        stratum_level: [StratumLevel],
        #[rasn(identifier = "synchronisationStatus")]
        synchronisation_status: [SynchronisationStatus],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TimeSynchronisationInfo {
        pub fn new(
            stratum_level: StratumLevel,
            synchronisation_status: SynchronisationStatus,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                stratum_level,
                synchronisation_status,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TimeToTrigger {
        ms0 = 0,
        ms40 = 1,
        ms64 = 2,
        ms80 = 3,
        ms100 = 4,
        ms128 = 5,
        ms160 = 6,
        ms256 = 7,
        ms320 = 8,
        ms480 = 9,
        ms512 = 10,
        ms640 = 11,
        ms1024 = 12,
        ms1280 = 13,
        ms2560 = 14,
        ms5120 = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TimeToWait {
        v1s = 0,
        v2s = 1,
        v5s = 2,
        v10s = 3,
        v20s = 4,
        v60s = 5,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TraceActivation {
        #[rasn(identifier = "e-UTRAN-Trace-ID")]
        pub e_utran_trace_id: EUTRANTraceID,
        #[rasn(identifier = "interfacesToTrace")]
        pub interfaces_to_trace: InterfacesToTrace,
        #[rasn(identifier = "traceDepth")]
        pub trace_depth: TraceDepth,
        #[rasn(identifier = "traceCollectionEntityIPAddress")]
        pub trace_collection_entity_ipaddress: TransportLayerAddress,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TraceActivation {
        #[rasn(identifier = "e-UTRAN-Trace-ID")]
        e_utran_trace_id: [EUTRANTraceID],
        #[rasn(identifier = "interfacesToTrace")]
        interfaces_to_trace: [InterfacesToTrace],
        #[rasn(identifier = "traceDepth")]
        trace_depth: [TraceDepth],
        #[rasn(identifier = "traceCollectionEntityIPAddress")]
        trace_collection_entity_ipaddress: [TransportLayerAddress],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TraceActivation {
        pub fn new(
            e_utran_trace_id: EUTRANTraceID,
            interfaces_to_trace: InterfacesToTrace,
            trace_depth: TraceDepth,
            trace_collection_entity_ipaddress: TransportLayerAddress,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_utran_trace_id,
                interfaces_to_trace,
                trace_depth,
                trace_collection_entity_ipaddress,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TraceDepth {
        minimum = 0,
        medium = 1,
        maximum = 2,
        minimumWithoutVendorSpecificExtension = 3,
        mediumWithoutVendorSpecificExtension = 4,
        maximumWithoutVendorSpecificExtension = 5,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=99"))]
    pub struct TrafficLoadReductionIndication(pub u8);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TransportInformation {
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "uL-GTP-TEID")]
        pub u_l_gtp_teid: GTPTEID,
    }
    crate::per::decode_extensible_sequence! { TransportInformation {
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "uL-GTP-TEID")]
        u_l_gtp_teid: [GTPTEID],
    } }
    impl TransportInformation {
        pub fn new(transport_layer_address: TransportLayerAddress, u_l_gtp_teid: GTPTEID) -> Self {
            Self {
                transport_layer_address,
                u_l_gtp_teid,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=160", extensible))]
    pub struct TransportLayerAddress(pub BitString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TunnelInformation {
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "uDP-Port-Number")]
        pub u_dp_port_number: Option<PortNumber>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TunnelInformation {
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "uDP-Port-Number")]
        u_dp_port_number: [Option<PortNumber>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TunnelInformation {
        pub fn new(
            transport_layer_address: TransportLayerAddress,
            u_dp_port_number: Option<PortNumber>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                transport_layer_address,
                u_dp_port_number,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TypeOfError {
        #[rasn(identifier = "not-understood")]
        not_understood = 0,
        missing = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "UE-Application-Layer-Measurement-Capability")]
    pub struct UEApplicationLayerMeasurementCapability(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"), identifier = "UE-HistoryInformation")]
    pub struct UEHistoryInformation(pub SequenceOf<LastVisitedCellItem>);
    crate::per::sequence_of! { UEHistoryInformation, 1, 16 }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "UE-HistoryInformationFromTheUE")]
    pub struct UEHistoryInformationFromTheUE(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "UE-RLF-Report-Container")]
    pub struct UERLFReportContainer(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "UE-RLF-Report-Container-for-extended-bands")]
    pub struct UERLFReportContainerForExtendedBands(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated, identifier = "UE-RetentionInformation")]
    #[non_exhaustive]
    pub enum UERetentionInformation {
        #[rasn(identifier = "ues-retained")]
        ues_retained = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "UE-S1AP-ID-pair")]
    #[non_exhaustive]
    pub struct UES1APIDPair {
        #[rasn(identifier = "mME-UE-S1AP-ID")]
        pub m_me_ue_s1_ap_id: MMEUES1APID,
        #[rasn(identifier = "eNB-UE-S1AP-ID")]
        pub e_nb_ue_s1_ap_id: ENBUES1APID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { UES1APIDPair {
        #[rasn(identifier = "mME-UE-S1AP-ID")]
        m_me_ue_s1_ap_id: [MMEUES1APID],
        #[rasn(identifier = "eNB-UE-S1AP-ID")]
        e_nb_ue_s1_ap_id: [ENBUES1APID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl UES1APIDPair {
        pub fn new(
            m_me_ue_s1_ap_id: MMEUES1APID,
            e_nb_ue_s1_ap_id: ENBUES1APID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                m_me_ue_s1_ap_id,
                e_nb_ue_s1_ap_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "UE-S1AP-IDs")]
    #[non_exhaustive]
    pub enum UES1APIDs {
        #[rasn(identifier = "uE-S1AP-ID-pair")]
        uE_S1AP_ID_pair(UES1APIDPair),
        #[rasn(identifier = "mME-UE-S1AP-ID")]
        mME_UE_S1AP_ID(MMEUES1APID),
    }
    impl From<UES1APIDPair> for UES1APIDs {
        fn from(value: UES1APIDPair) -> Self {
            Self::uE_S1AP_ID_pair(value)
        }
    }
    impl From<MMEUES1APID> for UES1APIDs {
        fn from(value: MMEUES1APID) -> Self {
            Self::mME_UE_S1AP_ID(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "UE-Usage-Type", value("0..=255"))]
    pub struct UEUsageType(pub u8);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "UE-associatedLogicalS1-ConnectionItem")]
    #[non_exhaustive]
    pub struct UEAssociatedLogicalS1ConnectionItem {
        #[rasn(identifier = "mME-UE-S1AP-ID")]
        pub m_me_ue_s1_ap_id: Option<MMEUES1APID>,
        #[rasn(identifier = "eNB-UE-S1AP-ID")]
        pub e_nb_ue_s1_ap_id: Option<ENBUES1APID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { UEAssociatedLogicalS1ConnectionItem {
        #[rasn(identifier = "mME-UE-S1AP-ID")]
        m_me_ue_s1_ap_id: [Option<MMEUES1APID>],
        #[rasn(identifier = "eNB-UE-S1AP-ID")]
        e_nb_ue_s1_ap_id: [Option<ENBUES1APID>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl UEAssociatedLogicalS1ConnectionItem {
        pub fn new(
            m_me_ue_s1_ap_id: Option<MMEUES1APID>,
            e_nb_ue_s1_ap_id: Option<ENBUES1APID>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                m_me_ue_s1_ap_id,
                e_nb_ue_s1_ap_id,
                i_e_extensions,
            }
        }
    }
    #[doc = " U"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEAggregateMaximumBitrate {
        #[rasn(identifier = "uEaggregateMaximumBitRateDL")]
        pub u_eaggregate_maximum_bit_rate_dl: BitRate,
        #[rasn(identifier = "uEaggregateMaximumBitRateUL")]
        pub u_eaggregate_maximum_bit_rate_ul: BitRate,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { UEAggregateMaximumBitrate {
        #[rasn(identifier = "uEaggregateMaximumBitRateDL")]
        u_eaggregate_maximum_bit_rate_dl: [BitRate],
        #[rasn(identifier = "uEaggregateMaximumBitRateUL")]
        u_eaggregate_maximum_bit_rate_ul: [BitRate],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl UEAggregateMaximumBitrate {
        pub fn new(
            u_eaggregate_maximum_bit_rate_dl: BitRate,
            u_eaggregate_maximum_bit_rate_ul: BitRate,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                u_eaggregate_maximum_bit_rate_dl,
                u_eaggregate_maximum_bit_rate_ul,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEAppLayerMeasConfig {
        #[rasn(identifier = "containerForAppLayerMeasConfig")]
        pub container_for_app_layer_meas_config: crate::sized::SizedOctetString<1, 1000>,
        #[rasn(identifier = "areaScopeOfQMC")]
        pub area_scope_of_qmc: AreaScopeOfQMC,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { UEAppLayerMeasConfig {
        #[rasn(identifier = "containerForAppLayerMeasConfig")]
        container_for_app_layer_meas_config: [crate::sized::SizedOctetString<1, 1000>],
        #[rasn(identifier = "areaScopeOfQMC")]
        area_scope_of_qmc: [AreaScopeOfQMC],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl UEAppLayerMeasConfig {
        pub fn new(
            container_for_app_layer_meas_config: crate::sized::SizedOctetString<1, 1000>,
            area_scope_of_qmc: AreaScopeOfQMC,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                container_for_app_layer_meas_config,
                area_scope_of_qmc,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum UECapabilityInfoRequest {
        requested = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct UEIdentityIndexValue(pub FixedBitString<10usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum UEPagingID {
        #[rasn(identifier = "s-TMSI")]
        s_TMSI(STMSI),
        iMSI(IMSI),
    }
    impl From<STMSI> for UEPagingID {
        fn from(value: STMSI) -> Self {
            Self::s_TMSI(value)
        }
    }
    impl From<IMSI> for UEPagingID {
        fn from(value: IMSI) -> Self {
            Self::iMSI(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct UERadioCapability(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct UERadioCapabilityForPaging(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct UERadioCapabilityID(pub OctetString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UESecurityCapabilities {
        #[rasn(identifier = "encryptionAlgorithms")]
        pub encryption_algorithms: EncryptionAlgorithms,
        #[rasn(identifier = "integrityProtectionAlgorithms")]
        pub integrity_protection_algorithms: IntegrityProtectionAlgorithms,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { UESecurityCapabilities {
        #[rasn(identifier = "encryptionAlgorithms")]
        encryption_algorithms: [EncryptionAlgorithms],
        #[rasn(identifier = "integrityProtectionAlgorithms")]
        integrity_protection_algorithms: [IntegrityProtectionAlgorithms],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl UESecurityCapabilities {
        pub fn new(
            encryption_algorithms: EncryptionAlgorithms,
            integrity_protection_algorithms: IntegrityProtectionAlgorithms,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                encryption_algorithms,
                integrity_protection_algorithms,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UESidelinkAggregateMaximumBitrate {
        #[rasn(identifier = "uESidelinkAggregateMaximumBitRate")]
        pub u_esidelink_aggregate_maximum_bit_rate: BitRate,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { UESidelinkAggregateMaximumBitrate {
        #[rasn(identifier = "uESidelinkAggregateMaximumBitRate")]
        u_esidelink_aggregate_maximum_bit_rate: [BitRate],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl UESidelinkAggregateMaximumBitrate {
        pub fn new(
            u_esidelink_aggregate_maximum_bit_rate: BitRate,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                u_esidelink_aggregate_maximum_bit_rate,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum UEUserPlaneCIoTSupportIndicator {
        supported = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "UL-CP-SecurityInformation")]
    #[non_exhaustive]
    pub struct ULCPSecurityInformation {
        #[rasn(identifier = "ul-NAS-MAC")]
        pub ul_nas_mac: ULNASMAC,
        #[rasn(identifier = "ul-NAS-Count")]
        pub ul_nas_count: ULNASCount,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ULCPSecurityInformation {
        #[rasn(identifier = "ul-NAS-MAC")]
        ul_nas_mac: [ULNASMAC],
        #[rasn(identifier = "ul-NAS-Count")]
        ul_nas_count: [ULNASCount],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ULCPSecurityInformation {
        pub fn new(
            ul_nas_mac: ULNASMAC,
            ul_nas_count: ULNASCount,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                ul_nas_mac,
                ul_nas_count,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "UL-NAS-Count")]
    pub struct ULNASCount(pub FixedBitString<5usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "UL-NAS-MAC")]
    pub struct ULNASMAC(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "URI-Address")]
    pub struct URIAddress(pub VisibleString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum UnlicensedSpectrumRestriction {
        #[rasn(identifier = "unlicensed-restricted")]
        unlicensed_restricted = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UserLocationInformation {
        #[rasn(identifier = "eutran-cgi")]
        pub eutran_cgi: EUTRANCGI,
        pub tai: TAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { UserLocationInformation {
        #[rasn(identifier = "eutran-cgi")]
        eutran_cgi: [EUTRANCGI],
        tai: [TAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl UserLocationInformation {
        pub fn new(
            eutran_cgi: EUTRANCGI,
            tai: TAI,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                eutran_cgi,
                tai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct V2XServicesAuthorized {
        #[rasn(identifier = "vehicleUE")]
        pub vehicle_ue: Option<VehicleUE>,
        #[rasn(identifier = "pedestrianUE")]
        pub pedestrian_ue: Option<PedestrianUE>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { V2XServicesAuthorized {
        #[rasn(identifier = "vehicleUE")]
        vehicle_ue: [Option<VehicleUE>],
        #[rasn(identifier = "pedestrianUE")]
        pedestrian_ue: [Option<PedestrianUE>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl V2XServicesAuthorized {
        pub fn new(
            vehicle_ue: Option<VehicleUE>,
            pedestrian_ue: Option<PedestrianUE>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                vehicle_ue,
                pedestrian_ue,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum VehicleUE {
        authorized = 0,
        #[rasn(identifier = "not-authorized")]
        not_authorized = 1,
    }
    #[doc = " First bit: QoE Measurement for streaming service"]
    #[doc = " Second bit: QoE Measurement for MTSI service"]
    #[doc = " Note that undefined bits are considered as a spare bit and spare bits shall be set to 0 by the transmitter and shall be ignored by the receiver."]
    #[doc = " V"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum VoiceSupportMatchIndicator {
        supported = 0,
        #[rasn(identifier = "not-supported")]
        not_supported = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum WLANMeasConfig {
        setup = 0,
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct WLANMeasConfigNameList(pub SequenceOf<WLANName>);
    crate::per::sequence_of! { WLANMeasConfigNameList, 1, 4 }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum WLANMeasurementConfigurationWlanRssi {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum WLANMeasurementConfigurationWlanRtt {
        #[rasn(identifier = "true")]
        R_true = 0,
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct WLANMeasurementConfiguration {
        #[rasn(identifier = "wlanMeasConfig")]
        pub wlan_meas_config: WLANMeasConfig,
        #[rasn(identifier = "wlanMeasConfigNameList")]
        pub wlan_meas_config_name_list: Option<WLANMeasConfigNameList>,
        #[rasn(identifier = "wlan-rssi")]
        pub wlan_rssi: Option<WLANMeasurementConfigurationWlanRssi>,
        #[rasn(identifier = "wlan-rtt")]
        pub wlan_rtt: Option<WLANMeasurementConfigurationWlanRtt>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { WLANMeasurementConfiguration {
        #[rasn(identifier = "wlanMeasConfig")]
        wlan_meas_config: [WLANMeasConfig],
        #[rasn(identifier = "wlanMeasConfigNameList")]
        wlan_meas_config_name_list: [Option<WLANMeasConfigNameList>],
        #[rasn(identifier = "wlan-rssi")]
        wlan_rssi: [Option<WLANMeasurementConfigurationWlanRssi>],
        #[rasn(identifier = "wlan-rtt")]
        wlan_rtt: [Option<WLANMeasurementConfigurationWlanRtt>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl WLANMeasurementConfiguration {
        pub fn new(
            wlan_meas_config: WLANMeasConfig,
            wlan_meas_config_name_list: Option<WLANMeasConfigNameList>,
            wlan_rssi: Option<WLANMeasurementConfigurationWlanRssi>,
            wlan_rtt: Option<WLANMeasurementConfigurationWlanRtt>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                wlan_meas_config,
                wlan_meas_config_name_list,
                wlan_rssi,
                wlan_rtt,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct WLANName(pub OctetString);
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "WUS-Assistance-Information")]
    #[non_exhaustive]
    pub struct WUSAssistanceInformation {
        #[rasn(identifier = "pagingProbabilityInformation")]
        pub paging_probability_information: PagingProbabilityInformation,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { WUSAssistanceInformation {
        #[rasn(identifier = "pagingProbabilityInformation")]
        paging_probability_information: [PagingProbabilityInformation],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl WUSAssistanceInformation {
        pub fn new(
            paging_probability_information: PagingProbabilityInformation,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                paging_probability_information,
                i_e_extensions,
            }
        }
    }
    #[doc = " W"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=1024"))]
    pub struct WarningAreaCoordinates(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum WarningAreaList {
        cellIDList(ECGIList),
        trackingAreaListforWarning(TAIListforWarning),
        emergencyAreaIDList(EmergencyAreaIDList),
    }
    impl From<ECGIList> for WarningAreaList {
        fn from(value: ECGIList) -> Self {
            Self::cellIDList(value)
        }
    }
    impl From<TAIListforWarning> for WarningAreaList {
        fn from(value: TAIListforWarning) -> Self {
            Self::trackingAreaListforWarning(value)
        }
    }
    impl From<EmergencyAreaIDList> for WarningAreaList {
        fn from(value: EmergencyAreaIDList) -> Self {
            Self::emergencyAreaIDList(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=9600"))]
    pub struct WarningMessageContents(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct WarningSecurityInfo(pub FixedOctetString<50usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct WarningType(pub FixedOctetString<2usize>);
    #[doc = " X"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct X2TNLConfigurationInfo {
        #[rasn(identifier = "eNBX2TransportLayerAddresses")]
        pub e_nbx2_transport_layer_addresses: ENBX2TLAs,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { X2TNLConfigurationInfo {
        #[rasn(identifier = "eNBX2TransportLayerAddresses")]
        e_nbx2_transport_layer_addresses: [ENBX2TLAs],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl X2TNLConfigurationInfo {
        pub fn new(
            e_nbx2_transport_layer_addresses: ENBX2TLAs,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_nbx2_transport_layer_addresses,
                i_e_extensions,
            }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod s1_ap_pdu_contents {
    extern crate alloc;
    use super::s1_ap_common_data_types::{
        Criticality, Presence, PrivateIEID, ProcedureCode, ProtocolIEID,
    };
    use super::s1_ap_constants::{
        ID_ADDITIONAL_CSFALLBACK_INDICATOR, ID_ADDITIONAL_GUTI, ID_ADDITIONAL_RRMPRIORITY_INDEX,
        ID_AERIAL_UESUBSCRIPTION_INFORMATION, ID_ASSISTANCE_DATA_FOR_PAGING, ID_BEARER_TYPE,
        ID_BROADCAST_CANCELLED_AREA_LIST, ID_BROADCAST_COMPLETED_AREA_LIST, ID_CAUSE,
        ID_CDMA2000_HOREQUIRED_INDICATION, ID_CDMA2000_HOSTATUS, ID_CDMA2000_ONE_XRAND,
        ID_CDMA2000_ONE_XSRVCCINFO, ID_CDMA2000_PDU, ID_CDMA2000_RATTYPE, ID_CDMA2000_SECTOR_ID,
        ID_CE_MODE_B_SUPPORT_INDICATOR, ID_CE_MODE_BRESTRICTED, ID_CELL_ACCESS_MODE,
        ID_CELL_IDENTIFIER_AND_CELEVEL_FOR_CECAPABLE_UES, ID_CNDOMAIN, ID_COARSE_UELOCATION,
        ID_COARSE_UELOCATION_REQUESTED, ID_CONCURRENT_WARNING_MESSAGE_INDICATOR,
        ID_CONNECTEDENG_NBLIST, ID_CONNECTEDENG_NBTO_ADD_LIST, ID_CONNECTEDENG_NBTO_REMOVE_LIST,
        ID_CORRELATION_ID, ID_COVERAGE_LEVEL, ID_CRITICALITY_DIAGNOSTICS, ID_CSFALLBACK_INDICATOR,
        ID_CSG_ID, ID_CSG_ID_LIST, ID_CSGMEMBERSHIP_INFO, ID_CSGMEMBERSHIP_STATUS,
        ID_DATA_CODING_SCHEME, ID_DATA_FORWARDING_NOT_POSSIBLE, ID_DATA_SIZE, ID_DCN_ID,
        ID_DEFAULT_PAGING_DRX, ID_DIRECT_FORWARDING_PATH_AVAILABILITY,
        ID_DL_CP_SECURITY_INFORMATION, ID_DLNASPDUDELIVERY_ACK_REQUEST,
        ID_DOWNLINK_PACKET_LOSS_RATE, ID_E_NB_EARLY_STATUS_TRANSFER_TRANSPARENT_CONTAINER,
        ID_E_NB_STATUS_TRANSFER_TRANSPARENT_CONTAINER, ID_E_NB_UE_S1_AP_ID, ID_E_NBNAME,
        ID_E_RABADMITTED_ITEM, ID_E_RABADMITTED_LIST, ID_E_RABDATA_FORWARDING_ITEM,
        ID_E_RABFAILED_TO_BE_RELEASED_LIST, ID_E_RABFAILED_TO_MODIFY_LIST,
        ID_E_RABFAILED_TO_MODIFY_LIST_BEARER_MOD_CONF, ID_E_RABFAILED_TO_RELEASE_LIST,
        ID_E_RABFAILED_TO_RESUME_ITEM_RESUME_REQ, ID_E_RABFAILED_TO_RESUME_ITEM_RESUME_RES,
        ID_E_RABFAILED_TO_RESUME_LIST_RESUME_REQ, ID_E_RABFAILED_TO_RESUME_LIST_RESUME_RES,
        ID_E_RABFAILED_TO_SETUP_LIST_BEARER_SURES, ID_E_RABFAILED_TO_SETUP_LIST_CTXT_SURES,
        ID_E_RABFAILED_TO_SETUP_LIST_HOREQ_ACK, ID_E_RABFAILEDTO_SETUP_ITEM_HOREQ_ACK,
        ID_E_RABMODIFY, ID_E_RABMODIFY_ITEM_BEARER_MOD_CONF, ID_E_RABMODIFY_ITEM_BEARER_MOD_RES,
        ID_E_RABMODIFY_LIST_BEARER_MOD_CONF, ID_E_RABMODIFY_LIST_BEARER_MOD_RES,
        ID_E_RABNOT_TO_BE_MODIFIED_ITEM_BEARER_MOD_IND,
        ID_E_RABNOT_TO_BE_MODIFIED_LIST_BEARER_MOD_IND, ID_E_RABRELEASE,
        ID_E_RABRELEASE_INDICATION, ID_E_RABRELEASE_ITEM_BEARER_REL_COMP,
        ID_E_RABRELEASE_ITEM_HOCMD, ID_E_RABRELEASE_LIST_BEARER_REL_COMP, ID_E_RABRELEASED_LIST,
        ID_E_RABSETUP, ID_E_RABSETUP_ITEM_BEARER_SURES, ID_E_RABSETUP_ITEM_CTXT_SURES,
        ID_E_RABSETUP_LIST_BEARER_SURES, ID_E_RABSETUP_LIST_CTXT_SURES,
        ID_E_RABSUBJECTTO_DATA_FORWARDING_LIST, ID_E_RABTO_BE_MODIFIED_ITEM_BEARER_MOD_IND,
        ID_E_RABTO_BE_MODIFIED_ITEM_BEARER_MOD_REQ, ID_E_RABTO_BE_MODIFIED_LIST_BEARER_MOD_IND,
        ID_E_RABTO_BE_MODIFIED_LIST_BEARER_MOD_REQ, ID_E_RABTO_BE_RELEASED_LIST,
        ID_E_RABTO_BE_RELEASED_LIST_BEARER_MOD_CONF, ID_E_RABTO_BE_SETUP_ITEM_BEARER_SUREQ,
        ID_E_RABTO_BE_SETUP_ITEM_CTXT_SUREQ, ID_E_RABTO_BE_SETUP_ITEM_HOREQ,
        ID_E_RABTO_BE_SETUP_LIST_BEARER_SUREQ, ID_E_RABTO_BE_SETUP_LIST_CTXT_SUREQ,
        ID_E_RABTO_BE_SETUP_LIST_HOREQ, ID_E_RABTO_BE_SWITCHED_DLITEM,
        ID_E_RABTO_BE_SWITCHED_DLLIST, ID_E_RABTO_BE_SWITCHED_ULITEM,
        ID_E_RABTO_BE_SWITCHED_ULLIST, ID_E_RABTO_BE_UPDATED_ITEM, ID_E_RABTO_BE_UPDATED_LIST,
        ID_E_RABTO_RELEASE_LIST_HOCMD, ID_E_UTRAN_TRACE_ID, ID_ECGILIST_FOR_RESTART,
        ID_EDT_SESSION, ID_EMERGENCY_AREA_IDLIST_FOR_RESTART,
        ID_EN_DCSONCONFIGURATION_TRANSFER_ECT, ID_EN_DCSONCONFIGURATION_TRANSFER_MCT,
        ID_END_INDICATION, ID_ENHANCED_COVERAGE_RESTRICTED, ID_ETHERNET_TYPE, ID_EUTRAN_CGI,
        ID_EUTRANROUND_TRIP_DELAY_ESTIMATION_INFO, ID_EXPECTED_UEBEHAVIOUR,
        ID_EXTENDED_REPETITION_PERIOD, ID_EXTENDED_UEIDENTITY_INDEX_VALUE,
        ID_GERANTO_LTEHOINFORMATION_RES, ID_GLOBAL_ENB_ID, ID_GUMMEI_ID, ID_GUMMEILIST,
        ID_GUMMEITYPE, ID_GW_TRANSPORT_LAYER_ADDRESS, ID_GWCONTEXT_RELEASE_INDICATION,
        ID_HANDOVER_FLAG, ID_HANDOVER_RESTRICTION_LIST, ID_HANDOVER_TYPE, ID_IAB_AUTHORIZED,
        ID_IAB_NODE_INDICATION, ID_IAB_SUPPORTED,
        ID_INFORMATION_ON_RECOMMENDED_CELLS_AND_ENBS_FOR_PAGING, ID_INITIAL_CONTEXT_SETUP,
        ID_INTER_SYSTEM_INFORMATION_TRANSFER_TYPE_EDT,
        ID_INTER_SYSTEM_INFORMATION_TRANSFER_TYPE_MDT,
        ID_INTERSYSTEM_SONCONFIGURATION_TRANSFER_ECT, ID_INTERSYSTEM_SONCONFIGURATION_TRANSFER_MCT,
        ID_KILL_ALL_WARNING_MESSAGES, ID_LHN_ID, ID_LPPA_PDU, ID_LTE_M_INDICATION,
        ID_LTE_NTN_TAI_INFORMATION, ID_MANAGEMENT_BASED_MDTALLOWED,
        ID_MANAGEMENT_BASED_MDTPLMNLIST, ID_MASKED_IMEISV, ID_MDTCONFIGURATION_NR,
        ID_MESSAGE_IDENTIFIER, ID_MME_GROUP_ID, ID_MME_UE_S1_AP_ID, ID_MME_UE_S1_AP_ID_2,
        ID_MMENAME, ID_MMERELAY_SUPPORT_INDICATOR, ID_MSCLASSMARK2, ID_MSCLASSMARK3,
        ID_NAS_DOWNLINK_COUNT, ID_NAS_PDU, ID_NASSECURITY_PARAMETERSFROM_E_UTRAN,
        ID_NASSECURITY_PARAMETERSTO_E_UTRAN, ID_NB_IO_T_DEFAULT_PAGING_DRX, ID_NB_IO_T_PAGING_DRX,
        ID_NB_IO_T_PAGING_E_DRXINFORMATION, ID_NB_IO_T_UEIDENTITY_INDEX_VALUE,
        ID_NOTIFY_SOURCEE_NB, ID_NRUESECURITY_CAPABILITIES,
        ID_NRUESIDELINK_AGGREGATE_MAXIMUM_BITRATE, ID_NRV2_XSERVICES_AUTHORIZED,
        ID_NUMBEROF_BROADCAST_REQUEST, ID_OVERLOAD_RESPONSE, ID_PAGING_CAUSE, ID_PAGING_DRX,
        ID_PAGING_E_DRXINFORMATION, ID_PAGING_PRIORITY, ID_PC5_QO_SPARAMETERS,
        ID_PENDING_DATA_INDICATION, ID_PRIVACY_INDICATOR, ID_PRO_SE_AUTHORIZED,
        ID_PS_SERVICE_NOT_AVAILABLE, ID_PSCELL_INFORMATION, ID_PWSFAILED_ECGILIST,
        ID_PWSFAILURE_INDICATION, ID_REGISTERED_LAI, ID_RELATIVE_MMECAPACITY,
        ID_RELAY_NODE_INDICATOR, ID_REPETITION_PERIOD, ID_REQUEST_TYPE, ID_RESET_TYPE,
        ID_ROUTING_ID, ID_RRC_ESTABLISHMENT_CAUSE, ID_RRC_RESUME_CAUSE, ID_S_TMSI, ID_S1_MESSAGE,
        ID_SECONDARY_RATDATA_USAGE_REPORT_LIST, ID_SECONDARY_RATDATA_USAGE_REQUEST,
        ID_SECURITY_CONTEXT, ID_SECURITY_INDICATION, ID_SECURITY_KEY, ID_SECURITY_RESULT,
        ID_SERIAL_NUMBER, ID_SERVED_DCNS, ID_SERVED_GUMMEIS, ID_SIPTO_CORRELATION_ID,
        ID_SIPTO_L_GW_TRANSPORT_LAYER_ADDRESS, ID_SONCONFIGURATION_TRANSFER_ECT,
        ID_SONCONFIGURATION_TRANSFER_MCT, ID_SOURCE_MME_GUMMEI, ID_SOURCE_MME_UE_S1_AP_ID,
        ID_SOURCE_TO_TARGET_TRANSPARENT_CONTAINER,
        ID_SOURCE_TO_TARGET_TRANSPARENT_CONTAINER_SECONDARY, ID_SRVCCHOINDICATION,
        ID_SRVCCOPERATION_NOT_POSSIBLE, ID_SRVCCOPERATION_POSSIBLE,
        ID_SUBSCRIBER_PROFILE_IDFOR_RFP, ID_SUBSCRIPTION_BASED_UE_DIFFERENTIATION_INFO,
        ID_SUPPORTED_TAS, ID_TAI, ID_TAIITEM, ID_TAILIST, ID_TAILIST_FOR_RESTART, ID_TARGET_ID,
        ID_TARGET_TO_SOURCE_TRANSPARENT_CONTAINER,
        ID_TARGET_TO_SOURCE_TRANSPARENT_CONTAINER_SECONDARY, ID_TIME_REF_DISTRIBUTION,
        ID_TIME_SINCE_SECONDARY_NODE_RELEASE, ID_TIME_TO_WAIT, ID_TRACE_ACTIVATION,
        ID_TRACE_COLLECTION_ENTITY_IPADDRESS, ID_TRAFFIC_LOAD_REDUCTION_INDICATION,
        ID_TRANSPORT_INFORMATION, ID_TUNNEL_INFORMATION_FOR_BBF, ID_U_EAGGREGATE_MAXIMUM_BITRATE,
        ID_UE_APPLICATION_LAYER_MEASUREMENT_CAPABILITY,
        ID_UE_ASSOCIATED_LOGICAL_S1_CONNECTION_ITEM,
        ID_UE_ASSOCIATED_LOGICAL_S1_CONNECTION_LIST_RES_ACK, ID_UE_LEVEL_QO_S_PARAMETERS,
        ID_UE_RETENTION_INFORMATION, ID_UE_S1_AP_IDS, ID_UE_USAGE_TYPE,
        ID_UECAPABILITY_INFO_REQUEST, ID_UEIDENTITY_INDEX_VALUE, ID_UEPAGING_ID,
        ID_UERADIO_CAPABILITY, ID_UERADIO_CAPABILITY_FOR_PAGING,
        ID_UERADIO_CAPABILITY_FOR_PAGING_NR_FORMAT, ID_UERADIO_CAPABILITY_ID,
        ID_UERADIO_CAPABILITY_NR_FORMAT, ID_UESECURITY_CAPABILITIES,
        ID_UESIDELINK_AGGREGATE_MAXIMUM_BITRATE, ID_UEUSER_PLANE_CIO_TSUPPORT_INDICATOR,
        ID_UL_CP_SECURITY_INFORMATION, ID_UPLINK_PACKET_LOSS_RATE, ID_USER_LOCATION_INFORMATION,
        ID_UTRANTO_LTEHOINFORMATION_RES, ID_V2_XSERVICES_AUTHORIZED,
        ID_VOICE_SUPPORT_MATCH_INDICATOR, ID_WARNING_AREA_COORDINATES, ID_WARNING_AREA_LIST,
        ID_WARNING_MESSAGE_CONTENTS, ID_WARNING_SECURITY_INFO, ID_WARNING_TYPE,
        ID_WUS_ASSISTANCE_INFORMATION, MAXNOOF_CELL_ID, MAXNOOF_CELLIN_EAI, MAXNOOF_CELLIN_TAI,
        MAXNOOF_E_RABS, MAXNOOF_EMERGENCY_AREA_ID, MAXNOOF_ERRORS,
        MAXNOOF_INDIVIDUAL_S1_CONNECTIONS_TO_RESET, MAXNOOF_TAIFOR_WARNING, MAXNOOF_TAIS,
    };
    use super::s1_ap_containers::*;
    use super::s1_ap_ies::*;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CSGMembershipInfo {
        #[rasn(identifier = "cSGMembershipStatus")]
        pub c_sgmembership_status: CSGMembershipStatus,
        #[rasn(identifier = "cSG-Id")]
        pub c_sg_id: CSGId,
        #[rasn(identifier = "cellAccessMode")]
        pub cell_access_mode: Option<CellAccessMode>,
        #[rasn(identifier = "pLMNidentity")]
        pub p_lmnidentity: Option<PLMNidentity>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { CSGMembershipInfo {
        #[rasn(identifier = "cSGMembershipStatus")]
        c_sgmembership_status: [CSGMembershipStatus],
        #[rasn(identifier = "cSG-Id")]
        c_sg_id: [CSGId],
        #[rasn(identifier = "cellAccessMode")]
        cell_access_mode: [Option<CellAccessMode>],
        #[rasn(identifier = "pLMNidentity")]
        p_lmnidentity: [Option<PLMNidentity>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl CSGMembershipInfo {
        pub fn new(
            c_sgmembership_status: CSGMembershipStatus,
            c_sg_id: CSGId,
            cell_access_mode: Option<CellAccessMode>,
            p_lmnidentity: Option<PLMNidentity>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                c_sgmembership_status,
                c_sg_id,
                cell_access_mode,
                p_lmnidentity,
                i_e_extensions,
            }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " CELL TRAFFIC TRACE ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Cell Traffic Trace"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CellTrafficTrace {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { CellTrafficTrace {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl CellTrafficTrace {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Connection Establishment Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ConnectionEstablishmentIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ConnectionEstablishmentIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ConnectionEstablishmentIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " DEACTIVATE TRACE ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Deactivate Trace"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DeactivateTrace {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { DeactivateTrace {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl DeactivateTrace {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " NAS TRANSPORT ELEMENTARY PROCEDURES"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " DOWNLINK NAS TRANSPORT"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DownlinkNASTransport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { DownlinkNASTransport {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl DownlinkNASTransport {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " DOWNLINK NON UE ASSOCIATED LPPA TRANSPORT"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DownlinkNonUEAssociatedLPPaTransport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { DownlinkNonUEAssociatedLPPaTransport {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl DownlinkNonUEAssociatedLPPaTransport {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " DOWNLINK S1 CDMA2000 TUNNELLING ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Downlink S1 CDMA2000 Tunnelling"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DownlinkS1cdma2000tunnelling {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { DownlinkS1cdma2000tunnelling {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl DownlinkS1cdma2000tunnelling {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " LPPA TRANSPORT ELEMENTARY PROCEDURES"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " DOWNLINK UE ASSOCIATED LPPA TRANSPORT"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DownlinkUEAssociatedLPPaTransport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { DownlinkUEAssociatedLPPaTransport {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl DownlinkUEAssociatedLPPaTransport {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABAdmittedItem")]
    #[non_exhaustive]
    pub struct ERABAdmittedItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTP-TEID")]
        pub g_tp_teid: GTPTEID,
        #[rasn(identifier = "dL-transportLayerAddress")]
        pub d_l_transport_layer_address: Option<TransportLayerAddress>,
        #[rasn(identifier = "dL-gTP-TEID")]
        pub d_l_g_tp_teid: Option<GTPTEID>,
        #[rasn(identifier = "uL-TransportLayerAddress")]
        pub u_l_transport_layer_address: Option<TransportLayerAddress>,
        #[rasn(identifier = "uL-GTP-TEID")]
        pub u_l_gtp_teid: Option<GTPTEID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABAdmittedItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "gTP-TEID")]
        g_tp_teid: [GTPTEID],
        #[rasn(identifier = "dL-transportLayerAddress")]
        d_l_transport_layer_address: [Option<TransportLayerAddress>],
        #[rasn(identifier = "dL-gTP-TEID")]
        d_l_g_tp_teid: [Option<GTPTEID>],
        #[rasn(identifier = "uL-TransportLayerAddress")]
        u_l_transport_layer_address: [Option<TransportLayerAddress>],
        #[rasn(identifier = "uL-GTP-TEID")]
        u_l_gtp_teid: [Option<GTPTEID>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABAdmittedItem {
        pub fn new(
            e_rab_id: ERABID,
            transport_layer_address: TransportLayerAddress,
            g_tp_teid: GTPTEID,
            d_l_transport_layer_address: Option<TransportLayerAddress>,
            d_l_g_tp_teid: Option<GTPTEID>,
            u_l_transport_layer_address: Option<TransportLayerAddress>,
            u_l_gtp_teid: Option<GTPTEID>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                transport_layer_address,
                g_tp_teid,
                d_l_transport_layer_address,
                d_l_g_tp_teid,
                u_l_transport_layer_address,
                u_l_gtp_teid,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABAdmittedList")]
    pub struct ERABAdmittedList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABAdmittedList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABDataForwardingItem")]
    #[non_exhaustive]
    pub struct ERABDataForwardingItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "dL-transportLayerAddress")]
        pub d_l_transport_layer_address: Option<TransportLayerAddress>,
        #[rasn(identifier = "dL-gTP-TEID")]
        pub d_l_g_tp_teid: Option<GTPTEID>,
        #[rasn(identifier = "uL-TransportLayerAddress")]
        pub u_l_transport_layer_address: Option<TransportLayerAddress>,
        #[rasn(identifier = "uL-GTP-TEID")]
        pub u_l_gtp_teid: Option<GTPTEID>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABDataForwardingItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "dL-transportLayerAddress")]
        d_l_transport_layer_address: [Option<TransportLayerAddress>],
        #[rasn(identifier = "dL-gTP-TEID")]
        d_l_g_tp_teid: [Option<GTPTEID>],
        #[rasn(identifier = "uL-TransportLayerAddress")]
        u_l_transport_layer_address: [Option<TransportLayerAddress>],
        #[rasn(identifier = "uL-GTP-TEID")]
        u_l_gtp_teid: [Option<GTPTEID>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABDataForwardingItem {
        pub fn new(
            e_rab_id: ERABID,
            d_l_transport_layer_address: Option<TransportLayerAddress>,
            d_l_g_tp_teid: Option<GTPTEID>,
            u_l_transport_layer_address: Option<TransportLayerAddress>,
            u_l_gtp_teid: Option<GTPTEID>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                d_l_transport_layer_address,
                d_l_g_tp_teid,
                u_l_transport_layer_address,
                u_l_gtp_teid,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABFailedToResumeItemResumeReq")]
    #[non_exhaustive]
    pub struct ERABFailedToResumeItemResumeReq {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABFailedToResumeItemResumeReq {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        cause: [Cause],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABFailedToResumeItemResumeReq {
        pub fn new(
            e_rab_id: ERABID,
            cause: Cause,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABFailedToResumeItemResumeRes")]
    #[non_exhaustive]
    pub struct ERABFailedToResumeItemResumeRes {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABFailedToResumeItemResumeRes {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        cause: [Cause],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABFailedToResumeItemResumeRes {
        pub fn new(
            e_rab_id: ERABID,
            cause: Cause,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "E-RABFailedToResumeListResumeReq"
    )]
    pub struct ERABFailedToResumeListResumeReq(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABFailedToResumeListResumeReq, 1, 256 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "E-RABFailedToResumeListResumeRes"
    )]
    pub struct ERABFailedToResumeListResumeRes(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABFailedToResumeListResumeRes, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABFailedToSetupItemHOReqAck")]
    #[non_exhaustive]
    pub struct ERABFailedToSetupItemHOReqAck {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        pub cause: Cause,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABFailedToSetupItemHOReqAck {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        cause: [Cause],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABFailedToSetupItemHOReqAck {
        pub fn new(
            e_rab_id: ERABID,
            cause: Cause,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                cause,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "E-RABFailedtoSetupListHOReqAck"
    )]
    pub struct ERABFailedtoSetupListHOReqAck(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABFailedtoSetupListHOReqAck, 1, 256 }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB Modification Confirm"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABModificationConfirm")]
    #[non_exhaustive]
    pub struct ERABModificationConfirm {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ERABModificationConfirm {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ERABModificationConfirm {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB MODIFICATION INDICATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB Modification Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABModificationIndication")]
    #[non_exhaustive]
    pub struct ERABModificationIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ERABModificationIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ERABModificationIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABModifyItemBearerModConf")]
    #[non_exhaustive]
    pub struct ERABModifyItemBearerModConf {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABModifyItemBearerModConf {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABModifyItemBearerModConf {
        pub fn new(e_rab_id: ERABID, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                e_rab_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABModifyItemBearerModRes")]
    #[non_exhaustive]
    pub struct ERABModifyItemBearerModRes {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABModifyItemBearerModRes {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABModifyItemBearerModRes {
        pub fn new(e_rab_id: ERABID, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                e_rab_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABModifyListBearerModConf")]
    pub struct ERABModifyListBearerModConf(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABModifyListBearerModConf, 1, 256 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABModifyListBearerModRes")]
    pub struct ERABModifyListBearerModRes(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABModifyListBearerModRes, 1, 256 }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB MODIFY ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB Modify Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABModifyRequest")]
    #[non_exhaustive]
    pub struct ERABModifyRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ERABModifyRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ERABModifyRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB Modify Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABModifyResponse")]
    #[non_exhaustive]
    pub struct ERABModifyResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ERABModifyResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ERABModifyResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABNotToBeModifiedItemBearerModInd")]
    #[non_exhaustive]
    pub struct ERABNotToBeModifiedItemBearerModInd {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "dL-GTP-TEID")]
        pub d_l_gtp_teid: GTPTEID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABNotToBeModifiedItemBearerModInd {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "dL-GTP-TEID")]
        d_l_gtp_teid: [GTPTEID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABNotToBeModifiedItemBearerModInd {
        pub fn new(
            e_rab_id: ERABID,
            transport_layer_address: TransportLayerAddress,
            d_l_gtp_teid: GTPTEID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                transport_layer_address,
                d_l_gtp_teid,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "E-RABNotToBeModifiedListBearerModInd"
    )]
    pub struct ERABNotToBeModifiedListBearerModInd(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABNotToBeModifiedListBearerModInd, 1, 256 }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB RELEASE ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB Release Command"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABReleaseCommand")]
    #[non_exhaustive]
    pub struct ERABReleaseCommand {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ERABReleaseCommand {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ERABReleaseCommand {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB RELEASE INDICATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB Release Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABReleaseIndication")]
    #[non_exhaustive]
    pub struct ERABReleaseIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ERABReleaseIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ERABReleaseIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABReleaseItemBearerRelComp")]
    #[non_exhaustive]
    pub struct ERABReleaseItemBearerRelComp {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABReleaseItemBearerRelComp {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABReleaseItemBearerRelComp {
        pub fn new(e_rab_id: ERABID, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                e_rab_id,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "E-RABReleaseListBearerRelComp"
    )]
    pub struct ERABReleaseListBearerRelComp(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABReleaseListBearerRelComp, 1, 256 }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB Release Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABReleaseResponse")]
    #[non_exhaustive]
    pub struct ERABReleaseResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ERABReleaseResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ERABReleaseResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABSetupItemBearerSURes")]
    #[non_exhaustive]
    pub struct ERABSetupItemBearerSURes {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTP-TEID")]
        pub g_tp_teid: GTPTEID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABSetupItemBearerSURes {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "gTP-TEID")]
        g_tp_teid: [GTPTEID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABSetupItemBearerSURes {
        pub fn new(
            e_rab_id: ERABID,
            transport_layer_address: TransportLayerAddress,
            g_tp_teid: GTPTEID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                transport_layer_address,
                g_tp_teid,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABSetupItemCtxtSURes")]
    #[non_exhaustive]
    pub struct ERABSetupItemCtxtSURes {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTP-TEID")]
        pub g_tp_teid: GTPTEID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABSetupItemCtxtSURes {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "gTP-TEID")]
        g_tp_teid: [GTPTEID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABSetupItemCtxtSURes {
        pub fn new(
            e_rab_id: ERABID,
            transport_layer_address: TransportLayerAddress,
            g_tp_teid: GTPTEID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                transport_layer_address,
                g_tp_teid,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABSetupListBearerSURes")]
    pub struct ERABSetupListBearerSURes(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABSetupListBearerSURes, 1, 256 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABSetupListCtxtSURes")]
    pub struct ERABSetupListCtxtSURes(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABSetupListCtxtSURes, 1, 256 }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB SETUP ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB Setup Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABSetupRequest")]
    #[non_exhaustive]
    pub struct ERABSetupRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ERABSetupRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ERABSetupRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " E-RAB Setup Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABSetupResponse")]
    #[non_exhaustive]
    pub struct ERABSetupResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ERABSetupResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ERABSetupResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "E-RABSubjecttoDataForwardingList"
    )]
    pub struct ERABSubjecttoDataForwardingList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABSubjecttoDataForwardingList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABToBeModifiedItemBearerModInd")]
    #[non_exhaustive]
    pub struct ERABToBeModifiedItemBearerModInd {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "dL-GTP-TEID")]
        pub d_l_gtp_teid: GTPTEID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABToBeModifiedItemBearerModInd {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "dL-GTP-TEID")]
        d_l_gtp_teid: [GTPTEID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABToBeModifiedItemBearerModInd {
        pub fn new(
            e_rab_id: ERABID,
            transport_layer_address: TransportLayerAddress,
            d_l_gtp_teid: GTPTEID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                transport_layer_address,
                d_l_gtp_teid,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABToBeModifiedItemBearerModReq")]
    #[non_exhaustive]
    pub struct ERABToBeModifiedItemBearerModReq {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "e-RABLevelQoSParameters")]
        pub e_rablevel_qo_sparameters: ERABLevelQoSParameters,
        #[rasn(identifier = "nAS-PDU")]
        pub n_as_pdu: NASPDU,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABToBeModifiedItemBearerModReq {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "e-RABLevelQoSParameters")]
        e_rablevel_qo_sparameters: [ERABLevelQoSParameters],
        #[rasn(identifier = "nAS-PDU")]
        n_as_pdu: [NASPDU],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABToBeModifiedItemBearerModReq {
        pub fn new(
            e_rab_id: ERABID,
            e_rablevel_qo_sparameters: ERABLevelQoSParameters,
            n_as_pdu: NASPDU,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                e_rablevel_qo_sparameters,
                n_as_pdu,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "E-RABToBeModifiedListBearerModInd"
    )]
    pub struct ERABToBeModifiedListBearerModInd(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABToBeModifiedListBearerModInd, 1, 256 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "E-RABToBeModifiedListBearerModReq"
    )]
    pub struct ERABToBeModifiedListBearerModReq(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABToBeModifiedListBearerModReq, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABToBeSetupItemBearerSUReq")]
    #[non_exhaustive]
    pub struct ERABToBeSetupItemBearerSUReq {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "e-RABlevelQoSParameters")]
        pub e_rablevel_qo_sparameters: ERABLevelQoSParameters,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTP-TEID")]
        pub g_tp_teid: GTPTEID,
        #[rasn(identifier = "nAS-PDU")]
        pub n_as_pdu: NASPDU,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABToBeSetupItemBearerSUReq {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "e-RABlevelQoSParameters")]
        e_rablevel_qo_sparameters: [ERABLevelQoSParameters],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "gTP-TEID")]
        g_tp_teid: [GTPTEID],
        #[rasn(identifier = "nAS-PDU")]
        n_as_pdu: [NASPDU],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABToBeSetupItemBearerSUReq {
        pub fn new(
            e_rab_id: ERABID,
            e_rablevel_qo_sparameters: ERABLevelQoSParameters,
            transport_layer_address: TransportLayerAddress,
            g_tp_teid: GTPTEID,
            n_as_pdu: NASPDU,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                e_rablevel_qo_sparameters,
                transport_layer_address,
                g_tp_teid,
                n_as_pdu,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABToBeSetupItemCtxtSUReq")]
    #[non_exhaustive]
    pub struct ERABToBeSetupItemCtxtSUReq {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "e-RABlevelQoSParameters")]
        pub e_rablevel_qo_sparameters: ERABLevelQoSParameters,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTP-TEID")]
        pub g_tp_teid: GTPTEID,
        #[rasn(identifier = "nAS-PDU")]
        pub n_as_pdu: Option<NASPDU>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABToBeSetupItemCtxtSUReq {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "e-RABlevelQoSParameters")]
        e_rablevel_qo_sparameters: [ERABLevelQoSParameters],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "gTP-TEID")]
        g_tp_teid: [GTPTEID],
        #[rasn(identifier = "nAS-PDU")]
        n_as_pdu: [Option<NASPDU>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABToBeSetupItemCtxtSUReq {
        pub fn new(
            e_rab_id: ERABID,
            e_rablevel_qo_sparameters: ERABLevelQoSParameters,
            transport_layer_address: TransportLayerAddress,
            g_tp_teid: GTPTEID,
            n_as_pdu: Option<NASPDU>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                e_rablevel_qo_sparameters,
                transport_layer_address,
                g_tp_teid,
                n_as_pdu,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABToBeSetupItemHOReq")]
    #[non_exhaustive]
    pub struct ERABToBeSetupItemHOReq {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTP-TEID")]
        pub g_tp_teid: GTPTEID,
        #[rasn(identifier = "e-RABlevelQosParameters")]
        pub e_rablevel_qos_parameters: ERABLevelQoSParameters,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABToBeSetupItemHOReq {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "gTP-TEID")]
        g_tp_teid: [GTPTEID],
        #[rasn(identifier = "e-RABlevelQosParameters")]
        e_rablevel_qos_parameters: [ERABLevelQoSParameters],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABToBeSetupItemHOReq {
        pub fn new(
            e_rab_id: ERABID,
            transport_layer_address: TransportLayerAddress,
            g_tp_teid: GTPTEID,
            e_rablevel_qos_parameters: ERABLevelQoSParameters,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                transport_layer_address,
                g_tp_teid,
                e_rablevel_qos_parameters,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "E-RABToBeSetupListBearerSUReq"
    )]
    pub struct ERABToBeSetupListBearerSUReq(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABToBeSetupListBearerSUReq, 1, 256 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABToBeSetupListCtxtSUReq")]
    pub struct ERABToBeSetupListCtxtSUReq(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABToBeSetupListCtxtSUReq, 1, 256 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABToBeSetupListHOReq")]
    pub struct ERABToBeSetupListHOReq(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABToBeSetupListHOReq, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABToBeSwitchedDLItem")]
    #[non_exhaustive]
    pub struct ERABToBeSwitchedDLItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTP-TEID")]
        pub g_tp_teid: GTPTEID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABToBeSwitchedDLItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "gTP-TEID")]
        g_tp_teid: [GTPTEID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABToBeSwitchedDLItem {
        pub fn new(
            e_rab_id: ERABID,
            transport_layer_address: TransportLayerAddress,
            g_tp_teid: GTPTEID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                transport_layer_address,
                g_tp_teid,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABToBeSwitchedDLList")]
    pub struct ERABToBeSwitchedDLList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABToBeSwitchedDLList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABToBeSwitchedULItem")]
    #[non_exhaustive]
    pub struct ERABToBeSwitchedULItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "transportLayerAddress")]
        pub transport_layer_address: TransportLayerAddress,
        #[rasn(identifier = "gTP-TEID")]
        pub g_tp_teid: GTPTEID,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABToBeSwitchedULItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "transportLayerAddress")]
        transport_layer_address: [TransportLayerAddress],
        #[rasn(identifier = "gTP-TEID")]
        g_tp_teid: [GTPTEID],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABToBeSwitchedULItem {
        pub fn new(
            e_rab_id: ERABID,
            transport_layer_address: TransportLayerAddress,
            g_tp_teid: GTPTEID,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                transport_layer_address,
                g_tp_teid,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABToBeSwitchedULList")]
    pub struct ERABToBeSwitchedULList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABToBeSwitchedULList, 1, 256 }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "E-RABToBeUpdatedItem")]
    #[non_exhaustive]
    pub struct ERABToBeUpdatedItem {
        #[rasn(identifier = "e-RAB-ID")]
        pub e_rab_id: ERABID,
        #[rasn(identifier = "securityIndication")]
        pub security_indication: Option<SecurityIndication>,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { ERABToBeUpdatedItem {
        #[rasn(identifier = "e-RAB-ID")]
        e_rab_id: [ERABID],
        #[rasn(identifier = "securityIndication")]
        security_indication: [Option<SecurityIndication>],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl ERABToBeUpdatedItem {
        pub fn new(
            e_rab_id: ERABID,
            security_indication: Option<SecurityIndication>,
            i_e_extensions: Option<ProtocolExtensionContainer>,
        ) -> Self {
            Self {
                e_rab_id,
                security_indication,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"), identifier = "E-RABToBeUpdatedList")]
    pub struct ERABToBeUpdatedList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { ERABToBeUpdatedList, 1, 256 }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB CP Relocation Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ENBCPRelocationIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ENBCPRelocationIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ENBCPRelocationIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB CONFIGURATION TRANSFER ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB Configuration Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ENBConfigurationTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ENBConfigurationTransfer {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ENBConfigurationTransfer {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " ENB CONFIGURATION UPDATE ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB Configuration Update"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ENBConfigurationUpdate {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ENBConfigurationUpdate {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ENBConfigurationUpdate {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB Configuration Update Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ENBConfigurationUpdateAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ENBConfigurationUpdateAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ENBConfigurationUpdateAcknowledge {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB Configuration Update Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ENBConfigurationUpdateFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ENBConfigurationUpdateFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ENBConfigurationUpdateFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB DIRECT INFORMATION TRANSFER ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB Direct Information Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ENBDirectInformationTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ENBDirectInformationTransfer {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ENBDirectInformationTransfer {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB EARLY STATUS TRANSFER ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB Early Status Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ENBEarlyStatusTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ENBEarlyStatusTransfer {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ENBEarlyStatusTransfer {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB STATUS TRANSFER ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " eNB Status Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ENBStatusTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ENBStatusTransfer {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ENBStatusTransfer {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " ERROR INDICATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Error Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ErrorIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ErrorIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ErrorIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " HANDOVER CANCEL ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Cancel"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverCancel {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverCancel {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverCancel {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Cancel Request Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverCancelAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverCancelAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverCancelAcknowledge {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Command"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverCommand {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverCommand {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverCommand {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " HANDOVER NOTIFICATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Notify"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverNotify {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverNotify {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverNotify {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Preparation Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverPreparationFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverPreparationFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverPreparationFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " HANDOVER RESOURCE ALLOCATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Request Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverRequestAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverRequestAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverRequestAcknowledge {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " HANDOVER PREPARATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Required"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverRequired {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverRequired {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverRequired {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " HANDOVER SUCCESS ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Handover Success"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct HandoverSuccess {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { HandoverSuccess {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl HandoverSuccess {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Initial Context Setup Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct InitialContextSetupFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { InitialContextSetupFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl InitialContextSetupFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " INITIAL CONTEXT SETUP ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Initial Context Setup Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct InitialContextSetupRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { InitialContextSetupRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl InitialContextSetupRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Initial Context Setup Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct InitialContextSetupResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { InitialContextSetupResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl InitialContextSetupResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " INITIAL UE MESSAGE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct InitialUEMessage {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { InitialUEMessage {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl InitialUEMessage {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        choice,
        automatic_tags,
        identifier = "Inter-SystemInformationTransferType"
    )]
    #[non_exhaustive]
    pub enum InterSystemInformationTransferType {
        rIMTransfer(RIMTransfer),
    }
    impl From<RIMTransfer> for InterSystemInformationTransferType {
        fn from(value: RIMTransfer) -> Self {
            Self::rIMTransfer(value)
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " KILL PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Kill Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct KillRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { KillRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl KillRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Kill Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct KillResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { KillResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl KillResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Location Report"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LocationReport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { LocationReport {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl LocationReport {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " LOCATION ELEMENTARY PROCEDURES"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Location Reporting Control"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LocationReportingControl {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { LocationReportingControl {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl LocationReportingControl {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Location Report Failure Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LocationReportingFailureIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { LocationReportingFailureIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl LocationReportingFailureIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME CP Relocation Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MMECPRelocationIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { MMECPRelocationIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl MMECPRelocationIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME CONFIGURATION TRANSFER ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME Configuration Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MMEConfigurationTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { MMEConfigurationTransfer {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl MMEConfigurationTransfer {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME CONFIGURATION UPDATE ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME Configuration Update"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MMEConfigurationUpdate {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { MMEConfigurationUpdate {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl MMEConfigurationUpdate {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME Configuration Update Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MMEConfigurationUpdateAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { MMEConfigurationUpdateAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl MMEConfigurationUpdateAcknowledge {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME Configuration Update Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MMEConfigurationUpdateFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { MMEConfigurationUpdateFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl MMEConfigurationUpdateFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME DIRECT INFORMATION TRANSFER ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME Direct Information Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MMEDirectInformationTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { MMEDirectInformationTransfer {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl MMEDirectInformationTransfer {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME EARLY STATUS TRANSFER ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME Early Status Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MMEEarlyStatusTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { MMEEarlyStatusTransfer {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl MMEEarlyStatusTransfer {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME STATUS TRANSFER ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " MME Status Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MMEStatusTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { MMEStatusTransfer {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl MMEStatusTransfer {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " NAS DELIVERY INDICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct NASDeliveryIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { NASDeliveryIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl NASDeliveryIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " NAS NON DELIVERY INDICATION"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct NASNonDeliveryIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { NASNonDeliveryIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl NASNonDeliveryIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " OVERLOAD ELEMENTARY PROCEDURES"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Overload Start"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct OverloadStart {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { OverloadStart {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl OverloadStart {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Overload Stop"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct OverloadStop {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { OverloadStop {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl OverloadStop {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " PWS Failure Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PWSFailureIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { PWSFailureIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl PWSFailureIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " PWS RESTART INDICATION PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " PWS Restart Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PWSRestartIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { PWSRestartIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl PWSRestartIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " PAGING ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Paging"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Paging {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { Paging {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl Paging {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " PATH SWITCH REQUEST ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Path Switch Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PathSwitchRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { PathSwitchRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl PathSwitchRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Path Switch Request Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PathSwitchRequestAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { PathSwitchRequestAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl PathSwitchRequestAcknowledge {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Path Switch Request Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PathSwitchRequestFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { PathSwitchRequestFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl PathSwitchRequestFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPrivateMessagePrivateIEs {
        pub id: PrivateIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl AnonymousPrivateMessagePrivateIEs {
        pub fn new(id: PrivateIEID, criticality: Criticality, value: Any) -> Self {
            Self {
                id,
                criticality,
                value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"))]
    pub struct PrivateMessagePrivateIEs(pub SequenceOf<AnonymousPrivateMessagePrivateIEs>);
    crate::per::sequence_of! { PrivateMessagePrivateIEs, 1, 65535 }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " PRIVATE MESSAGE ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Private Message"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PrivateMessage {
        #[rasn(identifier = "privateIEs")]
        pub private_ies: PrivateMessagePrivateIEs,
    }
    crate::per::decode_extensible_sequence! { PrivateMessage {
        #[rasn(identifier = "privateIEs")]
        private_ies: [PrivateMessagePrivateIEs],
    } }
    impl PrivateMessage {
        pub fn new(private_ies: PrivateMessagePrivateIEs) -> Self {
            Self { private_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " REROUTE NAS REQUEST"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RerouteNASRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { RerouteNASRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl RerouteNASRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " RESET ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Reset"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Reset {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { Reset {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl Reset {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Reset Acknowledge"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ResetAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { ResetAcknowledge {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl ResetAcknowledge {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ResetAll {
        #[rasn(identifier = "reset-all")]
        reset_all = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum ResetType {
        #[rasn(identifier = "s1-Interface")]
        s1_Interface(ResetAll),
        #[rasn(identifier = "partOfS1-Interface")]
        partOfS1_Interface(UEAssociatedLogicalS1ConnectionListRes),
    }
    impl From<ResetAll> for ResetType {
        fn from(value: ResetAll) -> Self {
            Self::s1_Interface(value)
        }
    }
    impl From<UEAssociatedLogicalS1ConnectionListRes> for ResetType {
        fn from(value: UEAssociatedLogicalS1ConnectionListRes) -> Self {
            Self::partOfS1_Interface(value)
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Retrieve UE Information"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RetrieveUEInformation {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { RetrieveUEInformation {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl RetrieveUEInformation {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " S1 Removal Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct S1RemovalFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { S1RemovalFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl S1RemovalFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " S1 REMOVAL ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " S1 Removal Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct S1RemovalRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { S1RemovalRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl S1RemovalRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " S1 Removal Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct S1RemovalResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { S1RemovalResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl S1RemovalResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " S1 Setup Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct S1SetupFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { S1SetupFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl S1SetupFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " S1 SETUP ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " S1 Setup Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct S1SetupRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { S1SetupRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl S1SetupRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " S1 Setup Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct S1SetupResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { S1SetupResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl S1SetupResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Secondary RAT Data Usage Report"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SecondaryRATDataUsageReport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { SecondaryRATDataUsageReport {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl SecondaryRATDataUsageReport {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TAIItem {
        #[rasn(identifier = "tAI")]
        pub t_ai: TAI,
        #[rasn(identifier = "iE-Extensions")]
        pub i_e_extensions: Option<ProtocolExtensionContainer>,
    }
    crate::per::decode_extensible_sequence! { TAIItem {
        #[rasn(identifier = "tAI")]
        t_ai: [TAI],
        #[rasn(identifier = "iE-Extensions")]
        i_e_extensions: [Option<ProtocolExtensionContainer>],
    } }
    impl TAIItem {
        pub fn new(t_ai: TAI, i_e_extensions: Option<ProtocolExtensionContainer>) -> Self {
            Self {
                t_ai,
                i_e_extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct TAIList(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { TAIList, 1, 256 }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Trace Failure Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TraceFailureIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { TraceFailureIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl TraceFailureIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " TRACE ELEMENTARY PROCEDURES"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Trace Start"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TraceStart {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { TraceStart {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl TraceStart {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "UE-associatedLogicalS1-ConnectionListRes"
    )]
    pub struct UEAssociatedLogicalS1ConnectionListRes(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { UEAssociatedLogicalS1ConnectionListRes, 1, 256 }
    #[derive(AsnType, Debug, Clone, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        size("1..=256"),
        identifier = "UE-associatedLogicalS1-ConnectionListResAck"
    )]
    pub struct UEAssociatedLogicalS1ConnectionListResAck(pub SequenceOf<ProtocolIEField>);
    crate::per::sequence_of! { UEAssociatedLogicalS1ConnectionListResAck, 1, 256 }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE CAPABILITY INFO INDICATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Capability Info Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UECapabilityInfoIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UECapabilityInfoIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UECapabilityInfoIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Modification Confirm"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextModificationConfirm {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextModificationConfirm {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextModificationConfirm {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Modification Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextModificationFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextModificationFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextModificationFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE CONTEXT MODIFICATION INDICATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Modification Indication"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextModificationIndication {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextModificationIndication {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextModificationIndication {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE CONTEXT MODIFICATION ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Modification Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextModificationRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextModificationRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextModificationRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Modification Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextModificationResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextModificationResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextModificationResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Release Command"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextReleaseCommand {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextReleaseCommand {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextReleaseCommand {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Release Complete"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextReleaseComplete {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextReleaseComplete {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextReleaseComplete {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE CONTEXT RELEASE ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Release Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextReleaseRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextReleaseRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextReleaseRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Resume Failure"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextResumeFailure {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextResumeFailure {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextResumeFailure {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE CONTEXT RESUME ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Resume Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextResumeRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextResumeRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextResumeRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Resume Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextResumeResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextResumeResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextResumeResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE CONTEXT SUSPEND ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Suspend Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextSuspendRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextSuspendRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextSuspendRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Context Suspend Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEContextSuspendResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEContextSuspendResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEContextSuspendResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = " UE Information Transfer"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UEInformationTransfer {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UEInformationTransfer {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UEInformationTransfer {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE RADIO CAPABILITY ID MAPPING PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Radio Capability ID Mapping Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UERadioCapabilityIDMappingRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UERadioCapabilityIDMappingRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UERadioCapabilityIDMappingRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Radio Capability ID Mapping Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UERadioCapabilityIDMappingResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UERadioCapabilityIDMappingResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UERadioCapabilityIDMappingResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE RADIO CAPABILITY MATCH ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Radio Capability Match Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UERadioCapabilityMatchRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UERadioCapabilityMatchRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UERadioCapabilityMatchRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UE Radio Capability Match Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UERadioCapabilityMatchResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UERadioCapabilityMatchResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UERadioCapabilityMatchResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UPLINK NAS TRANSPORT"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UplinkNASTransport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UplinkNASTransport {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UplinkNASTransport {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UPLINK NON UE ASSOCIATED LPPA TRANSPORT"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UplinkNonUEAssociatedLPPaTransport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UplinkNonUEAssociatedLPPaTransport {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UplinkNonUEAssociatedLPPaTransport {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UPLINK S1 CDMA2000 TUNNELLING ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Uplink S1 CDMA2000 Tunnelling"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UplinkS1cdma2000tunnelling {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UplinkS1cdma2000tunnelling {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UplinkS1cdma2000tunnelling {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " UPLINK UE ASSOCIATED LPPA TRANSPORT"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct UplinkUEAssociatedLPPaTransport {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { UplinkUEAssociatedLPPaTransport {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl UplinkUEAssociatedLPPaTransport {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " WRITE-REPLACE WARNING ELEMENTARY PROCEDURE"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Write-Replace Warning Request"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct WriteReplaceWarningRequest {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { WriteReplaceWarningRequest {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl WriteReplaceWarningRequest {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Write-Replace Warning Response"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct WriteReplaceWarningResponse {
        #[rasn(identifier = "protocolIEs")]
        pub protocol_ies: ProtocolIEContainer,
    }
    crate::per::decode_extensible_sequence! { WriteReplaceWarningResponse {
        #[rasn(identifier = "protocolIEs")]
        protocol_ies: [ProtocolIEContainer],
    } }
    impl WriteReplaceWarningResponse {
        pub fn new(protocol_ies: ProtocolIEContainer) -> Self {
            Self { protocol_ies }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod s1_ap_pdu_descriptions {
    extern crate alloc;
    use super::s1_ap_common_data_types::{Criticality, ProcedureCode};
    use super::s1_ap_constants::{
        ID_CELL_TRAFFIC_TRACE, ID_CONNECTION_ESTABLISHMENT_INDICATION, ID_DEACTIVATE_TRACE,
        ID_DOWNLINK_NASTRANSPORT, ID_DOWNLINK_NON_UEASSOCIATED_LPPA_TRANSPORT,
        ID_DOWNLINK_S1CDMA2000TUNNELLING, ID_DOWNLINK_UEASSOCIATED_LPPA_TRANSPORT,
        ID_E_NBCONFIGURATION_TRANSFER, ID_E_NBCPRELOCATION_INDICATION,
        ID_E_NBDIRECT_INFORMATION_TRANSFER, ID_E_NBEARLY_STATUS_TRANSFER, ID_E_NBSTATUS_TRANSFER,
        ID_E_RABMODIFICATION_INDICATION, ID_E_RABMODIFY, ID_E_RABRELEASE,
        ID_E_RABRELEASE_INDICATION, ID_E_RABSETUP, ID_ENBCONFIGURATION_UPDATE, ID_ERROR_INDICATION,
        ID_HANDOVER_CANCEL, ID_HANDOVER_NOTIFICATION, ID_HANDOVER_PREPARATION,
        ID_HANDOVER_RESOURCE_ALLOCATION, ID_HANDOVER_SUCCESS, ID_INITIAL_CONTEXT_SETUP,
        ID_INITIAL_UEMESSAGE, ID_KILL, ID_LOCATION_REPORT, ID_LOCATION_REPORTING_CONTROL,
        ID_LOCATION_REPORTING_FAILURE_INDICATION, ID_MMECONFIGURATION_TRANSFER,
        ID_MMECONFIGURATION_UPDATE, ID_MMECPRELOCATION_INDICATION,
        ID_MMEDIRECT_INFORMATION_TRANSFER, ID_MMEEARLY_STATUS_TRANSFER, ID_MMESTATUS_TRANSFER,
        ID_NASDELIVERY_INDICATION, ID_NASNON_DELIVERY_INDICATION, ID_OVERLOAD_START,
        ID_OVERLOAD_STOP, ID_PAGING, ID_PATH_SWITCH_REQUEST, ID_PRIVATE_MESSAGE,
        ID_PWSFAILURE_INDICATION, ID_PWSRESTART_INDICATION, ID_REROUTE_NASREQUEST, ID_RESET,
        ID_RETRIEVE_UEINFORMATION, ID_S1_REMOVAL, ID_S1_SETUP, ID_SECONDARY_RATDATA_USAGE_REPORT,
        ID_TRACE_FAILURE_INDICATION, ID_TRACE_START, ID_UECAPABILITY_INFO_INDICATION,
        ID_UECONTEXT_MODIFICATION, ID_UECONTEXT_MODIFICATION_INDICATION, ID_UECONTEXT_RELEASE,
        ID_UECONTEXT_RELEASE_REQUEST, ID_UECONTEXT_RESUME, ID_UECONTEXT_SUSPEND,
        ID_UEINFORMATION_TRANSFER, ID_UERADIO_CAPABILITY_IDMAPPING, ID_UERADIO_CAPABILITY_MATCH,
        ID_UPLINK_NASTRANSPORT, ID_UPLINK_NON_UEASSOCIATED_LPPA_TRANSPORT,
        ID_UPLINK_S1CDMA2000TUNNELLING, ID_UPLINK_UEASSOCIATED_LPPA_TRANSPORT,
        ID_WRITE_REPLACE_WARNING,
    };
    use super::s1_ap_pdu_contents::{
        CellTrafficTrace, ConnectionEstablishmentIndication, DeactivateTrace, DownlinkNASTransport,
        DownlinkNonUEAssociatedLPPaTransport, DownlinkS1cdma2000tunnelling,
        DownlinkUEAssociatedLPPaTransport, ENBCPRelocationIndication, ENBConfigurationTransfer,
        ENBConfigurationUpdate, ENBConfigurationUpdateAcknowledge, ENBConfigurationUpdateFailure,
        ENBDirectInformationTransfer, ENBEarlyStatusTransfer, ENBStatusTransfer,
        ERABModificationConfirm, ERABModificationIndication, ERABModifyRequest, ERABModifyResponse,
        ERABReleaseCommand, ERABReleaseIndication, ERABReleaseResponse, ERABSetupRequest,
        ERABSetupResponse, ErrorIndication, HandoverCancel, HandoverCancelAcknowledge,
        HandoverCommand, HandoverFailure, HandoverNotify, HandoverPreparationFailure,
        HandoverRequest, HandoverRequestAcknowledge, HandoverRequired, HandoverSuccess,
        InitialContextSetupFailure, InitialContextSetupRequest, InitialContextSetupResponse,
        InitialUEMessage, KillRequest, KillResponse, LocationReport, LocationReportingControl,
        LocationReportingFailureIndication, MMECPRelocationIndication, MMEConfigurationTransfer,
        MMEConfigurationUpdate, MMEConfigurationUpdateAcknowledge, MMEConfigurationUpdateFailure,
        MMEDirectInformationTransfer, MMEEarlyStatusTransfer, MMEStatusTransfer,
        NASDeliveryIndication, NASNonDeliveryIndication, OverloadStart, OverloadStop,
        PWSFailureIndication, PWSRestartIndication, Paging, PathSwitchRequest,
        PathSwitchRequestAcknowledge, PathSwitchRequestFailure, PrivateMessage, RerouteNASRequest,
        Reset, ResetAcknowledge, RetrieveUEInformation, S1RemovalFailure, S1RemovalRequest,
        S1RemovalResponse, S1SetupFailure, S1SetupRequest, S1SetupResponse,
        SecondaryRATDataUsageReport, TraceFailureIndication, TraceStart,
        UECapabilityInfoIndication, UEContextModificationConfirm, UEContextModificationFailure,
        UEContextModificationIndication, UEContextModificationRequest,
        UEContextModificationResponse, UEContextReleaseCommand, UEContextReleaseComplete,
        UEContextReleaseRequest, UEContextResumeFailure, UEContextResumeRequest,
        UEContextResumeResponse, UEContextSuspendRequest, UEContextSuspendResponse,
        UEInformationTransfer, UERadioCapabilityIDMappingRequest,
        UERadioCapabilityIDMappingResponse, UERadioCapabilityMatchRequest,
        UERadioCapabilityMatchResponse, UplinkNASTransport, UplinkNonUEAssociatedLPPaTransport,
        UplinkS1cdma2000tunnelling, UplinkUEAssociatedLPPaTransport, WriteReplaceWarningRequest,
        WriteReplaceWarningResponse,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct InitiatingMessage {
        #[rasn(identifier = "procedureCode")]
        pub procedure_code: ProcedureCode,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl InitiatingMessage {
        pub fn new(procedure_code: ProcedureCode, criticality: Criticality, value: Any) -> Self {
            Self {
                procedure_code,
                criticality,
                value,
            }
        }
    }
    #[doc = " **************************************************************"]
    #[doc = ""]
    #[doc = " Interface PDU Definition"]
    #[doc = ""]
    #[doc = " **************************************************************"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags, identifier = "S1AP-PDU")]
    #[non_exhaustive]
    pub enum S1APPDU {
        initiatingMessage(InitiatingMessage),
        successfulOutcome(SuccessfulOutcome),
        unsuccessfulOutcome(UnsuccessfulOutcome),
    }
    impl From<InitiatingMessage> for S1APPDU {
        fn from(value: InitiatingMessage) -> Self {
            Self::initiatingMessage(value)
        }
    }
    impl From<SuccessfulOutcome> for S1APPDU {
        fn from(value: SuccessfulOutcome) -> Self {
            Self::successfulOutcome(value)
        }
    }
    impl From<UnsuccessfulOutcome> for S1APPDU {
        fn from(value: UnsuccessfulOutcome) -> Self {
            Self::unsuccessfulOutcome(value)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SuccessfulOutcome {
        #[rasn(identifier = "procedureCode")]
        pub procedure_code: ProcedureCode,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl SuccessfulOutcome {
        pub fn new(procedure_code: ProcedureCode, criticality: Criticality, value: Any) -> Self {
            Self {
                procedure_code,
                criticality,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct UnsuccessfulOutcome {
        #[rasn(identifier = "procedureCode")]
        pub procedure_code: ProcedureCode,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl UnsuccessfulOutcome {
        pub fn new(procedure_code: ProcedureCode, criticality: Criticality, value: Any) -> Self {
            Self {
                procedure_code,
                criticality,
                value,
            }
        }
    }
}

// Auto-generated S1AP compatibility surface.
pub use s1_ap_common_data_types::*;
pub use s1_ap_constants::*;
pub use s1_ap_containers::*;
pub use s1_ap_ies::*;
pub use s1_ap_pdu_contents::*;
pub use s1_ap_pdu_descriptions::*;
#[allow(non_camel_case_types)]
pub type S1AP_PDU = S1APPDU;
#[allow(non_camel_case_types)]
pub type MME_UE_S1AP_ID = MMEUES1APID;
#[allow(non_camel_case_types)]
pub type ENB_UE_S1AP_ID = ENBUES1APID;
use rasn::prelude::{
    BitString, FixedBitString, FixedOctetString, Integer, OctetString, PrintableString, SequenceOf,
    VisibleString,
};

// Auto-generated newtype conversions used by the builder macros.
impl From<FixedBitString<32usize>> for AdditionalRRMPriorityIndex {
    fn from(value: FixedBitString<32usize>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<PLMNidentity>> for BPLMNs {
    fn from(value: SequenceOf<PLMNidentity>) -> Self {
        Self(value)
    }
}
impl From<u64> for BitRate {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<BluetoothName>> for BluetoothMeasConfigNameList {
    fn from(value: SequenceOf<BluetoothName>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for BluetoothName {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for BluetoothName {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for CELevel {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for CELevel {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<FixedOctetString<2usize>> for CI {
    fn from(value: FixedOctetString<2usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 2usize]> for CI {
    fn from(value: [u8; 2usize]) -> Self {
        Self(value.into())
    }
}
impl From<SequenceOf<CNTypeRestrictionsItem>> for CNTypeRestrictions {
    fn from(value: SequenceOf<CNTypeRestrictionsItem>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<27usize>> for CSGId {
    fn from(value: FixedBitString<27usize>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<CSGIdListItem>> for CSGIdList {
    fn from(value: SequenceOf<CSGIdListItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<CancelledCellinEAIItem>> for CancelledCellinEAI {
    fn from(value: SequenceOf<CancelledCellinEAIItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<CancelledCellinTAIItem>> for CancelledCellinTAI {
    fn from(value: SequenceOf<CancelledCellinTAIItem>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for Cdma2000OneXMEID {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for Cdma2000OneXMEID {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for Cdma2000OneXMSI {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for Cdma2000OneXMSI {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for Cdma2000OneXPilot {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for Cdma2000OneXPilot {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for Cdma2000OneXRAND {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for Cdma2000OneXRAND {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for Cdma2000PDU {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for Cdma2000PDU {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for Cdma2000SectorID {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for Cdma2000SectorID {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<SequenceOf<CellIDBroadcastItem>> for CellIDBroadcast {
    fn from(value: SequenceOf<CellIDBroadcastItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<CellIDCancelledItem>> for CellIDCancelled {
    fn from(value: SequenceOf<CellIDCancelledItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<EUTRANCGI>> for CellIdListforMDT {
    fn from(value: SequenceOf<EUTRANCGI>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<EUTRANCGI>> for CellIdListforQMC {
    fn from(value: SequenceOf<EUTRANCGI>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<28usize>> for CellIdentity {
    fn from(value: FixedBitString<28usize>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for CoarseUELocation {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for CoarseUELocation {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<SequenceOf<CompletedCellinEAIItem>> for CompletedCellinEAI {
    fn from(value: SequenceOf<CompletedCellinEAIItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<CompletedCellinTAIItem>> for CompletedCellinTAI {
    fn from(value: SequenceOf<CompletedCellinTAIItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ConnectedengNBItem>> for ConnectedengNBList {
    fn from(value: SequenceOf<ConnectedengNBItem>) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<4usize>> for CorrelationID {
    fn from(value: FixedOctetString<4usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 4usize]> for CorrelationID {
    fn from(value: [u8; 4usize]) -> Self {
        Self(value.into())
    }
}
impl From<SequenceOf<CriticalityDiagnosticsIEItem>> for CriticalityDiagnosticsIEList {
    fn from(value: SequenceOf<CriticalityDiagnosticsIEItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for DAPSResponseInfoList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<u16> for DCNID {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<16usize>> for DLNASMAC {
    fn from(value: FixedBitString<16usize>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<8usize>> for DataCodingScheme {
    fn from(value: FixedBitString<8usize>) -> Self {
        Self(value)
    }
}
impl From<Integer> for DataSize {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<Integer> for EARFCN {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<EUTRANCGI>> for ECGIList {
    fn from(value: SequenceOf<EUTRANCGI>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<EUTRANCGI>> for ECGIListForRestart {
    fn from(value: SequenceOf<EUTRANCGI>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<EUTRANCGI>> for ECGI_List {
    fn from(value: SequenceOf<EUTRANCGI>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TransportLayerAddress>> for ENBIndirectX2TransportLayerAddresses {
    fn from(value: SequenceOf<TransportLayerAddress>) -> Self {
        Self(value)
    }
}
impl From<u32> for ENBUES1APID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ENBX2ExtTLA>> for ENBX2ExtTLAs {
    fn from(value: SequenceOf<ENBX2ExtTLA>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TransportLayerAddress>> for ENBX2GTPTLAs {
    fn from(value: SequenceOf<TransportLayerAddress>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TransportLayerAddress>> for ENBX2TLAs {
    fn from(value: SequenceOf<TransportLayerAddress>) -> Self {
        Self(value)
    }
}
impl From<PrintableString> for ENBname {
    fn from(value: PrintableString) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<PLMNidentity>> for EPLMNs {
    fn from(value: SequenceOf<PLMNidentity>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABAdmittedList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<Integer> for ERABID {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABInformationList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABModifyListBearerModConf {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABModifyListBearerModRes {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABReleaseListBearerRelComp {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABSecurityResultList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABSetupListBearerSURes {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABSetupListCtxtSURes {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABToBeSetupListBearerSUReq {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABToBeSetupListCtxtSUReq {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABToBeSetupListHOReq {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABToBeSwitchedDLList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABToBeSwitchedULList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABToBeUpdatedList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ERABUsageReportList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<u16> for EUTRANRoundTripDelayEstimationInfo {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<8usize>> for EUTRANTraceID {
    fn from(value: FixedOctetString<8usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 8usize]> for EUTRANTraceID {
    fn from(value: [u8; 8usize]) -> Self {
        Self(value.into())
    }
}
impl From<FixedOctetString<3usize>> for EmergencyAreaID {
    fn from(value: FixedOctetString<3usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 3usize]> for EmergencyAreaID {
    fn from(value: [u8; 3usize]) -> Self {
        Self(value.into())
    }
}
impl From<SequenceOf<EmergencyAreaIDBroadcastItem>> for EmergencyAreaIDBroadcast {
    fn from(value: SequenceOf<EmergencyAreaIDBroadcastItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<EmergencyAreaIDCancelledItem>> for EmergencyAreaIDCancelled {
    fn from(value: SequenceOf<EmergencyAreaIDCancelledItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<EmergencyAreaID>> for EmergencyAreaIDList {
    fn from(value: SequenceOf<EmergencyAreaID>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<EmergencyAreaID>> for EmergencyAreaIDListForRestart {
    fn from(value: SequenceOf<EmergencyAreaID>) -> Self {
        Self(value)
    }
}
impl From<BitString> for EnGNBID {
    fn from(value: BitString) -> Self {
        Self(value)
    }
}
impl From<BitString> for EncryptionAlgorithms {
    fn from(value: BitString) -> Self {
        Self(value)
    }
}
impl From<Integer> for ExpectedActivityPeriod {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<Integer> for ExpectedIdlePeriod {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<Integer> for ExtendedBitRate {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<u16> for ExtendedRNCID {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u32> for ExtendedRepetitionPeriod {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<14usize>> for ExtendedUEIdentityIndexValue {
    fn from(value: FixedBitString<14usize>) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<3usize>> for FiveGSTAC {
    fn from(value: FixedOctetString<3usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 3usize]> for FiveGSTAC {
    fn from(value: [u8; 3usize]) -> Self {
        Self(value.into())
    }
}
impl From<Integer> for FiveQI {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<LAC>> for ForbiddenLACs {
    fn from(value: SequenceOf<LAC>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ForbiddenLAsItem>> for ForbiddenLAs {
    fn from(value: SequenceOf<ForbiddenLAsItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TAC>> for ForbiddenTACs {
    fn from(value: SequenceOf<TAC>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ForbiddenTAsItem>> for ForbiddenTAs {
    fn from(value: SequenceOf<ForbiddenTAsItem>) -> Self {
        Self(value)
    }
}
impl From<BitString> for GNBID {
    fn from(value: BitString) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<4usize>> for GTPTEID {
    fn from(value: FixedOctetString<4usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 4usize]> for GTPTEID {
    fn from(value: [u8; 4usize]) -> Self {
        Self(value.into())
    }
}
impl From<SequenceOf<GUMMEI>> for GUMMEIList {
    fn from(value: SequenceOf<GUMMEI>) -> Self {
        Self(value)
    }
}
impl From<u32> for HFN {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<u32> for HFNModified {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<u16> for HFNforPDCPSNlength18 {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u16> for HandoverWindowDuration {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u32> for HandoverWindowStart {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<u8> for Hysteresis {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<OctetString> for IMSI {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for IMSI {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<BitString> for IntegrityProtectionAlgorithms {
    fn from(value: BitString) -> Self {
        Self(value)
    }
}
impl From<Integer> for IntendedNumberOfPagingAttempts {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<InterSystemMeasurementItem>> for InterSystemMeasurementList {
    fn from(value: SequenceOf<InterSystemMeasurementItem>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<8usize>> for InterfacesToTrace {
    fn from(value: FixedBitString<8usize>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for IntersystemSONConfigurationTransfer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for IntersystemSONConfigurationTransfer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for L3Information {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for L3Information {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<FixedOctetString<2usize>> for LAC {
    fn from(value: FixedOctetString<2usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 2usize]> for LAC {
    fn from(value: [u8; 2usize]) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for LHNID {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for LHNID {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for LPPaPDU {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for LPPaPDU {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for LastVisitedNGRANCellInformation {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for LastVisitedNGRANCellInformation {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<SequenceOf<LastVisitedPSCellInformation>> for LastVisitedPSCellList {
    fn from(value: SequenceOf<LastVisitedPSCellInformation>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for LastVisitedUTRANCellInformation {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for LastVisitedUTRANCellInformation {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<Integer> for M7period {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<MBSFNResultToLogInfo>> for MBSFNResultToLog {
    fn from(value: SequenceOf<MBSFNResultToLogInfo>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for MDTConfigurationNR {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for MDTConfigurationNR {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<FixedBitString<8usize>> for MDTLocationInfo {
    fn from(value: FixedBitString<8usize>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<PLMNidentity>> for MDTPLMNList {
    fn from(value: SequenceOf<PLMNidentity>) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<1usize>> for MMECode {
    fn from(value: FixedOctetString<1usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 1usize]> for MMECode {
    fn from(value: [u8; 1usize]) -> Self {
        Self(value.into())
    }
}
impl From<FixedOctetString<2usize>> for MMEGroupID {
    fn from(value: FixedOctetString<2usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 2usize]> for MMEGroupID {
    fn from(value: [u8; 2usize]) -> Self {
        Self(value.into())
    }
}
impl From<u32> for MMEUES1APID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<PrintableString> for MMEname {
    fn from(value: PrintableString) -> Self {
        Self(value)
    }
}
impl From<OctetString> for MSClassmark2 {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for MSClassmark2 {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for MSClassmark3 {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for MSClassmark3 {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<FixedOctetString<4usize>> for MTMSI {
    fn from(value: FixedOctetString<4usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 4usize]> for MTMSI {
    fn from(value: [u8; 4usize]) -> Self {
        Self(value.into())
    }
}
impl From<FixedBitString<64usize>> for MaskedIMEISV {
    fn from(value: FixedBitString<64usize>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<8usize>> for MeasurementsToActivate {
    fn from(value: FixedBitString<8usize>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<16usize>> for MessageIdentifier {
    fn from(value: FixedBitString<16usize>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<32usize>> for MobilityInformation {
    fn from(value: FixedBitString<32usize>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for NASPDU {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for NASPDU {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for NASSecurityParametersfromEUTRAN {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for NASSecurityParametersfromEUTRAN {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for NASSecurityParameterstoEUTRAN {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for NASSecurityParameterstoEUTRAN {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for NBIoTRLFReportContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for NBIoTRLFReportContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<FixedBitString<12usize>> for NBIoTUEIdentityIndexValue {
    fn from(value: FixedBitString<12usize>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<36usize>> for NRCellIdentity {
    fn from(value: FixedBitString<36usize>) -> Self {
        Self(value)
    }
}
impl From<BitString> for NRencryptionAlgorithms {
    fn from(value: BitString) -> Self {
        Self(value)
    }
}
impl From<BitString> for NRintegrityProtectionAlgorithms {
    fn from(value: BitString) -> Self {
        Self(value)
    }
}
impl From<u16> for NumberOfBroadcasts {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u16> for NumberofBroadcastRequest {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<OctetString> for OldBSSToNewBSSInformation {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for OldBSSToNewBSSInformation {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<SequenceOf<PC5QoSFlowItem>> for PC5QoSFlowList {
    fn from(value: SequenceOf<PC5QoSFlowItem>) -> Self {
        Self(value)
    }
}
impl From<u16> for PDCPSN {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u16> for PDCPSNExtended {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u32> for PDCPSNlength18 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<PLMNidentity>> for PLMNListforQMC {
    fn from(value: SequenceOf<PLMNidentity>) -> Self {
        Self(value)
    }
}
impl From<TBCDSTRING> for PLMNidentity {
    fn from(value: TBCDSTRING) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<EUTRANCGI>> for PWSfailedECGIList {
    fn from(value: SequenceOf<EUTRANCGI>) -> Self {
        Self(value)
    }
}
impl From<u16> for PacketLossRate {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<Integer> for PagingAttemptCount {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<2usize>> for PortNumber {
    fn from(value: FixedOctetString<2usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 2usize]> for PortNumber {
    fn from(value: [u8; 2usize]) -> Self {
        Self(value.into())
    }
}
impl From<u8> for PriorityLevel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<AnonymousPrivateMessagePrivateIEs>> for PrivateMessagePrivateIEs {
    fn from(value: SequenceOf<AnonymousPrivateMessagePrivateIEs>) -> Self {
        Self(value)
    }
}
impl From<u8> for ProcedureCode {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolExtensionField>> for ProtocolExtensionContainer {
    fn from(value: SequenceOf<ProtocolExtensionField>) -> Self {
        Self(value)
    }
}
impl From<u16> for ProtocolExtensionID {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for ProtocolIEContainer {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<u16> for ProtocolIEID {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u8> for QCI {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<1usize>> for RAC {
    fn from(value: FixedOctetString<1usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 1usize]> for RAC {
    fn from(value: [u8; 1usize]) -> Self {
        Self(value.into())
    }
}
impl From<u32> for RANUENGAPID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<RATRestrictionsItem>> for RATRestrictions {
    fn from(value: SequenceOf<RATRestrictionsItem>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for RIMInformation {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for RIMInformation {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<u16> for RNCID {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<OctetString> for RRCContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for RRCContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<BitString> for ReceiveStatusOfULPDCPSDUsExtended {
    fn from(value: BitString) -> Self {
        Self(value)
    }
}
impl From<BitString> for ReceiveStatusOfULPDCPSDUsPDCPSNlength18 {
    fn from(value: BitString) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<4096usize>> for ReceiveStatusofULPDCPSDUs {
    fn from(value: FixedBitString<4096usize>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for RecommendedCellList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for RecommendedENBList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<u8> for RelativeMMECapacity {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<u16> for RepetitionPeriod {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u8> for RoutingID {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<256usize>> for SecurityKey {
    fn from(value: FixedBitString<256usize>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<SensorMeasConfigNameItem>> for SensorMeasConfigNameList {
    fn from(value: SequenceOf<SensorMeasConfigNameItem>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<16usize>> for SerialNumber {
    fn from(value: FixedBitString<16usize>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ServedDCNsItem>> for ServedDCNs {
    fn from(value: SequenceOf<ServedDCNsItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ServedGUMMEIsItem>> for ServedGUMMEIs {
    fn from(value: SequenceOf<ServedGUMMEIsItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<MMEGroupID>> for ServedGroupIDs {
    fn from(value: SequenceOf<MMEGroupID>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<MMECode>> for ServedMMECs {
    fn from(value: SequenceOf<MMECode>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<PLMNidentity>> for ServedPLMNs {
    fn from(value: SequenceOf<PLMNidentity>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for SourceBSSToTargetBSSTransparentContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for SourceBSSToTargetBSSTransparentContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for SourceNgRanNodeToTargetNgRanNodeTransparentContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for SourceNgRanNodeToTargetNgRanNodeTransparentContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for SourceRNCToTargetRNCTransparentContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for SourceRNCToTargetRNCTransparentContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for SourceToTargetTransparentContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for SourceToTargetTransparentContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<Integer> for StratumLevel {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}
impl From<u16> for SubscriberProfileIDforRFP {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<SupportedTAsItem>> for SupportedTAs {
    fn from(value: SequenceOf<SupportedTAsItem>) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<2usize>> for TAC {
    fn from(value: FixedOctetString<2usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 2usize]> for TAC {
    fn from(value: [u8; 2usize]) -> Self {
        Self(value.into())
    }
}
impl From<SequenceOf<TAC>> for TACListInLTENTN {
    fn from(value: SequenceOf<TAC>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TAIBroadcastItem>> for TAIBroadcast {
    fn from(value: SequenceOf<TAIBroadcastItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TAICancelledItem>> for TAICancelled {
    fn from(value: SequenceOf<TAICancelledItem>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<ProtocolIEField>> for TAIList {
    fn from(value: SequenceOf<ProtocolIEField>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TAI>> for TAIListForRestart {
    fn from(value: SequenceOf<TAI>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TAI>> for TAIListforMDT {
    fn from(value: SequenceOf<TAI>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TAI>> for TAIListforQMC {
    fn from(value: SequenceOf<TAI>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TAI>> for TAIListforWarning {
    fn from(value: SequenceOf<TAI>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TAC>> for TAListforMDT {
    fn from(value: SequenceOf<TAC>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<TAC>> for TAListforQMC {
    fn from(value: SequenceOf<TAC>) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<3usize>> for TBCDSTRING {
    fn from(value: FixedOctetString<3usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 3usize]> for TBCDSTRING {
    fn from(value: [u8; 3usize]) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for TargetBSSToSourceBSSTransparentContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for TargetBSSToSourceBSSTransparentContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for TargetNgRanNodeToSourceNgRanNodeTransparentContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for TargetNgRanNodeToSourceNgRanNodeTransparentContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for TargetRNCToSourceRNCTransparentContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for TargetRNCToSourceRNCTransparentContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for TargetToSourceTransparentContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for TargetToSourceTransparentContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<u8> for ThresholdRSRP {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<u8> for ThresholdRSRQ {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<FixedOctetString<4usize>> for TimeSinceSecondaryNodeRelease {
    fn from(value: FixedOctetString<4usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 4usize]> for TimeSinceSecondaryNodeRelease {
    fn from(value: [u8; 4usize]) -> Self {
        Self(value.into())
    }
}
impl From<u16> for TimeUEStayedInCell {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u16> for TimeUEStayedInCellEnhancedGranularity {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<u8> for TrafficLoadReductionIndication {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<BitString> for TransportLayerAddress {
    fn from(value: BitString) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<8usize>> for UEApplicationLayerMeasurementCapability {
    fn from(value: FixedBitString<8usize>) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<LastVisitedCellItem>> for UEHistoryInformation {
    fn from(value: SequenceOf<LastVisitedCellItem>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for UEHistoryInformationFromTheUE {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for UEHistoryInformationFromTheUE {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<FixedBitString<10usize>> for UEIdentityIndexValue {
    fn from(value: FixedBitString<10usize>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for UERLFReportContainer {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for UERLFReportContainer {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for UERLFReportContainerForExtendedBands {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for UERLFReportContainerForExtendedBands {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for UERadioCapability {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for UERadioCapability {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for UERadioCapabilityForPaging {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for UERadioCapabilityForPaging {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for UERadioCapabilityID {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for UERadioCapabilityID {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<u8> for UEUsageType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<5usize>> for ULNASCount {
    fn from(value: FixedBitString<5usize>) -> Self {
        Self(value)
    }
}
impl From<FixedBitString<16usize>> for ULNASMAC {
    fn from(value: FixedBitString<16usize>) -> Self {
        Self(value)
    }
}
impl From<VisibleString> for URIAddress {
    fn from(value: VisibleString) -> Self {
        Self(value)
    }
}
impl From<SequenceOf<WLANName>> for WLANMeasConfigNameList {
    fn from(value: SequenceOf<WLANName>) -> Self {
        Self(value)
    }
}
impl From<OctetString> for WLANName {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for WLANName {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for WarningAreaCoordinates {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for WarningAreaCoordinates {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<OctetString> for WarningMessageContents {
    fn from(value: OctetString) -> Self {
        Self(value)
    }
}
impl From<Vec<u8>> for WarningMessageContents {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}
impl From<FixedOctetString<50usize>> for WarningSecurityInfo {
    fn from(value: FixedOctetString<50usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 50usize]> for WarningSecurityInfo {
    fn from(value: [u8; 50usize]) -> Self {
        Self(value.into())
    }
}
impl From<FixedOctetString<2usize>> for WarningType {
    fn from(value: FixedOctetString<2usize>) -> Self {
        Self(value)
    }
}
impl From<[u8; 2usize]> for WarningType {
    fn from(value: [u8; 2usize]) -> Self {
        Self(value.into())
    }
}
pub use crate::registry::S1apPduKind;
impl S1APPDU {
    /// Encode this PDU using Aligned PER.
    pub fn encode(&self) -> Result<Vec<u8>, rasn::error::EncodeError> {
        rasn::aper::encode(self)
    }
    /// Decode an S1AP PDU from Aligned PER bytes.
    pub fn decode(bytes: &[u8]) -> Result<Self, rasn::error::DecodeError> {
        decode_complete(bytes)
    }
    /// Decode the typed message held by this PDU's open type.
    pub fn decode_value<T: rasn::Decode>(&self) -> Result<T, rasn::error::DecodeError> {
        let value = match self {
            S1APPDU::initiatingMessage(message) => &message.value,
            S1APPDU::successfulOutcome(message) => &message.value,
            S1APPDU::unsuccessfulOutcome(message) => &message.value,
        };
        decode_open_type(value)
    }
    /// Return the procedure code of this PDU.
    pub fn procedure_code(&self) -> u8 {
        match self {
            S1APPDU::initiatingMessage(message) => message.procedure_code.0,
            S1APPDU::successfulOutcome(message) => message.procedure_code.0,
            S1APPDU::unsuccessfulOutcome(message) => message.procedure_code.0,
        }
    }
    /// Return the PDU direction as a human-readable string.
    pub fn direction(&self) -> &'static str {
        match self {
            S1APPDU::initiatingMessage(_) => "InitiatingMessage",
            S1APPDU::successfulOutcome(_) => "SuccessfulOutcome",
            S1APPDU::unsuccessfulOutcome(_) => "UnsuccessfulOutcome",
        }
    }
    /// Returns `true` for an initiating message.
    pub fn is_initiating(&self) -> bool {
        matches!(self, S1APPDU::initiatingMessage(_))
    }
    /// Returns `true` for a successful outcome.
    pub fn is_successful(&self) -> bool {
        matches!(self, S1APPDU::successfulOutcome(_))
    }
    /// Returns `true` for an unsuccessful outcome.
    pub fn is_unsuccessful(&self) -> bool {
        matches!(self, S1APPDU::unsuccessfulOutcome(_))
    }
}
impl std::fmt::Display for S1APPDU {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} {} (code={})",
            self.direction(),
            self.procedure_name(),
            self.procedure_code()
        )
    }
}
/// Encode a typed ASN.1 value for an S1AP open type using APER.
///
/// An open type holds a complete encoding, in which an empty encoding
/// becomes one zero octet (X.691 (07/2002) §10.1.4, §10.2.1).
pub fn encode_open_type<T: rasn::Encode>(
    value: &T,
) -> Result<rasn::types::Any, rasn::error::EncodeError> {
    let mut bytes = rasn::aper::encode(value)?;
    if bytes.is_empty() {
        bytes.push(0);
    }
    Ok(rasn::types::Any::new(bytes))
}
/// Decode a typed ASN.1 value from an S1AP open type using APER.
pub fn decode_open_type<T: rasn::Decode>(
    value: &rasn::types::Any,
) -> Result<T, rasn::error::DecodeError> {
    decode_complete(value.as_bytes())
}
fn decode_complete<T: rasn::Decode>(bytes: &[u8]) -> Result<T, rasn::error::DecodeError> {
    if bytes.is_empty() {
        return Err(<rasn::error::DecodeError as rasn::de::Error>::custom(
            "APER value must contain a complete encoding",
            rasn::Codec::Aper,
        ));
    }
    let (decoded, remainder) = rasn::aper::decode_with_remainder(bytes)?;
    // A zero-bit field-list has exactly one zero octet as its complete encoding.
    let zero_bit_encoding = bytes == [0] && remainder == bytes;
    if !remainder.is_empty() && !zero_bit_encoding {
        return Err(<rasn::error::DecodeError as rasn::de::Error>::custom(
            "APER complete encoding has trailing whole octets",
            rasn::Codec::Aper,
        ));
    }
    Ok(decoded)
}
