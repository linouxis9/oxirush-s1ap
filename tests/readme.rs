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
    (block_1, r#"UEContextReleaseRequest (procedure 18, initiating message), MME-UE-S1AP-ID 42, eNB-UE-S1AP-ID 7
MME UE 42, eNB UE 7, cause Some(radioNetwork(user_inactivity))
0012401500000300000002002a0008000200070002400202a0
"#),
    (block_2, r#"S1SetupRequest (procedure 17, initiating message): enb-1, 1 tracking area
S1SetupResponse (procedure 17, successful outcome)
InitialUEMessage (procedure 12, initiating message), eNB-UE-S1AP-ID 7: a NAS-PDU of 21 octets
DownlinkNASTransport (procedure 11, initiating message), MME-UE-S1AP-ID 1, eNB-UE-S1AP-ID 7
InitialContextSetupRequest (procedure 9, initiating message), MME-UE-S1AP-ID 1, eNB-UE-S1AP-ID 7: 0 deviations, KeNB as sent: true
"#),
    (block_3, r#"release of MME UE 42, eNB UE 7: Some(radioNetwork(user_inactivity))
Unsuccessful_S1Setup is not handled here
"#),
    (block_4, r#"ignore 000c4011000002000800020007001a000403074171
true
TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no criticality: write its criticality
"#),
    (block_5, r#"E-RAB 5 with QCI 9
EUTRANCGI { plmn_identity: PLMNidentity(208-93), cell_id: CellIdentity(0x1234/28), ie_extensions: None }
"#),
    (block_6, r#"the criticality of the PDU is ignore, and that of its procedure is reject
TimeToWait (IE 65), at position 1, is not an IE of the message
Global-ENB-ID (IE 59) is mandatory, and the message does not have it
SupportedTAs (IE 64) is mandatory, and the message does not have it
S1SetupFailure (procedure 17, unsuccessful outcome): 401100180000020002400131003a400c190200004100003b40004040
"#),
    (block_7, r#"{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
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
    (block_8, r#"0012402700000500000002002a000800020007000240020280003c40070200656e622d31fde80003c0ffee
"InitiatingMessage" 18 "ignore"
["reject","reject","ignore","ignore","reject"]
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
        println!("{pdu}");

        // Read its IEs by name: `req` fails when the IE is missing, `opt` gives an Option
        extract_s1ap_ies!(pdu, UEContextReleaseRequest,
            req mme_id: u32 = MME_UE_S1AP_ID(id),
            req enb_id: u32 = eNB_UE_S1AP_ID(id),
            opt cause: Cause = Cause(cause) => cause,
        );
        println!("MME UE {mme_id}, eNB UE {enb_id}, cause {cause:?}");

        // Build the same message with another cause, and encode it
        let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest, UEContextReleaseRequest,
            MME_UE_S1AP_ID(42u32),
            eNB_UE_S1AP_ID(7u32),
            Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
        )?;
        println!("{}", hex::encode(pdu.encode()?));
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 2 of README.md.
#[rustfmt::skip]
fn block_2() {
    use oxirush_s1ap::helpers::*;
    use oxirush_s1ap::{BitString, build_s1ap, build_s1ap_ie, extract_s1ap_ies, s1ap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let network = plmn("208", "93")?;

        // The eNB: its identity, its name and the tracking area it serves
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
        println!("{received}: {}, {} tracking area", name.unwrap_or_default(), areas.0.len());
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
        println!("{response}");

        // The eNB forwards an ATTACH REQUEST, with the cell the UE is in
        let attach = hex::decode("07410108298039000000001002e0e000040201d031")?;
        let initial = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
            eNB_UE_S1AP_ID(7u32),
            NAS_PDU(attach),
            TAI(tai(network, [0, 1])),
            EUTRAN_CGI(eutran_cgi(network, EnbKind::Macro, 0x12345, 1)?),
            RRC_Establishment_Cause(RRCEstablishmentCause::mo_Signalling),
        )?;

        // The MME reads the two IEs it needs, in place in the received octets
        let octets = initial.encode()?;
        let received = S1AP_PDU::decode_borrowed(&octets)?;
        extract_s1ap_ies!(received, InitialUEMessage,
            req enb_id: u32 = eNB_UE_S1AP_ID(id),
            req nas: NASPDU = NAS_PDU(nas) => nas,
        );
        println!("{received}: a NAS-PDU of {} octets", nas.0.len());

        // It gives the UE an MME UE S1AP ID, and answers with an AUTHENTICATION REQUEST
        let authentication = hex::decode(
            "07520023553cbe9637a89d218ae64dae47bf3510aa689c648350b9b9a4a8043ac07aa7e0",
        )?;
        let downlink = build_s1ap!(InitiatingMessage, downlinkNASTransport, DownlinkNASTransport,
            MME_UE_S1AP_ID(1u32),
            eNB_UE_S1AP_ID(enb_id),
            NAS_PDU(authentication),
        )?;
        println!("{downlink}");

        // After the security mode: the default bearer, whose item carries the
        // ATTACH ACCEPT, and the UE's security capabilities and KeNB
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
        let mut k_enb = [0; 32];
        hex::decode_to_slice("cd49a4ae7001cf3c810a89f47ced1b130f7b3f9b42bc9707431b3fb4e62b20fc", &mut k_enb)?;
        let setup = build_s1ap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
            MME_UE_S1AP_ID(1u32),
            eNB_UE_S1AP_ID(enb_id),
            uEaggregateMaximumBitrate(rates),
            E_RABToBeSetupListCtxtSUReq(ERABToBeSetupListCtxtSUReq(bearers)),
            UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
            SecurityKey(k_enb),
        )?;

        // The eNB checks the message against the specification, and takes the key
        let received = S1AP_PDU::decode(&setup.encode()?)?;
        let deviations = received.check()?;
        extract_s1ap_ies!(received, InitialContextSetupRequest,
            req key: SecurityKey = SecurityKey(key) => key,
        );
        println!("{received}: {} deviations, KeNB as sent: {}", deviations.len(), key.0.as_octets() == k_enb);
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 3 of README.md.
#[rustfmt::skip]
fn block_3() {
    use oxirush_s1ap::{S1apPduKind, extract_s1ap_ies, ie, s1ap::*};

    fn receive(octets: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // `decode_borrowed` copies nothing: the PDU borrows from `octets`
        let pdu = S1AP_PDU::decode_borrowed(octets)?;
        match pdu.kind() {
            S1apPduKind::Initiating_UEContextReleaseRequest => {
                // Only the IEs that a field asks for are decoded
                extract_s1ap_ies!(pdu, UEContextReleaseRequest,
                    req mme_id: u32 = MME_UE_S1AP_ID(id),
                    req enb_id: u32 = eNB_UE_S1AP_ID(id),
                );
                // One IE, by its marker type
                let cause = pdu.ie::<ie::Cause>().transpose()?;
                println!("release of MME UE {mme_id}, eNB UE {enb_id}: {cause:?}");
            }
            S1apPduKind::Other { direction, procedure_code } => {
                println!("procedure {procedure_code} has no {direction}");
            }
            kind => println!("{kind:?} is not handled here"),
        }
        Ok(())
    }

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // A UE CONTEXT RELEASE REQUEST, then an S1 SETUP FAILURE
        receive(&hex::decode("0012401500000300000002002a000800020007000240020280")?)?;
        receive(&hex::decode("401100080000010002400145")?)?;
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 4 of README.md.
#[rustfmt::skip]
fn block_4() {
    use oxirush_s1ap::{Message, build_s1ap, s1ap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // The criticalities of the procedure and of the message's object set
        let pdu = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
            eNB_UE_S1AP_ID(7u32),
            NAS_PDU(vec![0x07, 0x41, 0x71]),
        )?;
        println!("{:?} {}", pdu.criticality(), hex::encode(pdu.encode()?));

        // The PDU of a message you already have
        let message: InitialUEMessage = pdu.decode_value()?;
        let again = S1AP_PDU::of_message(&message, InitialUEMessage::CRITICALITY)?;
        println!("{}", again == pdu);

        // An IE that the message cannot have has no criticality to take
        let error = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
            TimeToWait(TimeToWait::v10s),
        );
        if let Err(error) = error {
            println!("{error}");
        }
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 5 of README.md.
#[rustfmt::skip]
fn block_5() {
    use oxirush_s1ap::{BitString, build_s1ap, build_s1ap_ie, ie, s1ap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        let pdu = build_s1ap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
            MME_UE_S1AP_ID(1u32),
            eNB_UE_S1AP_ID(7u32),
            E_RABToBeSetupListCtxtSUReq(vec![item]),
        )?;

        // Each item of the list, read in the PDU without building the list
        for bearer in pdu.items::<ie::E_RABToBeSetupListCtxtSUReq>() {
            let bearer = bearer?;
            println!("E-RAB {} with QCI {}", bearer.e_rab_id.0, bearer.e_rab_level_qos_parameters.qci.0);
        }

        // A Source-ToTarget-TransparentContainer, as the type it carries
        let octets = hex::decode("000201020002f83900012340200100")?;
        let container: SourceeNBToTargeteNBTransparentContainer = decode_open_type(&octets)?;
        println!("{:?}", container.target_cell_id);
        assert_eq!(encode_open_type(&container)?.as_ref(), &octets[..]);
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 6 of README.md.
#[rustfmt::skip]
fn block_6() {
    use oxirush_s1ap::{Deviation, build_s1ap, s1ap::*};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // An S1 SETUP REQUEST with the wrong criticality, an IE it cannot have,
        // and none of its mandatory IEs but the paging DRX
        let request = build_s1ap!(InitiatingMessage, S1Setup, IGNORE, S1SetupRequest,
            IGNORE DefaultPagingDRX(PagingDRX::v128),
            REJECT TimeToWait(TimeToWait::v5s),
        )?;
        let received = S1AP_PDU::decode(&request.encode()?)?;
        let deviations = received.check()?;
        for deviation in &deviations {
            println!("{deviation}");
        }

        // The MME rejects the procedure, and reports the IEs at fault
        let cause = deviations.iter().find_map(Deviation::cause).ok_or("no cause")?;
        let failure = build_s1ap!(UnsuccessfulOutcome, S1Setup, S1SetupFailure,
            Cause(Cause::protocol(cause)),
            CriticalityDiagnostics(received.criticality_diagnostics_for_response(&deviations)),
        )?;
        println!("{failure}: {}", hex::encode(failure.encode()?));
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 7 of README.md.
#[rustfmt::skip]
fn block_7() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU, tree};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // A UE CONTEXT RELEASE REQUEST, as a tree
        let bytes = hex::decode("0012401500000300000002002a000800020007000240020280")?;
        let pdu = S1AP_PDU::decode(&bytes)?;
        println!("{}", pdu.to_tree());

        // Its view: each value with the path that selects it
        let mut view = pdu.to_view()?;
        for (path, value) in inspect::paths(&view) {
            println!("{path} = {value}");
        }

        // Set a value, and pick another alternative of a CHOICE
        inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", tree!(9))?;
        inspect::set(&mut view, "/s1ap/Cause/nas", tree!("Normal Release"))?;
        println!("{}", pdu.with_view(&view)?.to_tree());

        // A misspelt name is refused, and the view stays as it was
        if let Err(error) = inspect::set(&mut view, "/s1ap/Cause/radioNetwrok", tree!("unspecified")) {
            println!("{error}");
        }
        Ok(())
    }

    main().unwrap();
}

/// The Rust block 8 of README.md.
#[rustfmt::skip]
fn block_8() {
    use oxirush_s1ap::{inspect, s1ap::S1AP_PDU, tree};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let request = tree!({
            "message": "ue context release request",
            "ies": [
                {"MME-UE-S1AP-ID": 42},
                {"eNB-UE-S1AP-ID": 7},
                {"Cause": {"radioNetwork": "user-inactivity"}},
                // An IE that the message does not have says its criticality
                {"eNBname": "enb-1", "criticality": "ignore"},
                // An IE that no release has, as its octets
                {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
            ],
        });
        let pdu = S1AP_PDU::from_tree(&request)?;
        println!("{}", hex::encode(pdu.encode()?));

        let view = pdu.to_view()?;
        println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
        println!("{}", tree!(inspect::select(&view, "/s1ap/*/criticality")?));
        Ok(())
    }

    main().unwrap();
}
