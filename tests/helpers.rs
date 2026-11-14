//! The helpers that build the values that a program of S1AP writes most from their
//! parts, and what they refuse, and the PLMN identity of S1AP, a TBCD-STRING, which
//! prints as its MCC and its MNC. TOY, the protocol of the tests of oxirush-aper, has
//! the tests of how the other identities print.

use oxirush_s1ap::ErrorKind;
use oxirush_s1ap::aper::{BitString, Bits};
use oxirush_s1ap::build_s1ap;
use oxirush_s1ap::helpers::*;
use oxirush_s1ap::identity::Plmn;
use oxirush_s1ap::s1ap::*;

#[test]
fn a_plmn_identity_prints_as_its_mcc_and_its_mnc() {
    let identity = plmn("208", "93").unwrap();
    assert_eq!(
        format!("{identity} {identity:?}"),
        "208-93 PLMNidentity(208-93)"
    );
    // An MNC of three digits, and one that starts with a zero
    assert_eq!(plmn("310", "410").unwrap().to_string(), "310-410");
    assert_eq!(plmn("208", "093").unwrap().to_string(), "208-093");

    // In a value that holds one
    let printed = format!(
        "{:?}",
        TAI {
            plmn_identity: identity,
            tac: TAC([0x00, 0x01]),
            ie_extensions: None
        }
    );
    assert!(printed.contains("PLMNidentity(208-93)"), "{printed}");
    assert!(printed.contains("TAC(0001)"), "{printed}");

    // Octets that are no PLMN identity print as the octets that they are
    let none = PLMNidentity(TBCDSTRING([0xab, 0xcd, 0xef]));
    assert_eq!(format!("{none} {none:?}"), "abcdef PLMNidentity(abcdef)");
}

/// TS 36.413 clause 9.2.3.8: octet 2 has the first digit of an MNC of three digits, and
/// octet 3 its second and its third.
#[test]
fn a_plmn_identity_converts_to_and_from_a_plmn() {
    let network: Plmn = "310-410".parse().unwrap();
    let identity = PLMNidentity::from(network);
    assert_eq!(identity.0.0, [0x13, 0x40, 0x01]);
    assert_eq!(identity, plmn("310", "410").unwrap());
    assert_eq!(Plmn::try_from(identity), Ok(network));
    assert_eq!((network.mcc(), network.mnc()), ("310", "410"));

    // A filler in the MNC: no PLMN, and the error says the member
    let error = Plmn::try_from(PLMNidentity(TBCDSTRING([0x02, 0xf8, 0xf9]))).unwrap_err();
    assert_eq!(error.member, "mnc");
}

#[test]
fn an_argument_that_a_helper_does_not_take_is_an_error_and_no_panic() {
    let network = || plmn("208", "93").unwrap();

    // What panicked: an MCC of two digits, and an MNC of one.
    let panicked = [plmn("20", "93").unwrap_err(), plmn("208", "9").unwrap_err()];
    // What was cut short: a cell over the 8 bits after a macro eNB ID in a Cell Identity
    // of 28 (TS 36.413 clauses 9.2.1.37 and 9.2.1.38).
    let enb = ENBID::macroENB_ID(oxirush_s1ap::Bits::new(1, 20).unwrap());
    let cut_short = [eutran_cgi(network(), &enb, 0x100).unwrap_err()];
    for error in panicked.iter().chain(&cut_short) {
        assert_eq!(error.kind(), ErrorKind::Argument, "{error}");
        assert!(std::error::Error::source(error).is_none());
    }
    assert_eq!(
        cut_short[0].to_string(),
        "the cell of the eNB 256 does not fit in 8 bits"
    );

    // A TAC is two octets (clause 9.2.3.7), which the type of the argument says.
    let area = tai(network(), [0, 1]);
    assert_eq!(area.tac.0[..], [0, 1]);
}

/// TS 36.413 clause 9.2.1.61: ENUMERATED (1s, 2s, 5s, 10s, 20s, 60s).
#[test]
fn a_time_to_wait_is_its_seconds() {
    let seconds = [
        TimeToWait::v1s,
        TimeToWait::v2s,
        TimeToWait::v5s,
        TimeToWait::v10s,
        TimeToWait::v20s,
        TimeToWait::v60s,
    ]
    .map(TimeToWait::seconds);
    assert_eq!(seconds, [1, 2, 5, 10, 20, 60]);
}

#[test]
fn security_capability_octets_lead_with_the_first_algorithm() {
    // 128-EEA1 and 128-EEA2; 128-EIA2.
    let capabilities = ue_security_capabilities(0xC0, 0x40);
    let leading = |bits: &BitString| (0..3).map(|at| bits.get(at).unwrap()).collect::<Vec<_>>();
    assert_eq!(
        leading(&capabilities.encryption_algorithms.0),
        [true, true, false]
    );
    assert_eq!(
        leading(&capabilities.integrity_protection_algorithms.0),
        [false, true, false]
    );
}

#[test]
fn a_plmn_identity_has_the_order_of_the_application_protocols() {
    let read = |identity: PLMNidentity| {
        let plmn = Plmn::try_from(identity).unwrap();
        (plmn.mcc().to_string(), plmn.mnc().to_string())
    };
    let identity = plmn("208", "93").unwrap();
    assert_eq!(identity.0.0, [0x02, 0xF8, 0x39]);
    assert_eq!(read(identity), ("208".into(), "93".into()));
    // The digits 9 9 9 0 7 0, two in each octet, the second over the first.
    let identity = plmn("999", "070").unwrap();
    assert_eq!(identity.0.0, [0x99, 0x09, 0x07]);
    assert_eq!(read(identity), ("999".into(), "070".into()));
    // What is no MCC or no MNC is an error of the argument.
    for (mcc, mnc) in [("20", "93"), ("208", "9300"), ("20a", "93"), ("", "")] {
        let error = plmn(mcc, mnc).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Argument, "{mcc}-{mnc}");
    }
}

/// Each alternative of an eNB ID of TS 36.413 clause 9.2.1.37 has its bits, the
/// leftmost of the Cell Identity, and the cell the rest of the 28.
#[test]
fn each_enb_id_has_its_bits_and_the_cell_the_rest() {
    let network = plmn("208", "93").unwrap();
    for (enb, bits) in [
        (ENBID::macroENB_ID as fn(Bits) -> ENBID, 20),
        (ENBID::homeENB_ID, 28),
        (ENBID::short_macroENB_ID, 18),
        (ENBID::long_macroENB_ID, 21),
    ] {
        let id = enb(Bits::new(1, bits).unwrap());
        let cell = ((1u32 << (28 - bits)) - 1) as u16;
        let cgi = eutran_cgi(network, &id, cell).unwrap();
        assert_eq!(cgi.cell_id.0.value(), (1 << (28 - bits)) | u64::from(cell));
        let beyond = u32::from(cell) + 1;
        if let Ok(beyond) = u16::try_from(beyond) {
            assert_eq!(
                eutran_cgi(network, &id, beyond).unwrap_err().to_string(),
                format!(
                    "the cell of the eNB {beyond} does not fit in {} bits",
                    28 - bits
                )
            );
        }
    }
    // The 20-bit eNB ID and the 8-bit cell: a 28-bit cell identity, octet-aligned
    // after the PLMN (X.691 (02/2021) §16.10).
    let enb = ENBID::macroENB_ID(Bits::new(0x12345, 20).unwrap());
    let cgi = eutran_cgi(network, &enb, 0x67).unwrap();
    assert_eq!(cgi.cell_id.0, Bits::new(0x123_4567, 28).unwrap());
    assert_eq!(
        oxirush_aper::encode(&cgi).unwrap(),
        [0x00, 0x02, 0xf8, 0x39, 0x12, 0x34, 0x56, 0x70]
    );
}

#[test]
fn pdu_encode_decode_round_trip() {
    let pdu = build_s1ap!(IGNORE UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )
    .expect("build PDU");
    let bytes = pdu.encode().expect("encode PDU");
    let decoded = S1APPDU::decode(&bytes).expect("decode PDU");
    assert_eq!(pdu, decoded);
    assert_eq!(
        decoded.kind(),
        oxirush_s1ap::S1apPduKind::UEContextReleaseRequest
    );
    assert_eq!(decoded.direction(), oxirush_s1ap::Direction::Initiating);
}
