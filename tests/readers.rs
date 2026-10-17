//! The readers of the IEs of a PDU and of a message: `ies::<T>()` and `ie::<T>()`, the
//! macro that extracts IEs from a PDU without `decode_value`, and `check_structure()`. On
//! the PDUs of the fixtures, they read what the codec of the message reads.

use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{
    BitString, Deviation, Error, ErrorKind, Octets, build_s1ap, build_s1ap_ie, extract_s1ap_ies, ie,
};

/// The PDUs of the fixtures and of the captures: the name of each, and its octets.
fn fixtures() -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let files = [
        include_str!("fixtures/messages.tsv"),
        include_str!("fixtures/captured.tsv"),
    ];
    let lines = files.into_iter().flat_map(str::lines);
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let mut fields = line.split('\t');
        let name = fields.next().unwrap();
        (name, hex::decode(fields.next().unwrap()).unwrap())
    })
}

/// A UE CONTEXT RELEASE REQUEST of these IEs.
fn release_request() -> S1AP_PDU {
    build_s1ap!(
        InitiatingMessage,
        UEContextReleaseRequest,
        UEContextReleaseRequest,
        MME_UE_S1AP_ID(42u32),
        eNB_UE_S1AP_ID(7u32),
        Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )
    .unwrap()
}

#[test]
fn an_ie_is_read_from_the_pdu_as_from_its_message() -> Result<(), Error> {
    let octets = release_request().encode()?;
    let pdu = S1AP_PDU::decode_borrowed(&octets)?;
    assert_eq!(pdu.ie::<ie::eNB_UE_S1AP_ID>().unwrap()?, ENBUES1APID(7));
    assert_eq!(pdu.ie::<ie::MME_UE_S1AP_ID>().unwrap()?, MMEUES1APID(42));
    assert!(pdu.ie::<ie::NAS_PDU>().is_none());
    let message: UEContextReleaseRequest = pdu.decode_value()?;
    assert_eq!(message.ie::<ie::eNB_UE_S1AP_ID>().unwrap()?, ENBUES1APID(7));
    // The name of the type is a name of the IE of that type.
    assert_eq!(message.ie::<ie::ENBUES1APID>().unwrap()?, ENBUES1APID(7));
    Ok(())
}

#[test]
fn the_macro_reads_a_pdu_without_decode_value() -> Result<(), Error> {
    fn read(pdu: &S1AP_PDU) -> Result<(u32, u32), Error> {
        extract_s1ap_ies!(pdu, UEContextReleaseRequest,
            req mme: u32 = MME_UE_S1AP_ID(id),
            req enb: u32 = eNB_UE_S1AP_ID(id),
        );
        Ok((mme, enb))
    }
    assert_eq!(read(&release_request())?, (42, 7));
    // A PDU of another message is an error of the kind `WrongMessage`.
    let setup = build_s1ap!(
        InitiatingMessage,
        S1Setup,
        S1SetupRequest,
        DefaultPagingDRX(PagingDRX::v128),
    )?;
    assert_eq!(read(&setup).unwrap_err().kind(), ErrorKind::WrongMessage);
    Ok(())
}

#[test]
fn a_nas_pdu_in_fragments_is_read_and_kept() -> Result<(), Error> {
    // 20,000 octets come in two fragments in the open type of the IE, and in two more in
    // the OCTET STRING of the NAS-PDU.
    let nas: Vec<u8> = (0..20_000).map(|at| (at % 253) as u8).collect();
    let pdu = build_s1ap!(
        InitiatingMessage,
        downlinkNASTransport,
        DownlinkNASTransport,
        MME_UE_S1AP_ID(42u32),
        eNB_UE_S1AP_ID(7u32),
        NAS_PDU(nas.clone()),
    )?;
    let octets = pdu.encode()?;
    let pdu = S1AP_PDU::decode_borrowed(&octets)?;
    assert_eq!(pdu.ie::<ie::NAS_PDU>().unwrap()?.0[..], nas[..]);
    fn read(pdu: &S1APPDU<'_>) -> Result<Vec<u8>, Error> {
        extract_s1ap_ies!(pdu, DownlinkNASTransport,
            req nas: Vec<u8> = NAS_PDU(nas) => nas.0.to_vec(),
        );
        Ok(nas)
    }
    assert_eq!(read(&pdu)?, nas);
    assert!(pdu.check()?.is_empty());
    Ok(())
}

#[test]
fn every_ie_of_the_fixtures_is_read_as_the_codec_of_its_message_reads_it() -> Result<(), Error> {
    let mut read = 0;
    for (name, octets) in fixtures() {
        let pdu = S1AP_PDU::decode(&octets)?;
        let Ok(ies) = pdu.protocol_ies() else {
            continue;
        };
        // The NAS-PDUs and the causes, as each reader gives them: a value that does not
        // decode is an error of the kind `UndecodableIe` of the reader.
        let of = |id: u16| ies.0.iter().filter(move |ie| ie.id.0 == id);
        let nas: Vec<Result<NASPDU, ErrorKind>> = of(26)
            .map(|ie| decode_open_type(&ie.value).map_err(|_| ErrorKind::UndecodableIe))
            .collect();
        let read_nas = pdu
            .ies::<ie::NAS_PDU>()
            .map(|nas| nas.map_err(|error| error.kind()));
        assert_eq!(read_nas.collect::<Vec<_>>(), nas, "{name}");
        let causes: Vec<Result<Cause, ErrorKind>> = of(2)
            .map(|ie| decode_open_type(&ie.value).map_err(|_| ErrorKind::UndecodableIe))
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
        let pdu = S1AP_PDU::decode(&octets)?;
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
fn a_repeated_ie_is_there_twice_for_ies_and_once_for_the_macro() -> Result<(), Error> {
    let pdu = build_s1ap!(
        InitiatingMessage,
        UEContextReleaseRequest,
        UEContextReleaseRequest,
        MME_UE_S1AP_ID(42u32),
        eNB_UE_S1AP_ID(7u32),
        eNB_UE_S1AP_ID(8u32),
        Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )?;
    let every: Vec<_> = pdu.ies::<ie::eNB_UE_S1AP_ID>().collect::<Result<_, _>>()?;
    assert_eq!(every, [ENBUES1APID(7), ENBUES1APID(8)]);
    extract_s1ap_ies!(pdu, UEContextReleaseRequest, lax enb: u32 = eNB_UE_S1AP_ID(id));
    assert_eq!(enb, Some(7));
    assert!(matches!(
        pdu.check_structure()?[..],
        [Deviation::RepeatedIe {
            id: 8,
            position: 2,
            first: 1,
            ..
        }]
    ));
    Ok(())
}

#[test]
fn octets_of_the_message_that_end_early_are_an_error_of_every_reader() -> Result<(), Error> {
    let pdu = release_request();
    let S1APPDU::initiatingMessage(mut message) = pdu else {
        unreachable!("a request")
    };
    // The message without its last octet, which its Cause takes.
    let mut octets = message.value.to_vec();
    octets.pop();
    message.value = Octets::from(octets);
    let cut = S1APPDU::initiatingMessage(message);
    assert_eq!(cut.check_structure().unwrap_err().kind(), ErrorKind::Decode);
    assert_eq!(cut.check().unwrap_err().kind(), ErrorKind::Decode);
    let ids: Vec<_> = cut.ies::<ie::eNB_UE_S1AP_ID>().collect();
    assert_eq!(ids[0], Ok(ENBUES1APID(7)));
    assert_eq!(ids[1].as_ref().unwrap_err().kind(), ErrorKind::Decode);
    fn read(pdu: &S1AP_PDU) -> Result<u32, Error> {
        extract_s1ap_ies!(pdu, UEContextReleaseRequest, req enb: u32 = eNB_UE_S1AP_ID(id));
        Ok(enb)
    }
    assert_eq!(read(&cut).unwrap_err().kind(), ErrorKind::Decode);
    Ok(())
}

/// An E-RAB to set up, as the item of its list.
fn bearer(id: u64) -> Result<ProtocolIEField<'static>, Error> {
    let priority = AllocationAndRetentionPriority::new(
        PriorityLevel(8),
        PreEmptionCapability::shall_not_trigger_pre_emption,
        PreEmptionVulnerability::not_pre_emptable,
        None,
    );
    let item = ERABToBeSetupItemCtxtSUReq::new(
        ERABID(id),
        ERABLevelQoSParameters::new(QCI(9), priority, None, None),
        TransportLayerAddress(BitString::from([10, 0, 0, 1])),
        GTPTEID([0, 0, 0, id as u8]),
        None,
        None,
    );
    build_s1ap_ie!(REJECT E_RABToBeSetupItemCtxtSUReq(item))
}

#[test]
fn the_items_of_an_e_rab_list_are_walked_without_building_the_list() -> Result<(), Error> {
    let pdu = build_s1ap!(
        InitiatingMessage,
        InitialContextSetup,
        InitialContextSetupRequest,
        MME_UE_S1AP_ID(1u32),
        eNB_UE_S1AP_ID(7u32),
        E_RABToBeSetupListCtxtSUReq(vec![bearer(5)?, bearer(6)?]),
    )?;
    let octets = pdu.encode()?;
    let pdu = S1AP_PDU::decode_borrowed(&octets)?;
    let ids: Vec<u64> = pdu
        .items::<ie::E_RABToBeSetupListCtxtSUReq>()
        .map(|item| item.map(|item| item.e_rab_id.0))
        .collect::<Result<_, _>>()?;
    assert_eq!(ids, [5, 6]);
    // The same of the message, and each is the item that the codec of the list reads.
    let message: InitialContextSetupRequest = pdu.decode_value()?;
    let list = message.ie::<ie::E_RABToBeSetupListCtxtSUReq>().unwrap()?;
    let items: Vec<ERABToBeSetupItemCtxtSUReq> = list
        .0
        .iter()
        .map(|item| decode_open_type(&item.value).unwrap())
        .collect();
    let walked: Vec<_> = message
        .items::<ie::E_RABToBeSetupListCtxtSUReq>()
        .collect::<Result<_, _>>()?;
    assert_eq!(walked, items);
    // A message without the list has no item.
    let release = release_request();
    assert!(release.ie::<ie::E_RABToBeSetupListCtxtSUReq>().is_none());
    Ok(())
}
