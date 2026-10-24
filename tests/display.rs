//! `Display` of a PDU: the name of its message, its procedure code and its direction,
//! then the identifiers of its UE, whether the message has them as two IEs or as one,
//! UE-S1AP-IDs, which holds the pair or the MME UE S1AP ID alone.

use oxirush_s1ap::s1ap::*;
use oxirush_s1ap::{Error, build_s1ap};

/// The PDU that the octets of `pdu` decode to: what a peer receives.
fn received(pdu: &S1AP_PDU) -> Result<S1AP_PDU, Error> {
    S1AP_PDU::decode(&pdu.encode()?)
}

#[test]
fn a_pdu_shows_the_identifiers_of_its_ue_however_its_message_carries_them() -> Result<(), Error> {
    let cause = || Cause::nas(CauseNas::normal_release);

    // UE CONTEXT RELEASE COMMAND, with the pair
    let pair = UES1APIDPair::new(MMEUES1APID(42), ENBUES1APID(7), None);
    let command = build_s1ap!(
        InitiatingMessage,
        UEContextRelease,
        UEContextReleaseCommand,
        UE_S1AP_IDs(UES1APIDs::uE_S1AP_ID_pair(pair)),
        Cause(cause()),
    )?;
    assert_eq!(
        received(&command)?.to_string(),
        "UEContextReleaseCommand (procedure 23, initiating message), MME-UE-S1AP-ID 42, eNB-UE-S1AP-ID 7"
    );

    // and with the MME UE S1AP ID alone
    let command = build_s1ap!(
        InitiatingMessage,
        UEContextRelease,
        UEContextReleaseCommand,
        UE_S1AP_IDs(UES1APIDs::mME_UE_S1AP_ID(MMEUES1APID(42))),
        Cause(cause()),
    )?;
    assert_eq!(
        received(&command)?.to_string(),
        "UEContextReleaseCommand (procedure 23, initiating message), MME-UE-S1AP-ID 42"
    );

    // UE CONTEXT RELEASE COMPLETE has them as two IEs, and S1 SETUP REQUEST none
    let complete = build_s1ap!(
        SuccessfulOutcome,
        UEContextRelease,
        UEContextReleaseComplete,
        MME_UE_S1AP_ID(42u32),
        ENB_UE_S1AP_ID(7u32),
    )?;
    assert_eq!(
        received(&complete)?.to_string(),
        "UEContextReleaseComplete (procedure 23, successful outcome), MME-UE-S1AP-ID 42, eNB-UE-S1AP-ID 7"
    );
    let setup = build_s1ap!(
        InitiatingMessage,
        S1Setup,
        S1SetupRequest,
        DefaultPagingDRX(PagingDRX::v128),
    )?;
    assert_eq!(
        received(&setup)?.to_string(),
        "S1SetupRequest (procedure 17, initiating message)"
    );
    Ok(())
}
