# oxirush-s1ap

[![Crates.io](https://img.shields.io/crates/v/oxirush-s1ap.svg)](https://crates.io/crates/oxirush-s1ap)
[![Documentation](https://docs.rs/oxirush-s1ap/badge.svg)](https://docs.rs/oxirush-s1ap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A complete LTE S1 Application Protocol (S1AP) APER codec generated from the
3GPP TS 36.413 ASN.1 modules with
[`rasn-compiler`](https://crates.io/crates/rasn-compiler) and encoded with
[`rasn`](https://crates.io/crates/rasn).

Part of [OxiRush](https://github.com/linouxis9/oxirush), a mobile-core testing
framework. The 5G companion crate,
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

## Quick start

```toml
[dependencies]
oxirush-s1ap = "0.3"
```

```rust
use oxirush_s1ap::{build_s1ap, s1ap::*};

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
assert_eq!(request.protocol_ies.0.len(), 3);
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

## Code generation

Cargo compiles the checked-in `src/s1ap.rs` module directly. Normal builds,
docs.rs, and crates.io package verification do not run a generator. The
published crate deliberately excludes both the generator and the raw ASN.1
inputs.

For maintainers, `build/main.rs` contains the `rasn-compiler` generation
pipeline. It reads the six locally supplied `.asn` modules in `s1ap/`, then
adds the flat API, typed-open-type macros, procedure metadata, convenience
methods, and `Display` implementation. The `.asn` and `.asn1` inputs are
ignored by Git and must be obtained directly from the official 3GPP 36.413
v19.2.0 (`36413-j20.zip`) archive before regeneration.

Commit `src/s1ap.rs` after regenerating it. Do not edit generated bindings by
hand.

## rasn integration

`rasn-compiler`'s stable opaque-open-type mode is used because TS 36.413 relies
heavily on parameterized information object classes. Its experimental typed
mode emits unresolved object-set warnings and uncompilable bindings for these
six modules. `Any` stays inside the generated representation; the public macros
provide typed construction, extraction, and mutation.

The crate enables rasn's efficient `bytes` storage and leaves its unused `f32`
and `f64` features disabled. rasn 0.28's derived APER codec can disagree with
its decoder for the constrained `SEQUENCE OF` containers used by S1AP. The
generator therefore emits narrowly scoped `Encode` and `Decode`
implementations for those containers through rasn's public codec traits. All
other values use rasn-derived codecs unchanged.

## Key types

| Type | Description |
| --- | --- |
| `S1AP_PDU` / `S1APPDU` | Top-level initiating/successful/unsuccessful PDU choice |
| `InitiatingMessage` | Procedure code, criticality, and message open type |
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
```

- `build_pdu` covers Initial Context Setup request/response, UE Context Release,
  Handover Required/Acknowledge, S1 Setup Failure, and standalone IE creation.
- `decode_manually` constructs and inspects an S1 Setup Response through rasn's
  raw APER API.
- `extract_ies` extracts UE release, handover, and nested EPS-bearer/NAS data
  from decoded S1AP messages.

The three examples intentionally mirror the corresponding `oxirush-ngap`
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
