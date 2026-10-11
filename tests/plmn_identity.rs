//! The octets of a PLMN identity are those of TS 36.413 clause 9.2.3.8, and another
//! decoder reads them as the same network.
//!
//! The clause has two digits in each octet, "bits 4 to 1 of octet n encoding digit 2n-1"
//! and "bits 8 to 5 of octet n encoding digit 2n", of "3 digits from MCC followed by
//! either a filler digit plus 2 digits from MNC (in case of 2 digit MNC) or 3 digits
//! from MNC (in case of a 3 digit MNC)", with 1111 as the filler. The octets of
//! [`IDENTITIES`] are written out from it by hand, and so are those of the PLMN
//! identities of [`S1_SETUP`]: none of them comes from the code that they test.
//!
//! tshark 4.6.4 was given [`S1_SETUP`] and printed [`TSHARK`] for it. The test that runs
//! tshark again is ignored, as tshark may not be installed:
//!
//! ```sh
//! cargo test --test plmn_identity -- --ignored
//! ```
//!
//! It needs `text2pcap` and `tshark` of Wireshark, and does what these commands do, with
//! `0000`, a space and the octets of the PDU in hexadecimal, a space between two, as the
//! one line of `pdu.txt`. 18 is the SCTP payload protocol identifier of S1AP.
//!
//! ```sh
//! text2pcap -q -S 36412,36412,18 pdu.txt pdu.pcap
//! tshark -r pdu.pcap -V | grep -E 'LMNidentity:|Mobile (Country|Network) Code'
//! ```

use oxirush_s1ap::helpers::{global_enb_id, plmn, plmn_from};
use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{build_s1ap_ie, extract_s1ap_ies, macros::MissingIeError};

/// An MCC, an MNC and the octets of their PLMN identity, digit by digit from the clause.
const IDENTITIES: &[(&str, &str, [u8; 3])] = &[
    // The digits 2 0 8, a filler, 9 3: octet 1 has 0 over 2, octet 2 the filler over 8,
    // octet 3 has 3 over 9.
    ("208", "93", [0x02, 0xf8, 0x39]),
    // The digits 0 0 1, a filler, 0 1.
    ("001", "01", [0x00, 0xf1, 0x10]),
    // The digits 3 1 0 4 1 0: octet 2 has the first digit of the MNC over the last of
    // the MCC, and octet 3 the last digit of the MNC over its second.
    ("310", "410", [0x13, 0x40, 0x01]),
    // The digits 1 2 3 4 5 6, two in each octet, the second over the first.
    ("123", "456", [0x21, 0x43, 0x65]),
    // The digits 9 9 9 0 7 0.
    ("999", "070", [0x99, 0x09, 0x07]),
];

/// An S1 SETUP REQUEST of the macro eNB 1 of 310-410, whose tracking area 1 broadcasts
/// 208-93 and 310-410: its PLMN identities are the octets `13 40 01`, `02 f8 39` and
/// `13 40 01`.
const S1_SETUP: &str =
    "00110022000003003b000800134001000000100040000a0000004802f8391340010089400140";

/// What tshark 4.6.4 printed of the PLMN identities of [`S1_SETUP`].
const TSHARK: &str = "\
pLMNidentity: 134001
Mobile Country Code (MCC): United States (310)
Mobile Network Code (MNC): AT&T Mobility (410)
PLMNidentity: 02f839
Mobile Country Code (MCC): France (208)
Mobile Network Code (MNC): Thales communications & Security (93)
PLMNidentity: 134001
Mobile Country Code (MCC): United States (310)
Mobile Network Code (MNC): AT&T Mobility (410)
";

/// What tshark 4.6.4 printed of the same message with the octets `13 00 14` in the place
/// of `13 40 01`: the order of TS 24.008, which a NAS message has and which this crate
/// wrote for 310-410, is another network to a decoder of S1AP.
const TSHARK_OF_THE_NAS_ORDER: &str = "\
pLMNidentity: 130014
Mobile Country Code (MCC): United States (310)
Mobile Network Code (MNC): Unknown (041)
PLMNidentity: 02f839
Mobile Country Code (MCC): France (208)
Mobile Network Code (MNC): Thales communications & Security (93)
PLMNidentity: 130014
Mobile Country Code (MCC): United States (310)
Mobile Network Code (MNC): Unknown (041)
";

/// The MCC and the MNC of each PLMN identity that tshark printed: the digits in the
/// parentheses that end its lines, two lines for each identity.
fn networks(printed: &str) -> Vec<(String, String)> {
    let codes = printed.lines().filter(|line| line.contains("Mobile "));
    let codes = codes.map(|line| {
        let code = line.rsplit('(').next().unwrap_or_default();
        code.trim_end().trim_end_matches(')').to_string()
    });
    let codes: Vec<String> = codes.collect();
    let pairs = codes.chunks(2);
    pairs
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .collect()
}

/// The tracking areas that the eNB of [`S1_SETUP`] supports, built with the helpers.
fn tracking_areas() -> SupportedTAs {
    let broadcast = BPLMNs(vec![plmn("208", "93"), plmn("310", "410")]);
    SupportedTAs(vec![SupportedTAsItem::new(
        TAC::from([0, 1]),
        broadcast,
        None,
    )])
}

#[test]
fn the_octets_of_a_plmn_identity_are_those_of_the_clause() {
    for (mcc, mnc, octets) in IDENTITIES {
        let identity = plmn(mcc, mnc);
        assert_eq!(&identity.0.0[..], octets, "{mcc}-{mnc}");
        let read = plmn_from(&PLMNidentity(TBCDSTRING::from(*octets)));
        assert_eq!(read, (mcc.to_string(), mnc.to_string()), "{octets:02x?}");
    }
}

#[test]
fn the_identities_of_an_s1_setup_are_those_that_tshark_reads() -> Result<(), MissingIeError> {
    // What tshark printed names the three networks, in the order of the message.
    let read = networks(TSHARK);
    let (visited, home) = (("310".into(), "410".into()), ("208".into(), "93".into()));
    assert_eq!(read, [visited.clone(), home, visited]);

    // The helpers read the same networks in the same octets.
    let pdu = S1AP_PDU::decode(&hex::decode(S1_SETUP).unwrap()).expect("decode PDU");
    let request: S1SetupRequest = pdu.decode_value().expect("decode message");
    extract_s1ap_ies!(&request, S1SetupRequest,
        req enb: GlobalENBID = Global_ENB_ID(enb) => enb,
        req areas: SupportedTAs = SupportedTAs(areas) => areas,
    );
    let broadcast = areas.0[0].broadcast_plmns.0.iter();
    let identities = std::iter::once(&enb.p_lmnidentity).chain(broadcast);
    assert_eq!(identities.map(plmn_from).collect::<Vec<_>>(), read);

    // And write the octets of the message for them.
    assert_eq!(enb, global_enb_id(plmn("310", "410"), 1));
    assert_eq!(areas, tracking_areas());
    let written = build_s1ap_ie!(S1SetupRequest, REJECT Global_ENB_ID(enb));
    assert_eq!(written, request.protocol_ies.0[0]);
    let written = build_s1ap_ie!(S1SetupRequest, REJECT SupportedTAs(tracking_areas()));
    assert_eq!(written, request.protocol_ies.0[1]);
    assert_eq!(hex::encode(pdu.encode().unwrap()), S1_SETUP);
    Ok(())
}

#[test]
fn the_order_of_a_nas_message_is_another_network() {
    // The octets that TS 24.008 has for 310-410 are those of 310-041 here, as tshark
    // reads them.
    let read = networks(TSHARK_OF_THE_NAS_ORDER);
    assert_eq!(read[0], ("310".into(), "041".into()));
    let identity = PLMNidentity(TBCDSTRING::from([0x13, 0x00, 0x14]));
    assert_eq!(plmn_from(&identity), read[0]);
    assert_eq!(&plmn("310", "041").0.0[..], [0x13, 0x00, 0x14]);
}

/// With the `inspect` feature a PLMN identity is written and shown as its MCC and its
/// MNC, `"310-410"`, in a view and in a tree: the same octets.
#[cfg(feature = "inspect")]
#[test]
fn the_notation_of_a_plmn_identity_has_the_octets_of_the_clause() {
    use oxirush_s1ap::inspect;
    use serde_json::json;

    // Shown: the tree and the view of the request name the networks that tshark reads.
    let wire = hex::decode(S1_SETUP).unwrap();
    let pdu = S1AP_PDU::decode(&wire).unwrap();
    let tree = inspect::message_tree(&pdu).unwrap();
    assert_eq!(tree["ies"][0]["Global-ENB-ID"]["pLMNidentity"], "310-410");
    let broadcast = &tree["ies"][1]["SupportedTAs"][0]["broadcastPLMNs"];
    assert_eq!(*broadcast, json!(["208-93", "310-410"]));
    let mut view = inspect::message_view(&pdu).unwrap();
    let of_the_enb = "/s1ap/Global-ENB-ID/pLMNidentity";
    assert_eq!(
        inspect::select(&view, of_the_enb).unwrap(),
        [&json!("310-410")]
    );

    // Written: the tree is the octets of the message, and so is each identity of the
    // vectors, set through the view.
    assert_eq!(
        inspect::message_from_tree(&tree).unwrap().encode().unwrap(),
        wire
    );
    for (mcc, mnc, octets) in IDENTITIES {
        inspect::set(&mut view, of_the_enb, json!(format!("{mcc}-{mnc}"))).unwrap();
        let edited = inspect::message_from_view(&view).unwrap();
        let request: S1SetupRequest = edited.decode_value().unwrap();
        let enb: GlobalENBID = decode_open_type(&request.protocol_ies.0[0].value).unwrap();
        assert_eq!(&enb.p_lmnidentity.0.0[..], octets, "{mcc}-{mnc}");
        let shown = inspect::message_view(&edited).unwrap();
        let shown = inspect::select(&shown, of_the_enb).unwrap();
        assert_eq!(shown, [&json!(format!("{mcc}-{mnc}"))]);
    }
}

/// Run tshark on [`S1_SETUP`], and on the same message with the octets of the order of a
/// NAS message: it prints the networks that the tests above have from its recorded
/// output.
#[test]
#[ignore = "needs text2pcap and tshark of Wireshark"]
fn tshark_reads_the_identities_of_the_s1_setup() {
    let tshark = |pdu: &str| -> String {
        let directory = std::env::temp_dir().join(format!("plmn-identity-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let (text, capture) = (directory.join("pdu.txt"), directory.join("pdu.pcap"));
        let octets: Vec<&str> = (0..pdu.len() / 2)
            .map(|at| &pdu[2 * at..2 * at + 2])
            .collect();
        std::fs::write(&text, format!("0000 {}\n", octets.join(" "))).unwrap();
        let converted = std::process::Command::new("text2pcap")
            .args(["-q", "-S", "36412,36412,18"])
            .args([&text, &capture])
            .status();
        assert!(converted.expect("text2pcap runs").success());
        let read = std::process::Command::new("tshark")
            .arg("-r")
            .arg(&capture)
            .arg("-V")
            .output();
        let read = read.expect("tshark runs");
        std::fs::remove_dir_all(&directory).unwrap();
        String::from_utf8(read.stdout).unwrap()
    };
    assert_eq!(networks(&tshark(S1_SETUP)), networks(TSHARK));
    let nas_order = S1_SETUP.replace("134001", "130014");
    assert_eq!(
        networks(&tshark(&nas_order)),
        networks(TSHARK_OF_THE_NAS_ORDER)
    );
}
