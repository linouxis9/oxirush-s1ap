// The blocks of the README that use other crates of the family, each as a
// function. `tests/readme.rs` writes this file and says how: do not edit it.
#![allow(unused)]

use std::cell::RefCell;

thread_local!(static PRINTED: RefCell<String> = const { RefCell::new(String::new()) });

// What a block prints is kept, to be compared with what the README shows.
macro_rules! println {
    () => {
        PRINTED.with(|printed| printed.borrow_mut().push('\n'))
    };
    // The line is made first, where the block is: its arguments may have a
    // `?` or an `await`.
    ($($argument:tt)*) => {{
        let line = format!($($argument)*);
        PRINTED.with(|printed| {
            let mut printed = printed.borrow_mut();
            printed.push_str(&line);
            printed.push('\n');
        })
    }};
}

/// What `block` prints.
pub fn printed(block: fn()) -> String {
    PRINTED.with(|printed| printed.borrow_mut().clear());
    block();
    PRINTED.with(|printed| printed.take())
}

pub const BLOCKS: &[fn()] = &[
    block_1,
];

/// The block 1 of the README that uses other crates.
fn block_1() {
    use oxirush_nas::nas_eps::*;
    use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, helpers, s1ap::*};
    use oxirush_security::milenage::{Amf, K, Milenage, Opc, Sqn};
    use oxirush_security::nas_eps::{derive_kasme, derive_kenb};
    use oxirush_security::Rand;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // The UE: an ATTACH REQUEST with its IMSI, read from a capture
        let attach = NasEpsMessage::decode(&hex::decode("07410108298039000000001002e0e000040201d031")?)?;

        // The eNB: the INITIAL UE MESSAGE carries its octets
        let network = helpers::plmn("208", "93")?;
        let cell = helpers::eutran_cgi(network, helpers::EnbKind::Macro, 0x12345, 1)?;
        let initial = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
            eNB_UE_S1AP_ID(7u32),
            NAS_PDU(attach.encode()?),
            TAI(helpers::tai(network, [0, 1])),
            EUTRAN_CGI(cell),
            RRC_Establishment_Cause(RRCEstablishmentCause::mo_Signalling),
        )?;

        // The MME reads the NAS-PDU as a NAS message
        extract_s1ap_ies!(initial, InitialUEMessage,
            req enb_id: u32 = eNB_UE_S1AP_ID(id),
            req nas: NASPDU = NAS_PDU(nas) => nas,
        );
        let NasEpsMessage::Emm(_, NasEmmMessage::AttachRequest(request)) = NasEpsMessage::decode(&nas.0)?
        else {
            unreachable!()
        };
        println!("eNB UE {enb_id}: IMSI {}", request.eps_mobile_identity.as_imsi().unwrap());

        // EPS AKA gives KASME, from the vector of the subscriber of TS 35.208 test set 1
        let octets = |text: &str| hex::decode(text).unwrap();
        let k = K::from_slice(&octets("465b5ce8b199b49faa5f0a2ee238a6bc"))?;
        let opc = Opc::from_slice(&octets("cd63cb71954a9f4e48a5994e37a02baf"))?;
        let rand = Rand::from_slice(&octets("23553cbe9637a89d218ae64dae47bf35"))?;
        let (sqn, amf) = (Sqn::try_from(32)?, Amf::new(&[0xb9, 0xb9]));
        let vector = Milenage::new(&k, &opc).authentication_vector(&rand, &sqn, &amf);
        let kasme = derive_kasme(vector.ck(), vector.ik(), Plmn::new("208", "93")?, vector.autn());

        // The SECURITY MODE COMMAND, protected with the new context, in DOWNLINK NAS TRANSPORT
        let (eia, eea) = (IntegrityAlgorithm::EIA2, CipheringAlgorithm::EEA2);
        let mut context = NasSecurityContext::from_fresh_kasme(&kasme, eia, eea)?;
        let ksi = NasKeySetIdentifier::from_key_set_identifier(KeySetIdentifier::Native(0))?;
        let capability = NasUeSecurityCapability::from_ue_network_capability(&request.ue_network_capability);
        let algorithms = NasSecurityAlgorithms::from_algorithms(eea, eia);
        let command = NasEpsMessage::new_emm(NasSecurityModeCommand::new(algorithms, ksi, capability));
        let header = NasEpsSecurityHeaderType::IntegrityProtectedWithNewEpsSecurityContext;
        let downlink = build_s1ap!(InitiatingMessage, downlinkNASTransport, DownlinkNASTransport,
            MME_UE_S1AP_ID(1u32),
            eNB_UE_S1AP_ID(enb_id),
            NAS_PDU(context.protect(&command, header, Direction::Downlink)?),
        )?;
        println!("{}", hex::encode(downlink.encode()?));

        // The SECURITY MODE COMPLETE came with NAS COUNT 0: KeNB is derived from
        // it, for the INITIAL CONTEXT SETUP REQUEST
        let k_enb = derive_kenb(&kasme, NasCount::ZERO);
        println!("KeNB {}", hex::encode(k_enb.as_octets()));
        Ok(())
    }
    main().unwrap();
}
