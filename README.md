# oxirush-s1ap

[![Crates.io](https://img.shields.io/crates/v/oxirush-s1ap.svg)](https://crates.io/crates/oxirush-s1ap)
[![Documentation](https://docs.rs/oxirush-s1ap/badge.svg)](https://docs.rs/oxirush-s1ap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A Rust codec for S1AP, the S1 Application Protocol between an eNB and an MME
(3GPP TS 36.413): types generated from the ASN.1 of the specification,
aligned PER by [`oxirush-aper`](https://github.com/linouxis9/oxirush-aper), and
macros that build a message and read its IEs by name.

```rust
use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, s1ap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Decode a UE CONTEXT RELEASE REQUEST
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280")?;
    let pdu = S1AP_PDU::decode(&bytes)?;
    assert_eq!((pdu.procedure_code(), pdu.is_initiating()), (18, true));
    println!("{}", pdu.procedure_name());

    // Read its IEs by name. An `opt` field is `None` when its IE is not there,
    // and what follows `=>` is what the field keeps of the decoded value
    let request: UEContextReleaseRequest = pdu.decode_value()?;
    extract_s1ap_ies!(request, UEContextReleaseRequest,
        opt mme_id: u32 = MME_UE_S1AP_ID(id),
        opt enb_id: u32 = eNB_UE_S1AP_ID(id),
        opt cause: Cause = Cause(cause) => cause,
    );
    println!("MME={mme_id:?} eNB={enb_id:?} cause={cause:?}");

    // Build it with another cause: the direction, the procedure, the criticality
    // of the PDU and the message, then each IE with its criticality
    let rebuilt = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,

        REJECT MME_UE_S1AP_ID(42u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
    )?;

    // Encode it
    let encoded = hex::encode(rebuilt.encode()?);
    assert_eq!(encoded, "0012401500000300000002002a0008000200070002400202a0");
    Ok(())
}
```

```text
UEContextReleaseRequest
MME=Some(42) eNB=Some(7) cause=Some(radioNetwork(user_inactivity))
```

The text under a block is what the block prints.

This page describes oxirush-s1ap 0.3.0, which is not on crates.io yet.

## Installation

```toml
[dependencies]
oxirush-s1ap = "0.3"
# With the JSON forms:
# oxirush-s1ap = { version = "0.3", features = ["inspect"] }
# The examples of this page also use:
# hex = "0.4"
# serde_json = "1"
```

| Feature | Adds |
|---------|------|
| `inspect` | A PDU as JSON, a `oxirush_s1ap::tree::Value`: its tree, its view and the paths of its values, with the members in the order of the specification |
| `serde` | `Serialize` and `Deserialize` for every type of the bindings, through its tree. It turns on `inspect` |

Until 0.3.0 is on crates.io, a program takes the crate from its repository, and
oxirush-aper and oxirush-utils at the revisions that it is tested with:

```toml
[dependencies]
oxirush-s1ap = { git = "https://github.com/linouxis9/oxirush-s1ap", features = ["inspect"] }

[patch.crates-io]
oxirush-aper = { git = "https://github.com/linouxis9/oxirush-aper", rev = "48d0bb4607f1ac11ada9966db59297a96d5ca154" }
oxirush-utils = { git = "https://github.com/linouxis9/oxirush-utils", rev = "1fb4d2720fa0b1c036e54d3e9907b278adac0f80" }
```

In a clone, cargo takes them with `--config`, as CI does:

```sh
cargo test --all-features \
  --config 'patch.crates-io.oxirush-aper.git="https://github.com/linouxis9/oxirush-aper"' \
  --config 'patch.crates-io.oxirush-aper.rev="48d0bb4607f1ac11ada9966db59297a96d5ca154"' \
  --config 'patch.crates-io.oxirush-utils.git="https://github.com/linouxis9/oxirush-utils"' \
  --config 'patch.crates-io.oxirush-utils.rev="1fb4d2720fa0b1c036e54d3e9907b278adac0f80"'
```

The minimum supported Rust version is 1.88. The crate depends on one version of
`oxirush-aper` exactly, which is released with it. The values of the bindings
are of its types, as `Octets`, `Bits` and `Error`, which a program names
through `oxirush_s1ap::aper`.

## Answer an eNB, from S1 SETUP to the INITIAL CONTEXT SETUP

The MME's side of an eNB and of the first UE that it serves: the eNB sets up
the S1 interface, then sends the first NAS message of the UE; the MME answers
the UE in a DOWNLINK NAS TRANSPORT, and sets up the default bearer of the UE
with its security context, KeNB, in an INITIAL CONTEXT SETUP REQUEST. The
NAS-PDUs are octets of [oxirush-nas](https://github.com/linouxis9/oxirush-nas),
and KeNB is the one that the next section derives with
[oxirush-security](https://github.com/linouxis9/oxirush-security).

```rust
use oxirush_s1ap::helpers::*;
use oxirush_s1ap::{BitString, build_s1ap, build_s1ap_ie, extract_s1ap_ies, ie, s1ap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network = plmn("208", "93")?;

    // The eNB: S1 SETUP REQUEST, its identity, its name and the tracking
    // area that it serves
    let enb = global_enb_id(network, EnbKind::Macro, 0x12345)?;
    let area = SupportedTAsItem::new(TAC([0, 1]), BPLMNs(vec![network]), None);
    let request = build_s1ap!(InitiatingMessage, S1Setup, S1SetupRequest,
        Global_ENB_ID(enb),
        eNBname("enb-1"),
        SupportedTAs(SupportedTAs(vec![area])),
        DefaultPagingDRX(PagingDRX::v128),
    )?;

    // The MME reads it, and answers with its name and its GUMMEIs
    let received = S1AP_PDU::decode(&request.encode()?)?;
    extract_s1ap_ies!(received, S1SetupRequest,
        opt name: String = eNBname(name) => name.0.into_owned(),
        req areas: SupportedTAs = SupportedTAs(areas) => areas,
    );
    println!("S1 SETUP REQUEST from {}: {} tracking area", name.unwrap_or_default(), areas.0.len());
    let served = ServedGUMMEIsItem::new(
        ServedPLMNs(vec![network]),
        ServedGroupIDs(vec![MMEGroupID([0, 1])]),
        ServedMMECs(vec![MMECode([1])]),
        None,
    );
    let response = build_s1ap!(SuccessfulOutcome, S1Setup, S1SetupResponse,
        MMEname("mme-1"),
        ServedGUMMEIs(ServedGUMMEIs(vec![served])),
        RelativeMMECapacity(RelativeMMECapacity(255)),
    )?;
    println!("S1 SETUP RESPONSE {}", hex::encode(response.encode()?));

    // The eNB: INITIAL UE MESSAGE, with the ATTACH REQUEST of a UE and the
    // cell that the UE is in
    let attach = hex::decode("07410108298039000000001002e0e000040201d031")?;
    let initial = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
        eNB_UE_S1AP_ID(7u32),
        NAS_PDU(attach),
        TAI(tai(network, [0, 1])),
        EUTRAN_CGI(eutran_cgi(network, EnbKind::Macro, 0x12345, 1)?),
        RRC_Establishment_Cause(RRCEstablishmentCause::mo_Signalling),
    )?;

    // The MME reads the IEs that it needs where they are in the octets
    let octets = initial.encode()?;
    let received = S1AP_PDU::decode_borrowed(&octets)?;
    extract_s1ap_ies!(received, InitialUEMessage,
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        req nas: NASPDU = NAS_PDU(nas) => nas,
    );
    println!("INITIAL UE MESSAGE of eNB UE {enb_id}: NAS-PDU {}", hex::encode(&nas.0));

    // It answers the UE, under the MME UE S1AP ID that it gives it: here an
    // AUTHENTICATION REQUEST
    let authentication = hex::decode(
        "07520023553cbe9637a89d218ae64dae47bf3510aa689c648350b9b9a4a8043ac07aa7e0",
    )?;
    let downlink = build_s1ap!(InitiatingMessage, downlinkNASTransport, DownlinkNASTransport,
        MME_UE_S1AP_ID(1u32),
        eNB_UE_S1AP_ID(enb_id),
        NAS_PDU(authentication),
    )?;
    println!("DOWNLINK NAS TRANSPORT {}", hex::encode(downlink.encode()?));

    // After the security mode, the default bearer of the UE, whose item
    // carries the ATTACH ACCEPT, and its security context for the eNB: its
    // capabilities and KeNB
    let priority = AllocationAndRetentionPriority::new(
        PriorityLevel(8),
        PreEmptionCapability::shall_not_trigger_pre_emption,
        PreEmptionVulnerability::not_pre_emptable,
        None,
    );
    let accept = hex::decode("07420121060002f839000100155201c101090908696e7465726e657405010a000001")?;
    let bearer = ERABToBeSetupItemCtxtSUReq::new(
        ERABID(5),
        ERABLevelQoSParameters::new(QCI(9), priority, None, None),
        TransportLayerAddress(BitString::from([10, 0, 0, 1])),
        GTPTEID([0, 0, 0, 1]),
        Some(NASPDU::from(accept)),
        None,
    );
    let bearers = vec![build_s1ap_ie!(REJECT E_RABToBeSetupItemCtxtSUReq(bearer))?];
    let rates = UEAggregateMaximumBitrate::new(BitRate(100_000_000), BitRate(50_000_000), None);
    let k_enb = hex::decode("cd49a4ae7001cf3c810a89f47ced1b130f7b3f9b42bc9707431b3fb4e62b20fc")?;
    let setup = build_s1ap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
        MME_UE_S1AP_ID(1u32),
        eNB_UE_S1AP_ID(enb_id),
        uEaggregateMaximumBitrate(rates),
        E_RABToBeSetupListCtxtSUReq(ERABToBeSetupListCtxtSUReq(bearers)),
        UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
        SecurityKey(<[u8; 32]>::try_from(k_enb).unwrap()),
    )?;

    // The eNB checks it against its object set, and takes the key
    let received = S1AP_PDU::decode(&setup.encode()?)?;
    let deviations = received.check()?;
    let key = received.ie::<ie::SecurityKey>().unwrap()?;
    let key = hex::encode(key.0.as_octets());
    println!("INITIAL CONTEXT SETUP REQUEST, {} deviations: KeNB {key}", deviations.len());
    Ok(())
}
```

```text
S1 SETUP REQUEST from enb-1: 1 tracking area
S1 SETUP RESPONSE 20110022000003003d400702006d6d652d310069000b000002f83900000001000100574001ff
INITIAL UE MESSAGE of eNB UE 7: NAS-PDU 07410108298039000000001002e0e000040201d031
DOWNLINK NAS TRANSPORT 000b4038000003000000020001000800020007001a00252407520023553cbe9637a89d218ae64dae47bf3510aa689c648350b9b9a4a8043ac07aa7e0
INITIAL CONTEXT SETUP REQUEST, 0 deviations: KeNB cd49a4ae7001cf3c810a89f47ced1b130f7b3f9b42bc9707431b3fb4e62b20fc
```

## With oxirush-nas and oxirush-security

The same exchange, with the NAS-PDUs built and read by
[oxirush-nas](https://github.com/linouxis9/oxirush-nas) and the keys of
[oxirush-security](https://github.com/linouxis9/oxirush-security): the ATTACH
REQUEST of the INITIAL UE MESSAGE, the SECURITY MODE COMMAND that the MME
protects with the new EPS security context in a DOWNLINK NAS TRANSPORT, and
KeNB from KASME and the uplink NAS COUNT.

```rust,ignore
use oxirush_nas::nas_eps::*;
use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, helpers, s1ap::*};
use oxirush_security::milenage::{Amf, K, Milenage, Opc, Sqn};
use oxirush_security::nas_eps::{derive_kasme, derive_kenb};
use oxirush_security::Rand;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The UE: an ATTACH REQUEST with its IMSI, read from a capture
    let attach = NasEpsMessage::decode(&hex::decode("07410108298039000000001002e0e000040201d031")?)?;

    // The eNB: the INITIAL UE MESSAGE carries its octets
    let network = helpers::plmn("208", "93")?;
    let cell = helpers::eutran_cgi(network, helpers::EnbKind::Macro, 0x12345, 1)?;
    let initial = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
        eNB_UE_S1AP_ID(7u32),
        NAS_PDU(attach.encode()?),
        TAI(helpers::tai(network, [0, 1])),
        EUTRAN_CGI(cell),
        RRC_Establishment_Cause(RRCEstablishmentCause::mo_Signalling),
    )?;

    // The MME reads the NAS-PDU as a NAS message
    extract_s1ap_ies!(initial, InitialUEMessage,
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        req nas: NASPDU = NAS_PDU(nas) => nas,
    );
    let NasEpsMessage::Emm(_, NasEmmMessage::AttachRequest(request)) = NasEpsMessage::decode(&nas.0)?
    else {
        unreachable!()
    };
    println!("eNB UE {enb_id}: IMSI {}", request.eps_mobile_identity.as_imsi().unwrap());

    // EPS AKA gives KASME, from the vector of the subscriber of TS 35.208 test set 1
    let octets = |text: &str| hex::decode(text).unwrap();
    let k = K::from_slice(&octets("465b5ce8b199b49faa5f0a2ee238a6bc"))?;
    let opc = Opc::from_slice(&octets("cd63cb71954a9f4e48a5994e37a02baf"))?;
    let rand = Rand::from_slice(&octets("23553cbe9637a89d218ae64dae47bf35"))?;
    let (sqn, amf) = (Sqn::try_from(32)?, Amf::new(&[0xb9, 0xb9]));
    let vector = Milenage::new(&k, &opc).authentication_vector(&rand, &sqn, &amf);
    let kasme = derive_kasme(vector.ck(), vector.ik(), Plmn::new("208", "93")?, vector.autn());

    // The SECURITY MODE COMMAND, protected with the new context, in DOWNLINK NAS TRANSPORT
    let (eia, eea) = (IntegrityAlgorithm::EIA2, CipheringAlgorithm::EEA2);
    let mut context = NasSecurityContext::from_fresh_kasme(&kasme, eia, eea)?;
    let ksi = NasKeySetIdentifier::from_key_set_identifier(KeySetIdentifier::Native(0))?;
    let capability = NasUeSecurityCapability::from_ue_network_capability(&request.ue_network_capability);
    let algorithms = NasSecurityAlgorithms::from_algorithms(eea, eia);
    let command = NasEpsMessage::new_emm(NasSecurityModeCommand::new(algorithms, ksi, capability));
    let header = NasEpsSecurityHeaderType::IntegrityProtectedWithNewEpsSecurityContext;
    let downlink = build_s1ap!(InitiatingMessage, downlinkNASTransport, DownlinkNASTransport,
        MME_UE_S1AP_ID(1u32),
        eNB_UE_S1AP_ID(enb_id),
        NAS_PDU(context.protect(&command, header, Direction::Downlink)?),
    )?;
    println!("{}", hex::encode(downlink.encode()?));

    // The SECURITY MODE COMPLETE came with NAS COUNT 0: KeNB is derived from
    // it, for the INITIAL CONTEXT SETUP REQUEST
    let k_enb = derive_kenb(&kasme, NasCount::ZERO);
    println!("KeNB {}", hex::encode(k_enb.as_octets()));
    Ok(())
}
```

```text
eNB UE 7: IMSI 208930000000001
000b4021000003000000020001000800020007001a000e0d37470f7b5100075d220002e0e0
KeNB cd49a4ae7001cf3c810a89f47ced1b130f7b3f9b42bc9707431b3fb4e62b20fc
```

## Decode a PDU

```rust
use oxirush_s1ap::{S1apPduKind, s1ap::*};

fn receive(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let pdu = S1AP_PDU::decode(bytes)?;

    Ok(match pdu.kind() {
        S1apPduKind::Initiating_UEContextReleaseRequest => {
            let request: UEContextReleaseRequest = pdu.decode_value()?;
            format!("a release request with {} IEs", request.protocol_ies.0.len())
        }
        S1apPduKind::Other { direction, procedure_code } => {
            format!("procedure {procedure_code} has no {direction}")
        }
        kind => format!("{kind:?} is not handled here"),
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A UE CONTEXT RELEASE REQUEST, then an S1 SETUP FAILURE
    let request = hex::decode("0012401500000300000002002a000800020007000240020280")?;
    println!("{}", receive(&request)?);

    let failure = hex::decode("401100080000010002400145")?;
    println!("{}", receive(&failure)?);
    Ok(())
}
```

```text
a release request with 3 IEs
Unsuccessful_S1Setup is not handled here
```

`decode` reads the envelope of a PDU alone: its direction, its procedure code,
its criticality and the octets of its message. `decode_value`, `protocol_ies`
and `check` read the message, and what does not decode there is their error.

| Of an `S1AP_PDU` | Gives |
|------------------|-------|
| `decode(&bytes)`, `decode_borrowed(&bytes)` | The PDU of the octets of one message, whole. `decode_borrowed` copies nothing: the PDU borrows from `bytes` |
| `encode()`, `encode_into(&mut octets)` | Its octets |
| `procedure_code()`, `procedure_name()`, `direction()`, `kind()` | `18`, `"UEContextReleaseRequest"`, `"InitiatingMessage"`, and an `S1apPduKind`, which is `Other` for a procedure that the specification does not have in that direction |
| `decode_value()`, `message()` | The message as its type, which borrows from the PDU, or from what the PDU borrows |
| `protocol_ies()`, `set_protocol_ies(ies)` | The IEs of its message, whatever the message is |
| `check()` | How it deviates from the object set of its message |

## Build a PDU

An IE is named by its ASN.1 identifier without `id-` and with `_` for `-`
(`MME_UE_S1AP_ID`). A type is named by the name that ASN.1 gives it without its
hyphens, with the letter after each hyphen in upper case:
`Pre-emptionCapability` is `PreEmptionCapability`, the IE `NAS_PDU` has the
type `NASPDU` and `E_RAB_ID` `ERABID`. A field has the words of its member in
lower case (`pLMNidentity` is `plmn_identity`). The
two identifiers of a UE are types of the names of their IEs too:
`MME_UE_S1AP_ID` and `ENB_UE_S1AP_ID`, whose IE is `eNB_UE_S1AP_ID`. A
criticality that a call leaves out is the one that TS 36.413 assigns, and a
name that the crate does not have, or a message that the procedure does not
have in that direction, does not compile.

```rust
use oxirush_s1ap::{Message, build_s1ap, s1ap::*};

fn main() -> Result<(), oxirush_s1ap::Error> {
    // The criticality of the procedure, and those of the object set of the message
    let pdu = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
        eNB_UE_S1AP_ID(7u32),
        NAS_PDU(vec![0x07, 0x41, 0x71]),
    )?;
    println!("{:?} {}", pdu.criticality(), hex::encode(pdu.encode()?));

    // The PDU of a message that is at hand
    let message: InitialUEMessage = pdu.decode_value()?;
    let again = S1AP_PDU::of_message(&message, InitialUEMessage::CRITICALITY)?;
    println!("{}", again == pdu);

    // An IE that the object set of the message does not have is assigned none
    let error = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
        TimeToWait(TimeToWait::v10s),
    );
    println!("{}", error.unwrap_err());
    Ok(())
}
```

```text
ignore 000c4011000002000800020007001a000403074171
true
TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no criticality: write its criticality
```

## Read the IEs of a message

```rust
use oxirush_s1ap::{Error, extract_s1ap_ies, s1ap::*};

fn handle(request: &UEContextReleaseRequest) -> Result<(), Error> {
    extract_s1ap_ies!(request, UEContextReleaseRequest,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        req cause: Cause = Cause(value) => value,
    );

    println!("MME={mme_id} eNB={enb_id} cause={cause:?}");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A UE CONTEXT RELEASE REQUEST
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280")?;
    handle(&S1AP_PDU::decode(&bytes)?.decode_value()?)?;
    Ok(())
}
```

```text
MME=42 eNB=7 cause=radioNetwork(user_inactivity)
```

A field is `req`, `opt` or `lax`, its name and its type, then the IE with a
name for its decoded value; after `=>` comes what the field keeps of it.

The macro takes the PDU too, as the message that it holds, without
`decode_value()`: it walks the IEs of the message once, where they are, and
decodes those that a field asks for. `ie::<T>()` and `ies::<T>()` of a PDU and
of a message read one IE, by its marker type in `oxirush_s1ap::ie`: the first
occurrence, and every occurrence in order.

```rust
use oxirush_s1ap::{Error, extract_s1ap_ies, ie, s1ap::*};

fn main() -> Result<(), Error> {
    // A UE CONTEXT RELEASE REQUEST, read in the buffer that it was received in
    let octets = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    let pdu = S1AP_PDU::decode_borrowed(&octets)?;

    // The IEs of its message, from the PDU: the Cause is not decoded
    extract_s1ap_ies!(pdu, UEContextReleaseRequest,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
    );
    println!("MME={mme_id} eNB={enb_id}");

    // One IE, by its marker type
    let cause = pdu.ie::<ie::Cause>().unwrap()?;
    println!("{cause:?}");
    Ok(())
}
```

```text
MME=42 eNB=7
radioNetwork(user_inactivity)
```

When a message has an IE twice, the macro, `ie` and `with_s1ap_ie_mut!` take
the first one and decode no other; `ies` gives each, and `check()` reports the
second as `Deviation::RepeatedIe`.

| Field | Is | When the IE is not there | When it does not decode |
|-------|----|--------------------------|-------------------------|
| `req` | A `T` | An error of the kind `MissingIe` | An error of the kind `UndecodableIe` |
| `opt` | An `Option<T>` | `None` | The same error |
| `lax` | An `Option<T>` | `None` | `None` |

The error is that of the enclosing function, converted by `?`. A `lax` field
reads an IE as TS 36.413 clause 10.3.4.2 has a receiver read one of the
criticality `ignore`: as if it had not been received when it does not decode.

`with_s1ap_ie_mut!` changes one IE of a message in place, and `build_s1ap_ie!`
builds one: their documentation has an example of each.

## A list of single containers, and a handover container, typed

An item of a list of E-RABs is an IE alone in its container: `build_s1ap_ie!`
builds it without the name of a message. The container of a handover holds
the type that TS 36.413 clause 9.2.1.56 names. `decode_open_type` reads such
octets as the type that the caller names, and `encode_open_type` writes them.

```rust
use oxirush_s1ap::{BitString, build_s1ap, build_s1ap_ie, extract_s1ap_ies, s1ap::*};

fn main() -> Result<(), oxirush_s1ap::Error> {
    // One E-RAB to set up, as the item of its list
    let priority = AllocationAndRetentionPriority::new(
        PriorityLevel(1),
        PreEmptionCapability::shall_not_trigger_pre_emption,
        PreEmptionVulnerability::not_pre_emptable,
        None,
    );
    let bearer = ERABToBeSetupItemCtxtSUReq::new(
        ERABID(5),
        ERABLevelQoSParameters::new(QCI(9), priority, None, None),
        TransportLayerAddress(BitString::from([10, 0, 0, 1])),
        GTPTEID([0, 0, 0, 1]),
        None,
        None,
    );
    let item = build_s1ap_ie!(REJECT E_RABToBeSetupItemCtxtSUReq(bearer))?;
    let pdu = build_s1ap!(InitiatingMessage, InitialContextSetup, REJECT,
        InitialContextSetupRequest,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT E_RABToBeSetupListCtxtSUReq(vec![item]),
    )?;

    // Read back: the list, then each item as its type
    let request: InitialContextSetupRequest = pdu.decode_value()?;
    extract_s1ap_ies!(request, InitialContextSetupRequest,
        req bearers: ERABToBeSetupListCtxtSUReq = E_RABToBeSetupListCtxtSUReq(list) => list,
    );
    for item in &bearers.0 {
        let bearer: ERABToBeSetupItemCtxtSUReq = decode_open_type(&item.value)?;
        println!("E-RAB {} with QCI {}", bearer.e_rab_id.0, bearer.e_rab_level_qos_parameters.qci.0);
    }

    // Or each item of the list, walked in the PDU without building the list
    for bearer in pdu.items::<oxirush_s1ap::ie::E_RABToBeSetupListCtxtSUReq>() {
        println!("E-RAB {}", bearer?.e_rab_id.0);
    }

    // The octets of a Source-ToTarget-TransparentContainer, as the type that they carry
    let octets = hex::decode("000201020002f83900012340200100").unwrap();
    let container: SourceeNBToTargeteNBTransparentContainer = decode_open_type(&octets)?;
    println!("{:?}", container.target_cell_id);
    assert_eq!(encode_open_type(&container)?.as_ref(), &octets[..]);
    Ok(())
}
```

```text
E-RAB 5 with QCI 9
E-RAB 5
EUTRANCGI { plmn_identity: PLMNidentity(208-93), cell_id: CellIdentity(0x1234/28), ie_extensions: None }
```

## Check a message, and answer one that is faulty

`check()` reports how a message deviates from the object set that the ASN.1
gives it, and refuses nothing.

```rust
use oxirush_s1ap::{build_s1ap, s1ap::*};

fn main() -> Result<(), oxirush_s1ap::Error> {
    // A UE CONTEXT RELEASE REQUEST that deviates in each way, built all the same
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
        REJECT, UEContextReleaseRequest,
        REJECT eNB_UE_S1AP_ID(7u32),
        IGNORE MME_UE_S1AP_ID(42u32),
        IGNORE TimeToWait(TimeToWait::v5s),
        REJECT eNB_UE_S1AP_ID(8u32),
    )?;

    // It is encoded and decoded as any other, and `check()` reports
    let received = S1AP_PDU::decode(&pdu.encode()?)?;
    for deviation in received.check()? {
        println!("{deviation}");
    }

    // The message of the first example deviates in nothing
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    assert!(S1AP_PDU::decode(&bytes)?.check()?.is_empty());
    Ok(())
}
```

```text
the criticality of the PDU is reject, and that of its procedure is ignore
MME-UE-S1AP-ID (IE 0), at position 1, comes after eNB-UE-S1AP-ID (IE 8), which the object set of the message has after it
the criticality of MME-UE-S1AP-ID (IE 0), at position 1, is ignore, and the message assigns it reject
TimeToWait (IE 65), at position 2, is not an IE of the message
eNB-UE-S1AP-ID (IE 8), at position 3, is in the message already, at position 0
Cause (IE 2) is mandatory, and the message does not have it
```

| `Deviation` | Is |
|-------------|----|
| `PduCriticality`, `IeCriticality` | A criticality of the PDU or of an IE that is not the assigned one |
| `UnexpectedIe`, `RepeatedIe`, `OutOfOrder`, `MissingIe` | An IE that the message cannot have, has twice, has in a wrong order, or lacks |
| `UndecodableIe` | An IE whose value does not decode as its type |
| `OutsideRoot` | A value of a later version of its type, outside the root of an extensible constraint |
| `NotCanonical` | A value that is whole in another form than the one that ITU-T X.691 gives it: a `Fault`, with its clause and its bit |
| `UnknownProcedure` | A procedure that the specification does not have in the direction of the PDU, as one that a later release adds |

`check()` decodes the value of each IE. It does not look into what a value
holds as octets, and does not judge a conditional IE, whose condition is
prose.

`check_structure()` compares the container alone with the object set: the
presence, the order, the repetition and the criticality of the IEs, which TS
36.413 clause 10.3 has a receiver answer. It decodes no value. A receiver that
checks the structure and then reads the IEs that it needs, with the macro or
`ie`, pays for those IEs alone. Decoding an IE checks its value: one that does
not decode is an error of the kind `UndecodableIe`, which `Deviation::of`
makes a deviation of. `check()` is for a tool that looks at every value, and
costs 5 to 8 times as much ([`benches/`](benches/README.md)).

Clause 10 of TS 36.413 has what the receiver of a message that deviates
answers: `cause()` of a deviation is the protocol cause, and
`criticality_diagnostics_for_response()` of the PDU the Criticality Diagnostics
IE of a response, with an item for each IE of the criticality `reject` or
`notify` (`criticality_diagnostics()` is that of an ERROR INDICATION).

```rust
use oxirush_s1ap::{Deviation, build_s1ap, s1ap::*};

fn main() -> Result<(), oxirush_s1ap::Error> {
    // An S1 SETUP REQUEST with none of its mandatory IEs but the paging DRX
    let request = build_s1ap!(InitiatingMessage, S1Setup, REJECT, S1SetupRequest,
        IGNORE DefaultPagingDRX(PagingDRX::v128),
    )?;
    let received = S1AP_PDU::decode(&request.encode()?)?;
    let deviations = received.check()?;

    // Clause 10.3.5: the procedure is rejected, and the missing IEs are reported
    let cause = deviations.iter().find_map(Deviation::cause).unwrap();
    let failure = build_s1ap!(UnsuccessfulOutcome, S1Setup, REJECT, S1SetupFailure,
        IGNORE Cause(Cause::protocol(cause)),
        IGNORE CriticalityDiagnostics(received.criticality_diagnostics_for_response(&deviations)),
    )?;
    for deviation in &deviations {
        println!("{deviation}");
    }
    let octets = hex::encode(failure.encode()?);
    println!("{octets}");
    Ok(())
}
```

```text
Global-ENB-ID (IE 59) is mandatory, and the message does not have it
SupportedTAs (IE 64) is mandatory, and the message does not have it
401100150000020002400131003a4009180100003b40004040
```

## A PDU as JSON

With the `inspect` feature a PDU is JSON in two forms: a tree, in which a
message is shown and written, and a view, which keeps what was received and
is read and edited by paths with the functions of the [`inspect`] module.
`/s1ap` is the IEs of the message, and an IE goes by its name. The two forms,
and the serde form, carry the keys of a message, as KeNB, in hexadecimal: they
are no form for a log.

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A UE CONTEXT RELEASE REQUEST, as a tree
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280")?;
    let pdu = S1AP_PDU::decode(&bytes)?;
    println!("{}", pdu.to_tree());

    // Its view: each value with the path that selects it
    let mut view = pdu.to_view()?;
    for (path, value) in inspect::paths(&view) {
        println!("{path} = {value}");
    }

    // A value, and another alternative of a CHOICE in place of the one that is there
    inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", tree!(9))?;
    inspect::set(&mut view, "/s1ap/Cause/nas", tree!("Normal Release"))?;
    let edited = pdu.with_view(&view)?;
    println!("{}", edited.to_tree());

    // A name that the type does not have is refused, and the view is as it was
    let wrong = inspect::set(&mut view, "/s1ap/Cause/radioNetwrok", tree!("unspecified"));
    println!("{}", wrong.unwrap_err());
    Ok(())
}
```

```text
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
/procedure_code = 18
/direction = "InitiatingMessage"
/criticality = "ignore"
/s1ap/MME-UE-S1AP-ID/id = 0
/s1ap/MME-UE-S1AP-ID/criticality = "reject"
/s1ap/MME-UE-S1AP-ID/value = 42
/s1ap/eNB-UE-S1AP-ID/id = 8
/s1ap/eNB-UE-S1AP-ID/criticality = "reject"
/s1ap/eNB-UE-S1AP-ID/value = 7
/s1ap/Cause/id = 2
/s1ap/Cause/criticality = "ignore"
/s1ap/Cause/value/radioNetwork = "user-inactivity"
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":9},{"Cause":{"nas":"normal-release"}}]}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc; an IE itself has id, criticality, value, octets, at /s1ap/Cause/radioNetwrok
```

The [`inspect`] module has examples of the other tasks: add or remove an IE,
a repeated IE, an item of a list, the names of an ENUMERATED, a path checked
before use, the container of a handover; and the rules of the paths, of the
edits and of the values.

## Write a message from its tree

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A tree: the name of the message, whatever its case and its separators, then its IEs
    let request = tree!({
        "message": "ue context release request",
        "ies": [
            {"MME-UE-S1AP-ID": 42},
            {"eNB-UE-S1AP-ID": 7},
            {"Cause": {"radioNetwork": "user-inactivity"}},
            // An IE that the message does not have says its criticality
            {"eNBname": "enb-1", "criticality": "ignore"},
        ],
    });

    let pdu = S1AP_PDU::from_tree(&request)?;
    println!("{}", hex::encode(pdu.encode()?));

    // The name gave the direction, the procedure code and the criticalities
    let view = pdu.to_view()?;
    println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
    println!("{}", tree!(inspect::select(&view, "/s1ap/*/criticality")?));
    Ok(())
}
```

```text
0012402000000400000002002a000800020007000240020280003c40070200656e622d31
"InitiatingMessage" 18 "ignore"
["reject","reject","ignore","ignore"]
```

An IE may be its octets, whatever they are:

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
    let request = tree!({"message": "UEContextReleaseRequest", "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
    ]});
    let pdu = S1AP_PDU::from_tree(&request)?;
    println!("{}", hex::encode(pdu.encode()?));

    // A view shows an IE that does not decode by its octets, and says why
    let mut view = pdu.to_view()?;
    println!("{}", view["message"]["protocolIEs"][1]);

    // In a view, the `octets` of an IE are set in place of its value
    inspect::set(&mut view, "/s1ap/MME-UE-S1AP-ID/octets", tree!("00ff"))?;
    let edited = pdu.with_view(&view)?;
    println!("{}", edited.to_tree());
    Ok(())
}
```

```text
0012401000000200000002002afde80003c0ffee
{"id":65000,"criticality":"reject","value":"c0ffee","_decode_error":"S1AP IE 65000 is unknown","_raw_value":"c0ffee","_original_id":65000}
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":255},{"65000":{"octets":"c0ffee"},"criticality":"reject"}]}
```

## Any format of serde

With the `serde` feature, a PDU is serialized as its tree and deserialized
from one, in any format that serde has, and each other type of the bindings
as the tree of a PDU shows it.

```rust
use oxirush_s1ap::helpers::{plmn, tai};
use oxirush_s1ap::s1ap::S1AP_PDU;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280")?;
    let pdu = S1AP_PDU::decode(&bytes)?;
    let text = serde_json::to_string(&pdu)?;
    let read: S1AP_PDU = serde_json::from_str(&text)?;
    assert_eq!(read.encode()?, bytes);

    // A value alone has the form that a tree shows it in: the MCC and the MNC, and a number
    let area = tai(plmn("208", "93")?, [0x00, 0x01]);
    println!("{}", serde_json::to_string(&area)?);
    Ok(())
}
```

```text
{"pLMNidentity":"208-93","tAC":1}
```

## Limits

- Procedures, state and transport: it is a codec.
- Judge a conditional IE: `check()` reports what the object sets say, and a
  condition is prose.
- Keep what a later release added to a value that is decoded as its type and
  encoded again: a PDU keeps the octets of its IEs until one is decoded, and
  a tree and a view keep them.
- Write or read the JSON Encoding Rules (ITU-T X.697): a tree is the JSON of
  the crate, by the names of the specification. The tests compare it with the
  JER of rasn, and read the values of pycrate in those rules, through
  functions that the documentation hides.
- Look into an RRC container, or into the container of a handover for
  another system than E-UTRAN: they stay octets.
- The crate has no limit on the octets of a PDU, and each call takes time and
  memory in proportion to them, a view up to 143 octets for an octet
  ([`to_view`]): a receiver bounds what a peer makes it spend by what it reads
  from SCTP.
- The bindings are those of TS 36.413 V19.2.0 (Release 19), from the archive
  `36413-j20.zip`, in aligned PER (ITU-T X.691 (02/2021)).
- No PDU of a commercial RAN or core is among the fixtures: what such equipment
  sends that open-source nodes did not is untested.

## Performance

At 774c1b5, on one core of an AMD Ryzen 7 5700G (16 threads, Linux 7.0, Rust
1.98.1) of a quiet machine, on 2026-10-18, the median of three runs: the
macro on a PDU reads the IEs that its fields ask for in 24 to 33 ns, and
`ie::<T>()` one IE in 28 to 29 ns, without an allocation; `check_structure()`
takes 48 to 156 ns, and `check()` 5 to 8 times as much.
[`benches/`](benches/README.md) has these figures by message, every figure
of the comparison for InitialContextSetupRequest, the clean builds and a
message of 65,535 IEs.

### Against other implementations

The crate against four other implementations of S1AP, on the six PDUs of
`benches/compare/pdus.tsv`, which each decodes and encodes. The envelope alone
is `S1AP_PDU::decode`, which leaves the value of each IE in its octets; "this
crate" decodes the PDU and then every IE of its message as its type, which is
what the others do; and encoding writes the PDU from the typed values of its
IEs, as `build_s1ap!` does.

Decoding, ns per call, and in parentheses the ratio of each other implementation, <other> / this crate:

| PDU (octets) | this crate, the envelope alone | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| DownlinkNASTransport (73) | 8 | 79 | 691 (8.8x) | 860 (11x) | 1,750 (22x) | 69,190 (880x) |
| InitialContextSetupRequest (197) | 8 | 221 | 2,123 (9.6x) | 3,320 (15x) | 4,956 (22x) | 218,657 (990x) |
| S1SetupResponse (45) | 8 | 176 | 1,104 (6.3x) | 1,308 (7.4x) | 2,098 (12x) | 95,205 (540x) |
| HandoverRequest (146) | 8 | 266 | 2,519 (9.5x) | 3,554 (13x) | 5,369 (20x) | 255,711 (960x) |
| UEContextReleaseCommand (21) | 8 | 95 | 611 (6.4x) | 637 (6.7x) | 1,327 (14x) | 66,970 (700x) |
| InitialUEMessage (48) | 8 | 136 | 1,277 (9.4x) | 1,391 (10x) | 2,482 (18x) | 120,191 (880x) |

Encoding, ns per call, and in parentheses the ratio of each other implementation, <other> / this crate:

| PDU (octets) | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|
| DownlinkNASTransport (73) | 64 | 880 (14x) | 1,528 (24x) | 1,113 (18x) | 45,097 (710x) |
| InitialContextSetupRequest (197) | 144 | 1,757 (12x) | 4,420 (31x) | 2,920 (20x) | 135,987 (940x) |
| S1SetupResponse (45) | 85 | 1,047 (12x) | 1,701 (20x) | 977 (12x) | 61,261 (720x) |
| HandoverRequest (146) | 170 | 2,158 (13x) | 4,799 (28x) | 3,014 (18x) | 154,533 (910x) |
| UEContextReleaseCommand (21) | 64 | 770 (12x) | 1,226 (19x) | 715 (11x) | 43,780 (680x) |
| InitialUEMessage (48) | 95 | 1,183 (12x) | 2,178 (23x) | 1,332 (14x) | 72,281 (760x) |

The figures were taken at 774c1b5 on an AMD Ryzen 7 5700G (16 threads, Linux 7.0,
Rust 1.98.1), each program on one core of a quiet machine, on 2026-10-18.
[`benches/`](benches/README.md#the-method-of-the-comparison) has the method and
the version of each implementation that the script fetches. From the root of
the repository, with oxirush-aper beside it, whose `protocols/benches/compare/`
has the script:

```sh
python3 ../oxirush-aper/protocols/benches/compare/compare.py --core 3 --perf "sudo -n perf"
```

## Tests and fuzzing

The tests read the 101 messages and the 284 vectors that pycrate 0.8.1
encoded from the ASN.1, and 81 PDUs that the MME of Open5GS 2.8.0 sent in lab
runs, through the codec, the views and the trees; `oracle/` compares each,
cut, lengthened and with changed bits, with the codec of rasn 0.29, and
`protocols/tests/package.sh` of oxirush-aper runs the tests from the package
alone. `fuzz/` has three
[cargo-fuzz](https://crates.io/crates/cargo-fuzz) targets, `decode`, `view`
and `paths`, which CI runs for a minute each from the PDUs of the tests and
from each of them under a procedure code that the specification does not
have. `examples/` has four programs, which build, read, decode by hand and
inspect messages.

```sh
cargo test --all-features   # with the --config lines of Installation
cd oracle && cargo test     # against the codec of rasn
../oxirush-aper/protocols/fuzz/seed.sh && RUSTFLAGS="--cfg oxirush_s1ap_hooks" cargo +nightly fuzz run view -- -max_total_time=60
cargo run --example s1ap_build_pdu
```

The bindings are generated: [CONTRIBUTING.md](CONTRIBUTING.md) says how, how
the vectors were made and the PDUs captured, and what a change to them or to
the pinned revisions takes. The Rust blocks of this page are checked with what
they print (`tests/readme.rs`, which `protocols/tests/readme.py` of
oxirush-aper writes from this page). The blocks that use other crates of the
family run in the package of [`family/`](family/), which takes those crates at
the revisions that its manifest names.

A pull request comes with a test for what it changes. Sign off your commits
(`git commit -s`): by contributing you agree to the [Developer Certificate
of Origin](https://developercertificate.org/). [SECURITY.md](SECURITY.md)
says how to report a vulnerability.

## The OxiRush family

- [oxirush-utils](https://github.com/linouxis9/oxirush-utils): the 3GPP identities, and the JSON tree with its paths that the others share.
- [oxirush-aper](https://github.com/linouxis9/oxirush-aper): aligned PER, the runtime of the NGAP and S1AP codecs.
- [oxirush-nas](https://github.com/linouxis9/oxirush-nas): the NAS messages of 5GS and EPS.
- [oxirush-security](https://github.com/linouxis9/oxirush-security): NAS security, AKA and the derivation of keys.
- [oxirush-ngap](https://github.com/linouxis9/oxirush-ngap): NGAP, between a gNB and an AMF.
- [oxirush-s1ap](https://github.com/linouxis9/oxirush-s1ap): S1AP, between an eNB and an MME.
- [oxirush-gtp-u](https://github.com/linouxis9/oxirush-gtp-u): GTP-U tunnels, with a test UPF.
- [oxirush-sctp](https://github.com/linouxis9/oxirush-sctp): SCTP sockets, which carry NGAP and S1AP.

## License

Copyright 2025-2026 Valentin D'Emmanuele. Licensed under the Apache License,
Version 2.0: see [LICENSE](LICENSE).

[`inspect`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/index.html
[`to_view`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/s1ap/s1_ap_pdu_descriptions/enum.S1APPDU.html#method.to_view
