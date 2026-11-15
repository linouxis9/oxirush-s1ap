# Benchmarks of oxirush-s1ap

A package of its own, which the published crate does not have. Its programs
are those of `protocols/benches/` of
[oxirush-aper](https://github.com/linouxis9/oxirush-aper), in the checkout
beside this repository, which oxirush-ngap and oxirush-s1ap share; what is of
the protocol is in `src/` and `compare/` here.

The README sums up the comparison with the other implementations of S1AP,
and [The six PDUs](#the-six-pdus) has its figures. The figures below were taken on the same
machine, an Intel Xeon E5-2680 v4 (Broadwell, 2.4 GHz; a KVM guest bound to
NUMA node 0; Linux 7.0, Rust 1.100.0), on 2026-11-15, at 24ee03c.

## The six PDUs

The comparison of the [README](../README.md#performance), on the six PDUs of `compare/pdus.tsv`, which each implementation decodes and
encodes. The envelope alone is `S1APPDU::decode`, which leaves the value of
each IE in its octets; "this crate" decodes the PDU and then every IE of its
message as its type, which is what the others do; and encoding writes the PDU
from the typed values of its IEs, as `build_s1ap!` does.

Decoding, ns per call:

| PDU (octets) | this crate, the envelope alone | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|---|
| DownlinkNASTransport (73) | 30 | 185 | 1,100 | 1,654 | 3,633 | 124,940 |
| InitialContextSetupRequest (197) | 30 | 464 | 3,484 | 6,226 | 10,326 | 406,374 |
| S1SetupResponse (45) | 30 | 470 | 1,892 | 2,842 | 4,231 | 175,564 |
| HandoverRequest (146) | 31 | 552 | 4,224 | 6,508 | 11,122 | 472,698 |
| UEContextReleaseCommand (21) | 30 | 224 | 952 | 1,240 | 2,716 | 120,403 |
| InitialUEMessage (48) | 30 | 274 | 2,208 | 2,706 | 5,057 | 215,092 |

The time of each other implementation over this crate's, decoding:

| PDU (octets) | rasn / this crate | hampi / this crate | asn1c / this crate | pycrate / this crate |
|---|---|---|---|---|
| DownlinkNASTransport (73) | 5.9x | 8.9x | 20x | 680x |
| InitialContextSetupRequest (197) | 7.5x | 13x | 22x | 880x |
| S1SetupResponse (45) | 4.0x | 6.0x | 9.0x | 370x |
| HandoverRequest (146) | 7.7x | 12x | 20x | 860x |
| UEContextReleaseCommand (21) | 4.3x | 5.5x | 12x | 540x |
| InitialUEMessage (48) | 8.1x | 9.9x | 18x | 780x |

Encoding, ns per call:

| PDU (octets) | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|
| DownlinkNASTransport (73) | 161 | 2,222 | 2,576 | 2,492 | 81,502 |
| InitialContextSetupRequest (197) | 334 | 4,316 | 7,370 | 6,390 | 257,612 |
| S1SetupResponse (45) | 206 | 2,598 | 2,834 | 2,258 | 112,376 |
| HandoverRequest (146) | 385 | 5,350 | 7,838 | 6,560 | 293,377 |
| UEContextReleaseCommand (21) | 170 | 1,874 | 1,970 | 1,628 | 79,536 |
| InitialUEMessage (48) | 204 | 2,992 | 3,574 | 2,902 | 129,785 |

The time of each other implementation over this crate's, encoding:

| PDU (octets) | rasn / this crate | hampi / this crate | asn1c / this crate | pycrate / this crate |
|---|---|---|---|---|
| DownlinkNASTransport (73) | 14x | 16x | 16x | 510x |
| InitialContextSetupRequest (197) | 13x | 22x | 19x | 770x |
| S1SetupResponse (45) | 13x | 14x | 11x | 550x |
| HandoverRequest (146) | 14x | 20x | 17x | 760x |
| UEContextReleaseCommand (21) | 11x | 12x | 9.6x | 470x |
| InitialUEMessage (48) | 15x | 18x | 14x | 640x |

The figures were taken at 24ee03c, by [the method of the comparison](#the-method-of-the-comparison).

## Every figure of the comparison for InitialContextSetupRequest

The programs, the machine and the method of [the six PDUs](#the-six-pdus), per call:

|  | this crate | rasn 0.29 | hampi 0.7.2 | asn1c (Open5GS 2.8.0) | pycrate 0.8.1 |
|---|---|---|---|---|---|
| decode ns | 464 | 3,484 | 6,226 | 10,326 | 406,374 |
| decode cycles | 1,350 | 10,352 | 17,984 | 29,530 | 1,153,508 |
| decode instructions | 3,395 | 22,013 | 47,296 | 81,737 | 2,091,056 |
| decode allocations | 2 | 20 | 37 | 79 |  |
| decode octets allocated | 256 | 1,074 | 3,569 | 10,046 |  |
| encode ns | 334 | 4,316 | 7,370 | 6,390 | 257,612 |
| encode cycles | 964 | 12,334 | 21,770 | 18,496 | 746,479 |
| encode instructions | 2,361 | 26,513 | 64,129 | 50,503 | 1,279,096 |
| encode allocations | 2 | 33 | 17 | 25 |  |
| encode octets allocated | 456 | 1,698 | 1,797 | 3,728 |  |
| most memory held, KiB | 3,836 | 4,050 | 3,770 | 4,606 | 27,462 |

The time, the cycles and the instructions of each other implementation over this crate's:

|  | rasn / this crate | hampi / this crate | asn1c / this crate | pycrate / this crate |
|---|---|---|---|---|
| decode ns | 7.5x | 13x | 22x | 880x |
| decode cycles | 7.7x | 13x | 22x | 850x |
| decode instructions | 6.5x | 14x | 24x | 620x |
| encode ns | 13x | 22x | 19x | 770x |
| encode cycles | 13x | 23x | 19x | 770x |
| encode instructions | 11x | 27x | 21x | 540x |

## The method of the comparison

Every operation of every implementation ran once a round, a call repeated
for about 0.2 s, in turns, ten rounds (ten runs of the script with
`--rounds 1`): a time is the median of the ten, and a ratio of times is given
where the ten times of the two do not overlap (they overlap for none here).
The cycles and the instructions, the median of the rounds, are those that
`perf stat` counts for the calls alone: the difference between a run of
twice the calls and one of the calls.
The allocations and the octets that they ask for are counted by an allocator
that counts in Rust, by `malloc`, `calloc` and `realloc` for asn1c; pycrate's
are not counted. The most memory held is the high-water mark of the resident
memory of the program (`VmHWM`), its runtime included. The figures were taken
at 24ee03c on an Intel Xeon E5-2680 v4 (Broadwell, 2.4 GHz; a KVM guest bound
to NUMA node 0; Linux 7.0, Rust 1.100.0, Python 3.14), each program on core 3,
on 2026-11-15, with the script, which fetches each implementation at a
pinned version: hampi 0.7.2 and rasn 0.29.0 from crates.io, Open5GS v2.8.0
from GitHub, pycrate 0.8.1 from PyPI, and the ASN.1 from the archive of 3GPP.
From the root of the repository, with oxirush-aper beside it:

```sh
python3 ../oxirush-aper/protocols/benches/compare/compare.py --core 3 --perf "sudo -n perf"
```

## Reading the IEs, and checking a PDU

Reading the IEs of a message and checking a PDU, per call, on the four
messages that `benches/` writes: `decode_value()` and then the macro on the
message, which decodes the container of the message first; the macro on the
PDU (`extract_s1ap_ies!(pdu, ...)`), which walks the octets of its message once and
decodes the IEs that its fields ask for; and `ie::<T>()` of the PDU, which
decodes one IE. In `benches/`, `cargo run --release --bin readers --features
readers` measures them:

| Message | The first IE: `decode_value()`, then the macro | Allocations | The macro on the PDU | Allocations | `ie::<T>()` | Allocations | Every IE: `decode_value()`, then the macro | Allocations | The macro on the PDU | Allocations |
|---|---|---|---|---|---|---|---|---|---|---|
| An INITIAL CONTEXT SETUP REQUEST of 8 E-RABs | 186 ns | 1 | 63 ns | 0 | 48 ns | 0 | 676 ns | 2 | 518 ns | 1 |
| An E-RAB SETUP REQUEST of 8 E-RABs | 132 ns | 1 | 47 ns | 0 | 48 ns | 0 | 371 ns | 2 | 274 ns | 1 |
| A HANDOVER REQUIRED of a container of 4,000 octets | 158 ns | 1 | 54 ns | 0 | 48 ns | 0 | 454 ns | 1 | 320 ns | 0 |
| A PAGING of 16 tracking areas | 152 ns | 1 | 54 ns | 0 | 49 ns | 0 | 509 ns | 2 | 399 ns | 1 |

`items::<L>()` walks the eight E-RABs of the E-RAB SETUP REQUEST in the octets
of the PDU in 1,396 ns without an allocation, where `decode_value()`, the
macro and then the decoding of each item take 1,301 ns and 2 allocations:
the time is that of decoding the items.

`check()`, which decodes the value of each IE, and `check_structure()`, which
compares the container alone with the object set of its message, on the PDUs
of the comparison (`cargo run --release --bin checks` in `benches/`):

| PDU (octets) | `check()` | Allocations | `check_structure()` | Allocations | `check()` / `check_structure()` |
|---|---|---|---|---|---|
| DownlinkNASTransport (73) | 862 ns | 7 | 294 ns | 3 | 2.9x |
| InitialContextSetupRequest (197) | 1,996 ns | 13 | 410 ns | 3 | 4.9x |
| S1SetupResponse (45) | 1,178 ns | 12 | 262 ns | 3 | 4.5x |
| HandoverRequest (146) | 2,540 ns | 17 | 440 ns | 3 | 5.8x |
| UEContextReleaseCommand (21) | 655 ns | 5 | 234 ns | 3 | 2.8x |
| InitialUEMessage (48) | 1,195 ns | 8 | 330 ns | 3 | 3.6x |

Each program ran on one core, and each figure is the median of ten runs.

A clean `cargo build -j 5` of the crate and its dependencies on Rust 1.88, on
five cores of the same machine, the median of five builds, with the debug
information of the default profile and without it:

| Build | Time | Peak memory | Peak memory, `CARGO_PROFILE_DEV_DEBUG=0` |
|-------|------|-------------|------------------------------------------|
| No feature | 10 s | 0.63 GiB | 0.56 GiB |
| `inspect` | 20 s | 0.95 GiB | 0.80 GiB |
| `inspect`, `--release` | 33 s | 0.89 GiB | the same |

On a message of 65,535 IEs of one name, 393 KB, `decode` takes 0.04 ms, `check`
13.4 ms, `to_tree` 27 ms and 9.4 MB, `from_tree` 11.9 ms, `inspect::paths`
29 ms, and an edit at a path 0.011 ms, on one core of the same machine, the
median of ten runs. In `benches/`, `cargo bench --bench scale`
measures them, and `cargo bench --bench codec` and `--bench allocations` the
codec on the PDUs of the tests; from the root of the repository,
`python3 ../oxirush-aper/protocols/benches/build_cost.py +1.88.0 --features inspect`
measures a clean build.
