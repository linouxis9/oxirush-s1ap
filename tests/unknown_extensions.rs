use oxirush_s1ap::aper::{AsnType, Decode, Error, Reader, decode, encode};
use oxirush_s1ap::s1ap::*;

/// Unknown SEQUENCE additions must be consumed before the next TAI in
/// TAIListforWarning (TS 36.413 §9.3.4; X.691 (02/2021) §19.8-19.9).
#[test]
fn unknown_sequence_addition_preserves_the_following_tai() {
    let wire = hex::decode("00018002f83900010101000002f8390002").unwrap();
    let value = decode::<TAIListforWarning>(&wire).unwrap();
    assert_eq!(
        value.0.iter().map(|tai| &tai.tac.0[..]).collect::<Vec<_>>(),
        [&[0, 1][..], &[0, 2][..]],
    );
    // The public type retains the known root, while future additions are ignored.
    assert_eq!(
        encode(&value).unwrap(),
        hex::decode("00010002f83900010002f8390002").unwrap(),
    );
}

/// Wireshark independently decodes both TACs through the complete PDU's
/// message and IE open types. Opaque outer bytes retain unknown additions.
#[test]
fn complete_warning_request_preserves_unknown_addition_bytes() {
    let wire = hex::decode(
        "00240031000005006f00028000007000028000007140122000018002f83900010101000002f839000200720002000a007300020001",
    )
    .unwrap();
    let pdu = S1AP_PDU::decode(&wire).unwrap();
    assert_eq!(pdu.encode().unwrap(), wire);
    let request: WriteReplaceWarningRequest = pdu.decode_value().unwrap();
    let ie = request
        .protocol_ies
        .0
        .iter()
        .find(|ie| ie.id == ID_WARNING_AREA_LIST)
        .unwrap();
    let WarningAreaList::trackingAreaListforWarning(list) =
        decode_open_type::<WarningAreaList>(&ie.value).unwrap()
    else {
        panic!("expected tracking area warning list");
    };
    assert_eq!(
        list.0.iter().map(|tai| &tai.tac.0[..]).collect::<Vec<_>>(),
        [&[0, 1][..], &[0, 2][..]],
    );
}

/// A TAI, then a number of one octet: what follows the additions of a value is read
/// where they end.
#[derive(Debug, PartialEq)]
struct FollowingTAI {
    value: TAI,
    following: u8,
}

impl AsnType for FollowingTAI {
    const NAME: &'static str = "FollowingTAI";
}

impl<'a> Decode<'a> for FollowingTAI {
    fn decode(reader: &mut Reader<'a>) -> Result<Self, Error> {
        Ok(Self {
            value: TAI::decode(reader)?,
            following: reader.unsigned(0, 255)? as u8,
        })
    }
}

/// Hand-built X.691 fields, independent of the writer of the crate. In particular,
/// normally small lengths <=64 encode n-1 in six unaligned bits; larger
/// bitmaps use an unconstrained length (X.691 (02/2021) §11.9.3.4),
/// including fragments.
#[derive(Default)]
struct Wire(Vec<bool>);

impl Wire {
    fn bits(&mut self, value: usize, count: usize) {
        for bit in (0..count).rev() {
            self.0.push(value & (1 << bit) != 0);
        }
    }

    fn align(&mut self) {
        while !self.0.len().is_multiple_of(8) {
            self.0.push(false);
        }
    }

    fn byte(&mut self, value: u8) {
        self.align();
        self.bits(value.into(), 8);
    }

    fn length(&mut self, value: usize) {
        if value < 128 {
            self.byte(value as u8);
        } else {
            self.byte(0x80 | (value >> 8) as u8);
            self.byte(value as u8);
        }
    }

    fn finish(mut self) -> Vec<u8> {
        self.align();
        let octet = |bits: &[bool]| {
            bits.iter()
                .fold(0, |octet, bit| octet << 1 | u8::from(*bit))
        };
        self.0.chunks(8).map(octet).collect()
    }
}

fn extended_tai(count: usize, fragmented_open_type: bool) -> Vec<u8> {
    let mut wire = Wire::default();
    wire.bits(1, 1); // TAI has extension additions.
    wire.bits(0, 1); // iE-Extensions is absent.
    for octet in [0x02, 0xf8, 0x39, 0, 1] {
        wire.byte(octet); // The three-octet PLMN and two-octet TAC are aligned.
    }
    if count <= 64 {
        wire.bits(0, 1);
        wire.bits(count - 1, 6);
        for bit in 0..count {
            wire.bits(usize::from(bit == 0 || bit == count - 1), 1);
        }
    } else {
        wire.bits(1, 1);
        let mut offset = 0;
        while count - offset >= 16384 {
            let blocks = ((count - offset) / 16384).min(4);
            wire.byte(0xc0 | blocks as u8);
            for bit in offset..offset + blocks * 16384 {
                wire.bits(usize::from(bit == 0 || bit == count - 1), 1);
            }
            offset += blocks * 16384;
        }
        wire.length(count - offset);
        for bit in offset..count {
            wire.bits(usize::from(bit == 0 || bit == count - 1), 1);
        }
    }
    for _ in 0..if count == 1 { 1 } else { 2 } {
        if fragmented_open_type {
            wire.byte(0xc1);
            for _ in 0..16384 {
                wire.byte(0x5a);
            }
            wire.byte(0); // Exact-size fragments need a final zero length.
        } else {
            wire.byte(1);
            wire.byte(0x5a);
        }
    }
    wire.byte(0x7c); // The following field must remain intact.
    wire.finish()
}

#[test]
fn unknown_additions_handle_small_large_and_fragmented_bitmaps() {
    let expected = FollowingTAI {
        value: oxirush_s1ap::helpers::tai(
            oxirush_s1ap::helpers::plmn("208", "93").unwrap(),
            [0, 1],
        ),
        following: 0x7c,
    };
    for count in [1, 64, 65, 128, 16384, 65536] {
        let wire = extended_tai(count, false);
        assert_eq!(decode::<FollowingTAI>(&wire).unwrap(), expected);
        assert!(decode::<FollowingTAI>(&wire[..wire.len() - 1]).is_err());
    }
}

#[test]
fn unknown_fragmented_open_types_preserve_following_fields_and_reject_truncation() {
    for count in [1, 64, 65, 128] {
        let wire = extended_tai(count, true);
        assert_eq!(decode::<FollowingTAI>(&wire).unwrap().following, 0x7c);
        for cut in 0..wire.len() {
            assert!(
                decode::<FollowingTAI>(&wire[..cut]).is_err(),
                "accepted truncated addition: bitmap={count}, prefix={cut}",
            );
        }
    }
}

#[test]
fn unknown_additions_reject_truncated_small_open_types() {
    for count in [1, 64, 65, 128] {
        let wire = extended_tai(count, false);
        for cut in 0..wire.len() {
            assert!(decode::<FollowingTAI>(&wire[..cut]).is_err());
        }
    }
}

#[test]
fn root_fields_keep_the_original_sequence_layout() {
    // The extension bit and the bit of the member that may be absent, then the PLMN
    // identity, which is three octets and so starts an octet, and the TAC, two octets
    // that follow it where it ends.
    let value =
        oxirush_s1ap::helpers::tai(oxirush_s1ap::helpers::plmn("208", "93").unwrap(), [0, 1]);
    let wire = encode(&value).unwrap();
    assert_eq!(wire, [0x00, 0x02, 0xf8, 0x39, 0x00, 0x01]);
    assert_eq!(decode::<TAI>(&wire).unwrap(), value);
}

/// Named extension containers are ordinary optional root fields, distinct
/// from SEQUENCE additions. This wire is independently encoded by pycrate.
#[test]
fn optional_root_extension_container_is_preserved() {
    let wire = hex::decode("4002f83900010000fde840015a").unwrap();
    let value = TAI::new(
        oxirush_s1ap::helpers::plmn("208", "93").unwrap(),
        TAC([0, 1]),
        Some(ProtocolExtensionContainer(vec![
            ProtocolExtensionField::new(
                ProtocolExtensionID(65000),
                Criticality::ignore,
                vec![0x5a].into(),
            ),
        ])),
    );
    assert_eq!(decode::<TAI>(&wire).unwrap(), value);
    assert_eq!(encode(&value).unwrap(), wire);
}

/// What X.691 (02/2021) gives no encoding for is refused. The extension bit of a
/// SEQUENCE is one when an addition is present, and zero otherwise (§19.1), and the
/// length of the bitmap of the additions is one at least (§19.8, §11.9.3.1 a)).
///
/// A differential run against the codec of rasn found the first of these, which the
/// crate read as a value without additions.
#[test]
fn an_extension_bit_without_an_addition_that_is_present_is_refused() {
    let refused = |octets: &[u8]| {
        let error = decode::<ULCPSecurityInformation>(octets).unwrap_err();
        error.to_string()
    };
    let none = "the extension bit of a SEQUENCE is set, and it has no extension addition";
    // The extension bit, no iE-Extensions, the 16 bits of the MAC and the 5 of the
    // count, then the bitmap of the additions: a one bit for the form of more than 64
    // bits, and its length, which is zero.
    let error = refused(&[0x80, 0x02, 0x01, 0x00]);
    assert!(error.ends_with(none), "{error}");
    // A bitmap of one addition, which is not present.
    let error = refused(&[0x80, 0x02, 0x00, 0x00]);
    assert!(error.ends_with(none), "{error}");
    // One addition that is present, counted in the form of more than 64 bits, where 64
    // bits at most have their length, less one, in six bits (§11.9.3.4).
    let error = refused(&[0x80, 0x02, 0x01, 0x01, 0x80, 0x01, 0xaa]);
    assert!(error.ends_with("has its length in six bits"), "{error}");
    // One addition that is present, in an open type of no octet: a complete encoding
    // is one octet at least (§11.1.3.1).
    let error = refused(&[0x80, 0x02, 0x00, 0x02, 0x00]);
    assert!(
        error.contains("an extension addition has no octet"),
        "{error}"
    );
    // The same addition in one octet is skipped, and the value is read.
    let value = decode::<ULCPSecurityInformation>(&[0x80, 0x02, 0x00, 0x02, 0x01, 0x00]).unwrap();
    assert_eq!(encode(&value).unwrap(), [0x00, 0x02, 0x00]);
}
