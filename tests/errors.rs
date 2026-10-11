//! The errors of the crate, each where a caller meets it: its kind, which is what a
//! caller matches on, and what its message says.

use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{Error, ErrorKind, Message};
use rasn::types::Any;

/// A UE CONTEXT RELEASE REQUEST with its three IEs, the message of the README.
const RELEASE_REQUEST: &str = "0012401500000300000002002a000800020007000240020280";

fn release_request() -> S1AP_PDU {
    S1AP_PDU::decode(&hex::decode(RELEASE_REQUEST).unwrap()).unwrap()
}

/// What an error has for the one who handles it.
fn source(error: &Error) -> Option<String> {
    std::error::Error::source(error).map(|source| source.to_string())
}

#[test]
fn a_pdu_is_decoded_as_the_message_that_it_holds_and_as_no_other() {
    let pdu = release_request();
    let request: UEContextReleaseRequest = pdu.decode_value().unwrap();
    assert_eq!(request.protocol_ies.0.len(), 3);

    // The octets of the message are those of an S1 SETUP REQUEST too, which is a list
    // of IEs as well: nothing but the procedure of the PDU says which message it is.
    let S1AP_PDU::initiatingMessage(held) = &pdu else {
        panic!("an initiating message");
    };
    let mistaken: S1SetupRequest = decode_open_type(&held.value).unwrap();
    assert_eq!(mistaken.protocol_ies.0.len(), 3);

    let error = pdu.decode_value::<S1SetupRequest>().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::WrongMessage);
    assert_eq!(
        error.to_string(),
        "the message of this PDU is UEContextReleaseRequest, not S1SetupRequest: kind() \
         says which message a PDU holds"
    );
    assert_eq!(source(&error), None);
}

#[test]
fn a_message_of_the_same_procedure_in_another_direction_is_another_message() {
    // An S1 SETUP FAILURE: the unsuccessful outcome of the procedure 17.
    let failure = S1AP_PDU::decode(&hex::decode("401100080000010002400145").unwrap()).unwrap();
    assert!(failure.decode_value::<S1SetupFailure>().is_ok());
    for kind in [
        failure.decode_value::<S1SetupRequest>().unwrap_err().kind(),
        failure
            .decode_value::<S1SetupResponse>()
            .unwrap_err()
            .kind(),
    ] {
        assert_eq!(kind, ErrorKind::WrongMessage);
    }
}

#[test]
fn a_pdu_of_a_procedure_that_the_specification_does_not_have_is_no_message() {
    let S1AP_PDU::initiatingMessage(mut held) = release_request() else {
        panic!("an initiating message");
    };
    held.procedure_code = ProcedureCode(200);
    let pdu = S1AP_PDU::initiatingMessage(held);
    let error = pdu.decode_value::<UEContextReleaseRequest>().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::WrongMessage);
    assert_eq!(
        error.to_string(),
        "the message of this PDU is not UEContextReleaseRequest: the specification has no \
         InitiatingMessage of procedure 200"
    );
}

#[test]
fn the_type_of_a_message_says_which_message_it_is() {
    assert_eq!(UEContextReleaseRequest::NAME, "UEContextReleaseRequest");
    assert_eq!(UEContextReleaseRequest::DIRECTION, "InitiatingMessage");
    assert_eq!(UEContextReleaseRequest::PROCEDURE_CODE, 18);
    // The name is the one that ASN.1 gives the message, which is not always that of
    // its procedure.
    assert_eq!(S1SetupFailure::NAME, "S1SetupFailure");
    assert_eq!(S1SetupFailure::DIRECTION, "UnsuccessfulOutcome");
    assert_eq!(S1SetupFailure::PROCEDURE_CODE, 17);
    let pdu = release_request();
    let of = |direction, code| (pdu.direction(), pdu.procedure_code()) == (direction, code);
    assert!(of(
        UEContextReleaseRequest::DIRECTION,
        UEContextReleaseRequest::PROCEDURE_CODE
    ));
}

#[test]
fn octets_that_are_no_pdu_are_a_decode_error() {
    let octets = hex::decode(RELEASE_REQUEST).unwrap();

    // No octet, octets that end early, and an octet after the PDU.
    let none = S1AP_PDU::decode(&[]).unwrap_err();
    assert_eq!(none.kind(), ErrorKind::Decode);
    assert_eq!(
        none.to_string(),
        "these octets do not decode as S1AP-PDU: there are none"
    );
    for cut in 1..octets.len() {
        let short = S1AP_PDU::decode(&octets[..cut]).unwrap_err();
        assert_eq!(short.kind(), ErrorKind::Decode, "{cut}");
        let text = short.to_string();
        assert!(
            text.starts_with("these octets do not decode as S1AP-PDU: "),
            "{text}"
        );
        // What rasn says is the source, and is not the message.
        assert!(!text.contains("Codec"), "{text}");
        assert!(source(&short).is_some_and(|source| source.contains("Codec: APER")));
        // And is that error itself, for the one who reads its kind.
        let rasn = std::error::Error::source(&short).unwrap();
        assert!(rasn.downcast_ref::<rasn::error::DecodeError>().is_some());
    }
    let long = S1AP_PDU::decode(&[&octets[..], &[0]].concat()).unwrap_err();
    assert_eq!(long.kind(), ErrorKind::Decode);
    assert_eq!(
        long.to_string(),
        "these octets do not decode as S1AP-PDU: the value is followed by 1 octet"
    );
}

#[test]
fn an_open_type_that_is_not_the_value_asked_for_is_a_decode_error() {
    // An MME UE S1AP ID is an INTEGER (0..4294967295): a length of one octet, then
    // the octet.
    let id: MMEUES1APID = decode_open_type(&Any::new(vec![0x00, 0x2a])).unwrap();
    assert_eq!(id.0, 42);
    let short = decode_open_type::<MMEUES1APID>(&Any::new(vec![0x40, 0x2a])).unwrap_err();
    assert_eq!(short.kind(), ErrorKind::Decode);
    assert_eq!(
        short.to_string(),
        "these octets do not decode as MME-UE-S1AP-ID: they end before the value does"
    );
    let long = decode_open_type::<MMEUES1APID>(&Any::new(vec![0x00, 0x2a, 0x00, 0x00]));
    assert_eq!(
        long.unwrap_err().to_string(),
        "these octets do not decode as MME-UE-S1AP-ID: the value is followed by 2 octets"
    );
}

#[test]
fn a_value_outside_its_constraint_is_an_encode_error() {
    // INTEGER (1..256), TS 36.413 clause 9.2.1.39.
    let error = encode_open_type(&SubscriberProfileIDforRFP(1000)).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Encode);
    assert_eq!(
        error.to_string(),
        "a value of SubscriberProfileIDforRFP does not encode: 1000 is outside its \
         constraint (1..256)"
    );
    assert!(source(&error).is_some_and(|source| source.contains("Codec: APER")));
}

#[test]
fn an_error_is_the_error_of_a_function_of_the_caller() {
    // With `?`, in a function whose error is this one or holds any error.
    fn code(octets: &[u8]) -> Result<u8, Error> {
        Ok(S1AP_PDU::decode(octets)?.procedure_code())
    }
    fn boxed(octets: &[u8]) -> Result<u8, Box<dyn std::error::Error + Send + Sync>> {
        Ok(S1AP_PDU::decode(octets)?.procedure_code())
    }
    assert_eq!(code(&hex::decode(RELEASE_REQUEST).unwrap()), Ok(18));
    assert_eq!(code(&[]).unwrap_err().kind(), ErrorKind::Decode);
    let error = boxed(&[]).unwrap_err();
    let error = error
        .downcast_ref::<Error>()
        .expect("the error of the crate");
    assert_eq!(error.kind(), ErrorKind::Decode);
}
