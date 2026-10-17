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

With the `inspect` feature, a PDU is also JSON, read, edited and written by
the names of the specification:

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

// The same message as a tree: its name and its IEs by name
let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();
println!("{}", pdu.to_tree());

// Its view is edited at a path, and is a PDU again
let mut view = pdu.to_view().unwrap();
let lost = tree!("radio-connection-with-ue-lost");
inspect::set(&mut view, "/s1ap/Cause/radioNetwork", lost).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree());

// Its octets are those that the typed example built
let octets = hex::encode(edited.encode().unwrap());
assert_eq!(octets, "0012401500000300000002002a0008000200070002400202a0");
```

```json
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":7},{"Cause":{"radioNetwork":"radio-connection-with-ue-lost"}}]}
```

This page describes 0.3.0, which is not on crates.io yet.

## Installation

```toml
[dependencies]
oxirush-s1ap = "0.3"
# With the JSON forms:
# oxirush-s1ap = { version = "0.3", features = ["inspect"] }
# What the examples of this page write octets in hexadecimal and JSON as text with:
# hex = "0.4"
# serde_json = "1"
```

| Feature | Adds |
|---------|------|
| `inspect` | A PDU as JSON, a `oxirush_s1ap::tree::Value`: its tree, its view and the paths of its values, with the members in the order of the specification |
| `serde` | `Serialize` and `Deserialize` for every type of the bindings, through its tree. It turns on `inspect` |

Until 0.3.0 is on crates.io, with `oxirush-aper` 0.1.0 and `oxirush-utils`
0.1.0 that it takes, a program takes the crate from its repository and the two
others at the revisions that it is tested with:

```toml
[dependencies]
oxirush-s1ap = { git = "https://github.com/linouxis9/oxirush-s1ap", features = ["inspect"] }

[patch.crates-io]
oxirush-aper = { git = "https://github.com/linouxis9/oxirush-aper", rev = "c39d2331388c3adf9bc2393169fc4c02094d5e98" }
oxirush-utils = { git = "https://github.com/linouxis9/oxirush-utils", rev = "eb1c453cd08dc3d0b712f53d4273595793cb9106" }
```

In a clone of the repository, cargo takes them with `--config`, as CI does:

```bash
cargo test --all-features \
  --config 'patch.crates-io.oxirush-aper.git="https://github.com/linouxis9/oxirush-aper"' \
  --config 'patch.crates-io.oxirush-aper.rev="c39d2331388c3adf9bc2393169fc4c02094d5e98"' \
  --config 'patch.crates-io.oxirush-utils.git="https://github.com/linouxis9/oxirush-utils"' \
  --config 'patch.crates-io.oxirush-utils.rev="eb1c453cd08dc3d0b712f53d4273595793cb9106"'
```

The minimum supported Rust version is 1.88. The crate depends on one version of
`oxirush-aper` exactly, which is released with it, and the values of the
bindings are its types, as `Octets`, `Bits` and `Error`: a program names them
through `oxirush_s1ap::aper`.

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

// A UE CONTEXT RELEASE REQUEST, then an S1 SETUP FAILURE
let request = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
println!("{}", receive(&request).unwrap());

let failure = hex::decode("401100080000010002400145").unwrap();
println!("{}", receive(&failure).unwrap());
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

## Keep a value after its PDU

A type that holds octets has a lifetime, as `NASPDU<'a>`: a value that is
decoded borrows them from the PDU. A type that holds none, as `Cause`, `TAI`,
`EUTRANCGI` or `GUMMEI`, has no lifetime, and the extension container of a value
owns its octets so that it gives it none. `into_owned()` gives the value with
the lifetime `'static`, which is the type to write in a struct.

```rust
use oxirush_s1ap::s1ap::{Cause, EUTRANCGI, GUMMEI, TAI};

// What holds no octets of its own is kept as it is decoded
struct Cell {
    tai: TAI,
    cgi: EUTRANCGI,
    gummei: GUMMEI,
    last_cause: Option<Cause>,
}
```

```rust
use oxirush_s1ap::{Error, build_s1ap, extract_s1ap_ies, s1ap::*};

// What a node keeps of a UE after its first message
struct Ue {
    enb_id: u32,
    nas: NASPDU<'static>,
}

fn admit(pdu: &S1AP_PDU) -> Result<Ue, Error> {
    // `message` borrows from `pdu`, and `nas` from `message`
    let message: InitialUEMessage = pdu.decode_value()?;
    extract_s1ap_ies!(message, InitialUEMessage,
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        req nas: NASPDU = NAS_PDU(nas) => nas,
    );
    // Kept beyond the PDU: the octets are copied here, once
    Ok(Ue { enb_id, nas: nas.into_owned() })
}

fn main() -> Result<(), Error> {
    let pdu = build_s1ap!(InitiatingMessage, initialUEMessage, IGNORE, InitialUEMessage,
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT NAS_PDU(vec![0x07, 0x41, 0x71]),
    )?;
    let ue = admit(&pdu)?;
    drop(pdu);
    println!("UE {}: a NAS-PDU of {} octets", ue.enb_id, ue.nas.0.len());
    Ok(())
}
```

```text
UE 7: a NAS-PDU of 3 octets
```

To return a message that borrows the caller's buffer, use
`decode_borrowed(octets)?.message()`. A message may be held across an
`.await`, and every value of the bindings is `Send`:

```rust
use oxirush_s1ap::{Error, build_s1ap, extract_s1ap_ies, s1ap::*};

async fn deliver(nas: &[u8]) {
    println!("a NAS-PDU of {} octets", nas.len());
}

async fn handle(received: Vec<u8>) -> Result<(), Error> {
    // Nothing of `received` is copied
    let pdu = S1AP_PDU::decode_borrowed(&received)?;
    let message: UplinkNASTransport = pdu.decode_value()?;
    extract_s1ap_ies!(message, UplinkNASTransport,
        req nas: NASPDU = NAS_PDU(nas) => nas,
    );
    deliver(&nas.0).await;
    Ok(())
}

fn main() -> Result<(), Error> {
    let transport = build_s1ap!(InitiatingMessage, uplinkNASTransport, IGNORE, UplinkNASTransport,
        REJECT MME_UE_S1AP_ID(1u32),
        REJECT eNB_UE_S1AP_ID(7u32),
        REJECT NAS_PDU(vec![0x07, 0x5e, 0x03]),
    )?;
    let handler = handle(transport.encode()?);

    // A runtime may move the handler to another thread
    fn is_send(_: &impl Send) {}
    is_send(&handler);

    // Polled here without a runtime: this handler never waits
    let mut handler = std::pin::pin!(handler);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    assert!(handler.as_mut().poll(&mut context).is_ready());
    Ok(())
}
```

```text
a NAS-PDU of 3 octets
```

## Build a PDU

An IE is named by its ASN.1 identifier without `id-` and with `_` for `-`
(`MME_UE_S1AP_ID`), a type by the name that ASN.1 gives it without its hyphens
(the IE `NAS_PDU` has the type `NASPDU`, `E_RAB_ID` `ERABID`), and a field by
the words of its member in lower case (`pLMNidentity` is `plmn_identity`). The
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
A character string is checked when it is decoded, so an eNB name with a
character that a PrintableString does not have, as `_`, is `None` for a `lax`
field and an error for an `opt` one.

```rust
use oxirush_s1ap::{extract_s1ap_ies, s1ap::*};

// The cause of a request, when it has one that is read
fn cause(request: &UEContextReleaseRequest) -> Option<Cause> {
    extract_s1ap_ies!(request, UEContextReleaseRequest,
        lax cause: Cause = Cause(value) => value,
    );
    cause
}

// A UE CONTEXT RELEASE REQUEST, then the same with an octet of its Cause spoiled
for octets in [
    "0012401500000300000002002a000800020007000240020280",
    "0012401500000300000002002a000800020007000240027280",
] {
    let pdu = S1AP_PDU::decode(&hex::decode(octets).unwrap()).unwrap();
    println!("{:?}", cause(&pdu.decode_value().unwrap()));
}
```

```text
Some(radioNetwork(user_inactivity))
None
```

## Change or build one IE

```rust
use oxirush_s1ap::{build_s1ap_ie, s1ap::*, with_s1ap_ie_mut};

// A message with one IE, which is built alone
let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
    build_s1ap_ie!(UplinkNASTransport, REJECT MME_UE_S1AP_ID(1u32)).unwrap(),
]));

// The IE is decoded, changed and encoded again in its place
let updated = with_s1ap_ie_mut!(message, UplinkNASTransport,
    MME_UE_S1AP_ID(id) = 42u32
);
assert_eq!(updated, Ok(true));
```

`with_s1ap_ie_mut!` gives `Ok(false)` for an IE that is not in the message;
`IE(value) => { … }` in place of `= value` changes the decoded value, mutable
and its own, in a block that says whether it did: its documentation has an
example. Where an IE is in a message twice, it and
`extract_s1ap_ies!` take the last one.

## The IEs of any message

```rust
use oxirush_s1ap::{build_s1ap_ie, s1ap::*};

fn main() -> Result<(), oxirush_s1ap::Error> {
    // An S1 SETUP FAILURE, as any PDU that was received
    let bytes = hex::decode("401100080000010002400145").unwrap();
    let mut pdu = S1AP_PDU::decode(&bytes)?;

    // Its IEs as they were received: identifier, criticality and octets. They
    // borrow from the PDU, and are owned here, as the PDU is changed below
    let mut ies = pdu.protocol_ies()?.into_owned();
    for ie in &ies.0 {
        let octets = hex::encode(&ie.value);
        println!("{} {:?} {octets}", ie.id.0, ie.criticality);
    }

    // One more IE, and the container is the message of the same PDU again
    let wait = build_s1ap_ie!(S1SetupFailure, IGNORE TimeToWait(TimeToWait::v5s))?;
    ies.0.push(wait);
    pdu.set_protocol_ies(ies)?;
    let octets = hex::encode(pdu.encode()?);
    println!("{} {octets}", pdu.procedure_name());
    Ok(())
}
```

```text
2 ignore 45
S1Setup 4011000d00000200024001450041400120
```

`protocol_ies()` gives the IEs of any message but the private one in their
order, including those that the message cannot have or has twice, and
`set_protocol_ies` writes any list without checking it.

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

## Check a message against its object set

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

`check()` decodes the value of each IE, not the IEs that a value holds as
octets, and does not judge a conditional IE: its condition is prose.

`check_structure()` compares the container alone with the object set: the
presence, the order, the repetition and the criticality of the IEs, which is
what TS 36.413 clause 10.3 has a receiver answer, and decodes no value. A
receiver that checks the structure of a message and reads the IEs that it
needs, with the macro or `ie`, pays for those IEs alone: decoding an IE checks
its value, and one that does not decode is an error of the kind
`UndecodableIe` there, which `Deviation::of` makes a deviation of. `check()`
is for a tool that looks at every value: it costs 4 to 7 times as much
([Performance](#performance)).

## Answer a faulty message

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

## Identities and capabilities

```rust
use oxirush_s1ap::helpers::*;

// A PLMN from its MCC and its MNC, and what is identified within it
let network = plmn("208", "93").unwrap();
let mme = gummei(network, 1, 1); // group, code
let tracking_area = tai(network, [0x00, 0x01]); // TAC
let cell = eutran_cgi(network, EnbKind::Macro, 0x12345, 7).unwrap(); // eNB, cell
let enb = global_enb_id(network, EnbKind::Macro, 0x12345).unwrap();
let home = global_enb_id(network, EnbKind::Home, 0x123_4567).unwrap();

// A bit string from octets, and the algorithms of a UE: the first octet
// of its encryption bitmap, then of its integrity bitmap
let algorithm_mask = oxirush_s1ap::BitString::from([0xe0]);
let capabilities = ue_security_capabilities(0xe0, 0xe0);

// What is no MCC, and a macro eNB ID that does not fit in its 20 bits
println!("{}", plmn("20", "93").unwrap_err());
println!("{}", global_enb_id(network, EnbKind::Macro, 0x100000).unwrap_err());
```

```text
"20" is not an MCC: it has three digits, 0 to 9
the macro eNB ID 1048576 does not fit in 20 bits
```

A PLMN identity has its digits in the order of TS 36.413 clause 9.2.3.8:
`plmn("310", "410")` is the octets `13 40 01`, where a NAS message has
`13 00 14`. `plmn_from` reads the MCC and the MNC of a PLMN identity.

## Errors

```rust
use oxirush_s1ap::{ErrorKind, s1ap::*};

// A UE CONTEXT RELEASE REQUEST is not decoded as an S1 SETUP REQUEST
let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();
let wrong = pdu.decode_value::<S1SetupRequest>().unwrap_err();
assert_eq!(wrong.kind(), ErrorKind::WrongMessage);
println!("{wrong}");

// Octets that end early
let short = S1AP_PDU::decode(&bytes[..10]).unwrap_err();
assert_eq!(short.kind(), ErrorKind::Decode);
println!("{short}");
```

```text
the message of this PDU is UEContextReleaseRequest, not S1SetupRequest: kind() says which message a PDU holds
these octets do not decode as S1AP-PDU: the octets end before the value does, in its field `value`
```

Every call that can fail returns an `oxirush_s1ap::Error`. Its `kind()` is
what a caller matches on, and the documentation of `ErrorKind` lists the
kinds; `error.ie()` names the IE of an error of the macros.

## The tree and the view of a PDU

With the `inspect` feature a PDU is JSON in two forms: a tree, in which a
message is shown and written, and a view, which keeps what was received and is
read and edited by paths with the functions of the [`inspect`] module. Every
PDU that decodes has a tree, which is the same PDU again.

```rust
use oxirush_s1ap::s1ap::S1AP_PDU;

// UE CONTEXT RELEASE REQUEST
let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();

// Its tree, then its third IE as the view has it
println!("{}", pdu.to_tree());
let view = pdu.to_view().unwrap();
println!("{}", view["message"]["protocolIEs"][2]);
```

```json
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}]}
{"id":2,"criticality":"ignore","value":{"radioNetwork":"user-inactivity"},"_raw_value":"0280","_original_id":2,"_ie_name":"Cause"}
```

## Read an IE

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();
let view = pdu.to_view().unwrap();

// `/s1ap` is the IEs of the message, and an IE goes by its name
let select = |path| inspect::select(&view, path).unwrap();
assert_eq!(select("/s1ap/eNB-UE-S1AP-ID/value"), [&tree!(7)]);
assert_eq!(select("/s1ap/Cause/radioNetwork"), [&tree!("user-inactivity")]);
assert_eq!(select("/s1ap/Cause/octets"), [&tree!("0280")]);

// By its position, by its name whatever its case and its separators, by its
// identifier, and each IE with `*`
assert_eq!(select("/s1ap/1/value"), select("/S1AP/enb_ue_s1ap_id/value"));
assert_eq!(select("/s1ap/@id=8/value"), [&tree!(7)]);
assert_eq!(select("/s1ap/*/criticality").len(), 3);

// Nothing where the message could have a value and has none
assert!(select("/s1ap/Cause/nas").is_empty());
assert!(select("/s1ap/GUMMEI-ID/mME-Code").is_empty());

// Each value with the path that selects it
for (path, value) in inspect::paths(&view) {
    println!("{path} = {value}");
}
```

```text
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
```

## Change a value

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();
let mut view = pdu.to_view().unwrap();

// A value, and another alternative of a CHOICE in place of the one that is there
inspect::set(&mut view, "/s1ap/eNB-UE-S1AP-ID/value", tree!(9)).unwrap();
inspect::set(&mut view, "/s1ap/Cause/nas", tree!("Normal Release")).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree());

// What was not edited is the octets that were received
let after = edited.to_view().unwrap();
let octets = inspect::select(&after, "/s1ap/MME-UE-S1AP-ID/octets").unwrap();
assert_eq!(octets, [&tree!("002a")]);

// A name that the type does not have is refused, and the view is as it was
let wrong = inspect::set(&mut view, "/s1ap/Cause/radioNetwrok", tree!("unspecified"));
println!("{}", wrong.unwrap_err());
```

```text
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":9},{"Cause":{"nas":"normal-release"}}]}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc; an IE itself has id, criticality, value, octets, at /s1ap/Cause/radioNetwrok
```

## Add or remove an IE

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();
let mut view = pdu.to_view().unwrap();

// An IE is added at the end, by its name and its typed value
let name = tree!({"id": "eNBname", "criticality": "ignore", "value": "enb-1"});
inspect::insert(&mut view, "/s1ap/-", name).unwrap();

// Before another IE, by its number and its octets
let unknown = tree!({"id": 60000, "criticality": "ignore", "octets": "c0ffee"});
inspect::insert(&mut view, "/s1ap/Cause", unknown).unwrap();

// An IE is taken out by its name
inspect::remove(&mut view, "/s1ap/MME-UE-S1AP-ID").unwrap();

let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree()["ies"]);
```

```json
[{"eNB-UE-S1AP-ID":7},{"60000":{"octets":"c0ffee"},"criticality":"ignore"},{"Cause":{"radioNetwork":"user-inactivity"}},{"eNBname":"enb-1","criticality":"ignore"}]
```

## Write a message from its name and its IEs

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

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

let pdu = S1AP_PDU::from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// The name gave the direction, the procedure code and the criticalities
let view = pdu.to_view().unwrap();
println!("{} {} {}", view["direction"], view["procedure_code"], view["criticality"]);
println!("{}", tree!(inspect::select(&view, "/s1ap/*/criticality").unwrap()));
```

```text
0012402000000400000002002a000800020007000240020280003c40070200656e622d31
"InitiatingMessage" 18 "ignore"
["reject","reject","ignore","ignore"]
```

## Send octets as they are

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

// `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
let request = tree!({"message": "UEContextReleaseRequest", "ies": [
    {"MME-UE-S1AP-ID": 42},
    {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
]});
let pdu = S1AP_PDU::from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// A view shows an IE that does not decode by its octets, and says why
let mut view = pdu.to_view().unwrap();
println!("{}", view["message"]["protocolIEs"][1]);

// In a view, the `octets` of an IE are set in place of its value
inspect::set(&mut view, "/s1ap/MME-UE-S1AP-ID/octets", tree!("00ff")).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree());
```

```text
0012401000000200000002002afde80003c0ffee
{"id":65000,"criticality":"reject","value":"c0ffee","_decode_error":"S1AP IE 65000 is unknown","_raw_value":"c0ffee","_original_id":65000}
{"message":"UEContextReleaseRequest","ies":[{"MME-UE-S1AP-ID":255},{"65000":{"octets":"c0ffee"},"criticality":"reject"}]}
```

## A repeated IE

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

// A message that has one IE twice
let twice = tree!({"message": "UEContextReleaseRequest", "ies": [
    {"MME-UE-S1AP-ID": 1},
    {"eNB-UE-S1AP-ID": 7},
    {"MME-UE-S1AP-ID": 2},
]});
let pdu = S1AP_PDU::from_tree(&twice).unwrap();
let mut view = pdu.to_view().unwrap();

// The name selects both, in the order of the message
let both = inspect::select(&view, "/s1ap/MME-UE-S1AP-ID/value").unwrap();
assert_eq!(both, [&tree!(1), &tree!(2)]);

// Each has its position, which is how `paths` names it, and its name may follow
inspect::set(&mut view, "/s1ap/2/MME-UE-S1AP-ID/value", tree!(3)).unwrap();

// After a position, the name of another IE is an error that says which IE is there
println!("{}", inspect::select(&view, "/s1ap/2/eNB-UE-S1AP-ID/value").unwrap_err());

// The values, each with its path
for (path, value) in inspect::paths(&view) {
    if path.ends_with("/value") {
        println!("{path} = {value}");
    }
}
```

```text
this IE is MME-UE-S1AP-ID, not eNB-UE-S1AP-ID: "eNB-UE-S1AP-ID" is not a member of this value, which has none, at /s1ap/2/eNB-UE-S1AP-ID/value
/s1ap/0/value = 1
/s1ap/eNB-UE-S1AP-ID/value = 7
/s1ap/2/value = 3
```

## An item of a list

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::{tree, tree::Value};

// A message whose one IE is a list of two items
let command = tree!({"message": "E-RABReleaseCommand", "ies": [
    {"E-RABToBeReleasedList": [
        {"E-RABItem": {"e-RAB-ID": 5, "cause": {"nas": "normal-release"}}},
        {"E-RABItem": {"e-RAB-ID": 6, "cause": {"nas": "normal-release"}}},
    ]},
]});
let pdu = S1AP_PDU::from_tree(&command).unwrap();
let mut view = pdu.to_view().unwrap();

// An item by its position or by its name, which selects each item of that name
let select = |view, path| -> Vec<Value> {
    let selected = inspect::select(view, path).unwrap();
    selected.into_iter().cloned().collect()
};
assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/1/e-RAB-ID"), [tree!(6)]);
assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/E-RABItem/e-RAB-ID"), [tree!(5), tree!(6)]);

// The same value by the path that says each `value`
assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/value/1/value/e-RAB-ID"), [tree!(6)]);

// And by the names that the tree shows, joined: the item by its position, then its name
assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/1/E-RABItem/e-RAB-ID"), [tree!(6)]);

// The criticality of an item is the one of its object set
assert_eq!(select(&view, "/s1ap/E-RABToBeReleasedList/0/criticality"), [tree!("ignore")]);

// An item is added at the end, as the IE that it is, and one is taken out
let item = tree!({"e-RAB-ID": 7, "cause": {"radioNetwork": "unspecified"}});
let item = tree!({"id": "E-RABItem", "criticality": "ignore", "value": item});
inspect::insert(&mut view, "/s1ap/E-RABToBeReleasedList/-", item).unwrap();
inspect::remove(&mut view, "/s1ap/E-RABToBeReleasedList/0").unwrap();

let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree());
```

```json
{"message":"E-RABReleaseCommand","ies":[{"E-RABToBeReleasedList":[{"E-RABItem":{"e-RAB-ID":6,"cause":{"nas":"normal-release"}}},{"E-RABItem":{"e-RAB-ID":7,"cause":{"radioNetwork":"unspecified"}}}]}]}
```

## The names of an ENUMERATED

```rust
use oxirush_s1ap::inspect;

// The names that the value at a path can have, as the specification spells them
let names = inspect::enumerated_at("UEContextReleaseRequest", "/s1ap/Cause/nas").unwrap();
println!("{:?}", names.unwrap());

// A value of another type has none
let choice = inspect::enumerated_at("UEContextReleaseRequest", "/s1ap/Cause/value");
assert_eq!(choice, Ok(None));
```

```text
["normal-release", "authentication-failure", "detach", "unspecified", "csg-subscription-expiry", "uE-not-in-PLMN-serving-area", "iab-not-authorized"]
```

A name is read in any case and with any separators, as `"Normal Release"`
above, and shown as the specification spells it.

## Check a path before use

```rust
use oxirush_s1ap::inspect;

// Whether a path can select anything in a message of a name, without a view
let check = |path| inspect::check_path("UEContextReleaseRequest", path);
assert_eq!(check("/s1ap/Cause/nas"), Ok(()));
assert_eq!(check("/s1ap/*/radioNetwork"), Ok(()));

// A name that is written wrong, and an IE that the message does not have
for wrong in ["/s1ap/Cause/radioNetwrok", "/s1ap/eNB-UE-S1AP-ID/valeu", "/s1ap/GUMMEI-ID"] {
    println!("{}", check(wrong).unwrap_err());
}
```

```text
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc; an IE itself has id, criticality, value, octets, at /s1ap/Cause/radioNetwrok
"valeu" is not a member of this value, which has none; an IE itself has id, criticality, value, octets, at /s1ap/eNB-UE-S1AP-ID/valeu
"GUMMEI-ID" is not an IE of UEContextReleaseRequest, which has MME-UE-S1AP-ID, eNB-UE-S1AP-ID, Cause, GWContextReleaseIndication, SecondaryRATDataUsageReportList, at /s1ap/GUMMEI-ID
```

The functions `message_ies`, `message_ie_criticalities`, `item_criticality`
and the others of [`inspect`] give what the ASN.1 says of each message, which
`check_path` reads.

## The container of a handover

Two IEs carry octets that TS 36.413 gives by reference: between eNBs, a type
of the specification, as `Source-ToTarget-TransparentContainer` carries a
`SourceeNB-ToTargeteNB-TransparentContainer` (clause 9.2.1.56). Nothing in a message says which, so a tree and a view show the
octets, and the one that writes or reads them names the type.

```rust
use oxirush_s1ap::{ErrorKind, inspect, s1ap::S1AP_PDU};
use oxirush_s1ap::tree;

// Written: the name of the type with its value, in place of the octets
let name = "SourceeNB-ToTargeteNB-TransparentContainer";
let required = tree!({"message": "HandoverRequired", "ies": [
    {"MME-UE-S1AP-ID": 42},
    {"eNB-UE-S1AP-ID": 7},
    {"Source-ToTarget-TransparentContainer": {name: {
        "rRC-Container": "0102",
        "targetCell-ID": {"pLMNidentity": "208-93", "cell-ID": 4660},
        "uE-HistoryInformation": [{"uTRAN-Cell": "00"}],
    }}},
]});

// A tree shows the octets that the container was encoded as
let pdu = S1AP_PDU::from_tree(&required).unwrap();
println!("{}", pdu.to_tree()["ies"][2]);

// Read: a path goes through the name of the type
let mut view = pdu.to_view().unwrap();
let container = format!("/s1ap/Source-ToTarget-TransparentContainer/{name}");
let cell = format!("{container}/targetCell-ID/cell-ID");
assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));

// The path is an error until `open` has decoded the octets, and selects the value after that
assert_eq!(inspect::select(&view, &cell).unwrap_err().kind(), ErrorKind::NotOpened);
inspect::open(&mut view, &cell).unwrap();
assert_eq!(inspect::select(&view, &cell).unwrap(), [&tree!(4660)]);

// What the octets carry, decoded
let decoded = format!("{container}/decoded");
println!("{}", inspect::select(&view, &decoded).unwrap()[0]);

// An edit goes into it, and what is not edited is sent as it was received
inspect::set(&mut view, &cell, tree!(7)).unwrap();
let edited = pdu.with_view(&view).unwrap();
println!("{}", edited.to_tree()["ies"][2]);
```

```text
{"Source-ToTarget-TransparentContainer":"000201020002f83900012340200100"}
{"rRC-Container":"0102","targetCell-ID":{"pLMNidentity":"208-93","cell-ID":4660},"uE-HistoryInformation":[{"uTRAN-Cell":"00"}]}
{"Source-ToTarget-TransparentContainer":"000201020002f83900000070200100"}
```

## Any format of serde

With the `serde` feature, a PDU is serialized as its tree and deserialized
from one, in any format that serde has, and each other type of the bindings
as the tree of a PDU shows it.

```rust
use oxirush_s1ap::helpers::{plmn, tai};
use oxirush_s1ap::s1ap::S1AP_PDU;

let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();
let text = serde_json::to_string(&pdu).unwrap();
let read: S1AP_PDU = serde_json::from_str(&text).unwrap();
assert_eq!(read.encode().unwrap(), bytes);

// A value alone has the form that a tree shows it in: the MCC and the MNC, and a number
let area = tai(plmn("208", "93").unwrap(), [0x00, 0x01]);
println!("{}", serde_json::to_string(&area).unwrap());
```

```text
{"pLMNidentity":"208-93","tAC":1}
```

## Write values as they are usually written

A tree, a view and a value alone show the values of some types as they are
usually written, and take them so or in the form of their ASN.1 type. A string
of decimal digits is never read as octets: octets whose hexadecimal has no
letter are written after `0x`.

| Type | Notation |
|------|----------|
| `PLMNidentity` | `"208-93"`, the MCC and the MNC, or its octets: `"02f839"`, `"0x134001"`. `"310410"` is refused, as it is read as 310-410 and as the octets 0x310410 |
| `TransportLayerAddress` | `"10.0.0.1"`, `"2001:db8::1"`, or the two with a comma |
| `IMSI` | its digits |
| `TAC`, `FiveGSTAC`, `LAC`, `RAC`, `CI`, `GTP-TEID`, `M-TMSI`, `MME-Group-ID`, `MME-Code`, `Port-Number`, `CellIdentity`, `NRCellIdentity`, `UL-NAS-Count` | a number, also read in hexadecimal as `"0x…"`. A string of digits, as `"1234"`, is refused: it is the number 1234 and the octets 0x1234 |
| `ENUMERATED` | the name of the value, read in any case |
| `INTEGER` | a number, which 64 bits hold, as the bindings hold it |
| a bit string whose size varies, as the identifier of a gNB | `{"value": "000004", "length": 22}`: its octets in hexadecimal, from its first bit, and its length in bits |
| a bit string of another size than the one that its type gives beside an extension marker | `{"value": "c00000", "length": 24}`, where a mask of the 16 bits of its type is `"c000"` |
| the other strings: keys, the NAS-PDU and the other containers, and the identifiers of an eNB, whose length says which kind they are | hexadecimal, written in lower case and read in either |

The documentation of the [`inspect`] module has the rules of the paths, of
the edits and of the values, and the kinds of the errors of the JSON forms.

## Limits

- Procedures, state and transport: it is a codec.
- Judge a conditional IE: `check()` reports what the object sets say, and a
  condition is prose.
- Keep what a later release added to a value that is decoded as its type and
  encoded again: a PDU keeps the octets of its IEs until one is decoded, and
  a tree and a view keep them.
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

The crate against four other implementations of S1AP, on the six PDUs of
`benches/compare/pdus.tsv`, which each decodes and encodes. The envelope alone
is `S1AP_PDU::decode`, which leaves the value of each IE in its octets; "this
crate" decodes the PDU and then every IE of its message as its type, which is
what the others do; and encoding writes the PDU from the typed values of its
IEs, as `build_s1ap!` does.

Decoding, ns per call, and the ratio to this crate:

| PDU (octets) | this crate, the envelope alone | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| DownlinkNASTransport (73) | 9 | 87 | 779 (9.0x) | 913 (10.5x) | 1,816 (20.9x) | 73,347 (843.1x) |
| InitialContextSetupRequest (197) | 9 | 249 | 2,291 (9.2x) | 3,486 (14.0x) | 5,136 (20.6x) | 231,762 (930.4x) |
| S1SetupResponse (45) | 9 | 222 | 1,150 (5.2x) | 1,373 (6.2x) | 2,183 (9.8x) | 101,654 (456.9x) |
| HandoverRequest (146) | 10 | 287 | 2,770 (9.7x) | 3,657 (12.7x) | 5,545 (19.3x) | 269,845 (940.2x) |
| UEContextReleaseCommand (21) | 9 | 104 | 623 (6.0x) | 678 (6.5x) | 1,388 (13.3x) | 70,827 (677.8x) |
| InitialUEMessage (48) | 9 | 143 | 1,375 (9.6x) | 1,427 (10.0x) | 2,578 (18.0x) | 125,781 (879.6x) |

Encoding, ns per call, and the ratio to this crate:

| PDU (octets) | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|
| DownlinkNASTransport (73) | 66 | 925 (14.0x) | 1,670 (25.2x) | 1,177 (17.7x) | 46,668 (703.9x) |
| InitialContextSetupRequest (197) | 153 | 1,844 (12.0x) | 4,838 (31.5x) | 3,043 (19.8x) | 144,309 (940.7x) |
| S1SetupResponse (45) | 88 | 1,086 (12.4x) | 1,907 (21.7x) | 1,036 (11.8x) | 63,240 (721.1x) |
| HandoverRequest (146) | 180 | 2,291 (12.7x) | 5,145 (28.6x) | 3,170 (17.6x) | 165,840 (922.9x) |
| UEContextReleaseCommand (21) | 68 | 809 (11.9x) | 1,302 (19.1x) | 751 (11.0x) | 46,124 (677.3x) |
| InitialUEMessage (48) | 105 | 1,236 (11.8x) | 2,503 (23.8x) | 1,391 (13.2x) | 77,308 (735.6x) |

Every figure for InitialContextSetupRequest, per call:

|  | decode ns | cycles | instructions | allocations | octets allocated | encode ns | cycles | instructions | allocations | octets allocated | most memory held, KiB |
|---|---|---|---|---|---|---|---|---|---|---|---|
| this crate | 249 | 1,105 | 3,536 | 2 | 256 | 153 | 681 | 2,247 | 2 | 454 | 3,600 |
| rasn 0.29 | 2,291 (9.2x) | 9,990 (9.0x) | 22,941 (6.5x) | 20 | 1,074 | 1,844 (12.0x) | 8,256 (12.1x) | 26,486 (11.8x) | 33 | 1,698 | 4,420 |
| hampi 0.7.2 | 3,486 (14.0x) | 14,956 (13.5x) | 47,307 (13.4x) | 37 | 3,569 | 4,838 (31.5x) | 21,094 (31.0x) | 63,805 (28.4x) | 17 | 1,797 | 3,780 |
| asn1c (Open5GS 2.8.0) | 5,136 (20.6x) | 22,805 (20.6x) | 81,660 (23.1x) | 79 | 10,046 | 3,043 (19.8x) | 13,301 (19.5x) | 50,473 (22.5x) | 25 | 3,728 | 4,552 |
| pycrate 0.8.1 | 231,762 (930.4x) | 1,002,960 (907.7x) | 2,109,419 (596.5x) |  |  | 144,309 (940.7x) | 624,416 (917.0x) | 1,290,051 (574.0x) |  |  | 28,144 |

The time is the fastest of three runs of a call repeated for about 0.2 s, and
the cycles and the instructions those that `perf stat` counts for the calls
alone (the difference between a run of twice the calls and one of the calls).
The allocations and the octets that they ask for are counted by an allocator
that counts in Rust, by `malloc`, `calloc` and `realloc` for asn1c; pycrate's
are not counted. The most memory held is the high-water mark of the resident
memory of the program (`VmHWM`), its runtime included. The figures were taken on
an AMD Ryzen 7 5700G, each program on one core, while other work kept the load
average of the machine between 4.2 and 8.2 (of 16 threads, the other thread of
that core among them). The script fetches each implementation at a pinned
version: hampi 0.7.2 and rasn 0.29.0 from crates.io, Open5GS v2.8.0 from GitHub,
pycrate 0.8.1 from PyPI, and the ASN.1 from the archive of 3GPP. From the root
of the repository, with oxirush-aper beside it, whose `protocols/benches/compare/`
has the script, and with what the script says it needs:

```sh
python3 ../oxirush-aper/protocols/benches/compare/compare.py --core 3 --perf "sudo -n perf"
```

Reading the IEs of a message and checking a PDU, per call, on the four
messages that `benches/` writes: `decode_value()` and then the macro on the
message, which decodes the container of the message first; the macro on the
PDU (`extract_s1ap_ies!(pdu, ...)`), which walks the octets of its message once and
decodes the IEs that its fields ask for; and `ie::<T>()` of the PDU, which
decodes one IE. In `benches/`, `cargo run --release --bin readers --features
readers` measures them:

| Message | the first IE: `decode_value()`, then the macro | the macro on the PDU | `ie::<T>()` | every IE: `decode_value()`, then the macro | the macro on the PDU |
|---|---|---|---|---|---|
| An INITIAL CONTEXT SETUP REQUEST of 8 E-RABs | 176 ns, 1 allocation | 69 ns, no allocation | 51 ns, no allocation | 591 ns, 2 allocations | 469 ns, 1 allocation |
| An E-RAB SETUP REQUEST of 8 E-RABs | 113 ns, 1 allocation | 51 ns, no allocation | 50 ns, no allocation | 314 ns, 2 allocations | 244 ns, 1 allocation |
| A HANDOVER REQUIRED of a container of 4,000 octets | 159 ns, 1 allocation | 59 ns, no allocation | 50 ns, no allocation | 355 ns, 1 allocation | 246 ns, no allocation |
| A PAGING of 16 tracking areas | 136 ns, 1 allocation | 52 ns, no allocation | 46 ns, no allocation | 411 ns, 2 allocations | 315 ns, 1 allocation |

`items::<L>()` walks the eight E-RABs of the E-RAB SETUP REQUEST in the octets
of the PDU in 1,328 ns without an allocation, where `decode_value()`, the
macro and then the decoding of each item take 1,195 ns and 2 allocations:
the time is that of decoding the items.

`check()`, which decodes the value of each IE, and `check_structure()`, which
compares the container alone with the object set of its message, on the PDUs
of the comparison (`cargo run --release --bin checks` in `benches/`):

| PDU (octets) | `check()` | `check_structure()` | `check()` / `check_structure()` |
|---|---|---|---|
| DownlinkNASTransport (73) | 640 ns, 7 allocations | 173 ns, 2 allocations | 3.7x |
| InitialContextSetupRequest (197) | 1,672 ns, 13 allocations | 295 ns, 2 allocations | 5.7x |
| S1SetupResponse (45) | 814 ns, 12 allocations | 132 ns, 2 allocations | 6.2x |
| HandoverRequest (146) | 2,113 ns, 17 allocations | 318 ns, 2 allocations | 6.6x |
| UEContextReleaseCommand (21) | 457 ns, 5 allocations | 102 ns, 2 allocations | 4.5x |
| InitialUEMessage (48) | 917 ns, 8 allocations | 211 ns, 2 allocations | 4.3x |

These were taken on the same machine, each program on one core, at a load
average of 10 to 12.

A clean `cargo build -j 5` of the crate and its dependencies on Rust 1.88, on
five cores of the same machine while it ran other work (a load average of 9 to
12), with the debug information of the default profile and without it:

| Build | Time | Peak memory | Peak memory, `CARGO_PROFILE_DEV_DEBUG=0` |
|-------|------|-------------|------------------------------------------|
| No feature | 13 s | 0.75 GiB | 0.66 GiB |
| `inspect` | 21 s | 1.12 GiB | 0.88 GiB |
| `serde` | 24 s | 1.10 GiB | 0.90 GiB |
| `inspect`, `--release` | 28 s | 0.92 GiB | the same |

On a message of 65,535 IEs of one name, 393 KB, `decode` takes 0.01 ms, `check`
5.76 ms, `to_view` 71 ms and 37 MB, `with_view` 132 ms and 48 MB,
`inspect::paths` 40 ms, and an edit at a path 0.01 ms, on one core of the same
machine at a load average of 7 to 9. In `benches/`, `cargo bench --bench scale`
measures them, and `cargo bench --bench codec` and `--bench allocations` the
codec on the PDUs of the tests; from the root of the repository,
`python3 ../oxirush-aper/protocols/benches/build_cost.py +1.88.0 --features inspect`
measures a clean build.

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

```bash
cargo test --all-features   # with the --config lines of Installation
cd oracle && cargo test     # against the codec of rasn
../oxirush-aper/protocols/fuzz/seed.sh && RUSTFLAGS="--cfg oxirush_s1ap_hooks" cargo +nightly fuzz run view -- -max_total_time=60
cargo run --example s1ap_build_pdu
```

The bindings are generated: [CONTRIBUTING.md](CONTRIBUTING.md) says how, how
the vectors were made and the PDUs captured, and what a change to them or to
the pinned revisions takes. A pull request comes with a test for what it
changes; the Rust blocks of this page are checked with what they print
(`tests/readme.rs`, which `protocols/tests/readme.py` of oxirush-aper writes
from this page). Sign off your commits
(`git commit -s`): by contributing you agree to the
[Developer Certificate of Origin](https://developercertificate.org/).

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

[`inspect`]: https://docs.rs/oxirush-s1ap/0.3.0/oxirush_s1ap/inspect/index.html
[`from_tree()`]: https://docs.rs/oxirush-s1ap/0.3.0/oxirush_s1ap/s1ap/s1_ap_pdu_descriptions/enum.S1APPDU.html#method.from_tree
[`to_view`]: https://docs.rs/oxirush-s1ap/0.3.0/oxirush_s1ap/s1ap/s1_ap_pdu_descriptions/enum.S1APPDU.html#method.to_view
