# oxirush-s1ap

[![Crates.io](https://img.shields.io/crates/v/oxirush-s1ap.svg)](https://crates.io/crates/oxirush-s1ap)
[![Documentation](https://docs.rs/oxirush-s1ap/badge.svg)](https://docs.rs/oxirush-s1ap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A complete LTE S1 Application Protocol (S1AP) APER codec generated from the
3GPP TS 36.413 ASN.1 modules with
[`rasn-compiler`](https://crates.io/crates/rasn-compiler) and encoded with
[`rasn`](https://crates.io/crates/rasn).

The 5G companion crate,
[`oxirush-ngap`](https://crates.io/crates/oxirush-ngap), exposes the same API
shape, macros, generated-source release model, and example workflow for TS 38.413.

## Features

- Bindings generated from the six normative TS 36.413 ASN.1 modules sourced
  from the official 3GPP 36.413 v19.2.0 (`36413-j20.zip`) archive.
- Checked-in Rust bindings; normal builds and docs.rs do not run a generator,
  and the published crate excludes its raw ASN.1 inputs.
- Aligned PER encoding and decoding through `rasn`.
- Flat re-exports for generated S1AP types and compatibility aliases such as
  `S1AP_PDU`, `MME_UE_S1AP_ID`, and `ENB_UE_S1AP_ID`.
- `build_s1ap!`, `build_s1ap_ie!`, `extract_s1ap_ies!`, and
  `with_s1ap_ie_mut!` helpers for typed open types.
- ASN.1-derived IE IDs, procedure codes, procedure names, directions, and PDU
  kinds.
- Protocol helpers for PLMN, core-network, tracking-area, cell, and radio-node
  identities, plus bit strings and UE security capabilities.
- Optional `inspect` feature: a decoded PDU as a JSON tree that can be edited
  and encoded again. See [Inspection](#inspection).

## Quick start

```toml
[dependencies]
oxirush-s1ap = "0.3"
```

The minimum supported Rust version is 1.88.

```rust
use oxirush_s1ap::{build_s1ap, extract_s1ap_ies, s1ap::*};

let pdu = build_s1ap!(InitiatingMessage, UEContextReleaseRequest,
    IGNORE, UEContextReleaseRequest,
    REJECT MME_UE_S1AP_ID(42u32),
    REJECT eNB_UE_S1AP_ID(7u32),
    IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
);

let bytes = pdu.encode().unwrap();
let decoded = S1AP_PDU::decode(&bytes).unwrap();
assert_eq!(decoded.procedure_code(), 18);
assert_eq!(decoded.procedure_name(), "UEContextReleaseRequest");
assert!(decoded.is_initiating());

let request: UEContextReleaseRequest = decoded.decode_value().unwrap();
extract_s1ap_ies!(request, UEContextReleaseRequest, opt enb_id: u32 = eNB_UE_S1AP_ID(id));
assert_eq!(enb_id, Some(7));
```

`build_s1ap!` takes a direction, procedure name, outer criticality, message
name, and zero or more IEs. Each IE is written as
`CRITICALITY IeName(value)`. Numeric IE IDs and procedure codes are generated
from the ASN.1 object sets rather than maintained by hand.

## Extracting IEs

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

A `req` field returns `MissingIeError` from the enclosing function when the IE
is absent or invalid. An `opt` field remains `Option<T>`. Without a custom
`=> expression`, extraction unwraps the generated newtype's `.0` field.

## Common helpers

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

## Inspection

The `inspect` feature turns a decoded PDU into a `serde_json` tree in the
ASN.1 JSON encoding (JER), and a tree into a PDU. Each value of the tree has
a path, in which `/s1ap` stands for the IEs of the message and an IE goes by
its name:

```rust
use oxirush_s1ap::{inspect, s1ap::S1AP_PDU};
use serde_json::json;

fn edit(pdu: &S1AP_PDU) -> Result<S1AP_PDU, String> {
    let mut tree = inspect::inspect_pdu(pdu)?;
    // /s1ap/MME-UE-S1AP-ID/value = 42
    // /s1ap/eNB-UE-S1AP-ID/value = 7
    // /s1ap/Cause/value/radioNetwork = "user-inactivity"
    for (path, value) in inspect::paths(&tree) {
        println!("{path} = {value}");
    }
    let cause = inspect::select(&tree, "/s1ap/Cause/value/radioNetwork")?;
    assert_eq!(cause, [&json!("user-inactivity")]);

    inspect::set(&mut tree, "/s1ap/eNB-UE-S1AP-ID/value", json!(9))?;
    inspect::remove(&mut tree, "/s1ap/Cause")?;
    let name = json!({"id": "eNBname", "criticality": "ignore", "value": "enb-1"});
    inspect::insert(&mut tree, "/s1ap/-", name)?;
    inspect::encode_pdu(&tree)
}
```

An IE is named as ASN.1 names it after `id-`, in any case, and also selected
by its position (`/s1ap/0`), by its identifier (`/s1ap/@id=8`) or with all
the others (`/s1ap/*`); `paths` gives the position of an IE that has no name
or is there twice. Under an IE are its `criticality`, its typed `value` and
its `octets`, what it was received as. The entries of a list of IEs in a
value are selected the same way, each with its own `value`:

```text
/s1ap/E-RABToBeSetupListBearerSUReq/value/0/value/e-RAB-ID = 5
```

An IE that the message does not have selects nothing; a name that is no IE,
or a member that a value cannot have, is an error.

`inspect::message_name(&pdu)` is the name that ASN.1 gives the message of a
PDU, such as `E-RABSetupRequest`, and `inspect::message_named` finds
the `direction` and the `procedure_code` that a tree has for a message from
its name, whatever its case and its hyphens.

What is not edited keeps the octets received, and repeated IEs keep their
order. An IE that is unknown or does not decode stays as its octets beside a
`_decode_error`; a PDU whose message does not decode is an error. An edit is
refused when the value around it does not encode back to the octets received,
as with an unknown extension addition, and when it has a member that the ASN.1
type does not have. An IE is added with `insert`, before the IE that the path
selects or at the end for `/s1ap/-`, as its `id`, by name or by number, its
`criticality` and its `value`, which is typed whatever JSON it is: an
`ENUMERATED` is added by its name. Given octets are sent as its `octets`,
without `value`. The module documentation lists the members of the tree and
the rules of an edit.

The values of well-known types are shown, and taken, as they are usually
written:

| Type | In the tree |
| --- | --- |
| `PLMNidentity` | `"208-93"`, the MCC and the MNC |
| `TransportLayerAddress` | `"10.0.0.1"`, `"2001:db8::1"`, or the two with a comma |
| `IMSI` | its digits |
| `TAC`, `FiveGSTAC`, `LAC`, `RAC`, `CI`, `GTP-TEID`, `M-TMSI`, `MME-Group-ID`, `MME-Code`, `Port-Number`, `CellIdentity`, `NRCellIdentity`, `UL-NAS-Count` | a number |

A number is also taken as a `"0x…"` string, each of these values as JER writes
it, and the name of an `ENUMERATED` value whatever its case, with `-`, `_` and
space taken as the same. An IMSI of an even number of digits is taken as its
digits only. A value that does not fit its form, such as a transport layer
address of another length, stays as JER writes it. The other strings stay in
hexadecimal: keys and algorithm masks, the NAS-PDU and the other containers,
and the node identifiers, whose length says which kind they are.

Limits:

- An IE with an integer of 2^63 or more stays as its octets, and one with an
  extensible fixed-size `BIT STRING` of another size cannot be edited as a
  typed value: rasn's JER does not represent them.
- The feature compiles JER and APER code for every type of the protocol. A
  debug build of the crate takes two to three times as long as without it,
  and about three times the memory.

## Code generation

Cargo compiles the checked-in `src/s1ap.rs`, `src/registry.rs` and, with the
`inspect` feature, `src/inspect_registry.rs`.
Normal builds, docs.rs, and crates.io package verification do not run a
generator. The published crate excludes the generator and the ASN.1 inputs.

For maintainers, `build/` holds the generator, a package of its own whose
lockfile pins `rasn-compiler`. It reads the six `.asn` modules in `s1ap/`,
which Git ignores: the ASN.1 of clause 9.3 of TS 36.413 v19.2.0, one file per
module, taken out of the specification's document in the official
`36413-j20.zip` archive. The repository has no step that extracts them. From
the crate directory:

```sh
CARGO="$(command -v cargo)" cargo run --locked --manifest-path build/Cargo.toml
rustfmt --edition 2024 src/s1ap.rs src/registry.rs src/inspect_registry.rs
```

`rasn-compiler` finds rustfmt through `CARGO`. To the compiler's output the
generator adds the flat API, convenience methods, and `Display`
implementation. `src/registry.rs` is the list of the procedures and of the IEs:
one line for each, with its code or its identifier, its names and its types.
The names that the macros take, `S1apPduKind` and the names and the types of the
`inspect` feature all expand from it, so each is written once.
`src/inspect_registry.rs` has what the `inspect` feature alone needs. Commit
the three files after regenerating them. Do not edit them by hand.

## rasn integration

`rasn-compiler`'s stable opaque-open-type mode is used because TS 36.413 relies
heavily on parameterized information object classes. Its experimental typed
mode emits unresolved object-set warnings and uncompilable bindings for these
six modules. `Any` stays inside the generated representation; the public macros
provide typed construction, extraction, and mutation.

TS 36.413 gives its protocol IE and extension containers an object set as a
parameter, which an opaque open type does not use. Each is one type:
`ProtocolIEContainer` and `ProtocolExtensionContainer`, of `ProtocolIEField`
and `ProtocolExtensionField`. They take the place of the types that
`rasn-compiler` resolves for each use of one.

The crate enables rasn's efficient `bytes` storage and leaves its unused `f32`
and `f64` features disabled. rasn 0.28's APER codec departs from ITU-T X.691
where S1AP reaches it, so the generator replaces the derived codec there with
narrowly scoped `Encode` and `Decode` implementations written against rasn's
public codec traits:

- constrained `SEQUENCE OF` containers, whose length determinant and
  components rasn misaligns;
- fixed-size `BIT STRING` values longer than 16 bits, which are
  octet-aligned;
- `OCTET STRING` and `BIT STRING` components and alternatives with a
  two-octet length determinant, which take the types of the `sized` module;
- extensible `SEQUENCE` values, whose decoders consume and skip unknown
  future additions.

All other values use rasn-derived codecs unchanged.

`S1AP_PDU::decode`, `decode_value`, and typed IE extraction reject incomplete
values and trailing whole octets. Outer PDUs retain opaque open-type bytes;
decoding and re-encoding a typed value with unknown future `SEQUENCE`
additions drops those additions. The codec does not enforce procedure state
or all mandatory and conditional IE presence rules; callers enforce those.

## Key types

| Type | Description |
| --- | --- |
| `S1AP_PDU` / `S1APPDU` | Top-level initiating/successful/unsuccessful PDU choice |
| `InitiatingMessage` | Procedure code, criticality, and message open type |
| `ProtocolIEContainer` / `ProtocolIEField` | The IEs of a message: identifier, criticality, and value open type |
| `MMEUES1APID` | 32-bit MME UE S1AP identifier |
| `ENBUES1APID` | eNB UE S1AP identifier |
| `Cause` | Radio network, transport, NAS, protocol, or miscellaneous cause |
| `PLMNidentity` | Three-octet TBCD PLMN |
| `TAI` | LTE tracking-area identity with a two-octet TAC |
| `EUTRANCGI` | PLMN plus 28-bit E-UTRAN cell identity |
| `ERABID` | E-RAB identifier |
| `NASPDU` | Opaque EPS NAS payload |

## Examples

```bash
cargo run -p oxirush-s1ap --example build_pdu
cargo run -p oxirush-s1ap --example decode_manually
cargo run -p oxirush-s1ap --example extract_ies
cargo run -p oxirush-s1ap --example inspect --features inspect
```

- `build_pdu` covers Initial Context Setup request/response, UE Context Release,
  Handover Required/Acknowledge, S1 Setup Failure, and standalone IE creation.
- `decode_manually` constructs and inspects an S1 Setup Response through rasn's
  raw APER API.
- `extract_ies` extracts UE release, handover, and nested EPS-bearer/NAS data
  from decoded S1AP messages.
- `inspect` decodes an Initial UE Message, prints its values with their paths,
  edits one IE, adds one by its value and one by its octets, and encodes the
  PDU again.

The examples intentionally mirror the corresponding `oxirush-ngap`
examples, substituting the standards-defined S1AP messages and IEs.

## References

- 3GPP TS 36.413: E-UTRAN S1 Application Protocol (S1AP)
- ITU-T X.691: ASN.1 Packed Encoding Rules

## Documentation

Full API reference: [**https://docs.rs/oxirush-s1ap**](https://docs.rs/oxirush-s1ap)

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
