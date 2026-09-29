use oxirush_s1ap::s1ap::*;
use rasn::prelude::*;

/// Unknown SEQUENCE additions must be consumed before the next TAI in
/// TAIListforWarning (TS 36.413 §9.3.4; X.691 (02/2021) §19.8-19.9).
#[test]
fn unknown_sequence_addition_preserves_the_following_tai() {
    let wire = hex::decode("00018002f83900010101000002f8390002").unwrap();
    let value = rasn::aper::decode::<TAIListforWarning>(&wire).unwrap();
    assert_eq!(
        value
            .0
            .iter()
            .map(|tai| tai.t_ac.0.as_ref())
            .collect::<Vec<_>>(),
        [&[0, 1][..], &[0, 2][..]],
    );
    // The public type retains the known root, while future additions are ignored.
    assert_eq!(
        rasn::aper::encode(&value).unwrap(),
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
        list.0
            .iter()
            .map(|tai| tai.t_ac.0.as_ref())
            .collect::<Vec<_>>(),
        [&[0, 1][..], &[0, 2][..]],
    );
}

#[derive(AsnType, Decode, Debug, PartialEq)]
#[rasn(automatic_tags)]
struct FollowingTAI {
    value: TAI,
    #[rasn(value("0..=255"))]
    following: u8,
}

/// Hand-built X.691 fields, independent of rasn's encoder. In particular,
/// normally small lengths <=64 encode n-1 in six unaligned bits; larger
/// bitmaps use an unconstrained length (§11.9.3.4), including fragments.
#[derive(Default)]
struct Wire(BitString);

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
        self.0.into_vec()
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
        value: oxirush_s1ap::helpers::tai(oxirush_s1ap::helpers::plmn("208", "93"), &[0, 1]),
        following: 0x7c,
    };
    for count in [1, 64, 65, 128, 16384, 65536] {
        let wire = extended_tai(count, false);
        assert_eq!(rasn::aper::decode::<FollowingTAI>(&wire).unwrap(), expected);
        assert!(rasn::aper::decode::<FollowingTAI>(&wire[..wire.len() - 1]).is_err());
    }
}

#[test]
fn unknown_fragmented_open_types_preserve_following_fields_and_reject_truncation() {
    for count in [1, 64, 65, 128] {
        let wire = extended_tai(count, true);
        assert_eq!(
            rasn::aper::decode::<FollowingTAI>(&wire).unwrap().following,
            0x7c
        );
        for cut in 0..wire.len() {
            assert!(
                rasn::aper::decode::<FollowingTAI>(&wire[..cut]).is_err(),
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
            assert!(rasn::aper::decode::<FollowingTAI>(&wire[..cut]).is_err());
        }
    }
}

#[test]
fn root_fields_and_other_codecs_keep_the_original_sequence_layout() {
    let value = oxirush_s1ap::helpers::tai(oxirush_s1ap::helpers::plmn("208", "93"), &[0, 1]);
    macro_rules! round_trip {
        ($codec:ident) => {
            let wire = rasn::$codec::encode(&value).unwrap();
            assert_eq!(rasn::$codec::decode::<TAI>(&wire).unwrap(), value);
        };
    }
    round_trip!(aper);
    round_trip!(uper);
    round_trip!(ber);
    round_trip!(der);
    round_trip!(oer);
    round_trip!(coer);
    round_trip!(jer);
    round_trip!(xer);
}

/// Named extension containers are ordinary optional root fields, distinct
/// from SEQUENCE additions. This wire is independently encoded by pycrate.
#[test]
fn optional_root_extension_container_is_preserved() {
    let wire = hex::decode("4002f83900010000fde840015a").unwrap();
    let value = TAI::new(
        oxirush_s1ap::helpers::plmn("208", "93"),
        [0, 1].into(),
        Some(TAIIEExtensions(vec![AnonymousTAIIEExtensions::new(
            ProtocolExtensionID(65000),
            Criticality::ignore,
            Any::new(vec![0x5a]),
        )])),
    );
    assert_eq!(rasn::aper::decode::<TAI>(&wire).unwrap(), value);
    assert_eq!(rasn::aper::encode(&value).unwrap(), wire);
}
