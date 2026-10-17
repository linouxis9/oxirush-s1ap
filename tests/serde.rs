#![cfg(feature = "serde")]
//! `Serialize` and `Deserialize`, with the feature `serde`: a PDU as its tree, and each
//! other type of the bindings as its tree too, which is the form that the tree of a PDU
//! shows it in.

use oxirush_s1ap::build_s1ap;
use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{tree, tree::Value};

const RELEASE: &str = "0012401500000300000002002a000800020007000240020280";
const LATER: &str = include_str!("fixtures/later.tsv");

/// What serde_json writes of `value`, as a tree.
fn written<T: serde_core::Serialize + ?Sized>(value: &T) -> Value {
    serde_json::to_string(value).unwrap().parse().unwrap()
}

/// What serde_json reads of the text of `tree`.
fn read<T: serde_core::de::DeserializeOwned>(tree: &Value) -> Result<T, serde_json::Error> {
    serde_json::from_str(&tree.to_string())
}

#[test]
fn a_pdu_is_serialized_as_its_tree_and_deserialized_from_it() {
    let tree = tree!({"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
    ]});
    let pdu: S1AP_PDU = read(&tree).unwrap();
    assert_eq!(hex::encode(pdu.encode().unwrap()), RELEASE);
    assert_eq!(written(&pdu), tree);
    assert_eq!(
        serde_json::to_string(&pdu).unwrap(),
        pdu.to_tree().to_string()
    );
    // A PDU that borrows its octets is the same tree.
    let octets = hex::decode(RELEASE).unwrap();
    let borrowed = S1AP_PDU::decode_borrowed(&octets).unwrap();
    assert_eq!(written(&borrowed), tree);

    // What the tree refuses, the format refuses with the same words.
    let wrong = tree!({"message": "UEContextReleaseRequest", "ies": [{"MME-UE-S1AP-ID": -1}]});
    let error = read::<S1AP_PDU>(&wrong).unwrap_err();
    let refused = S1AP_PDU::from_tree(&wrong).unwrap_err();
    assert!(
        error.to_string().starts_with(&refused.to_string()),
        "{error}"
    );
}

/// The fifth finding of the review of the move of the paths to oxirush-utils: a PDU of a
/// procedure that the protocol does not have, and a private message, decoded and could
/// not be serialized.
#[test]
fn every_pdu_that_decodes_is_serialized_and_read_back() {
    for (octets, tree) in [
        // A procedure that the protocol does not have: its code and its direction.
        (
            "00ff0003000000",
            tree!({"procedure_code": 255, "direction": "InitiatingMessage", "ies": [],
                   "criticality": "reject"}),
        ),
        // A private message, which has no list of IEs: its octets.
        (
            "002740090000000000000001a1",
            tree!({"message": "PrivateMessage", "octets": "0000000000000001a1"}),
        ),
        // A message that does not decode as its type: its octets.
        (
            "0012400300ffff",
            tree!({"message": "UEContextReleaseRequest", "octets": "00ffff"}),
        ),
    ] {
        let pdu = S1AP_PDU::decode(&hex::decode(octets).unwrap()).unwrap();
        assert_eq!(written(&pdu), tree, "{octets}");
        let back: S1AP_PDU = read(&tree).unwrap();
        assert_eq!(hex::encode(back.encode().unwrap()), octets);
    }
}

/// The second finding of the review of the move of the paths to oxirush-utils: a round
/// trip through the tree or serde gave the octets without what a later release added.
/// An IE whose typed value has no place for it is its octets in the tree, and every
/// vector of `later.tsv` comes back as it was received.
#[test]
fn a_round_trip_gives_back_what_a_later_release_added() {
    let mut count = 0;
    for line in LATER.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<_> = line.split('\t').collect();
        let (name, later) = (fields[0], fields[1]);
        let pdu = S1AP_PDU::decode(&hex::decode(later).unwrap()).unwrap();
        let tree = pdu.to_tree();
        let again = S1AP_PDU::from_tree(&tree).unwrap();
        assert_eq!(hex::encode(again.encode().unwrap()), later, "{name}");
        let back: S1AP_PDU = read(&written(&pdu)).unwrap();
        assert_eq!(hex::encode(back.encode().unwrap()), later, "{name}");
        // What is kept as octets is an IE of the message, or what one holds.
        assert!(tree.to_string().contains("\"octets\""), "{name}: {tree}");
        count += 1;
    }
    assert_eq!(count, 9);
}

/// The sixth finding of the review of the move of the paths to oxirush-utils: a value
/// serialized alone had the form of its ASN.1 type, and the tree of a PDU the readable
/// one. It has the readable one alone too, and is read in either.
#[test]
fn a_type_of_the_bindings_is_serialized_as_the_tree_of_a_pdu_shows_it() {
    let cause = Cause::radioNetwork(CauseRadioNetwork::user_inactivity);
    let shown = tree!({"radioNetwork": "user-inactivity"});
    assert_eq!(written(&cause), shown);
    assert_eq!(read::<Cause>(&shown).unwrap(), cause);

    // A PLMN identity is its MCC and its MNC, a TAC a number, and the parts of a GUMMEI
    // numbers.
    let network = PLMNidentity(TBCDSTRING([0x02, 0xf8, 0x39]));
    let area = TAI::new(network, TAC([0, 0xab]), None);
    let shown = tree!({"pLMNidentity": "208-93", "tAC": 171});
    assert_eq!(written(&area), shown);
    assert_eq!(read::<TAI>(&shown).unwrap(), area);
    assert_eq!(written(&network), tree!("208-93"));
    let gummei = oxirush_s1ap::helpers::gummei(network, 1, 2);
    assert_eq!(
        written(&gummei),
        tree!({"pLMN-Identity": "208-93", "mME-Group-ID": 1, "mME-Code": 2})
    );
    // The form of the ASN.1 type is read too, in either case.
    let jer = tree!({"pLMNidentity": "02F839", "tAC": "00AB"});
    assert_eq!(read::<TAI>(&jer).unwrap(), area);
    // Octets that are no PLMN identity stay octets.
    let other = PLMNidentity(TBCDSTRING([0xff, 0xff, 0xff]));
    assert_eq!(written(&other), tree!("ffffff"));
    assert_eq!(read::<PLMNidentity>(&tree!("ffffff")).unwrap(), other);
    // The same value in a tree of a PDU.
    let pdu = build_s1ap!(InitiatingMessage, initialUEMessage, IGNORE, InitialUEMessage,
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT NAS_PDU(vec![0x07, 0x41]),
        REJECT TAI(area.clone()),
    )
    .unwrap();
    assert_eq!(pdu.to_tree()["ies"][2]["TAI"], written(&area));

    // A number outside the constraint of its type is refused, as the tree refuses it.
    let error = read::<MME_UE_S1AP_ID>(&tree!(1u64 << 32)).unwrap_err();
    assert_eq!(
        error.to_string(),
        "this is not a value of MME-UE-S1AP-ID: a number from 0 to 4294967295 is expected, \
         and 4294967296 is written"
    );
}

/// The seventh finding of the review of the move of the paths to oxirush-utils: a name
/// written twice in an object of JSON was read as its last value.
#[test]
fn a_name_written_twice_is_refused() {
    let twice = r#"{"message":"S1SetupRequest","message":"UEContextReleaseRequest","ies":[]}"#;
    let error = serde_json::from_str::<S1AP_PDU>(twice).unwrap_err();
    assert!(
        error
            .to_string()
            .starts_with("the member \"message\" is written twice in an object"),
        "{error}"
    );
    let twice = r#"{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42,"MME-UE-S1AP-ID":43}]}"#;
    assert!(serde_json::from_str::<S1AP_PDU>(twice).is_err());
    assert!(twice.parse::<Value>().is_err());
    let twice = r#"{"tAC":1,"tAC":2}"#;
    assert!(serde_json::from_str::<TAI>(twice).is_err());
}
