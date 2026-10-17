#![cfg(feature = "serde")]
//! `Serialize` and `Deserialize`, with the feature `serde`: a PDU as its tree, and each
//! other type of the bindings as its value in the form of its ASN.1 type.

use oxirush_s1ap::s1ap::*;
use serde_json::json;

const RELEASE: &str = "0012401500000300000002002a000800020007000240020280";

#[test]
fn a_pdu_is_serialized_as_its_tree_and_deserialized_from_it() {
    let tree = json!({"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
    ]});
    let pdu: S1AP_PDU = serde_json::from_value(tree.clone()).unwrap();
    assert_eq!(hex::encode(pdu.encode().unwrap()), RELEASE);
    assert_eq!(serde_json::to_value(&pdu).unwrap(), tree);
    assert_eq!(
        serde_json::to_string(&pdu).unwrap(),
        pdu.to_tree().unwrap().to_string()
    );
    // A PDU that borrows its octets is the same tree.
    let octets = hex::decode(RELEASE).unwrap();
    let borrowed = S1AP_PDU::decode_borrowed(&octets).unwrap();
    assert_eq!(serde_json::to_value(&borrowed).unwrap(), tree);

    // What the tree refuses, the format refuses with the same words.
    let wrong = json!({"message": "UEContextReleaseRequest", "ies": [{"MME-UE-S1AP-ID": -1}]});
    let error = serde_json::from_value::<S1AP_PDU>(wrong.clone()).unwrap_err();
    let refused = S1AP_PDU::from_tree(&wrong).unwrap_err();
    assert_eq!(error.to_string(), refused.to_string());

    // A PDU without a tree, of a procedure that the protocol does not have in its
    // direction, is not serialized.
    let unknown = hex::decode("00ff0003000000").unwrap();
    let unknown = S1AP_PDU::decode(&unknown).unwrap();
    let error = serde_json::to_value(&unknown).unwrap_err();
    assert_eq!(
        error.to_string(),
        unknown.to_tree().unwrap_err().to_string()
    );
}

#[test]
fn a_type_of_the_bindings_is_serialized_as_the_form_of_its_asn1_type() {
    let cause = Cause::radioNetwork(CauseRadioNetwork::user_inactivity);
    let written = json!({"radioNetwork": "user-inactivity"});
    assert_eq!(serde_json::to_value(&cause).unwrap(), written);
    assert_eq!(serde_json::from_value::<Cause>(written).unwrap(), cause);

    // The octets of a string, in hexadecimal: a PLMN identity and a TAC are octets
    // here, where the tree of a PDU shows them as they are usually written.
    let area = TAI::new(
        PLMNidentity(TBCDSTRING([0x02, 0xf8, 0x39])),
        TAC([0, 0xab]),
        None,
    );
    let written = json!({"pLMNidentity": "02f839", "tAC": "00ab"});
    assert_eq!(serde_json::to_value(&area).unwrap(), written);
    assert_eq!(serde_json::from_value::<TAI>(written).unwrap(), area);
    let upper: TAI =
        serde_json::from_value(json!({"pLMNidentity": "02F839", "tAC": "00AB"})).unwrap();
    assert_eq!(upper, area);

    // A number outside the constraint of its type is refused, as the tree refuses it.
    let error = serde_json::from_value::<MME_UE_S1AP_ID>(json!(1u64 << 32)).unwrap_err();
    assert_eq!(
        error.to_string(),
        "this is not a value of MME-UE-S1AP-ID: a number from 0 to 4294967295 is expected, \
         and 4294967296 is written"
    );
}
