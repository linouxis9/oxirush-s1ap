//! The errors of the crate, each where a caller meets it: its kind, which is what a
//! caller matches on, and what its message says.

use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{
    AsnType, Error, ErrorKind, Message, Octets, build_s1ap, build_s1ap_ie, extract_s1ap_ies,
    with_s1ap_ie_mut,
};

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
        "these octets do not decode as S1AP-PDU: there are no octets"
    );
    for cut in 1..octets.len() {
        let short = S1AP_PDU::decode(&octets[..cut]).unwrap_err();
        assert_eq!(short.kind(), ErrorKind::Decode, "{cut}");
        let text = short.to_string();
        assert!(
            text.starts_with("these octets do not decode as S1AP-PDU: "),
            "{text}"
        );
        // The codec is that of the family: the error is whole, and has no source.
        assert_eq!(source(&short), None);
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
    let id: MMEUES1APID = decode_open_type(&Octets::from(vec![0x00, 0x2a])).unwrap();
    assert_eq!(id.0, 42);
    let short = decode_open_type::<MMEUES1APID>(&Octets::from(vec![0x40, 0x2a])).unwrap_err();
    assert_eq!(short.kind(), ErrorKind::Decode);
    assert_eq!(
        short.to_string(),
        "these octets do not decode as MME-UE-S1AP-ID: the octets end before the value does"
    );
    let long = decode_open_type::<MMEUES1APID>(&Octets::from(vec![0x00, 0x2a, 0x00, 0x00]));
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
    assert_eq!(source(&error), None);
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

/// What the macro reads in a request: an identifier that it requires, and one that
/// the request may be without.
fn ids(request: &UEContextReleaseRequest) -> Result<(u32, Option<u32>), Error> {
    extract_s1ap_ies!(request, UEContextReleaseRequest,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        opt enb_id: u32 = eNB_UE_S1AP_ID(id),
    );
    Ok((mme_id, enb_id))
}

fn request(ies: Vec<ProtocolIEField>) -> UEContextReleaseRequest {
    UEContextReleaseRequest::new(ProtocolIEContainer(ies))
}

/// The entry of an MME UE S1AP ID and that of an eNB UE S1AP ID.
fn mme_and_enb() -> (ProtocolIEField<'static>, ProtocolIEField<'static>) {
    let mme = build_s1ap_ie!(UEContextReleaseRequest, REJECT MME_UE_S1AP_ID(42u32));
    let enb = build_s1ap_ie!(UEContextReleaseRequest, REJECT eNB_UE_S1AP_ID(7u32));
    (mme.unwrap(), enb.unwrap())
}

/// An entry with the identifier of an IE, whose octets are not the value of that IE.
fn undecodable(id: u16) -> ProtocolIEField<'static> {
    ProtocolIEField::new(id, Criticality::reject, Octets::from(vec![0xff]))
}

#[test]
fn a_required_ie_that_is_absent_is_missing_and_an_optional_one_is_none() {
    let (mme, enb) = mme_and_enb();
    assert_eq!(
        ids(&request(vec![mme.clone(), enb.clone()])),
        Ok((42, Some(7)))
    );
    assert_eq!(ids(&request(vec![mme])), Ok((42, None)));

    let missing = ids(&request(vec![enb])).unwrap_err();
    assert_eq!(missing.kind(), ErrorKind::MissingIe);
    assert_eq!(
        missing.to_string(),
        "this UEContextReleaseRequest has no MME-UE-S1AP-ID, which the field `mme_id` requires"
    );
    assert_eq!(source(&missing), None);
}

#[test]
fn an_ie_that_does_not_decode_is_an_error_for_a_required_and_for_an_optional_field() {
    let (mme, enb) = mme_and_enb();

    // The one that is required.
    let required = ids(&request(vec![undecodable(0), enb])).unwrap_err();
    assert_eq!(required.kind(), ErrorKind::UndecodableIe);
    assert_eq!(
        required.to_string(),
        "the MME-UE-S1AP-ID of this UEContextReleaseRequest does not decode: the octets end \
         before the value does"
    );
    assert_eq!(source(&required), None);

    // The one that may be absent is there: it is not `None`.
    let optional = ids(&request(vec![mme, undecodable(8)])).unwrap_err();
    assert_eq!(optional.kind(), ErrorKind::UndecodableIe);
    let text = optional.to_string();
    assert!(
        text.starts_with("the eNB-UE-S1AP-ID of this UEContextReleaseRequest does not decode: "),
        "{text}"
    );
}

/// The UE CONTEXT RELEASE REQUEST of the README with one octet of its Cause spoiled:
/// `72` for `02`, which is the alternative 7 of a CHOICE of five. The IE has the
/// criticality "ignore", so a receiver that does not comprehend it goes on as if it had
/// not been received (TS 36.413 clause 10.3.4.2).
const SPOILED_CAUSE: &str = "0012401500000300000002002a000800020007000240027280";

fn spoiled_request() -> UEContextReleaseRequest<'static> {
    let pdu = S1AP_PDU::decode(&hex::decode(SPOILED_CAUSE).unwrap()).unwrap();
    let request: UEContextReleaseRequest = pdu.decode_value().unwrap();
    request.into_owned()
}

/// What a receiver reads in a request when it takes the cause if it can.
fn id_and_cause(request: &UEContextReleaseRequest) -> Result<(u32, Option<Cause>), Error> {
    extract_s1ap_ies!(request, UEContextReleaseRequest,
        req id: u32 = MME_UE_S1AP_ID(id),
        lax cause: Cause = Cause(cause) => cause,
    );
    Ok((id, cause))
}

/// A call with `lax` fields alone has no error: its function returns no `Result`.
fn cause_if_read(request: &UEContextReleaseRequest) -> Option<Cause> {
    extract_s1ap_ies!(request, UEContextReleaseRequest,
        lax cause: Cause = Cause(cause) => cause,
    );
    cause
}

fn optional_cause(request: &UEContextReleaseRequest) -> Result<Option<Cause>, Error> {
    extract_s1ap_ies!(request, UEContextReleaseRequest,
        opt cause: Cause = Cause(cause) => cause,
    );
    Ok(cause)
}

fn required_cause(request: &UEContextReleaseRequest) -> Result<Cause, Error> {
    extract_s1ap_ies!(request, UEContextReleaseRequest,
        req cause: Cause = Cause(cause) => cause,
    );
    Ok(cause)
}

#[test]
fn a_lax_field_is_the_ie_if_it_decodes_and_none_otherwise() {
    let inactivity = Cause::radioNetwork(CauseRadioNetwork::user_inactivity);

    // The IE is there and decodes.
    let pdu = release_request();
    let received: UEContextReleaseRequest = pdu.decode_value().unwrap();
    assert_eq!(id_and_cause(&received), Ok((42, Some(inactivity.clone()))));
    assert_eq!(cause_if_read(&received), Some(inactivity));

    // It is absent.
    let (mme, _) = mme_and_enb();
    let without = request(vec![mme]);
    assert_eq!(id_and_cause(&without), Ok((42, None)));
    assert_eq!(cause_if_read(&without), None);

    // It is there and does not decode: the message that was received, with one octet
    // of the value of the IE spoiled.
    let spoiled = spoiled_request();
    assert_eq!(spoiled.protocol_ies.0[2].criticality, Criticality::ignore);
    assert_eq!(id_and_cause(&spoiled), Ok((42, None)));
    assert_eq!(cause_if_read(&spoiled), None);

    // The same octets are an error for an `opt` and for a `req` field.
    let optional = optional_cause(&spoiled).unwrap_err();
    let required = required_cause(&spoiled).unwrap_err();
    assert_eq!(optional, required);
    assert_eq!(optional.kind(), ErrorKind::UndecodableIe);
    assert_eq!(
        optional.to_string(),
        "the Cause of this UEContextReleaseRequest does not decode: 7 is no alternative of \
         the CHOICE"
    );
}

#[test]
fn the_last_of_an_ie_that_is_there_twice_decides_a_lax_field() {
    let inactivity = Cause::radioNetwork(CauseRadioNetwork::user_inactivity);
    let pdu = release_request();
    let received: UEContextReleaseRequest = pdu.decode_value().unwrap();
    let read = received.protocol_ies.0[2].clone();
    let unread = spoiled_request().protocol_ies.0[2].clone();

    // The last one does not decode: the field is `None`, though the first one decodes.
    let last_unread = request(vec![read.clone(), unread.clone()]);
    assert_eq!(cause_if_read(&last_unread), None);

    // The last one decodes: the field has it, whatever came before it.
    let last_read = request(vec![unread, read]);
    assert_eq!(cause_if_read(&last_read), Some(inactivity));

    // One that does not decode is an error for an `opt` and for a `req` field,
    // wherever it is.
    for request in [&last_unread, &last_read] {
        let optional = optional_cause(request).unwrap_err();
        assert_eq!(optional.kind(), ErrorKind::UndecodableIe);
        let required = required_cause(request).unwrap_err();
        assert_eq!(required.kind(), ErrorKind::UndecodableIe);
    }
}

#[test]
fn a_value_outside_its_constraint_is_an_error_of_the_macros_that_build() {
    // An eNB UE S1AP ID is an INTEGER (0..16777215), TS 36.413 clause 9.2.3.4, and the
    // macro takes a `u32`.
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(1u32 << 24),
    );
    let error = pdu.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Encode);
    assert_eq!(
        error.to_string(),
        "a value of ENB-UE-S1AP-ID does not encode: 16777216 is outside its constraint \
         (0..16777215)"
    );
    let ie = build_s1ap_ie!(UEContextReleaseRequest, REJECT eNB_UE_S1AP_ID(1u32 << 24));
    assert_eq!(ie.unwrap_err(), error);

    // The last value of the constraint is built.
    let last = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT eNB_UE_S1AP_ID((1u32 << 24) - 1),
    );
    assert!(last.is_ok());
}

#[test]
fn changing_an_ie_says_whether_it_did_or_why_it_did_not() {
    let (_, enb) = mme_and_enb();
    let mut message = request(vec![enb]);

    // The IE is changed, another one is not there, and the expression says no.
    let changed = with_s1ap_ie_mut!(message, UEContextReleaseRequest, eNB_UE_S1AP_ID(id) = 8u32);
    assert_eq!(changed, Ok(true));
    let absent = with_s1ap_ie_mut!(message, UEContextReleaseRequest, MME_UE_S1AP_ID(id) = 43u32);
    assert_eq!(absent, Ok(false));
    let refused =
        with_s1ap_ie_mut!(message, UEContextReleaseRequest, eNB_UE_S1AP_ID(id) => id.0 == 0);
    assert_eq!(refused, Ok(false));
    assert_eq!(ids(&message).unwrap_err().kind(), ErrorKind::MissingIe);

    // A value that does not encode: the message is as it was.
    let before = message.clone();
    let outside = with_s1ap_ie_mut!(
        message,
        UEContextReleaseRequest,
        eNB_UE_S1AP_ID(id) = 1u32 << 24
    );
    assert_eq!(outside.unwrap_err().kind(), ErrorKind::Encode);
    assert_eq!(message, before);

    // An IE that does not decode is not changed either.
    let mut message = request(vec![undecodable(8)]);
    let before = message.clone();
    let unread = with_s1ap_ie_mut!(message, UEContextReleaseRequest, eNB_UE_S1AP_ID(id) = 1u32);
    assert_eq!(unread.unwrap_err().kind(), ErrorKind::UndecodableIe);
    assert_eq!(message, before);
}

#[test]
fn an_argument_that_a_helper_does_not_take_is_an_error_and_no_panic() {
    use oxirush_s1ap::helpers::*;
    let network = || plmn("208", "93").unwrap();

    // What panicked: an MCC of two digits, an MNC of one, and a width over the 64 bits
    // of a number.
    let panicked = [
        plmn("20", "93").unwrap_err(),
        plmn("208", "9").unwrap_err(),
        oxirush_s1ap::Bits::new(1, 65).unwrap_err(),
    ];
    // What was cut short: a macro eNB ID over 20 bits (TS 36.413 clause 9.2.1.37),
    // alone and before the 8 bits of a cell in a Cell Identity of 28 (clause 9.2.1.38),
    // and a number over the width of its bit string.
    let cut_short = [
        global_enb_id(network(), 0x1f_ffff).unwrap_err(),
        eutran_cgi(network(), 0x1f_ffff, 1).unwrap_err(),
        oxirush_s1ap::Bits::new(0x100, 8).unwrap_err(),
    ];
    for error in panicked.iter().chain(&cut_short) {
        assert_eq!(error.kind(), ErrorKind::Argument, "{error}");
        assert_eq!(source(error), None);
    }
    assert_eq!(
        cut_short[0].to_string(),
        "the macro eNB ID 2097151 does not fit in 20 bits"
    );

    // A TAC is two octets (clause 9.2.3.7), which the type of the argument says.
    let area = tai(network(), [0, 1]);
    assert_eq!(area.t_ac.0[..], [0, 1]);
}

/// The errors of the `inspect` feature: a test for each kind, as a caller matches on it.
#[cfg(feature = "inspect")]
mod inspect_errors {
    use super::release_request;
    use oxirush_s1ap::Octets;
    use oxirush_s1ap::s1ap::*;
    use oxirush_s1ap::{Error, ErrorKind, inspect};
    use serde_json::{Value, json};

    /// The view of the UE CONTEXT RELEASE REQUEST.
    fn view() -> Value {
        release_request().to_view().unwrap()
    }

    /// The kind of the error that a call returned.
    fn kind<T: std::fmt::Debug>(result: Result<T, Error>) -> ErrorKind {
        result.unwrap_err().kind()
    }

    /// The PDU of a tree: a release request with these IEs.
    fn written(ies: Value) -> Result<S1AP_PDU, Error> {
        S1AP_PDU::from_tree(&json!({"message": "UEContextReleaseRequest", "ies": ies}))
    }

    /// A release request whose procedure code is one that the specification does not have.
    fn of_no_procedure() -> S1AP_PDU {
        let S1AP_PDU::initiatingMessage(mut held) = release_request() else {
            panic!("an initiating message");
        };
        held.procedure_code = ProcedureCode(200);
        S1AP_PDU::initiatingMessage(held)
    }

    #[test]
    fn a_message_that_the_protocol_does_not_have_is_unknown() {
        // A name that is no message, for a path and in a tree.
        let checked = inspect::check_path("NoSuchMessage", "/s1ap/Cause");
        assert_eq!(kind(checked), ErrorKind::UnknownMessage);
        let names = inspect::enumerated_at("NoSuchMessage", "/s1ap/Cause");
        assert_eq!(kind(names), ErrorKind::UnknownMessage);
        let tree = json!({"message": "NoSuchMessage", "ies": []});
        let error = S1AP_PDU::from_tree(&tree).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownMessage);
        assert_eq!(
            error.to_string(),
            "\"NoSuchMessage\" is not a message of S1AP"
        );

        // A procedure that the specification does not have, in a PDU and in a view.
        let error = of_no_procedure().to_view().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownMessage);
        assert_eq!(
            error.to_string(),
            "S1AP has no InitiatingMessage of procedure 200"
        );
        assert_eq!(kind(of_no_procedure().to_tree()), ErrorKind::UnknownMessage);
        let mut view = view();
        view["procedure_code"] = json!(200);
        assert_eq!(kind(S1AP_PDU::from_view(&view)), ErrorKind::UnknownMessage);
    }

    #[test]
    fn an_ie_that_the_protocol_does_not_have_is_unknown() {
        // A name that is no IE: after the root of a path, in a tree, and as the `id` of
        // an entry that an edit writes.
        let mut view = view();
        let error = inspect::select(&view, "/s1ap/NoSuchIe/value").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownIe);
        assert_eq!(
            error.to_string(),
            "\"NoSuchIe\" is not an IE of S1AP at /s1ap/NoSuchIe/value"
        );
        assert_eq!(
            kind(written(json!([{"NoSuchIe": 1}]))),
            ErrorKind::UnknownIe
        );
        let entry = json!({"id": "NoSuchIe", "criticality": "ignore", "value": 1});
        assert_eq!(
            kind(inspect::insert(&mut view, "/s1ap/-", entry)),
            ErrorKind::UnknownIe
        );

        // An identifier that has no type is written by its octets: its value has no
        // type to be encoded as.
        let error = written(json!([{"60000": 1, "criticality": "ignore"}])).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnknownIe);
        assert_eq!(
            error.to_string(),
            "S1AP IE 60000 is unknown: give its octets as _raw_value, without value"
        );
        let octets = json!([{"60000": {"octets": "c0ffee"}, "criticality": "ignore"}]);
        assert!(written(octets).is_ok());
    }

    #[test]
    fn a_path_that_is_not_well_formed_is_a_path_error() {
        let view = view();
        // No `/` before the first segment, `~` before what is no 0 and no 1, and
        // `@id=` before what is no identifier of an IE.
        for path in [
            "s1ap/Cause",
            "",
            "/s1ap/Cause/~2",
            "/s1ap/@id=x",
            "/s1ap/@id=70000",
        ] {
            assert_eq!(
                kind(inspect::select(&view, path)),
                ErrorKind::Path,
                "{path}"
            );
            let checked = inspect::check_path("UEContextReleaseRequest", path);
            assert_eq!(kind(checked), ErrorKind::Path, "{path}");
        }
        let error = inspect::select(&view, "s1ap/Cause").unwrap_err();
        assert_eq!(
            error.to_string(),
            "\"s1ap/Cause\" is no path: a path starts with \"/\", as /s1ap/0"
        );
    }

    #[test]
    fn a_name_that_the_type_at_its_place_cannot_have_is_no_member() {
        let mut view = view();
        // A member that a CHOICE does not have, a member of a value that has none, the
        // identifier of an IE where there is no list, and a name where a list goes by
        // position.
        let error = inspect::select(&view, "/s1ap/Cause/radioNetwrok").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NoSuchMember);
        let text = error.to_string();
        assert!(
            text.starts_with("\"radioNetwrok\" is not a member of Cause, which has radioNetwork, "),
            "{text}"
        );
        for path in [
            "/s1ap/Cause/radioNetwork/deeper",
            "/s1ap/Cause/@id=1",
            "/message/protocolIEs/first",
        ] {
            assert_eq!(
                kind(inspect::select(&view, path)),
                ErrorKind::NoSuchMember,
                "{path}"
            );
        }
        // An edit at such a path, and the view is as it was.
        let before = view.clone();
        let edit = inspect::set(&mut view, "/s1ap/Cause/radioNetwrok", json!(1));
        assert_eq!(kind(edit), ErrorKind::NoSuchMember);
        assert_eq!(view, before);
        // An IE that the object set of the message does not have, without a view.
        let checked = inspect::check_path("UEContextReleaseRequest", "/s1ap/GUMMEI-ID");
        assert_eq!(kind(checked), ErrorKind::NoSuchMember);
        // A member that the type of a written value does not have.
        let error = S1AP_PDU::from_tree(
            &json!({"message": "S1SetupRequest", "ies": [{"Global-ENB-ID": {
                "pLMNidentity": "208-93",
                "eNB-ID": {"macroENB-ID": "123450"},
                "other": 2,
            }}]}),
        )
        .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NoSuchMember);
        assert_eq!(
            error.to_string(),
            "this is not a value of Global-ENB-ID: \"other\" is not a member that its type has"
        );
    }

    #[test]
    fn an_edit_whose_path_selects_nothing_is_an_error() {
        let mut view = view();
        let before = view.clone();
        // An IE that the message could have and does not have, and a position after
        // its last IE: a selection has nothing there, and an edit says so.
        for path in ["/s1ap/GWContextReleaseIndication/value", "/s1ap/7"] {
            assert_eq!(inspect::select(&view, path), Ok(vec![]), "{path}");
            let edit = inspect::set(&mut view, path, json!(1));
            assert_eq!(kind(edit), ErrorKind::NothingSelected, "{path}");
        }
        let removed = inspect::remove(&mut view, "/s1ap/GWContextReleaseIndication");
        assert_eq!(kind(removed), ErrorKind::NothingSelected);
        assert_eq!(view, before);
    }

    #[test]
    fn octets_that_carry_a_type_are_not_opened_until_they_are() {
        let name = "SourceeNB-ToTargeteNB-TransparentContainer";
        let required = json!({"message": "HandoverRequired", "ies": [
            {"Source-ToTarget-TransparentContainer": {name: {
                "rRC-Container": "0102",
                "targetCell-ID": {"pLMNidentity": "208-93", "cell-ID": 4660},
                "uE-HistoryInformation": [{"uTRAN-Cell": "00"}],
            }}},
        ]});
        let pdu = S1AP_PDU::from_tree(&required).unwrap();
        let mut view = pdu.to_view().unwrap();
        let cell =
            format!("/s1ap/Source-ToTarget-TransparentContainer/{name}/targetCell-ID/cell-ID");

        let error = inspect::select(&view, &cell).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::NotOpened);
        let edit = inspect::set(&mut view, &cell, json!(7));
        assert_eq!(kind(edit), ErrorKind::NotOpened);

        // The kind says what to do about it.
        inspect::open(&mut view, &cell).unwrap();
        assert_eq!(inspect::select(&view, &cell), Ok(vec![&json!(4660)]));
    }

    #[test]
    fn a_path_into_what_did_not_decode_says_so() {
        // An IE whose identifier has no type keeps its octets in a view.
        let unknown = json!([{"65000": {"octets": "c0ffee"}, "criticality": "ignore"}]);
        let view = written(unknown).unwrap().to_view().unwrap();
        assert_eq!(
            inspect::select(&view, "/s1ap/@id=65000/octets"),
            Ok(vec![&json!("C0FFEE")])
        );
        let error = inspect::select(&view, "/s1ap/@id=65000/value/member").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UndecodableIe);
        assert_eq!(
            error.to_string(),
            "the IE did not decode at /s1ap/@id=65000/value/member"
        );
    }

    #[test]
    fn a_value_that_is_not_of_its_type_is_a_value_error() {
        // A string where the type has a number, a name that the ENUMERATED does not
        // have, a number outside the constraint of its type, and octets that are no
        // hexadecimal.
        let string = written(json!([{"MME-UE-S1AP-ID": "forty-two"}])).unwrap_err();
        let name = written(json!([{"Cause": {"radioNetwork": "no-such-cause"}}])).unwrap_err();
        let outside = written(json!([{"MME-UE-S1AP-ID": 1u64 << 32}])).unwrap_err();
        let octets = written(json!([{"MME-UE-S1AP-ID": {"octets": "zz"}}])).unwrap_err();
        for error in [&string, &name, &outside, &octets] {
            assert_eq!(error.kind(), ErrorKind::Value, "{error}");
            assert!(!error.to_string().contains("Codec"), "{error}");
        }
        assert_eq!(
            string.to_string(),
            "this is not a value of MME-UE-S1AP-ID: a number is expected, and \"forty-two\" is written"
        );
        assert_eq!(
            name.to_string(),
            "this is not a value of Cause, at radioNetwork: \"no-such-cause\" is no value of the \
             ENUMERATED"
        );
        assert_eq!(
            outside.to_string(),
            "this is not a value of MME-UE-S1AP-ID: 4294967296 is outside its constraint (0..4294967295)"
        );
        assert_eq!(
            octets.to_string(),
            "this is not a value of UEContextReleaseRequest, at protocolIEs/0/value: octets in \
             hexadecimal, two digits for each, are expected, and \"zz\" is written"
        );

        // A value of a member, by the names that ASN.1 gives the members on the way:
        // a PLMN identity that is none, and a number where the type has a CHOICE.
        let setup = |plmn: Value, gnb: Value| {
            S1AP_PDU::from_tree(
                &json!({"message": "S1SetupRequest", "ies": [{"Global-ENB-ID": {
                    "pLMNidentity": plmn,
                    "eNB-ID": gnb,
                }}]}),
            )
        };
        assert!(setup(json!("208-93"), json!({"macroENB-ID": "123450"})).is_ok());
        let plmn = setup(json!("20-93"), json!({"macroENB-ID": "123450"})).unwrap_err();
        assert_eq!(plmn.kind(), ErrorKind::Value);
        assert_eq!(
            plmn.to_string(),
            "20-93: a PLMN identity is MCC-MNC, as 208-93"
        );
        let member = setup(json!("208-93"), json!(5)).unwrap_err();
        assert_eq!(member.kind(), ErrorKind::Value);
        assert_eq!(
            member.to_string(),
            "this is not a value of Global-ENB-ID, at eNB-ID: an object with one member is \
             expected, the name of an alternative with its value, and 5 is written"
        );
        assert!(std::error::Error::source(&member).is_none());
    }

    #[test]
    fn json_that_is_no_tree_and_no_view_is_a_form_error() {
        // A tree is an object with the name of a message and its IEs, each one member:
        // an IE that the message does not have says its criticality.
        let message = "UEContextReleaseRequest";
        for tree in [
            json!([]),
            json!({"ies": []}),
            json!({"message": message}),
            json!({"message": message, "ies": [], "other": 1}),
            json!({"message": message, "ies": [{"a": 1, "b": 2}]}),
            json!({"message": message, "ies": [{"eNBname": "enb-1"}]}),
        ] {
            let error = S1AP_PDU::from_tree(&tree).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Form, "{tree}: {error}");
        }

        // A view is an object with a procedure code, a direction, a criticality and a
        // message.
        let view = view();
        let without = |member: &str| {
            let mut view = view.clone();
            view.as_object_mut().unwrap().remove(member);
            view
        };
        let with = |member: &str, value: Value| {
            let mut view = view.clone();
            view[member] = value;
            view
        };
        for view in [
            json!(1),
            without("message"),
            without("direction"),
            without("procedure_code"),
            with("direction", json!("Sideways")),
            with("procedure_code", json!(256)),
            with("other", json!(1)),
        ] {
            let error = S1AP_PDU::from_view(&view).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Form, "{error}");
            // The PDU that the view is of refuses it as any PDU does.
            let error = release_request().with_view(&view).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Form, "{error}");
        }

        // The view of another message is not one of this PDU: another procedure code,
        // another direction, and a procedure that the specification does not have.
        let pdu = release_request();
        for (member, value, other) in [
            ("procedure_code", json!(17), "S1SetupRequest"),
            (
                "direction",
                json!("SuccessfulOutcome"),
                "the SuccessfulOutcome of procedure 18",
            ),
            (
                "procedure_code",
                json!(200),
                "the InitiatingMessage of procedure 200",
            ),
        ] {
            let error = pdu.with_view(&with(member, value)).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Form, "{error}");
            assert_eq!(
                error.to_string(),
                format!(
                    "this view is of {other}, and the PDU is UEContextReleaseRequest: with_view \
                     takes a view of the PDU itself, and from_view any view"
                )
            );
        }

        // An IE is written with its value or with its octets.
        let both = json!({"id": "Cause", "criticality": "ignore", "value": 1, "octets": "00"});
        let insert = inspect::insert(&mut view.clone(), "/s1ap/-", both);
        assert_eq!(kind(insert), ErrorKind::Form);
    }

    #[test]
    fn an_edit_that_is_refused_is_an_edit_error() {
        let mut view = view();
        // A member that its type always has, the value of an IE and the alternative of
        // a CHOICE are not taken out, and nothing is inserted where there is no list.
        for path in [
            "/s1ap/Cause/criticality",
            "/s1ap/Cause/value",
            "/s1ap/Cause/radioNetwork",
        ] {
            let removed = inspect::remove(&mut view, path);
            assert_eq!(kind(removed), ErrorKind::Edit, "{path}");
        }
        let inserted = inspect::insert(&mut view, "/s1ap/Cause/criticality", json!("reject"));
        assert_eq!(kind(inserted), ErrorKind::Edit);

        // The octets of a value that an edit changed are no longer its octets.
        inspect::set(&mut view, "/s1ap/Cause/nas", json!("normal-release")).unwrap();
        let octets = inspect::select(&view, "/s1ap/Cause/octets");
        assert_eq!(kind(octets), ErrorKind::Edit);
    }

    #[test]
    fn what_is_over_the_limits_of_the_inspection_is_a_limit_error() {
        let view = view();
        // A path of more than 64 segments, and one of more than 4096 octets.
        let long = "/0".repeat(65);
        assert_eq!(kind(inspect::select(&view, &long)), ErrorKind::Limit);
        let wide = format!("/{}", "a".repeat(4096));
        assert_eq!(kind(inspect::select(&view, &wide)), ErrorKind::Limit);

        // A tree that is nested deeper than a tree is read.
        let mut deep = json!(1);
        for _ in 0..300 {
            deep = json!([deep]);
        }
        assert_eq!(
            kind(written(json!([{"MME-UE-S1AP-ID": deep}]))),
            ErrorKind::Limit
        );

        // A private message has no list of IEs to show as a tree: its view has it.
        let private =
            S1AP_PDU::decode(&hex::decode("002740090000000000000001a1").unwrap()).unwrap();
        let error = private.to_tree().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Limit);
        assert!(private.to_view().is_ok());
    }

    #[test]
    fn a_message_that_does_not_decode_is_a_decode_error() {
        // A PDU whose octets are not the message of its procedure.
        let S1AP_PDU::initiatingMessage(mut held) = release_request() else {
            panic!("an initiating message");
        };
        held.value = Octets::from(vec![0xff]);
        let pdu = S1AP_PDU::initiatingMessage(held);
        let error = pdu.to_view().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Decode);
        assert_eq!(kind(pdu.to_tree()), ErrorKind::Decode);

        // The octets that a view was received as, replaced by others, and edited.
        let mut view = view();
        view["_raw_message"] = json!("ff");
        inspect::set(&mut view, "/s1ap/Cause/nas", json!("normal-release")).unwrap();
        assert_eq!(kind(S1AP_PDU::from_view(&view)), ErrorKind::Decode);
    }
}
