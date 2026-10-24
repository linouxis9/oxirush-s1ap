# oxirush-s1ap

[![Crates.io](https://img.shields.io/crates/v/oxirush-s1ap.svg)](https://crates.io/crates/oxirush-s1ap)
[![Documentation](https://docs.rs/oxirush-s1ap/badge.svg)](https://docs.rs/oxirush-s1ap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

S1AP, the protocol between an eNB and an MME (3GPP TS 36.413), in Rust:
types generated from the specification's ASN.1, and macros that build a
message and read its IEs by name.

```rust
use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, s1ap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Decode a UE CONTEXT RELEASE REQUEST
    let bytes = hex::decode("0012401500000300000002002a000800020007000240020280")?;
    let pdu = S1AP_PDU::decode(&bytes)?;
    println!("{pdu}");

    // Read its IEs by name: `req` fails when the IE is missing, `opt` gives an Option
    extract_s1ap_ies!(pdu, UEContextReleaseRequest,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        opt cause: Cause = Cause(cause) => cause,
    );
    println!("MME UE {mme_id}, eNB UE {enb_id}, cause {cause:?}");

    // Build the same message with another cause, and encode it
    let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest, UEContextReleaseRequest,
        MME_UE_S1AP_ID(42u32),
        eNB_UE_S1AP_ID(7u32),
        Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
    )?;
    println!("{}", hex::encode(pdu.encode()?));
    Ok(())
}
```

```text
UEContextReleaseRequest (procedure 18, initiating message), MME-UE-S1AP-ID 42, eNB-UE-S1AP-ID 7
MME UE 42, eNB UE 7, cause Some(radioNetwork(user_inactivity))
0012401500000300000002002a0008000200070002400202a0
```

Each example is followed by its output.

This page describes oxirush-s1ap 0.3.0, which is not on crates.io yet.

## Installation

```toml
[dependencies]
oxirush-s1ap = "0.3"
# With the JSON forms:
# oxirush-s1ap = { version = "0.3", features = ["inspect"] }
# The examples of this page also use:
# hex = "0.4"
```

| Feature | Adds |
|---------|------|
| `inspect` | A PDU as JSON: its tree, its view, and paths to read and edit them |
| `serde` | `Serialize` and `Deserialize` for every type, through its tree. Turns on `inspect` |

Until 0.3.0 is on crates.io, a program takes the crate from its repository, and
oxirush-aper and oxirush-utils at the revisions that it is tested with:

```toml
[dependencies]
oxirush-s1ap = { git = "https://github.com/linouxis9/oxirush-s1ap", features = ["inspect"] }

[patch.crates-io]
oxirush-aper = { git = "https://github.com/linouxis9/oxirush-aper", rev = "93754a798eba4b3f36fb37928bb12d21d5ec8961" }
oxirush-utils = { git = "https://github.com/linouxis9/oxirush-utils", rev = "1fb4d2720fa0b1c036e54d3e9907b278adac0f80" }
```

In a clone, cargo takes them with `--config`, as CI does:

```sh
cargo test --all-features \
  --config 'patch.crates-io.oxirush-aper.git="https://github.com/linouxis9/oxirush-aper"' \
  --config 'patch.crates-io.oxirush-aper.rev="93754a798eba4b3f36fb37928bb12d21d5ec8961"' \
  --config 'patch.crates-io.oxirush-utils.git="https://github.com/linouxis9/oxirush-utils"' \
  --config 'patch.crates-io.oxirush-utils.rev="1fb4d2720fa0b1c036e54d3e9907b278adac0f80"'
```

The minimum supported Rust version is 1.88. The crate depends on one exact
version of oxirush-aper, whose types (`Octets`, `Bits`, `Error`) the
bindings use. It re-exports it as `oxirush_s1ap::aper`.

## Answer an eNB, from S1 SETUP to INITIAL CONTEXT SETUP

The MME's side of an eNB and its first UE. The eNB sets up the S1 interface,
then forwards the UE's first NAS message. The MME answers the UE in a
DOWNLINK NAS TRANSPORT, and once NAS security is on, sets up the UE's
default bearer in an INITIAL CONTEXT SETUP REQUEST, with its security
context and KeNB. S1AP carries each NAS message as octets, in a NAS-PDU.

```rust
use oxirush_s1ap::helpers::*;
use oxirush_s1ap::{BitString, build_s1ap, build_s1ap_ie, extract_s1ap_ies, s1ap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network = plmn("208", "93")?;

    // The eNB: its identity, its name and the tracking area it serves
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
    println!("{received}: {}, {} tracking area", name.unwrap_or_default(), areas.0.len());
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
    println!("{response}");

    // The eNB forwards an ATTACH REQUEST, with the cell the UE is in
    let attach = hex::decode("07410108298039000000001002e0e000040201d031")?;
    let initial = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
        eNB_UE_S1AP_ID(7u32),
        NAS_PDU(attach),
        TAI(tai(network, [0, 1])),
        EUTRAN_CGI(eutran_cgi(network, EnbKind::Macro, 0x12345, 1)?),
        RRC_Establishment_Cause(RRCEstablishmentCause::mo_Signalling),
    )?;

    // The MME reads the two IEs it needs, in place in the received octets
    let octets = initial.encode()?;
    let received = S1AP_PDU::decode_borrowed(&octets)?;
    extract_s1ap_ies!(received, InitialUEMessage,
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        req nas: NASPDU = NAS_PDU(nas) => nas,
    );
    println!("{received}: a NAS-PDU of {} octets", nas.0.len());

    // It gives the UE an MME UE S1AP ID, and answers with an AUTHENTICATION REQUEST
    let authentication = hex::decode(
        "07520023553cbe9637a89d218ae64dae47bf3510aa689c648350b9b9a4a8043ac07aa7e0",
    )?;
    let downlink = build_s1ap!(InitiatingMessage, downlinkNASTransport, DownlinkNASTransport,
        MME_UE_S1AP_ID(1u32),
        eNB_UE_S1AP_ID(enb_id),
        NAS_PDU(authentication),
    )?;
    println!("{downlink}");

    // After the security mode: the default bearer, whose item carries the
    // ATTACH ACCEPT, and the UE's security capabilities and KeNB
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
    let mut k_enb = [0; 32];
    hex::decode_to_slice("cd49a4ae7001cf3c810a89f47ced1b130f7b3f9b42bc9707431b3fb4e62b20fc", &mut k_enb)?;
    let setup = build_s1ap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
        MME_UE_S1AP_ID(1u32),
        eNB_UE_S1AP_ID(enb_id),
        uEaggregateMaximumBitrate(rates),
        E_RABToBeSetupListCtxtSUReq(ERABToBeSetupListCtxtSUReq(bearers)),
        UESecurityCapabilities(ue_security_capabilities(0xe0, 0xe0)),
        SecurityKey(k_enb),
    )?;

    // The eNB checks the message against the specification, and takes the key
    let received = S1AP_PDU::decode(&setup.encode()?)?;
    let deviations = received.check()?;
    extract_s1ap_ies!(received, InitialContextSetupRequest,
        req key: SecurityKey = SecurityKey(key) => key,
    );
    println!("{received}: {} deviations, KeNB as sent: {}", deviations.len(), key.0.as_octets() == k_enb);
    Ok(())
}
```

```text
S1SetupRequest (procedure 17, initiating message): enb-1, 1 tracking area
S1SetupResponse (procedure 17, successful outcome)
InitialUEMessage (procedure 12, initiating message), eNB-UE-S1AP-ID 7: a NAS-PDU of 21 octets
DownlinkNASTransport (procedure 11, initiating message), MME-UE-S1AP-ID 1, eNB-UE-S1AP-ID 7
InitialContextSetupRequest (procedure 9, initiating message), MME-UE-S1AP-ID 1, eNB-UE-S1AP-ID 7: 0 deviations, KeNB as sent: true
```

## Decode a PDU and read its IEs

`decode` reads the envelope of a PDU: its direction, its procedure, its
criticality and the octets of its message. An IE is decoded when you read
it: an IE that does not decode fails only the read that asks for it.

```rust
use oxirush_s1ap::{S1apPduKind, extract_s1ap_ies, ie, s1ap::*};

fn receive(octets: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // `decode_borrowed` copies nothing: the PDU borrows from `octets`
    let pdu = S1AP_PDU::decode_borrowed(octets)?;
    match pdu.kind() {
        S1apPduKind::Initiating_UEContextReleaseRequest => {
            // Only the IEs that a field asks for are decoded
            extract_s1ap_ies!(pdu, UEContextReleaseRequest,
                req mme_id: u32 = MME_UE_S1AP_ID(id),
                req enb_id: u32 = eNB_UE_S1AP_ID(id),
            );
            // One IE, by its marker type
            let cause = pdu.ie::<ie::Cause>().transpose()?;
            println!("release of MME UE {mme_id}, eNB UE {enb_id}: {cause:?}");
        }
        S1apPduKind::Other { direction, procedure_code } => {
            println!("procedure {procedure_code} has no {direction}");
        }
        kind => println!("{kind:?} is not handled here"),
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A UE CONTEXT RELEASE REQUEST, then an S1 SETUP FAILURE
    receive(&hex::decode("0012401500000300000002002a000800020007000240020280")?)?;
    receive(&hex::decode("401100080000010002400145")?)?;
    Ok(())
}
```

```text
release of MME UE 42, eNB UE 7: Some(radioNetwork(user_inactivity))
Unsuccessful_S1Setup is not handled here
```

| Method | Does |
|--------|------|
| `decode`, `decode_borrowed` | Decode one PDU (`decode_borrowed` copies nothing) |
| `encode`, `encode_into` | Encode it |
| `procedure_code`, `procedure_name`, `direction`, `kind` | `18`, `"UEContextReleaseRequest"`, `"InitiatingMessage"`, and an `S1apPduKind` (`Other` for a procedure that the specification lacks in that direction) |
| `decode_value`, `message` | The message as its type |
| `protocol_ies`, `set_protocol_ies` | The IEs of the message, whatever it is |
| `ie::<T>`, `ies::<T>` | The first IE of a type, or every one in order. A message has them too |

A field of `extract_s1ap_ies!` is `req`, `opt` or `lax`, then its name, its
type and the IE it reads. What follows `=>` is what the field keeps of the
decoded value. An error goes to the enclosing function through `?`.

| Field | Is | When the IE is missing | When it does not decode |
|-------|----|------------------------|-------------------------|
| `req` | A `T` | An error of the kind `MissingIe` | An error of the kind `UndecodableIe` |
| `opt` | An `Option<T>` | `None` | The same error |
| `lax` | An `Option<T>` | `None` | `None` |

A `lax` field reads an IE the way TS 36.413 (10.3.4.2) has a receiver treat
an IE of criticality `ignore`: one that does not decode counts as absent.
When an IE is repeated, the macro and `ie` take the first, and `ies` gives
each. `with_s1ap_ie_mut!` changes one IE of a message in place, and
`build_s1ap_ie!` builds one: their documentation has an example of each.

## Build a PDU

| ASN.1 | Rust |
|---|---|
| `id-MME-UE-S1AP-ID` (an IE) | `MME_UE_S1AP_ID` |
| `Pre-emptionCapability`, `E-RAB-ID` (types) | `PreEmptionCapability`, `ERABID` |
| `pLMNidentity` (a field) | `plmn_identity` |

The two UE identifiers have types named after their IEs: `MME_UE_S1AP_ID`,
and `ENB_UE_S1AP_ID` for the IE `eNB_UE_S1AP_ID`. Leave a criticality out
and the macro takes the one that TS 36.413 assigns. A name that does not
exist, or a message that the procedure does not have in that direction, does
not compile.

```rust
use oxirush_s1ap::{Message, build_s1ap, s1ap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The criticalities of the procedure and of the message's object set
    let pdu = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
        eNB_UE_S1AP_ID(7u32),
        NAS_PDU(vec![0x07, 0x41, 0x71]),
    )?;
    println!("{:?} {}", pdu.criticality(), hex::encode(pdu.encode()?));

    // The PDU of a message you already have
    let message: InitialUEMessage = pdu.decode_value()?;
    let again = S1AP_PDU::of_message(&message, InitialUEMessage::CRITICALITY)?;
    println!("{}", again == pdu);

    // An IE that the message cannot have has no criticality to take
    let error = build_s1ap!(InitiatingMessage, initialUEMessage, InitialUEMessage,
        TimeToWait(TimeToWait::v10s),
    );
    if let Err(error) = error {
        println!("{error}");
    }
    Ok(())
}
```

```text
ignore 000c4011000002000800020007001a000403074171
true
TimeToWait is no IE of the object set of InitialUEMessage, which assigns it no criticality: write its criticality
```

## E-RAB lists and handover containers

An item of an E-RAB list is an IE alone in its container: `build_s1ap_ie!`
builds it without naming a message, and `items::<L>()` reads each item of a
list where it is in the PDU. A handover container carries another encoded
value (TS 36.413, 9.2.1.56): `decode_open_type` reads it as the type you
name, and `encode_open_type` writes it.

```rust
use oxirush_s1ap::{BitString, build_s1ap, build_s1ap_ie, ie, s1ap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let pdu = build_s1ap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
        MME_UE_S1AP_ID(1u32),
        eNB_UE_S1AP_ID(7u32),
        E_RABToBeSetupListCtxtSUReq(vec![item]),
    )?;

    // Each item of the list, read in the PDU without building the list
    for bearer in pdu.items::<ie::E_RABToBeSetupListCtxtSUReq>() {
        let bearer = bearer?;
        println!("E-RAB {} with QCI {}", bearer.e_rab_id.0, bearer.e_rab_level_qos_parameters.qci.0);
    }

    // A Source-ToTarget-TransparentContainer, as the type it carries
    let octets = hex::decode("000201020002f83900012340200100")?;
    let container: SourceeNBToTargeteNBTransparentContainer = decode_open_type(&octets)?;
    println!("{:?}", container.target_cell_id);
    assert_eq!(encode_open_type(&container)?.as_ref(), &octets[..]);
    Ok(())
}
```

```text
E-RAB 5 with QCI 9
EUTRANCGI { plmn_identity: PLMNidentity(208-93), cell_id: CellIdentity(0x1234/28), ie_extensions: None }
```

## Check a message, and answer a faulty one

`check()` compares a message with the object set that the ASN.1 gives it,
and lists every deviation. It refuses nothing, so a faulty message can still
be read and answered. Clause 10 of TS 36.413 says how: `cause()` gives the
protocol cause of a deviation, and `criticality_diagnostics_for_response()`
the Criticality Diagnostics of the response.

```rust
use oxirush_s1ap::{Deviation, build_s1ap, s1ap::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An S1 SETUP REQUEST with the wrong criticality, an IE it cannot have,
    // and none of its mandatory IEs but the paging DRX
    let request = build_s1ap!(InitiatingMessage, S1Setup, IGNORE, S1SetupRequest,
        IGNORE DefaultPagingDRX(PagingDRX::v128),
        REJECT TimeToWait(TimeToWait::v5s),
    )?;
    let received = S1AP_PDU::decode(&request.encode()?)?;
    let deviations = received.check()?;
    for deviation in &deviations {
        println!("{deviation}");
    }

    // The MME rejects the procedure, and reports the IEs at fault
    let cause = deviations.iter().find_map(Deviation::cause).ok_or("no cause")?;
    let failure = build_s1ap!(UnsuccessfulOutcome, S1Setup, S1SetupFailure,
        Cause(Cause::protocol(cause)),
        CriticalityDiagnostics(received.criticality_diagnostics_for_response(&deviations)),
    )?;
    println!("{failure}: {}", hex::encode(failure.encode()?));
    Ok(())
}
```

```text
the criticality of the PDU is ignore, and that of its procedure is reject
TimeToWait (IE 65), at position 1, is not an IE of the message
Global-ENB-ID (IE 59) is mandatory, and the message does not have it
SupportedTAs (IE 64) is mandatory, and the message does not have it
S1SetupFailure (procedure 17, unsuccessful outcome): 401100180000020002400131003a400c190200004100003b40004040
```

`check()` decodes every IE. `check_structure()` looks at the container
alone: which IEs are there, in which order, how often and with which
criticality. It costs 5 to 8 times less, and a receiver that then reads the
IEs it needs pays for those alone. Neither looks inside the octets that an IE
carries. `criticality_diagnostics()` gives the diagnostics of an ERROR
INDICATION, the answer where a procedure has no failure message.

## A PDU as JSON

With the `inspect` feature, a PDU has two JSON forms. The tree is the short
one, to print a message or to write one. The view keeps everything that was
received, and the [`inspect`] functions read and edit it by path, under
`/s1ap` and the name of the IE. Both forms, and serde's, carry keys such as
KeNB in hexadecimal: do not log them.

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU, tree};

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

    // Set a value, and pick another alternative of a CHOICE
    inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", tree!(9))?;
    inspect::set(&mut view, "/s1ap/Cause/nas", tree!("Normal Release"))?;
    println!("{}", pdu.with_view(&view)?.to_tree());

    // A misspelt name is refused, and the view stays as it was
    if let Err(error) = inspect::set(&mut view, "/s1ap/Cause/radioNetwrok", tree!("unspecified")) {
        println!("{error}");
    }
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

The [`inspect`] module has the rules of the paths and more examples: adding
and removing IEs, repeated IEs, list items, ENUMERATED names and handover
containers.

## Write a message from its tree

A tree names the message, in any case, and lists its IEs. The name gives the
direction, the procedure code and the criticalities. An IE can also be
written as its octets, whatever they are, to send a broken message on
purpose. With the `serde` feature, `Serialize` and `Deserialize` go through
the tree, in any format that serde has.

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU, tree};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = tree!({
        "message": "ue context release request",
        "ies": [
            {"MME-UE-S1AP-ID": 42},
            {"eNB-UE-S1AP-ID": 7},
            {"Cause": {"radioNetwork": "user-inactivity"}},
            // An IE that the message does not have says its criticality
            {"eNBname": "enb-1", "criticality": "ignore"},
            // An IE that no release has, as its octets
            {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
        ],
    });
    let pdu = S1AP_PDU::from_tree(&request)?;
    println!("{}", hex::encode(pdu.encode()?));

    let view = pdu.to_view()?;
    println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
    println!("{}", tree!(inspect::select(&view, "/s1ap/*/criticality")?));
    Ok(())
}
```

```text
0012402700000500000002002a000800020007000240020280003c40070200656e622d31fde80003c0ffee
"InitiatingMessage" 18 "ignore"
["reject","reject","ignore","ignore","reject"]
```

## Limits

- It is a codec: it runs no procedure, keeps no state and carries no
  message.
- `check()` does not judge conditional IEs: their conditions are prose.
- A value that you decode as its type and encode again loses what a later
  release added to it. A PDU keeps the octets of its IEs until you decode
  one, and so do trees and views.
- The tree is the crate's own JSON, with the names of the specification. It
  is not the JSON Encoding Rules (ITU-T X.697).
- RRC containers, and handover containers of other systems than E-UTRAN,
  stay octets.
- A PDU has no size limit, and time and memory grow with its size: a view
  takes up to 143 octets per octet ([`to_view`]). Bound what you read from
  SCTP.
- The bindings are those of TS 36.413 V19.2.0 (Release 19), in aligned PER
  (ITU-T X.691 (02/2021)).
- No PDU of a commercial RAN or core is among the test fixtures.

## Performance

On one core of an AMD Ryzen 7 5700G (Linux 7.0, Rust 1.98.1), at 774c1b5 on
2026-10-18, the macro reads the IEs that its fields ask for in 24 to 33 ns
without allocating, and `check_structure()` takes 48 to 156 ns.
[`benches/`](benches/README.md) has the figures by message.

### Against other implementations

Six common PDUs, decoded and encoded by this crate and four other S1AP
codecs, at the same commit and on the same machine. Decoding is the PDU and
each of its IEs as its type, as the others do; `S1AP_PDU::decode` alone
takes 8 ns. A ratio is the time of the other implementation over this
crate's:

| | Decoding | Encoding |
|---|---|---|
| this crate | 79 to 266 ns | 64 to 170 ns |
| rasn 0.29 | 6.3x to 9.6x | 12x to 14x |
| hampi 0.7.2 | 6.7x to 15x | 19x to 31x |
| asn1c (Open5GS 2.8.0) | 12x to 22x | 11x to 20x |
| pycrate 0.8.1 | 540x to 990x | 680x to 940x |

[`benches/`](benches/README.md#the-six-pdus) has the figures of each PDU and
the method. From the root of the repository, with oxirush-aper beside it:

```sh
python3 ../oxirush-aper/protocols/benches/compare/compare.py --core 3 --perf "sudo -n perf"
```

## Tests and fuzzing

The tests run 101 messages and 284 vectors that pycrate 0.8.1 encoded from
the ASN.1, and 81 PDUs that the MME of Open5GS 2.8.0 sent in lab runs,
through the codec, the views and the trees. `oracle/` compares the crate with
the codec of rasn 0.29 on each of them, also cut short, lengthened and with
bits flipped. `fuzz/` has three
[cargo-fuzz](https://crates.io/crates/cargo-fuzz) targets, `decode`, `view`
and `paths`, which CI runs for a minute each. `examples/` has four programs.

```sh
cargo test --all-features   # with the --config lines of Installation
cd oracle && cargo test     # against the codec of rasn
../oxirush-aper/protocols/fuzz/seed.sh && RUSTFLAGS="--cfg oxirush_s1ap_hooks" cargo +nightly fuzz run view -- -max_total_time=60
cargo run --example s1ap_build_pdu
```

Each Rust block of this page is checked with what it prints, by
`tests/readme.rs`, which `protocols/tests/readme.py` of oxirush-aper writes
from this page. [CONTRIBUTING.md](CONTRIBUTING.md) says how the bindings are
generated, how the vectors were made and the PDUs captured, and what a change
to them or to the pinned revisions takes.

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
