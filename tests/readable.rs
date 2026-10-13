//! How an identity prints. It is printed whole, and as it is read: a PLMN identity as
//! its MCC and its MNC, and a fixed number of octets, as a TMSI or a TAC, in
//! hexadecimal, where `Debug` derived for a type gave a list of numbers.

use oxirush_s1ap::helpers::plmn;
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
    let printed = format!("{:?}", TAI::new(identity, TAC([0x00, 0x01]), None));
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
fn a_fixed_number_of_octets_prints_in_hexadecimal() {
    let tmsi = MTMSI([0xde, 0xad, 0xbe, 0xef]);
    assert_eq!(format!("{tmsi} {tmsi:?}"), "deadbeef MTMSI(deadbeef)");
    let tac = TAC([0x00, 0x01]);
    assert_eq!(format!("{tac} {tac:?}"), "0001 TAC(0001)");
    assert_eq!(format!("{:?}", GTPTEID([0, 0, 0, 1])), "GTPTEID(00000001)");

    // An S-TMSI is printed whole, with its M-TMSI as it is read
    let identity = STMSI::new(MMECode([0x01]), tmsi, None);
    let printed = format!("{identity:?}");
    assert!(printed.contains("MMECode(01)"), "{printed}");
    assert!(printed.contains("MTMSI(deadbeef)"), "{printed}");
}
