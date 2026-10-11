# oxirush-s1ap

[![Crates.io](https://img.shields.io/crates/v/oxirush-s1ap.svg)](https://crates.io/crates/oxirush-s1ap)
[![Documentation](https://docs.rs/oxirush-s1ap/badge.svg)](https://docs.rs/oxirush-s1ap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A Rust codec for S1AP, the S1 Application Protocol between an eNB and an MME,
per 3GPP TS 36.413. Its types are generated from the ASN.1 of the
specification and encoded in aligned PER with
[`rasn`](https://crates.io/crates/rasn), and macros build a message and read
its IEs by their names.

```rust
use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, s1ap::*};

// UE CONTEXT RELEASE REQUEST: decode it, and read its IEs
let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();
assert_eq!((pdu.procedure_code(), pdu.is_initiating()), (18, true));
println!("{}", pdu.procedure_name());
let request: UEContextReleaseRequest = pdu.decode_value().unwrap();
extract_s1ap_ies!(request, UEContextReleaseRequest,
    opt mme_id: u32 = MME_UE_S1AP_ID(id),
    opt enb_id: u32 = eNB_UE_S1AP_ID(id),
    opt cause: Cause = Cause(cause) => cause,
);
println!("MME={mme_id:?} eNB={enb_id:?} cause={cause:?}");

// Build it with another cause, and encode it
let rebuilt = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
    IGNORE, UEContextReleaseRequest,
    REJECT MME_UE_S1AP_ID(42u32),
    REJECT eNB_UE_S1AP_ID(7u32),
    IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::radio_connection_with_ue_lost)),
);
let encoded = hex::encode(rebuilt.encode().unwrap());
assert_eq!(encoded, "0012401500000300000002002a0008000200070002400202a0");
```

```text
UEContextReleaseRequest
MME=Some(42) eNB=Some(7) cause=Some(radioNetwork(user_inactivity))
```

With the optional `inspect` feature, a PDU is also JSON, which is read, edited
and written by the names of the specification:

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use serde_json::json;

let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();
println!("{}", inspect::message_tree(&pdu).unwrap());

let mut tree = inspect::inspect_pdu(&pdu).unwrap();
let lost = json!("radio-connection-with-ue-lost");
inspect::set(&mut tree, "/s1ap/Cause/radioNetwork", lost).unwrap();
let edited = inspect::encode_pdu(&tree).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap());
assert_eq!(hex::encode(edited.encode().unwrap()), "0012401500000300000002002a0008000200070002400202a0");
```

```json
{"ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}],"message":"UEContextReleaseRequest"}
{"ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":7},{"Cause":{"radioNetwork":"radio-connection-with-ue-lost"}}],"message":"UEContextReleaseRequest"}
```

[`oxirush-ngap`](https://crates.io/crates/oxirush-ngap) is the same crate for NGAP, TS 38.413,
and its README has the sections of this one.

[Installation](#installation) ·
[Covered](#what-the-crate-covers) ·
[Not covered](#what-the-crate-does-not-cover) ·
[Typed API](#typed-api) ·
[JSON cookbook](#json-trees-and-paths) ·
[Rules](#rules) ·
[Examples](#examples) ·
[Specifications](#specifications) ·
[Tests](#tests) ·
[Generated code](#generated-code) ·
[API reference](https://docs.rs/oxirush-s1ap)

## Installation

```toml
[dependencies]
oxirush-s1ap = "0.3"
# With the optional feature:
# oxirush-s1ap = { version = "0.3", features = ["inspect"] }
```

| Feature | Adds | Through |
|---------|------|---------|
| `inspect` | A PDU as a `serde_json::Value`: its tree, the paths of its values, and a message written from its name and its IEs | `serde_json` |

No feature is on by default. The minimum supported Rust version is 1.88. The
examples of this page also use the `hex` crate, and `serde_json` for JSON.

The bindings are checked in and large, and with `inspect` the compiler also
builds JER and APER code for every type of the protocol. A clean build of the
crate and its dependencies, in the `dev` profile with `-j 8`:

| Build | Time | Peak memory |
|-------|------|-------------|
| Rust 1.88, no feature | 17 s | 1.2 GiB |
| Rust 1.88, `inspect` | 36 s | 2.3 GiB |
| Rust 1.88, `inspect`, `CARGO_PROFILE_DEV_DEBUG=0` | 31 s | 2.0 GiB |
| Rust 1.88, `inspect`, `--release` | 52 s | 2.1 GiB |
| Rust 1.98, `inspect` | 34 s | 2.4 GiB |

The peak is the summed resident memory of cargo and its compilers, on one
x86-64 Linux machine. A build of the tests with every feature and `-j 4`
peaked at 2.4 GiB, and at 1.8 GiB without debug information.

## What the crate covers

| Area | Contents |
|------|----------|
| Bindings | Rust types for the six ASN.1 modules of TS 36.413 V19.2.0, generated with [`rasn-compiler`](https://crates.io/crates/rasn-compiler) and checked in |
| Codec | Aligned PER encoding and decoding through `rasn`, with the corrections of [rasn integration](#rasn-integration) |
| Names | Flat re-exports of the generated types, and aliases such as `S1AP_PDU`, `MME_UE_S1AP_ID` and `ENB_UE_S1AP_ID`. IE identifiers, procedure codes, procedure names, directions and the kinds of a PDU (`S1apPduKind`) come from the ASN.1 |
| Macros | `build_s1ap!`, `build_s1ap_ie!`, `extract_s1ap_ies!` and `with_s1ap_ie_mut!`: typed values in and out of the open types |
| Helpers | PLMN, core network, tracking area, cell and radio node identities, strings of bits and UE security capabilities |
| `inspect` | A decoded PDU as a JSON tree that is read and edited by paths and encoded again, and a message written from its name and its IEs |

## What the crate does not cover

| Subject | Limit |
|---------|-------|
| Procedures | It is a codec: no procedure, state or transport, and no check that the mandatory and conditional IEs of a message are there: the caller checks them. |
| Unknown additions | A PDU keeps the octets of its open types. A typed value that is decoded and encoded again loses the additions of a later release. |
| RRC, other systems | An RRC container stays octets. The container of a handover is read as the eNB type of TS 36.413 alone ([below](#the-container-of-a-handover)). |
| `inspect`: two values | An IE with an integer of 2^63 or more stays octets, and one with an extensible fixed-size `BIT STRING` of another size is not edited as a typed value: rasn's JER does not represent them. |
| `inspect`: criticalities | The tables have those of the object set of each message of a procedure, and of each single container. An item that two sets give two criticalities, and an IE of an extension container, says its own. |
| `inspect`: `check_path` | What the types do not say is not refused: the value of an IE selected by `@id=N` whose identifier is unknown or has several types, the entries of a container of a value when they are selected by position, and a segment under such an IE. |

## Typed API

### Decode and build a PDU

The example at the top of this page does both. A decoded `S1AP_PDU` has its
`procedure_code()`, its `procedure_name()`, its `direction()` and its `kind()`,
a `S1apPduKind`, and `decode_value()` gives its message. `build_s1ap!` takes:

| Argument | Is |
|----------|----|
| The direction | `InitiatingMessage`, `SuccessfulOutcome` or `UnsuccessfulOutcome` |
| The procedure | Its name, which gives the procedure code |
| The criticality of the PDU | `REJECT`, `IGNORE` or `NOTIFY` |
| The message | The name of its type, such as `UEContextReleaseRequest` |
| Each IE | `CRITICALITY IeName(value)`: the name gives the identifier, and the value is converted with `.into()` |

The identifiers of the IEs and the procedure codes are those of the ASN.1
object sets.

### Read the IEs of a message

```rust
use oxirush_s1ap::{extract_s1ap_ies, macros::MissingIeError, s1ap::*};

fn handle(request: &UEContextReleaseRequest) -> Result<(), MissingIeError> {
    extract_s1ap_ies!(request, UEContextReleaseRequest,
        req mme_id: u32 = MME_UE_S1AP_ID(id),
        req enb_id: u32 = eNB_UE_S1AP_ID(id),
        req cause: Cause = Cause(value) => value,
    );

    println!("MME={mme_id} eNB={enb_id} cause={cause:?}");
    Ok(())
}
```

| Field | When the IE is absent or does not decode |
|-------|------------------------------------------|
| `req` | The enclosing function returns `MissingIeError` |
| `opt` | The field is `None` |

Without `=> expression`, a field is the `.0` of the generated newtype.

### Change or build one IE

```rust
use oxirush_s1ap::{build_s1ap_ie, s1ap::*, with_s1ap_ie_mut};

let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
    build_s1ap_ie!(UplinkNASTransport, REJECT MME_UE_S1AP_ID(1u32)),
]));
let updated = with_s1ap_ie_mut!(message, UplinkNASTransport,
    MME_UE_S1AP_ID(id) = 42u32
);
assert!(updated);
```

`with_s1ap_ie_mut!` decodes one IE, changes it and encodes it again, and says
whether it did. Of an IE that a message has twice, it and `extract_s1ap_ies!`
take the same one: the last that decodes. The name of an IE is its ASN.1
identifier without `id-`, with `_` for `-`; the name of a type is an alias
for the one IE of that type only.

### Identities and capabilities

```rust
use oxirush_s1ap::helpers::*;

let network = plmn("208", "93");
let mme = gummei(network.clone(), 1, 1);
let tracking_area = tai(network.clone(), &[0x00, 0x01]);
let cell = eutran_cgi(network.clone(), 0x12345, 7);
let enb = global_enb_id(network, 0x12345);
let algorithm_mask = bytes_to_bitvec(&[0xe0]);
let capabilities = ue_security_capabilities(&[0xe0, 0xe0]);
```

### Key types

| Type | Description |
|------|-------------|
| `S1AP_PDU` / `S1APPDU` | The PDU: an initiating message, a successful or an unsuccessful outcome |
| `InitiatingMessage` | Procedure code, criticality, and the message as an open type |
| `ProtocolIEContainer` / `ProtocolIEField` | The IEs of a message: identifier, criticality, and value as an open type |
| `MMEUES1APID` | 32-bit MME UE S1AP identifier |
| `ENBUES1APID` | eNB UE S1AP identifier |
| `Cause` | Radio network, transport, NAS, protocol or miscellaneous cause |
| `PLMNidentity` | Three octets of TBCD |
| `TAI` | PLMN and a TAC of two octets |
| `EUTRANCGI` | PLMN and a 28-bit E-UTRAN cell identity |
| `ERABID` | E-RAB identifier |
| `NASPDU` | An EPS NAS message, as octets |

## JSON: trees and paths

With the `inspect` feature a PDU is a `serde_json::Value` in two forms. A
tree keeps what was received and is read and edited by paths; a message is
written and shown shortly.

| | Tree | Message |
|---|------|---------|
| From a PDU | [`inspect_pdu`] | [`message_tree`] |
| An IE is | an entry: its `id`, its `criticality`, its typed `value`, and what was received | its name with its value |
| It has | the procedure code, the direction, the criticality and the message, in the ASN.1 JSON encoding | the name of the message and its IEs, without what the specification assigns |
| Read and edited | by paths: [`select`], [`paths`], [`set`], [`insert`], [`remove`], [`open`] | as JSON |
| Back to a PDU | [`encode_pdu`]: what was not edited keeps the octets received | [`message_from_tree`]: a message written from the form alone |

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};

// UE CONTEXT RELEASE REQUEST
let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let pdu = S1AP_PDU::decode(&bytes).unwrap();
println!("{}", inspect::message_tree(&pdu).unwrap());
let tree = inspect::inspect_pdu(&pdu).unwrap();
println!("{}", tree["message"]["protocolIEs"][2]);
```

```json
{"ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":7},{"Cause":{"radioNetwork":"user-inactivity"}}],"message":"UEContextReleaseRequest"}
{"_ie_name":"Cause","_original_id":2,"_raw_value":"0280","criticality":"ignore","id":2,"value":{"radioNetwork":"user-inactivity"}}
```

The JSON of this page is what the Rust above it prints, and the recipes
start from that message.

### Read an IE

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use serde_json::json;

let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let tree = inspect::inspect_pdu(&S1AP_PDU::decode(&bytes).unwrap()).unwrap();

// `/s1ap` is the IEs of the message, and an IE goes by its name
let select = |path| inspect::select(&tree, path).unwrap();
assert_eq!(select("/s1ap/eNB-UE-S1AP-ID/value"), [&json!(7)]);
assert_eq!(select("/s1ap/Cause/radioNetwork"), [&json!("user-inactivity")]);
assert_eq!(select("/s1ap/Cause/octets"), [&json!("0280")]);
// By its position or its identifier, in any case, and each of them
assert_eq!(select("/s1ap/1/value"), select("/S1AP/enb_ue_s1ap_id/value"));
assert_eq!(select("/s1ap/@id=8/value"), [&json!(7)]);
assert_eq!(select("/s1ap/*/criticality").len(), 3);
// Nothing where the message could have a value and has none
assert!(select("/s1ap/Cause/nas").is_empty());
assert!(select("/s1ap/GUMMEI-ID/mME-Code").is_empty());

// Each value with the path that selects it
for (path, value) in inspect::paths(&tree) {
    println!("{path} = {value}");
}
```

```text
/criticality = "ignore"
/direction = "InitiatingMessage"
/s1ap/MME-UE-S1AP-ID/criticality = "reject"
/s1ap/MME-UE-S1AP-ID/id = 0
/s1ap/MME-UE-S1AP-ID/value = 42
/s1ap/eNB-UE-S1AP-ID/criticality = "reject"
/s1ap/eNB-UE-S1AP-ID/id = 8
/s1ap/eNB-UE-S1AP-ID/value = 7
/s1ap/Cause/criticality = "ignore"
/s1ap/Cause/id = 2
/s1ap/Cause/value/radioNetwork = "user-inactivity"
/procedure_code = 18
```

### Change a value

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use serde_json::json;

let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let mut tree = inspect::inspect_pdu(&S1AP_PDU::decode(&bytes).unwrap()).unwrap();

// A value, and another alternative of a CHOICE in place of the one that is there
inspect::set(&mut tree, "/s1ap/eNB-UE-S1AP-ID/value", json!(9)).unwrap();
inspect::set(&mut tree, "/s1ap/Cause/nas", json!("Normal Release")).unwrap();
let edited = inspect::encode_pdu(&tree).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap());

// What was not edited is the octets that were received
let after = inspect::inspect_pdu(&edited).unwrap();
let octets = inspect::select(&after, "/s1ap/MME-UE-S1AP-ID/octets").unwrap();
assert_eq!(octets, [&json!("002A")]);
// A name that the type does not have is refused, and the tree is as it was
let wrong = inspect::set(&mut tree, "/s1ap/Cause/radioNetwrok", json!("unspecified"));
println!("{}", wrong.unwrap_err());
```

```text
{"ies":[{"MME-UE-S1AP-ID":42},{"eNB-UE-S1AP-ID":9},{"Cause":{"nas":"normal-release"}}],"message":"UEContextReleaseRequest"}
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, at /s1ap/Cause/radioNetwrok; an IE itself has id, criticality, value, octets
```

### Add or remove an IE

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use serde_json::json;

let bytes = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
let mut tree = inspect::inspect_pdu(&S1AP_PDU::decode(&bytes).unwrap()).unwrap();

// At the end, by its name and its typed value
let name = json!({"id": "eNBname", "criticality": "ignore", "value": "enb-1"});
inspect::insert(&mut tree, "/s1ap/-", name).unwrap();
// Before another IE, by its number and its octets
let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "c0ffee"});
inspect::insert(&mut tree, "/s1ap/Cause", unknown).unwrap();
inspect::remove(&mut tree, "/s1ap/MME-UE-S1AP-ID").unwrap();

let edited = inspect::encode_pdu(&tree).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap()["ies"]);
```

```json
[{"eNB-UE-S1AP-ID":7},{"60000":{"octets":"C0FFEE"},"criticality":"ignore"},{"Cause":{"radioNetwork":"user-inactivity"}},{"criticality":"ignore","eNBname":"enb-1"}]
```

### A repeated IE

```rust
use oxirush_s1ap::inspect;
use serde_json::json;

let twice = json!({"message": "UEContextReleaseRequest", "ies": [
    {"MME-UE-S1AP-ID": 1},
    {"eNB-UE-S1AP-ID": 7},
    {"MME-UE-S1AP-ID": 2},
]});
let pdu = inspect::message_from_tree(&twice).unwrap();
let mut tree = inspect::inspect_pdu(&pdu).unwrap();

// The name selects both, in the order of the message
let both = inspect::select(&tree, "/s1ap/MME-UE-S1AP-ID/value").unwrap();
assert_eq!(both, [&json!(1), &json!(2)]);
// Each has its position, which is how `paths` names it, and its name may follow
inspect::set(&mut tree, "/s1ap/2/MME-UE-S1AP-ID/value", json!(3)).unwrap();
// The name of another IE says which IE is there
println!("{}", inspect::select(&tree, "/s1ap/2/eNB-UE-S1AP-ID/value").unwrap_err());
for (path, value) in inspect::paths(&tree) {
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

### An item of a list

The items of a list of E-RABs are IEs themselves, each alone in a container.

```rust
use oxirush_s1ap::inspect;
use serde_json::{Value, json};

let command = json!({"message": "E-RABReleaseCommand", "ies": [
    {"E-RABToBeReleasedList": [
        {"E-RABItem": {"e-RAB-ID": 5, "cause": {"nas": "normal-release"}}},
        {"E-RABItem": {"e-RAB-ID": 6, "cause": {"nas": "normal-release"}}},
    ]},
]});
let pdu = inspect::message_from_tree(&command).unwrap();
let mut tree = inspect::inspect_pdu(&pdu).unwrap();

// An item by its position or by its name, which selects each item of that name
let select = |tree, path| -> Vec<Value> {
    let selected = inspect::select(tree, path).unwrap();
    selected.into_iter().cloned().collect()
};
assert_eq!(select(&tree, "/s1ap/E-RABToBeReleasedList/1/e-RAB-ID"), [json!(6)]);
assert_eq!(select(&tree, "/s1ap/E-RABToBeReleasedList/E-RABItem/e-RAB-ID"), [json!(5), json!(6)]);
// The same value by the path that says each `value`
assert_eq!(select(&tree, "/s1ap/E-RABToBeReleasedList/value/1/value/e-RAB-ID"), [json!(6)]);
// And by the names that the message shows, joined: the item by its position, then its name
assert_eq!(select(&tree, "/s1ap/E-RABToBeReleasedList/1/E-RABItem/e-RAB-ID"), [json!(6)]);
// The criticality of an item is the one of its object set
assert_eq!(select(&tree, "/s1ap/E-RABToBeReleasedList/0/criticality"), [json!("ignore")]);

// An item is added at the end, as the IE that it is, and one is taken out
let item = json!({"e-RAB-ID": 7, "cause": {"radioNetwork": "unspecified"}});
let item = json!({"id": "E-RABItem", "criticality": "ignore", "value": item});
inspect::insert(&mut tree, "/s1ap/E-RABToBeReleasedList/-", item).unwrap();
inspect::remove(&mut tree, "/s1ap/E-RABToBeReleasedList/0").unwrap();
let edited = inspect::encode_pdu(&tree).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap());
```

```json
{"ies":[{"E-RABToBeReleasedList":[{"E-RABItem":{"cause":{"nas":"normal-release"},"e-RAB-ID":6}},{"E-RABItem":{"cause":{"radioNetwork":"unspecified"},"e-RAB-ID":7}}]}],"message":"E-RABReleaseCommand"}
```

### A transfer and its members

A transfer is an `OCTET STRING` that contains a type. TS 36.413 has none: the
one place where its octets carry a type is
[the container of a handover](#the-container-of-a-handover), which is a
transfer once it is opened.

### Write a message from its name and its IEs

```rust
use oxirush_s1ap::inspect;
use serde_json::json;

let request = json!({
    "message": "ue context release request",
    "ies": [
        {"MME-UE-S1AP-ID": 42},
        {"eNB-UE-S1AP-ID": 7},
        {"Cause": {"radioNetwork": "user-inactivity"}},
        // An IE that the message does not have says its criticality
        {"eNBname": "enb-1", "criticality": "ignore"},
    ],
});
let pdu = inspect::message_from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// The name gave the direction, the procedure code and the criticalities
let tree = inspect::inspect_pdu(&pdu).unwrap();
println!("{} {} {}", tree["direction"], tree["procedure_code"], tree["criticality"]);
println!("{}", json!(inspect::select(&tree, "/s1ap/*/criticality").unwrap()));
```

```text
0012402000000400000002002a000800020007000240020280003c40070200656e622d31
"InitiatingMessage" 18 "ignore"
["reject","reject","ignore","ignore"]
```

### Send octets as they are

```rust
use oxirush_s1ap::inspect;
use serde_json::json;

// `{"octets": "…"}` in place of a value is the octets of the IE, whatever they are
let request = json!({"message": "UEContextReleaseRequest", "ies": [
    {"MME-UE-S1AP-ID": 42},
    {"65000": {"octets": "c0ffee"}, "criticality": "reject"},
]});
let pdu = inspect::message_from_tree(&request).unwrap();
println!("{}", hex::encode(pdu.encode().unwrap()));

// A tree shows an IE that does not decode by its octets, and says why
let mut tree = inspect::inspect_pdu(&pdu).unwrap();
println!("{}", tree["message"]["protocolIEs"][1]);

// In a tree, the `octets` of an IE are set in place of its value
inspect::set(&mut tree, "/s1ap/MME-UE-S1AP-ID/octets", json!("00ff")).unwrap();
let edited = inspect::encode_pdu(&tree).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap());
```

```text
0012401000000200000002002afde80003c0ffee
{"_decode_error":"S1AP IE 65000 is unknown or has several types","_original_id":65000,"_raw_value":"C0FFEE","criticality":"reject","id":65000,"value":"C0FFEE"}
{"ies":[{"MME-UE-S1AP-ID":255},{"65000":{"octets":"C0FFEE"},"criticality":"reject"}],"message":"UEContextReleaseRequest"}
```

### The names of an ENUMERATED

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

A name is written in any case and with any separators, as `"Normal Release"`
above; a tree and a message show it as the specification spells it.

### Check a path before use

```rust
use oxirush_s1ap::inspect;

// Whether a path can select anything in a message of a name, without a tree
let check = |path| inspect::check_path("UEContextReleaseRequest", path);
assert_eq!(check("/s1ap/Cause/nas"), Ok(()));
assert_eq!(check("/s1ap/*/radioNetwork"), Ok(()));
for wrong in ["/s1ap/Cause/radioNetwrok", "/s1ap/eNB-UE-S1AP-ID/valeu", "/s1ap/GUMMEI-ID"] {
    println!("{}", check(wrong).unwrap_err());
}
```

```text
"radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, at /s1ap/Cause/radioNetwrok; an IE itself has id, criticality, value, octets
"valeu" is not a member of this value, which has none, at /s1ap/eNB-UE-S1AP-ID/valeu; an IE itself has id, criticality, value, octets
"GUMMEI-ID" is not an IE of UEContextReleaseRequest, which has MME-UE-S1AP-ID, eNB-UE-S1AP-ID, Cause, GWContextReleaseIndication, SecondaryRATDataUsageReportList, at /s1ap/GUMMEI-ID
```

The tables that `check_path` reads are also given as they are:

| Function | Gives |
|----------|-------|
| [`message_name`], [`message_named`], [`message_names`] | The name that ASN.1 gives the message of a PDU, such as `E-RABSetupRequest`; the direction and the procedure code of a name; all of them |
| [`message_ies`] | The IEs of the object set of a message: the identifier of each and whether it is mandatory, in the order of the set |
| [`message_ie_criticalities`], [`message_criticality`] | The criticality that the set assigns to each, and that of the procedure |
| [`item_criticality`] | The criticality of an IE that a value holds alone, in a `ProtocolIE-SingleContainer`, when its object sets give it one |
| [`ie_names`] | The identifier and the name of each IE of an object set |

### The container of a handover

TS 36.413 gives the content of two IEs by reference, where the ASN.1 has an
`OCTET STRING`: between eNBs they carry a type of this specification, and for
another target system octets of its own. Nothing in a message says which, so
a tree shows the octets, and the one that writes or reads them names the
type.

| The octets of | carry a | Clause |
|---------------|---------|--------|
| `Source-ToTarget-TransparentContainer` | `SourceeNB-ToTargeteNB-TransparentContainer` | 9.2.1.56 |
| `Target-ToSource-TransparentContainer` | `TargeteNB-ToSourceeNB-TransparentContainer` | 9.2.1.57 |

```rust
use oxirush_s1ap::inspect;
use serde_json::json;

// Written: the name of the type with its value, in place of the octets
let name = "SourceeNB-ToTargeteNB-TransparentContainer";
let required = json!({"message": "HandoverRequired", "ies": [
    {"MME-UE-S1AP-ID": 42},
    {"eNB-UE-S1AP-ID": 7},
    {"Source-ToTarget-TransparentContainer": {name: {
        "rRC-Container": "0102",
        "targetCell-ID": {"pLMNidentity": "208-93", "cell-ID": 4660},
        "uE-HistoryInformation": [{"uTRAN-Cell": "00"}],
    }}},
]});
let pdu = inspect::message_from_tree(&required).unwrap();
println!("{}", inspect::message_tree(&pdu).unwrap()["ies"][2]);

// Read: a path goes through the name of the type, once `open` decoded the octets
let mut tree = inspect::inspect_pdu(&pdu).unwrap();
let container = format!("/s1ap/Source-ToTarget-TransparentContainer/{name}");
let cell = format!("{container}/targetCell-ID/cell-ID");
assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));
assert!(inspect::select(&tree, &cell).unwrap_err().contains("open decodes them"));
inspect::open(&mut tree, &cell).unwrap();
assert_eq!(inspect::select(&tree, &cell).unwrap(), [&json!(4660)]);
println!("{}", inspect::select(&tree, &format!("{container}/decoded")).unwrap()[0]);

// An edit goes into it, and what is not edited is sent as it was received
inspect::set(&mut tree, &cell, json!(7)).unwrap();
let edited = inspect::encode_pdu(&tree).unwrap();
println!("{}", inspect::message_tree(&edited).unwrap()["ies"][2]);
```

```text
{"Source-ToTarget-TransparentContainer":"000201020002F83900012340200100"}
{"rRC-Container":"0102","targetCell-ID":{"cell-ID":4660,"pLMNidentity":"208-93"},"uE-HistoryInformation":[{"uTRAN-Cell":"00"}]}
{"Source-ToTarget-TransparentContainer":"000201020002F83900000070200100"}
```

The `rRC-Container` stays octets: it is a message of RRC. The secondary
containers, which a handover to GERAN has, carry no type of this
specification.

### Rules

The documentation of the [`inspect`] module has the full wording, under the
headings that the rows link to.

| Subject | Rule |
|---------|------|
| [Paths] | A JSON pointer into the tree, where `/s1ap` stands for the IEs of the message and `*` for each entry of a list or each member of a value. |
| An IE | The name that ASN.1 gives it after `id-`, its position (`/s1ap/0`) or its identifier (`/s1ap/@id=8`); `-` is the end of the list. A name selects each IE of that name. After the position of an IE, its name may follow, so the names and the positions that a message shows, joined, are a path; that of another IE is an error that says which IE is there. |
| Under an IE | `id`, `criticality`, the typed `value`, and `octets`: what it was received as, in hexadecimal. The members that start with `_` keep what was received. |
| `value`, `decoded` | May be left out of a path. What the IE or the transfer has itself comes first: `/s1ap/Cause/*` is the members of the IE, and `value/value` the one member of a value that is named as one of its IE, in a string of bits whose size varies. |
| [Names] | Letters and digits, whatever their case and whatever is between them, for the root, the IEs, the members of a value, the messages and the values of an `ENUMERATED`: `/S1AP/enb_ue_s1ap_id` is `/s1ap/eNB-UE-S1AP-ID`. |
| [What is not there] | An IE that the message does not have, an `OPTIONAL` member that is absent and another alternative of a `CHOICE` select nothing. A name that the type cannot have is an error, which lists the members of the value, then those of its IE or transfer. |
| [Edits] | `set` writes a member by its ASN.1 name, or an alternative of a `CHOICE` in place of the one that is there. `null` or `remove` takes an optional member out, and `insert` adds before the entry that the path selects, or at `-`. An edit that selects nothing is an error, and the tree is as it was. |
| Octets | What is not edited keeps the octets received. An edit is refused when the value around it does not encode back to them, as with an addition of a later release. The value of an IE or of a transfer is not taken out: the IE is removed, or its `octets` are set. |
| An IE that is written | `{"id", "criticality", "value"}`, or `octets` in place of `value`. The `id` is a number or a name, and the value has the type of the IE whatever JSON it is. |
| A transfer that is written | Its decoded value, alone or as `decoded`, or its octets, in hexadecimal or as `octets`. |
| A message that is written | The name gives the procedure code, the direction and the criticalities. An IE says its `criticality` where no table has it: when the message does not have it, when two object sets give an item two, and in an extension container. |
| What does not decode | An IE or a transfer that is unknown, does not decode or is nested deeper than 64 levels stays octets beside a `_decode_error`, and the rest of the PDU is read. |
| [Octets that carry a type] | The container of a handover is written as `{NAME: VALUE}` by the name of its type, and read once [`open`] decoded it in the tree. |
| [`check_path`], [`enumerated_at`] | Read a path against the types of a message of a name, without a tree: under `/s1ap`, the IE is one of the object set of the message. |

### Values

A tree and a message show the values of some types as they are usually
written, and take them so.

| Type | Notation |
|------|----------|
| `PLMNidentity` | `"208-93"`, the MCC and the MNC |
| `TransportLayerAddress` | `"10.0.0.1"`, `"2001:db8::1"`, or the two with a comma |
| `IMSI` | its digits; one of an even number of digits is read as its digits only |
| `TAC`, `FiveGSTAC`, `LAC`, `RAC`, `CI`, `GTP-TEID`, `M-TMSI`, `MME-Group-ID`, `MME-Code`, `Port-Number`, `CellIdentity`, `NRCellIdentity`, `UL-NAS-Count` | a number, also read as a `"0x…"` string |
| `ENUMERATED` | the name of the value, read in any case |
| the other strings: keys and algorithm masks, the NAS-PDU and the other containers, the node identifiers, whose length says which kind they are | hexadecimal, read in either case |

Each is also read as JER writes it, and a value that does not fit its
notation, such as a transport layer address of another length, stays as JER
writes it.

## Examples

| Example | Shows | Feature |
|---------|-------|---------|
| `build_pdu` | Initial Context Setup request and response, UE Context Release, Handover Required and Acknowledge, S1 Setup Failure, and one IE built alone | |
| `decode_manually` | An S1 Setup Response built and read through rasn's APER API | |
| `extract_ies` | UE release, handover, and nested EPS bearer and NAS data read from decoded messages | |
| `inspect` | An Initial UE Message: its values with their paths, one IE edited, one added by its value and one by its octets, and the PDU encoded again | `inspect` |

```bash
cargo run -p oxirush-s1ap --example build_pdu
cargo run -p oxirush-s1ap --example inspect --features inspect
```

Each has its counterpart in `oxirush-ngap`, with the messages and the IEs of
NGAP.

## Specifications

| Specification | Version | For |
|---------------|---------|-----|
| 3GPP TS 36.413 | V19.2.0 (Release 19), `36413-j20.zip` | S1AP: the six ASN.1 modules of clause 9.3, and clause 9.2 for what the ASN.1 leaves to prose |
| ITU-T X.691 | (02/2021) | Aligned PER |

## Tests

| Test | What it holds |
|------|---------------|
| `tests/aper_vectors.rs` | The 101 messages of `tests/fixtures/messages.tsv`, one for each message of each procedure, were encoded by pycrate 0.8.1. Each decodes to its type and encodes to the same octets, and each of its truncations and one more octet are refused. |
| `tests/aper_round_trip.rs` | The macros, and the encodings where the codec departs from rasn's: constrained lists, the lists of E-RABs and of cells, and their size constraints. |
| `tests/unknown_extensions.rs` | The additions of a later release to an extensible `SEQUENCE` are skipped, and what follows them is read. |
| `tests/decode_allocation.rs` | Decoding reserves memory for the elements that it reads, not for the count that a length determinant claims. |
| `tests/inspection.rs` | Each of the 101 messages is the same octets through its tree, and through its shown message where it has IEs to name. Each path of each selects its value, with and without `value`, and is one that `check_path` takes. Edits, written messages, criticalities, and the identifiers of the registry against the constants of the bindings. |
| `tests/shared_files.rs` | The files that this crate shares with `oxirush-ngap` are as they were last mirrored. |
| The unit tests of the paths | No two names that a path takes can be taken for each other, and a value has no member that is named as one of its IE, but the `value` of a string of bits. |
| The Rust blocks of this page | They compile and run as documentation tests, with the `inspect` feature. |

```bash
cargo test --all-features
cargo test
```

## Generated code

Cargo compiles three checked-in files. A build, docs.rs and the verification
of a package run no generator, and the published crate has neither the
generator nor the ASN.1.

| File | Contents |
|------|----------|
| `src/s1ap.rs` | The bindings of `rasn-compiler`, with the flat API, the convenience methods and the `Display` implementation that the generator adds |
| `src/registry.rs` | The list of the procedures and of the IEs, one line for each: its code or its identifier, its names and its types, and for each message of a procedure the IEs of its object set. The names that the macros take, `S1apPduKind` and the names and the types of `inspect` all expand from it, so each is written once |
| `src/inspect_registry.rs` | What `inspect` alone needs, with the `inspect` feature: among it the list of the types, one line for each `SEQUENCE` and each `CHOICE` with its members, which the paths are read against, and the types that a handover's containers carry, which the generator has from the prose of the specification |

For maintainers, `build/` holds the generator, a package of its own whose
lockfile pins `rasn-compiler`. It reads the six `.asn` modules in `s1ap/`,
which Git ignores: the ASN.1 of clause 9.3 of TS 36.413 V19.2.0, one file per
module, taken out of the document of the `36413-j20.zip` archive. The
repository has no step that extracts them. From the crate directory:

```sh
CARGO="$(command -v cargo)" cargo run --locked --manifest-path build/Cargo.toml
rustfmt --edition 2024 src/s1ap.rs src/registry.rs src/inspect_registry.rs
```

`rasn-compiler` finds rustfmt through `CARGO`. Commit the three files after
regenerating them, and do not edit them by hand. They were last regenerated
on 2026-10-11, with the generator of that revision and the rustfmt of Rust
1.88: they came out as they are committed, octet for octet. No CI job
regenerates them, as the ASN.1 is not in the repository.

oxirush-s1ap and oxirush-ngap share, octet for octet, the code that does not
depend on the protocol: `build/aper_fix.rs`, `build/containers.rs`,
`build/inspection.rs`, `build/registry.rs`, `src/inspect_paths.rs`,
`src/per.rs` and `src/sized.rs`. `tests/shared_files.rs` holds a digest of
each, so that an edit in one crate fails its tests until the file is the same
in the other and both record the new digest.

### rasn integration

`rasn-compiler` runs in its stable mode, which leaves an open type opaque:
TS 36.413 relies on parameterized information object classes, and the
experimental typed mode emits unresolved object sets and bindings that do not
compile for these modules. `Any` stays inside the generated types, and the
macros give typed construction, extraction and mutation.

| Subject | What the crate does |
|---------|---------------------|
| Containers | The protocol IE and extension containers take an object set as a parameter, which an opaque open type does not use. Each is one type, `ProtocolIEContainer` of `ProtocolIEField` and `ProtocolExtensionContainer` of `ProtocolExtensionField`, in place of the one that `rasn-compiler` resolves for each use. |
| rasn features | `bytes` storage is on; `f32` and `f64`, which nothing uses, are off. |
| Complete values | `S1AP_PDU::decode`, `decode_value` and the extraction of a typed IE refuse incomplete values and trailing whole octets. |

rasn 0.28's APER codec departs from ITU-T X.691 where S1AP reaches it. There
the generator replaces the derived codec with `Encode` and `Decode`
implementations written against rasn's public codec traits; every other value
uses the derived codec unchanged.

| Value | Correction |
|-------|------------|
| Constrained `SEQUENCE OF` | rasn misaligns the length determinant and the components |
| Fixed-size `BIT STRING` longer than 16 bits | It is octet-aligned |
| `OCTET STRING` and `BIT STRING` with a two-octet length determinant | Components and alternatives take the types of the `sized` module |
| Extensible `SEQUENCE` | Its decoder consumes and skips the additions that it does not know |

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Sign off your commits (`git commit -s`)
4. Open a Pull Request

### Developer Certificate of Origin (DCO)

By contributing to this project, you agree to the [Developer Certificate of Origin (DCO)](https://developercertificate.org/). This means that you have the right to submit your contributions and you agree to license them according to the project's license.

All commits should be signed-off with `git commit -s` to indicate your agreement to the DCO.

## License

Copyright 2025-2026 Valentin D'Emmanuele

Licensed under the Apache License, Version 2.0. See [LICENSE](https://github.com/linouxis9/oxirush-s1ap/blob/main/LICENSE) for details.

[`inspect`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/index.html
[Paths]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/index.html#paths
[Names]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/index.html#names
[What is not there]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/index.html#what-is-not-there
[Edits]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/index.html#edits
[Octets that carry a type]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/index.html#octets-that-carry-a-type
[`inspect_pdu`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.inspect_pdu.html
[`encode_pdu`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.encode_pdu.html
[`message_tree`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.message_tree.html
[`message_from_tree`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.message_from_tree.html
[`select`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.select.html
[`paths`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.paths.html
[`set`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.set.html
[`insert`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.insert.html
[`remove`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.remove.html
[`open`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.open.html
[`check_path`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.check_path.html
[`enumerated_at`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.enumerated_at.html
[`message_name`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.message_name.html
[`message_named`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.message_named.html
[`message_names`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.message_names.html
[`message_ies`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.message_ies.html
[`message_ie_criticalities`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.message_ie_criticalities.html
[`message_criticality`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.message_criticality.html
[`item_criticality`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.item_criticality.html
[`ie_names`]: https://docs.rs/oxirush-s1ap/latest/oxirush_s1ap/inspect/fn.ie_names.html
