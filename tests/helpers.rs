//! The helpers that build the values that a program of S1AP writes most from their
//! parts, and what they refuse, and the PLMN identity of S1AP, a TBCD-STRING, which
//! prints as its MCC and its MNC. TOY, the protocol of the tests of oxirush-aper, has
//! the tests of how the other identities print.

use oxirush_s1ap::ErrorKind;
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
