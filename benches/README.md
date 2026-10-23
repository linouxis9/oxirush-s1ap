# Benchmarks of oxirush-s1ap

A package of its own, which the published crate does not have. Its programs
are those of `protocols/benches/` of
[oxirush-aper](https://github.com/linouxis9/oxirush-aper), in the checkout
beside this repository, which oxirush-ngap and oxirush-s1ap share; what is of
the protocol is in `src/` and `compare/` here.

The README sums up the comparison with the other implementations of S1AP,
and [The six PDUs](#the-six-pdus) has its figures. The figures below were taken on the same
machine, an AMD Ryzen 7 5700G (16 threads, Linux 7.0, Rust 1.98.1), measured
on a quiet machine on 2026-10-18, at 774c1b5.

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

## The six PDUs

The comparison of the [README](../README.md#against-other-implementations),
on the six PDUs of `compare/pdus.tsv`, which each implementation decodes and
encodes. The envelope alone is `S1AP_PDU::decode`, which leaves the value of
each IE in its octets; "this crate" decodes the PDU and then every IE of its
message as its type, which is what the others do; and encoding writes the PDU
from the typed values of its IEs, as `build_s1ap!` does.

Decoding, ns per call:

| PDU (octets) | this crate, the envelope alone | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| DownlinkNASTransport (73) | 8 | 79 | 691 | 860 | 1,750 | 69,190 |
| InitialContextSetupRequest (197) | 8 | 221 | 2,123 | 3,320 | 4,956 | 218,657 |
| S1SetupResponse (45) | 8 | 176 | 1,104 | 1,308 | 2,098 | 95,205 |
| HandoverRequest (146) | 8 | 266 | 2,519 | 3,554 | 5,369 | 255,711 |
| UEContextReleaseCommand (21) | 8 | 95 | 611 | 637 | 1,327 | 66,970 |
| InitialUEMessage (48) | 8 | 136 | 1,277 | 1,391 | 2,482 | 120,191 |

The time of each other implementation over this crate's, decoding:

| PDU (octets) | rasn / this crate | hampi / this crate | asn1c / this crate | pycrate / this crate |
|---|---|---|---|---|
| DownlinkNASTransport (73) | 8.8x | 11x | 22x | 880x |
| InitialContextSetupRequest (197) | 9.6x | 15x | 22x | 990x |
| S1SetupResponse (45) | 6.3x | 7.4x | 12x | 540x |
| HandoverRequest (146) | 9.5x | 13x | 20x | 960x |
| UEContextReleaseCommand (21) | 6.4x | 6.7x | 14x | 700x |
| InitialUEMessage (48) | 9.4x | 10x | 18x | 880x |

Encoding, ns per call:

| PDU (octets) | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|
| DownlinkNASTransport (73) | 64 | 880 | 1,528 | 1,113 | 45,097 |
| InitialContextSetupRequest (197) | 144 | 1,757 | 4,420 | 2,920 | 135,987 |
| S1SetupResponse (45) | 85 | 1,047 | 1,701 | 977 | 61,261 |
| HandoverRequest (146) | 170 | 2,158 | 4,799 | 3,014 | 154,533 |
| UEContextReleaseCommand (21) | 64 | 770 | 1,226 | 715 | 43,780 |
| InitialUEMessage (48) | 95 | 1,183 | 2,178 | 1,332 | 72,281 |

The time of each other implementation over this crate's, encoding:

| PDU (octets) | rasn / this crate | hampi / this crate | asn1c / this crate | pycrate / this crate |
|---|---|---|---|---|
| DownlinkNASTransport (73) | 14x | 24x | 18x | 710x |
| InitialContextSetupRequest (197) | 12x | 31x | 20x | 940x |
| S1SetupResponse (45) | 12x | 20x | 12x | 720x |
| HandoverRequest (146) | 13x | 28x | 18x | 910x |
| UEContextReleaseCommand (21) | 12x | 19x | 11x | 680x |
| InitialUEMessage (48) | 12x | 23x | 14x | 760x |

The figures were taken at 774c1b5, by [the method of the comparison](#the-method-of-the-comparison).

## Every figure of the comparison for InitialContextSetupRequest

The programs, the machine and the method of the comparison of the
[README](../README.md#against-other-implementations), per call:

|  | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|
| decode ns | 221 | 2,123 | 3,320 | 4,956 | 218,657 |
| decode cycles | 1,014 | 10,072 | 15,135 | 22,442 | 985,521 |
| decode instructions | 3,401 | 22,939 | 48,070 | 81,650 | 2,090,371 |
| decode allocations | 2 | 20 | 37 | 79 |  |
| decode octets allocated | 256 | 1,074 | 3,569 | 10,046 |  |
| encode ns | 144 | 1,757 | 4,420 | 2,920 | 135,987 |
| encode cycles | 624 | 8,086 | 20,491 | 13,278 | 612,914 |
| encode instructions | 2,247 | 26,482 | 63,793 | 50,474 | 1,280,226 |
| encode allocations | 2 | 33 | 17 | 25 |  |
| encode octets allocated | 454 | 1,698 | 1,797 | 3,728 |  |
| most memory held, KiB | 3,326 | 3,812 | 3,330 | 4,546 | 28,128 |

The time, the cycles and the instructions of each other implementation over this crate's:

|  | rasn / this crate | hampi / this crate | asn1c / this crate | pycrate / this crate |
|---|---|---|---|---|
| decode ns | 9.6x | 15x | 22x | 990x |
| decode cycles | 9.9x | 15x | 22x | 970x |
| decode instructions | 6.7x | 14x | 24x | 610x |
| encode ns | 12x | 31x | 20x | 940x |
| encode cycles | 13x | 33x | 21x | 980x |
| encode instructions | 12x | 28x | 22x | 570x |

## The method of the comparison

A time is the fastest of three runs of a call repeated for about 0.2 s. The
cycles and the instructions are those that `perf stat` counts for the calls
alone: the difference between a run of twice the calls and one of the calls.
The allocations and the octets that they ask for are counted by an allocator
that counts in Rust, by `malloc`, `calloc` and `realloc` for asn1c; pycrate's
are not counted. The most memory held is the high-water mark of the resident
memory of the program (`VmHWM`), its runtime included. The figures were taken at 774c1b5 on
an AMD Ryzen 7 5700G (16 threads, Linux 7.0, Rust 1.98.1, Python 3.14), each
program on one core, measured on a quiet machine on 2026-10-18: each is the
median of four runs of the script, which fetches each implementation at a
pinned version: hampi 0.7.2 and rasn 0.29.0 from crates.io, Open5GS v2.8.0
from GitHub, pycrate 0.8.1 from PyPI, and the ASN.1 from the archive of 3GPP.
The [README](../README.md#against-other-implementations) has the command.
