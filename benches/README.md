# Benchmarks of oxirush-s1ap

A package of its own, which the published crate does not have. Its programs
are those of `protocols/benches/` of
[oxirush-aper](https://github.com/linouxis9/oxirush-aper), in the checkout
beside this repository, which oxirush-ngap and oxirush-s1ap share; what is of
the protocol is in `src/` and `compare/` here.

The comparison with the other implementations of S1AP is in the
[README](../README.md#performance). The figures below were taken on the same
machine, an AMD Ryzen 7 5700G (16 threads, Linux 7.0, Rust 1.98.1), measured
on a quiet machine on 2026-10-18.

## Reading the IEs, and checking a PDU

Reading the IEs of a message and checking a PDU, per call, on the four
messages that `benches/` writes: `decode_value()` and then the macro on the
message, which decodes the container of the message first; the macro on the
PDU (`extract_s1ap_ies!(pdu, ...)`), which walks the octets of its message once and
decodes the IEs that its fields ask for; and `ie::<T>()` of the PDU, which
decodes one IE. In `benches/`, `cargo run --release --bin readers --features
readers` measures them:

| Message | the first IE: `decode_value()`, then the macro | the macro on the PDU | `ie::<T>()` | every IE: `decode_value()`, then the macro | the macro on the PDU |
|---|---|---|---|---|---|
| An INITIAL CONTEXT SETUP REQUEST of 8 E-RABs | 96 ns, 1 allocation | 33 ns, no allocation | 28 ns, no allocation | 320 ns, 2 allocations | 279 ns, 1 allocation |
| An E-RAB SETUP REQUEST of 8 E-RABs | 63 ns, 1 allocation | 24 ns, no allocation | 28 ns, no allocation | 177 ns, 2 allocations | 134 ns, 1 allocation |
| A HANDOVER REQUIRED of a container of 4,000 octets | 88 ns, 1 allocation | 29 ns, no allocation | 28 ns, no allocation | 204 ns, 1 allocation | 156 ns, no allocation |
| A PAGING of 16 tracking areas | 81 ns, 1 allocation | 24 ns, no allocation | 29 ns, no allocation | 231 ns, 2 allocations | 189 ns, 1 allocation |

`items::<L>()` walks the eight E-RABs of the E-RAB SETUP REQUEST in the octets
of the PDU in 901 ns without an allocation, where `decode_value()`, the
macro and then the decoding of each item take 773 ns and 2 allocations:
the time is that of decoding the items.

`check()`, which decodes the value of each IE, and `check_structure()`, which
compares the container alone with the object set of its message, on the PDUs
of the comparison (`cargo run --release --bin checks` in `benches/`):

| PDU (octets) | `check()` | `check_structure()` | `check()` / `check_structure()` |
|---|---|---|---|
| DownlinkNASTransport (73) | 408 ns, 7 allocations | 85 ns, 2 allocations | 4.8x |
| InitialContextSetupRequest (197) | 989 ns, 13 allocations | 141 ns, 2 allocations | 7.0x |
| S1SetupResponse (45) | 495 ns, 12 allocations | 67 ns, 2 allocations | 7.4x |
| HandoverRequest (146) | 1,252 ns, 17 allocations | 156 ns, 2 allocations | 8.0x |
| UEContextReleaseCommand (21) | 288 ns, 5 allocations | 48 ns, 2 allocations | 6.0x |
| InitialUEMessage (48) | 607 ns, 8 allocations | 106 ns, 2 allocations | 5.7x |

Each program ran on one core, and each figure is the median of three runs.

A clean `cargo build -j 5` of the crate and its dependencies on Rust 1.88, on
five cores of the same machine, the median of three builds, with the debug
information of the default profile and without it:

| Build | Time | Peak memory | Peak memory, `CARGO_PROFILE_DEV_DEBUG=0` |
|-------|------|-------------|------------------------------------------|
| No feature | 8 s | 0.72 GiB | 0.68 GiB |
| `inspect` | 13 s | 1.06 GiB | 0.85 GiB |
| `serde` | 14 s | 1.10 GiB | 0.95 GiB |
| `inspect`, `--release` | 17 s | 0.91 GiB | the same |

On a message of 65,535 IEs of one name, 393 KB, `decode` takes 0.01 ms, `check`
5.27 ms, `to_view` 67 ms and 37 MB, `with_view` 119 ms and 48 MB,
`inspect::paths` 30 ms, and an edit at a path 0.01 ms, on one core of the same
machine, the median of three runs. In `benches/`, `cargo bench --bench scale`
measures them, and `cargo bench --bench codec` and `--bench allocations` the
codec on the PDUs of the tests; from the root of the repository,
`python3 ../oxirush-aper/protocols/benches/build_cost.py +1.88.0 --features inspect`
measures a clean build.

## Every figure of the comparison for InitialContextSetupRequest

The programs, the machine and the method of the comparison of the
[README](../README.md#against-other-implementations), per call:

|  | decode ns | cycles | instructions | allocations | octets allocated | encode ns | cycles | instructions | allocations | octets allocated | most memory held, KiB |
|---|---|---|---|---|---|---|---|---|---|---|---|
| this crate | 221 | 1,014 | 3,401 | 2 | 256 | 144 | 624 | 2,247 | 2 | 454 | 3,326 |
| rasn 0.29 | 2,123 (9.6x) | 10,072 (9.9x) | 22,939 (6.7x) | 20 | 1,074 | 1,757 (12.2x) | 8,086 (13.0x) | 26,482 (11.8x) | 33 | 1,698 | 3,812 |
| hampi 0.7.2 | 3,320 (15.0x) | 15,135 (14.9x) | 48,070 (14.1x) | 37 | 3,569 | 4,420 (30.7x) | 20,491 (32.8x) | 63,793 (28.4x) | 17 | 1,797 | 3,330 |
| asn1c (Open5GS 2.8.0) | 4,956 (22.5x) | 22,442 (22.1x) | 81,650 (24.0x) | 79 | 10,046 | 2,920 (20.3x) | 13,278 (21.3x) | 50,474 (22.5x) | 25 | 3,728 | 4,546 |
| pycrate 0.8.1 | 218,657 (990.5x) | 985,521 (971.9x) | 2,090,371 (614.6x) |  |  | 135,987 (945.0x) | 612,914 (982.0x) | 1,280,226 (569.7x) |  |  | 28,128 |
