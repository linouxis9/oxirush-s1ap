// The Rust blocks of README.md, each as a function, with what the README shows under
// each: `../oxirush-aper/protocols/tests/readme.py` writes this file from the README and
// says how. Do not edit it.
#![cfg(feature = "serde")]
#![allow(unused)]

use std::cell::RefCell;

thread_local!(static PRINTED: RefCell<String> = const { RefCell::new(String::new()) });

// What a block prints is kept, to be compared with what the README shows. The line
// is written before it is kept, so that `?` in what is printed returns from the block.
macro_rules! print {
    ($($argument:tt)*) => {{
        let text = format!($($argument)*);
        PRINTED.with(|printed| printed.borrow_mut().push_str(&text))
    }};
}

macro_rules! println {
    () => {
        print!("\n")
    };
    ($($argument:tt)*) => {{
        let line = format!($($argument)*);
        print!("{line}\n")
    }};
}

/// What `block` prints.
fn printed(block: fn()) -> String {
    PRINTED.with(|printed| printed.borrow_mut().clear());
    block();
    PRINTED.with(|printed| printed.take())
}

#[test]
fn each_block_prints_what_the_readme_shows() {
    let mut differences = String::new();
    for (rank, (block, shown)) in (1..).zip(BLOCKS) {
        let printed = printed(*block);
        if printed != *shown {
            differences += &format!("the Rust block {rank} of README.md prints:\n{printed}");
            differences += &format!("and the README shows:\n{shown}\n");
        }
    }
    assert!(differences.is_empty(), "{differences}");
}

/// Each Rust block of README.md, by its rank, and what the README shows under it.
#[rustfmt::skip]
const BLOCKS: &[(fn(), &str)] = &[
    (block_1, r#"UEContextReleaseRequest
MME=Some(42) eNB=Some(7) cause=Some(radioNetwork(user_inactivity))
"#),
    (block_2, r#"S1 SETUP REQUEST from enb-1: 1 tracking area
S1 SETUP RESPONSE 20110022000003003d400702006d6d652d310069000b000002f83900000001000100574001ff
INITIAL UE MESSAGE of eNB UE 7: NAS-PDU 07410108298039000000001002e0e000040201d031
DOWNLINK NAS TRANSPORT 000b4038000003000000020001000800020007001a00252407520023553cbe9637a89d218ae64dae47bf3510aa689c648350b9b9a4a8043ac07aa7e0
INITIAL CONTEXT SETUP REQUEST, 0 deviations: KeNB cd49a4ae7001cf3c810a89f47ced1b130f7b3f9b42bc9707431b3fb4e62b20fc
"#),
    (block_3, r#"a release request with 3 IEs
Unsuccessful_S1Setup is not handled here
"#),
    (block_4, r#"ignore 000c4011000002000800020007001a000403074171
true
TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no criticality: write its criticality
"#),
    (block_5, r#""#),
    (block_6, r#"MME=42 eNB=7
radioNetwork(user_inactivity)
"#),
    (block_7, r#"E-RAB 5 with QCI 9
E-RAB 5
EUTRANCGI { plmn_identity: PLMNidentity(208-93), cell_id: CellIdentity(0x1234/28), ie_extensions: None }
"#),
    (block_8, r#"the criticality of the PDU is reject, and that of its procedure is ignore
MME-UE-S1AP-ID (IE 0), at position 1, comes after eNB-UE-S1AP-ID (IE 8), which the object set of the message has after it
the criticality of MME-UE-S1AP-ID (IE 0), at position 1, is ignore, and the message assigns it reject
TimeToWait (IE 65), at position 2, is not an IE of the message
eNB-UE-S1AP-ID (IE 8), at position 3, is in the message already, at position 0
Cause (IE 2) is mandatory, and the message does not have it
"#),
    (block_9, r#"Global-ENB-ID (IE 59) is mandatory, and the message does not have it
SupportedTAs (IE 64) is mandatory, and the message does not have it
401100150000020002400131003a4009180100003b40004040
"#),
    (block_10, r#"{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
/procedure_code = 18
/direction = "InitiatingMessage"
/criticality = "ignore"
/s1ap/MME-UE-S1AP-ID/id = 0
/s1ap/MME-UE-S1AP-ID/criticality = "reject"
/s1ap/MME-UE-S1AP-ID/value = 42
/s1ap/eNB-UE-S1AP-ID/id = 8
/s1ap/eNB-UE-S1AP-ID/criticality = "reject"
/s1ap/eNB-UE-S1AP-ID/value = 7
/s1ap/Cause/id = 2
/s1ap/Cause/criticality = "ignore"
/s1ap/Cause/value/radioNetwork = "user-inactivity"
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":9},{"Cause":{"nas":"normal-release"}}]}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc; an IE itself has id, criticality, value, octets, at /s1ap/Cause/radioNetwrok
"#),
    (block_11, r#"0012402000000400000002002a000800020007000240020280003c40070200656e622d31
"InitiatingMessage" 18 "ignore"
["reject","reject","ignore","ignore"]
"#),
    (block_12, r#"0012401000000200000002002afde80003c0ffee
{"id":65000,"criticality":"reject","value":"c0ffee","_decode_error":"S1AP IE 65000 is unknown","_raw_value":"c0ffee","_original_id":65000}
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":255},{"65000":{"octets":"c0ffee"},"criticality":"reject"}]}
"#),
    (block_13, r#"{"pLMNidentity":"208-93","tAC":1}
"#),
];

/// The Rust block 1 of README.md.
#[rustfmt::skip]
fn block_1() {
    use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, s1ap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Decode a UE CONTEXT RELEASE REQUEST
        let bytes = hex::decode("0012401500000300000002002a000800020007000240020280")?;
        let pdu = S1AP_PDU::decode(&bytes)?;
        assert_eq!((pdu.procedure_code(), pdu.is_initiating()), (18, true));
        println!("{}", pdu.procedure_name());

        // Read its IEs by name. An `opt` field is `None` when its IE is not there,
        // and what follows `=>` is what the field keeps of the decoded value
        let request: UEContextReleaseRequest = pdu.decode_value()?;
        extract_s1ap_ies!(request, UEContextReleaseRequest,
            opt mme_id: u32 = MME_UE_S1AP_ID(id),
            opt enb_id: u32 = eNB_UE_S1AP_ID(id),
            opt cause: Cause = Cause(cause) => cause,
        );
        println!("MME={mme_id:?} eNB={enb_id:?} cause={cause:?}");

        // Build it with another cause: the direction, the procedure, the criticality
        // of the PDU and the message, then each IE with its criticality
        let rebuilt = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,

            REJECT MME_UE_S1AP_ID(42u32),
            REJECT eNB_UE_S1AP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
        )?;

        // Encode it
        let encoded = hex::encode(rebuilt.encode()?);
        assert_eq!(encoded, "0012401500000300000002002a0008000200070002400202a0");
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 2 of README.md.
#[rustfmt::skip]
fn block_2() {
    use oxirush_s1ap::helpers::*;
    use oxirush_s1ap::{BitString, build_s1ap, build_s1ap_ie, extract_s1ap_ies, ie, s1ap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let network = plmn("208", "93")?;

        // The eNB: S1 SETUP REQUEST, its identity, its name and the tracking
        // area that it serves
        let enb = global_enb_id(network, EnbKind::Macro, 0x12345)?;
        let area = SupportedTAsItem::new(TAC([0, 1]), BPLMNs(vec![network]), None);
        let request = build_s1ap!(InitiatingMessage, S1Setup, S1SetupRequest,
            Global_ENB_ID(enb),
            eNBname("enb-1"),
            SupportedTAs(SupportedTAs(vec![area])),
            DefaultPagingDRX(PagingDRX::v128),
        )?;

        // The MME reads it, and answers with its name and its GUMMEIs
        let received = S1AP_PDU::decode(&request.encode()?)?;
        extract_s1ap_ies!(received, S1SetupRequest,
            opt name: String = eNBname(name) => name.0.into_owned(),
            req areas: SupportedTAs = SupportedTAs(areas) => areas,
        );
        println!("S1 SETUP REQUEST from {}: {} tracking area", name.unwrap_or_default(), areas.0.len());
        let served = ServedGUMMEIsItem::new(
            ServedPLMNs(vec![network]),
            ServedGroupIDs(vec![MMEGroupID([0, 1])]),
            ServedMMECs(vec![MMECode([1])]),
            None,
        );
        let response = build_s1ap!(SuccessfulOutcome, S1Setup, S1SetupResponse,
            MMEname("mme-1"),
            ServedGUMMEIs(ServedGUMMEIs(vec![served])),
            RelativeMMECapacity(RelativeMMECapacity(255)),
        )?;
        println!("S1 SETUP RESPONSE {}", hex::encode(response.encode()?));

        // The eNB: INITIAL UE MESSAGE, with the ATTACH REQUEST of a UE and the
        // cell that the UE is in
        let attach = hex::decode("07410108298039000000001002e0e000040201d031")?;
        let initial = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
            eNB_UE_S1AP_ID(7u32),
            NAS_PDU(attach),
            TAI(tai(network, [0, 1])),
            EUTRAN_CGI(eutran_cgi(network, EnbKind::Macro, 0x12345, 1)?),
            RRC_Establishment_Cause(RRCEstablishmentCause::mo_Signalling),
        )?;

        // The MME reads the IEs that it needs where they are in the octets
        let octets = initial.encode()?;
        let received = S1AP_PDU::decode_borrowed(&octets)?;
        extract_s1ap_ies!(received, InitialUEMessage,
            req enb_id: u32 = eNB_UE_S1AP_ID(id),
            req nas: NASPDU = NAS_PDU(nas) => nas,
        );
        println!("INITIAL UE MESSAGE of eNB UE {enb_id}: NAS-PDU {}", hex::encode(&nas.0));

        // It answers the UE, under the MME UE S1AP ID that it gives it: here an
        // AUTHENTICATION REQUEST
        let authentication = hex::decode(
            "07520023553cbe9637a89d218ae64dae47bf3510aa689c648350b9b9a4a8043ac07aa7e0",
        )?;
        let downlink = build_s1ap!(InitiatingMessage, downlinkNASTransport, DownlinkNASTransport,
            MME_UE_S1AP_ID(1u32),
            eNB_UE_S1AP_ID(enb_id),
            NAS_PDU(authentication),
        )?;
        println!("DOWNLINK NAS TRANSPORT {}", hex::encode(downlink.encode()?));

        // After the security mode, the default bearer of the UE, whose item
        // carries the ATTACH ACCEPT, and its security context for the eNB: its
        // capabilities and KeNB
        let priority = AllocationAndRetentionPriority::new(
            PriorityLevel(8),
            PreEmptionCapability::shall_not_trigger_pre_emption,
            PreEmptionVulnerability::not_pre_emptable,
            None,
        );
        let accept = hex::decode("07420121060002f839000100155201c101090908696e7465726e657405010a000001")?;
        let bearer = ERABToBeSetupItemCtxtSUReq::new(
            ERABID(5),
            ERABLevelQoSParameters::new(QCI(9), priority, None, None),
            TransportLayerAddress(BitString::from([10, 0, 0, 1])),
            GTPTEID([0, 0, 0, 1]),
            Some(NASPDU::from(accept)),
            None,
        );
        let bearers = vec![build_s1ap_ie!(REJECT E_RABToBeSetupItemCtxtSUReq(bearer))?];
        let rates = UEAggregateMaximumBitrate::new(BitRate(100_000_000), BitRate(50_000_000), None);
        let k_enb = hex::decode("cd49a4ae7001cf3c810a89f47ced1b130f7b3f9b42bc9707431b3fb4e62b20fc")?;
        let setup = build_s1ap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
            MME_UE_S1AP_ID(1u32),
            eNB_UE_S1AP_ID(enb_id),
            uEaggregateMaximumBitrate(rates),
            E_RABToBeSetupListCtxtSUReq(ERABToBeSetupListCtxtSUReq(bearers)),
            UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
            SecurityKey(<[u8; 32]>::try_from(k_enb).unwrap()),
        )?;

        // The eNB checks it against its object set, and takes the key
        let received = S1AP_PDU::decode(&setup.encode()?)?;
        let deviations = received.check()?;
        let key = received.ie::<ie::SecurityKey>().unwrap()?;
        let key = hex::encode(key.0.as_octets());
        println!("INITIAL CONTEXT SETUP REQUEST, {} deviations: KeNB {key}", deviations.len());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 3 of README.md.
#[rustfmt::skip]
fn block_3() {
    use oxirush_s1ap::{S1apPduKind, s1ap::*};

    fn receive(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        let pdu = S1AP_PDU::decode(bytes)?;

        Ok(match pdu.kind() {
            S1apPduKind::Initiating_UEContextReleaseRequest => {
                let request: UEContextReleaseRequest = pdu.decode_value()?;
                format!("a release request with {} IEs", request.protocol_ies.0.len())
            }
            S1apPduKind::Other { direction, procedure_code } => {
                format!("procedure {procedure_code} has no {direction}")
            }
            kind => format!("{kind:?} is not handled here"),
        })
    }

    // A UE CONTEXT RELEASE REQUEST, then an S1 SETUP FAILURE
    let request = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    println!("{}", receive(&request).unwrap());

    let failure = hex::decode("401100080000010002400145").unwrap();
    println!("{}", receive(&failure).unwrap());
}

/// The Rust block 4 of README.md.
#[rustfmt::skip]
fn block_4() {
    use oxirush_s1ap::{Message, build_s1ap, s1ap::*};

    fn main() -> Result<(), oxirush_s1ap::Error> {
        // The criticality of the procedure, and those of the object set of the message
        let pdu = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
            eNB_UE_S1AP_ID(7u32),
            NAS_PDU(vec![0x07, 0x41, 0x71]),
        )?;
        println!("{:?} {}", pdu.criticality(), hex::encode(pdu.encode()?));

        // The PDU of a message that is at hand
        let message: InitialUEMessage = pdu.decode_value()?;
        let again = S1AP_PDU::of_message(&message, InitialUEMessage::CRITICALITY)?;
        println!("{}", again == pdu);

        // An IE that the object set of the message does not have is assigned none
        let error = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
            TimeToWait(TimeToWait::v10s),
        );
        println!("{}", error.unwrap_err());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 5 of README.md.
#[rustfmt::skip]
fn block_5() {
    use oxirush_s1ap::{Error, extract_s1ap_ies, s1ap::*};

    fn handle(request: &UEContextReleaseRequest) -> Result<(), Error> {
        extract_s1ap_ies!(request, UEContextReleaseRequest,
            req mme_id: u32 = MME_UE_S1AP_ID(id),
            req enb_id: u32 = eNB_UE_S1AP_ID(id),
            req cause: Cause = Cause(value) => value,
        );

        println!("MME={mme_id} eNB={enb_id} cause={cause:?}");
        Ok(())
    }
}

/// The Rust block 6 of README.md.
#[rustfmt::skip]
fn block_6() {
    use oxirush_s1ap::{Error, extract_s1ap_ies, ie, s1ap::*};

    fn main() -> Result<(), Error> {
        // A UE CONTEXT RELEASE REQUEST, read in the buffer that it was received in
        let octets = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
        let pdu = S1AP_PDU::decode_borrowed(&octets)?;

        // The IEs of its message, from the PDU: the Cause is not decoded
        extract_s1ap_ies!(pdu, UEContextReleaseRequest,
            req mme_id: u32 = MME_UE_S1AP_ID(id),
            req enb_id: u32 = eNB_UE_S1AP_ID(id),
        );
        println!("MME={mme_id} eNB={enb_id}");

        // One IE, by its marker type
        let cause = pdu.ie::<ie::Cause>().unwrap()?;
        println!("{cause:?}");
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 7 of README.md.
#[rustfmt::skip]
fn block_7() {
    use oxirush_s1ap::{BitString, build_s1ap, build_s1ap_ie, extract_s1ap_ies, s1ap::*};

    fn main() -> Result<(), oxirush_s1ap::Error> {
        // One E-RAB to set up, as the item of its list
        let priority = AllocationAndRetentionPriority::new(
            PriorityLevel(1),
            PreEmptionCapability::shall_not_trigger_pre_emption,
            PreEmptionVulnerability::not_pre_emptable,
            None,
        );
        let bearer = ERABToBeSetupItemCtxtSUReq::new(
            ERABID(5),
            ERABLevelQoSParameters::new(QCI(9), priority, None, None),
            TransportLayerAddress(BitString::from([10, 0, 0, 1])),
            GTPTEID([0, 0, 0, 1]),
            None,
            None,
        );
        let item = build_s1ap_ie!(REJECT E_RABToBeSetupItemCtxtSUReq(bearer))?;
        let pdu = build_s1ap!(InitiatingMessage, InitialContextSetup, REJECT,
            InitialContextSetupRequest,
            REJECT MME_UE_S1AP_ID(1u32),
            REJECT eNB_UE_S1AP_ID(7u32),
            REJECT E_RABToBeSetupListCtxtSUReq(vec![item]),
        )?;

        // Read back: the list, then each item as its type
        let request: InitialContextSetupRequest = pdu.decode_value()?;
        extract_s1ap_ies!(request, InitialContextSetupRequest,
            req bearers: ERABToBeSetupListCtxtSUReq = E_RABToBeSetupListCtxtSUReq(list) => list,
        );
        for item in &bearers.0 {
            let bearer: ERABToBeSetupItemCtxtSUReq = decode_open_type(&item.value)?;
            println!("E-RAB {} with QCI {}", bearer.e_rab_id.0, bearer.e_rab_level_qos_parameters.qci.0);
        }

        // Or each item of the list, walked in the PDU without building the list
        for bearer in pdu.items::<oxirush_s1ap::ie::E_RABToBeSetupListCtxtSUReq>() {
            println!("E-RAB {}", bearer?.e_rab_id.0);
        }

        // The octets of a Source-ToTarget-TransparentContainer, as the type that they carry
        let octets = hex::decode("000201020002f83900012340200100").unwrap();
        let container: SourceeNBToTargeteNBTransparentContainer = decode_open_type(&octets)?;
        println!("{:?}", container.target_cell_id);
        assert_eq!(encode_open_type(&container)?.as_ref(), &octets[..]);
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 8 of README.md.
#[rustfmt::skip]
fn block_8() {
    use oxirush_s1ap::{build_s1ap, s1ap::*};

    fn main() -> Result<(), oxirush_s1ap::Error> {
        // A UE CONTEXT RELEASE REQUEST that deviates in each way, built all the same
        let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
            REJECT, UEContextReleaseRequest,
            REJECT eNB_UE_S1AP_ID(7u32),
            IGNORE MME_UE_S1AP_ID(42u32),
            IGNORE TimeToWait(TimeToWait::v5s),
            REJECT eNB_UE_S1AP_ID(8u32),
        )?;

        // It is encoded and decoded as any other, and `check()` reports
        let received = S1AP_PDU::decode(&pdu.encode()?)?;
        for deviation in received.check()? {
            println!("{deviation}");
        }

        // The message of the first example deviates in nothing
        let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
        assert!(S1AP_PDU::decode(&bytes)?.check()?.is_empty());
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 9 of README.md.
#[rustfmt::skip]
fn block_9() {
    use oxirush_s1ap::{Deviation, build_s1ap, s1ap::*};

    fn main() -> Result<(), oxirush_s1ap::Error> {
        // An S1 SETUP REQUEST with none of its mandatory IEs but the paging DRX
        let request = build_s1ap!(InitiatingMessage, S1Setup, REJECT, S1SetupRequest,
            IGNORE DefaultPagingDRX(PagingDRX::v128),
        )?;
        let received = S1AP_PDU::decode(&request.encode()?)?;
        let deviations = received.check()?;

        // Clause 10.3.5: the procedure is rejected, and the missing IEs are reported
        let cause = deviations.iter().find_map(Deviation::cause).unwrap();
        let failure = build_s1ap!(UnsuccessfulOutcome, S1Setup, REJECT, S1SetupFailure,
            IGNORE Cause(Cause::protocol(cause)),
            IGNORE CriticalityDiagnostics(received.criticality_diagnostics_for_response(&deviations)),
        )?;
        for deviation in &deviations {
            println!("{deviation}");
        }
        let octets = hex::encode(failure.encode()?);
        println!("{octets}");
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 10 of README.md.
#[rustfmt::skip]
fn block_10() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
    use oxirush_s1ap::tree;

    // A UE CONTEXT RELEASE REQUEST, as a tree
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode(&bytes).unwrap();
    println!("{}", pdu.to_tree());

    // Its view: each value with the path that selects it
    let mut view = pdu.to_view().unwrap();
    for (path, value) in inspect::paths(&view) {
        println!("{path} = {value}");
    }

    // A value, and another alternative of a CHOICE in place of the one that is there
    inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", tree!(9)).unwrap();
    inspect::set(&mut view, "/s1ap/Cause/nas", tree!("Normal Release")).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree());

    // A name that the type does not have is refused, and the view is as it was
    let wrong = inspect::set(&mut view, "/s1ap/Cause/radioNetwrok", tree!("unspecified"));
    println!("{}", wrong.unwrap_err());
}

/// The Rust block 11 of README.md.
#[rustfmt::skip]
fn block_11() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
    use oxirush_s1ap::tree;

    // A tree: the name of the message, whatever its case and its separators, then its IEs
    let request = tree!({
        "message": "ue context release request",
        "ies": [
            {"MME-UE-S1AP-ID": 42},
            {"eNB-UE-S1AP-ID": 7},
            {"Cause": {"radioNetwork": "user-inactivity"}},
            // An IE that the message does not have says its criticality
            {"eNBname": "enb-1", "criticality": "ignore"},
        ],
    });

    let pdu = S1AP_PDU::from_tree(&request).unwrap();
    println!("{}", hex::encode(pdu.encode().unwrap()));

    // The name gave the direction, the procedure code and the criticalities
    let view = pdu.to_view().unwrap();
    println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
    println!("{}", tree!(inspect::select(&view, "/s1ap/*/criticality").unwrap()));
}

/// The Rust block 12 of README.md.
#[rustfmt::skip]
fn block_12() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
    use oxirush_s1ap::tree;

    // `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
    let request = tree!({"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
    ]});
    let pdu = S1AP_PDU::from_tree(&request).unwrap();
    println!("{}", hex::encode(pdu.encode().unwrap()));

    // A view shows an IE that does not decode by its octets, and says why
    let mut view = pdu.to_view().unwrap();
    println!("{}", view["message"]["protocolIEs"][1]);

    // In a view, the `octets` of an IE are set in place of its value
    inspect::set(&mut view, "/s1ap/MME-UE-S1AP-ID/octets", tree!("00ff")).unwrap();
    let edited = pdu.with_view(&view).unwrap();
    println!("{}", edited.to_tree());
}

/// The Rust block 13 of README.md.
#[rustfmt::skip]
fn block_13() {
    use oxirush_s1ap::helpers::{plmn, tai};
    use oxirush_s1ap::s1ap::S1AP_PDU;

    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode(&bytes).unwrap();
    let text = serde_json::to_string(&pdu).unwrap();
    let read: S1AP_PDU = serde_json::from_str(&text).unwrap();
    assert_eq!(read.encode().unwrap(), bytes);

    // A value alone has the form that a tree shows it in: the MCC and the MNC, and a number
    let area = tai(plmn("208", "93").unwrap(), [0x00, 0x01]);
    println!("{}", serde_json::to_string(&area).unwrap());
}
