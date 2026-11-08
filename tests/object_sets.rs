//! The PDUs of the fixtures, of the captures and of the vectors through what the crate
//! reads them with: the readers of one IE, `check()` and `check_structure()` against
//! the object set of each message, and the container of its protocol IEs, read and put
//! back. TOY, the protocol of the tests of oxirush-aper, has the tests of each of them.

use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{Deviation, Error, ErrorKind, ie};

/// The vectors of a file of the fixtures: the name of each, and the octets of its PDU.
fn vectors(file: &'static str) -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let lines = file.lines();
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let mut fields = line.split('\t');
        let name = fields.next().unwrap();
        (name, hex::decode(fields.next().unwrap()).unwrap())
    })
}

/// The PDUs of the fixtures and of the captures.
fn fixtures() -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let messages = vectors(include_str!("fixtures/messages.tsv"));
    messages.chain(vectors(include_str!("fixtures/captured.tsv")))
}

#[test]
fn every_ie_of_the_fixtures_is_read_as_the_codec_of_its_message_reads_it() -> Result<(), Error> {
    let mut read = 0;
    for (name, octets) in fixtures() {
        let pdu = S1APPDU::decode(&octets)?;
        let Ok(ies) = pdu.protocol_ies() else {
            continue;
        };
        // The NAS-PDUs and the causes, as each reader gives them: a value that does not
        // decode is an error of the kind `UndecodableIe` of the reader.
        let of = |id: u16| ies.0.iter().filter(move |ie| ie.id.0 == id);
        let nas: Vec<Result<NASPDU, ErrorKind>> = of(26)
            .map(|ie| oxirush_s1ap::decode(&ie.value).map_err(|_| ErrorKind::UndecodableIe))
            .collect();
        let read_nas = pdu
            .ies::<ie::NAS_PDU>()
            .map(|nas| nas.map_err(|error| error.kind()));
        assert_eq!(read_nas.collect::<Vec<_>>(), nas, "{name}");
        let causes: Vec<Result<Cause, ErrorKind>> = of(2)
            .map(|ie| oxirush_s1ap::decode(&ie.value).map_err(|_| ErrorKind::UndecodableIe))
            .collect();
        let read_causes = pdu
            .ies::<ie::Cause>()
            .map(|cause| cause.map_err(|error| error.kind()));
        assert_eq!(read_causes.collect::<Vec<_>>(), causes, "{name}");
        read += 1;
    }
    assert!(read > 150, "{read}");
    Ok(())
}

#[test]
fn check_structure_reports_what_check_reports_of_the_container() -> Result<(), Error> {
    let of_values = |deviation: &Deviation| {
        matches!(
            deviation,
            Deviation::UndecodableIe { .. }
                | Deviation::OutsideRoot { .. }
                | Deviation::NotCanonical { .. }
        )
    };
    let mut compared = 0;
    for (name, octets) in fixtures() {
        let pdu = S1APPDU::decode(&octets)?;
        let (Ok(all), Ok(structure)) = (pdu.check(), pdu.check_structure()) else {
            assert_eq!(
                pdu.check().is_err(),
                pdu.check_structure().is_err(),
                "{name}"
            );
            continue;
        };
        let of_container: Vec<_> = all
            .into_iter()
            .filter(|deviation| !of_values(deviation))
            .collect();
        assert_eq!(structure, of_container, "{name}");
        compared += 1;
    }
    assert!(compared > 150, "{compared}");
    Ok(())
}

#[test]
fn the_messages_of_the_fixtures_are_checked_against_their_object_sets() {
    let mut deviating = Vec::new();
    let mut checked = 0;
    for (name, wire) in vectors(include_str!("fixtures/messages.tsv")) {
        let pdu = S1APPDU::decode(&wire).unwrap();
        let found = pdu
            .check()
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        checked += 1;
        for deviation in found {
            deviating.push(format!("{name}: {deviation}"));
        }
    }
    assert_eq!(checked, 101);
    assert_eq!(deviating, Vec::<String>::new(), "{deviating:#?}");
}

/// The vectors with content have the IEs of the object sets as pycrate has them from
/// the ASN.1: each IE that a set has, in the order of the set and with its criticality.
/// The vectors that cover the extension of a constraint hold a value outside its root,
/// which is their one deviation.
#[test]
fn the_vectors_with_content_are_checked_against_their_object_sets() {
    let (mut checked, mut outside) = (0, 0);
    for (name, wire) in vectors(include_str!("fixtures/vectors.tsv")) {
        let pdu = S1APPDU::decode(&wire).unwrap();
        let found = pdu
            .check()
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        for deviation in &found {
            assert!(
                matches!(deviation, Deviation::OutsideRoot { .. }),
                "{name}: {deviation}"
            );
            // An encoding that is strict refuses what the vector holds.
            assert!(
                deviation
                    .to_string()
                    .contains("outside the root of its constraint")
            );
        }
        outside += found.len();
        checked += 1;
    }
    assert!(checked > 101, "{checked} vectors");
    assert_eq!(
        outside, 11,
        "the IEs of the vectors that are outside a root"
    );
}

#[test]
fn the_ies_of_every_message_are_read_and_put_back_as_the_same_octets() {
    let mut read = 0;
    let messages = vectors(include_str!("fixtures/messages.tsv"));
    for (name, wire) in messages.chain(vectors(include_str!("fixtures/vectors.tsv"))) {
        let mut pdu = S1APPDU::decode(&wire).unwrap();
        // The private message has private IEs, and no container of protocol IEs.
        if name.split('/').next() == Some("PrivateMessage") {
            continue;
        }
        // The IEs borrow from the PDU: they are owned before the PDU is changed.
        let ies = pdu.protocol_ies();
        let ies = ies.unwrap_or_else(|error| panic!("{name}: {error}"));
        let ies = ies.into_owned();
        pdu.set_protocol_ies(ies).unwrap();
        assert_eq!(pdu.encode().unwrap(), wire, "{name}");
        read += 1;
    }
    // Each message of the fixtures but the private one, and each vector.
    assert!(read > 100, "{read} messages");
}
