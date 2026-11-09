# Changelog

All notable changes to `oxirush-s1ap` are recorded here.

## 0.5.0 - 2026-11-09

A path of `inspect` is the names of the tree joined: `value` after the name of an
IE is refused, with the path to write. On oxirush-aper 0.3.0 exactly.

### Added

- `Deviation::rejects` and `check::answer` of `aper::check`, with oxirush-aper
  0.3.0: whether a receiver rejects a message for a deviation that `check()`
  reports, and the protocol cause of its answer to all of them, by the clause of
  the criticality.
- `TimeToWait::seconds`, the time that a Time to wait says to wait at least
  (TS 36.413 clause 9.2.1.61), beside the helpers.

### Changed

- A path of `inspect` is the names of the tree joined, with oxirush-aper 0.3.0:
  `value` after the name of an IE, or after its position, where its value has no
  member of that name, is refused by every function that takes a path, with the
  path to write: ``a path is the tree's names joined: write
  `/s1ap/MME-UE-S1AP-ID`, at /s1ap/MME-UE-S1AP-ID/value``. The `value` of a bit
  string whose size varies, beside its `length`, stays.

## 0.4.1 - 2026-11-08

### Fixed

- Each path that `inspect::paths` lists selects its value, with oxirush-aper
  0.2.1: an IE that a value holds alone and that has no name goes by its
  identifier in digits after the member that holds it, as the tree writes it,
  and an entry of a list of IEs, or of the IEs of a transfer in the place of the
  value of an IE, that holds that same IE is listed by its position, as the name
  of the IE after it stays on the IE. The fuzzing of 0.4.0 found the second in
  an `E-RABSetupListCtxtSURes` that holds an `E-RABSetupListCtxtSURes`, which
  `tests/fixtures/fuzzed.tsv` has now.
- `set_protocol_ies()` keeps the extension additions that a later version of the
  type of a message adds after its container, as they were, with oxirush-aper
  0.2.1, and `check()` and `check_structure()` report them as
  `Deviation::ExtensionAdditions`: it wrote the message without them. The
  nightly fuzzing of 0.4.0 found a RESET ACKNOWLEDGE with some, which
  `tests/fixtures/fuzzed.tsv` has now.

## 0.4.0 - 2026-11-08

### Changed

- A PDU as JSON is its tree alone: `to_tree` gives it and `from_tree` writes it, and
  the paths of `inspect` read and edit it. A path names an IE and selects its value,
  as the paths of a NAS message do: `/s1ap/MME-UE-S1AP-ID`, `/s1ap/Cause/radioNetwork`;
  the IEs of a transfer and of a list of single containers go by name in their place,
  and `criticality` after the value of an IE is the criticality beside it, which a tree
  writes where the message does not assign it. A tree has an IE that it cannot show as
  a value, or whose value has no place for all of its octets, as `{"octets": "…"}`,
  so `from_tree` writes the same octets for every IE that an edit leaves.
- The codec of each member of a `SEQUENCE` is one line in the bindings, which
  are about a sixth shorter, and a build takes less memory.
- The trees of the bindings read the names of their members and values in the
  inspection tables, which write each once: the bindings are 1259 lines shorter.
- `Criticality` is the one of oxirush-aper, which an error and a deviation have
  too: its values are `Criticality::Reject`, `Ignore` and `Notify`.
- The direction of a PDU is `Direction`: `pdu.direction()`, `Message::DIRECTION`,
  `S1apPduKind::Other` and `Deviation::UnknownProcedure` have it. It prints as
  before, `InitiatingMessage`.
- The trees and the identities are those of oxirush-utils 0.2.0: a tree is
  compared with a tree, as `tree!("ignore")`, and no longer with a string or a
  number.

- `build_s1ap!` takes the message alone, and the criticality of the PDU before it
  where it is not that of its procedure: `build_s1ap!(IGNORE UEContextReleaseRequest,
  ...)`. A variant of `S1apPduKind` is the name of the type of its message.
- `message.set_ie::<ie::X>(value)` changes an IE, and `ie::X::field(criticality,
  value)` and `ie::X::field_of::<Message>(value)` build one, with the trait `Ie`,
  which the crate re-exports, in scope.
- `S1ap::object_sets()`, `S1ap::object_set(direction, code)` and
  `S1ap::object_set_named(name)` give what the ASN.1 says of each message: its
  direction, its code, its name, the criticality of its procedure and its IEs
  with their criticalities and their `Presence`. `S1ap::ie_names()` and
  `S1ap::item_criticality(id)` give the names of the IEs and the criticality of
  an IE in a single container.
- A value that a hand-written codec reads or writes is read and written with
  the methods of oxirush-aper 0.2, whose constraint says whether it is
  extensible: `reader.unsigned(0, 255, false)`. An index of a `CHOICE` or an
  `ENUMERATED` that the type does not have is refused as `the type has no
  index 7`.
- No `SEQUENCE` has a `new`: a struct literal builds one, with its members by
  their names. The `new` of a bit string of one size, as `AMFSetID::new(1)?`,
  stays.
- A PDU decodes one way: `S1APPDU::decode` borrows the octets of its message
  from what it is given, and `into_owned()` gives the `S1APPDU<'static>` that
  is kept after them. `decode_value` is the one way to its message, which
  borrows from the PDU: `into_owned()` keeps it too.
- `oxirush_s1ap::decode` and `encode`, those of oxirush-aper, read and write any
  value of the bindings in its open type; `encode` gives the octets as a
  `Vec<u8>`.
- The methods of a PDU and its `Display` are those of oxirush-aper, written
  once for both protocols, with the same documentation.
- `decode` and `encode` of a PDU, and `set_protocol_ies`, are the codecs of the
  types: a PDU is read and written by one implementation.
- An error of a path or of a tree is of the kind `Path` or `Value`, those of
  oxirush-aper 0.2, and the functions that make an error are hidden.
- An IE has one name in the macros and in `ie`: the one after its identifier.
  The 106 IEs that were also named after their type no longer are.
- The documentation of a PDU cites TS 36.413 clause 9.3.0 for the PDU that a
  sender constructs, in each place.
- `helpers::plmn` is `Plmn::new` of oxirush-utils, whose errors it gives, and
  `eutran_cgi` takes the eNB ID, whose alternative has its bits:
  `eutran_cgi(plmn, &ENBID::macroENB_ID(Bits::new(id, 20)?), cell)`.

### Removed

- The oracle of the tests, rasn, which the benchmarks alone keep: the tests of
  `oracle/` (`compare`, `codec` and `allocation`), the list of its types
  `oracle/src/types.rs`, the comparison of the fuzz targets with rasn, and the
  hooks that the oracle took. The bindings for rasn are in `rasn/`, a package of
  its own that the benchmarks build and the crate does not; the vectors of
  pycrate, the captures and the comparison with Wireshark stay.
- The view of a PDU: `to_view`, `from_view` and `with_view`, with the paths of
  views (`/s1ap/X/value`, `octets` of an IE that has a value, `_raw_value`,
  `_edited`, `_decode_error`, `_lossy`, `decoded`, entries with an `id`). A tree and
  its paths do what a program did with a view.
  There is no `with_tree`, as there is none in oxirush-nas: `from_tree` writes the
  octets that each IE was received as for every IE that an edit leaves, which is
  what `with_view` kept.
- The tests of what the generated crates share, which oxirush-ngap and
  oxirush-s1ap held twice: they are those of TOY, the protocol that the generator
  of oxirush-aper writes from its own ASN.1 (`protocols/toy/` there). The crate
  keeps the tests of its fixtures, captures and vectors, its forms, its helpers,
  its keys, its registry, its handover containers and its feature `serde`.
- The fuzz target `view`, which is `json` and reads trees.
- `with_s1ap_ie_mut!` and `build_s1ap_ie!`.
- The lookups of `inspect`: `ie_names`, `message_names`, `message_name`,
  `message_named`, `message_ies`, `message_ie_criticalities`,
  `message_criticality` and `item_criticality`.
- `is_initiating()`, `is_successful()` and `is_unsuccessful()`: compare
  `direction()` with a `Direction`.
- `build/fields.tsv` and the hidden macros `__s1ap_ie_id!` and `__s1ap_proc_code!`.
- `S1AP_PDU`, which is `S1APPDU<'static>`; `decode_borrowed`, which is `decode`;
  `message()`; `s1ap::decode_open_type` and `s1ap::encode_open_type`; and
  `src/pdu.rs`.
- `encode_into` of a PDU: `oxirush_s1ap::aper::encode_into(&pdu, &mut octets)`
  keeps a buffer from one PDU to the next.
- The aliases of the types of the two identifiers of a UE, as
  `s1ap::MME_UE_S1AP_ID` for `MMEUES1APID`, and the name `ENB_UE_S1AP_ID` of the IE
  `eNB_UE_S1AP_ID` in the macros.
- The examples in the documentation of a few generated types: the README has
  each task.
- `procedure_name()`: `kind()` names the message, and `Display` shows it.
- `VERSION`, which is `env!("CARGO_PKG_VERSION")`.
- `helpers::plmn_from`, which `Plmn::try_from(identity)` is; `EnbKind`, which the
  alternatives of `ENBID` are, `enb_id` and `global_enb_id`.

### Migration

| Before | Now |
|--------|-----|
| `pdu.to_view()?` | `pdu.to_tree()` |
| `S1APPDU::from_view(&view)?`, `pdu.with_view(&view)?` | `S1APPDU::from_tree(&tree)?` |
| `/s1ap/MME-UE-S1AP-ID/value`, `/s1ap/Cause/value/nas` | `/s1ap/MME-UE-S1AP-ID`, `/s1ap/Cause/nas` |
| `/s1ap/X/criticality` of a view | `/s1ap/X/criticality`, there where the message does not assign it |
| `{"id": "X", "criticality": "c", "value": v}` inserted | `{"X": v, "criticality": "c"}`, the criticality where the message does not assign it |
| `/s1ap/X/octets` set | `/s1ap/X` set to `{"octets": "…"}` |
| a transfer's `decoded` | the transfer itself |
| `inspect::__view_of_jer(&values)` | `inspect::__tree_of_jer(&values)` |
| `Criticality::reject` | `Criticality::Reject` |
| `pdu.is_initiating()` | `pdu.direction() == Direction::Initiating` |
| `pdu.direction() == "SuccessfulOutcome"` | `pdu.direction() == Direction::Successful` |
| `S1AP_PDU::decode(&octets)?` | `S1APPDU::decode(&octets)?`, and `.into_owned()` to keep it |
| `S1AP_PDU::decode_borrowed(&octets)?` | `S1APPDU::decode(&octets)?` |
| `pdu.message::<M>()?` | `pdu.decode_value::<M>()?`, and `.into_owned()` to keep it |
| `decode_open_type::<T>(&octets)`, `encode_open_type(&value)` | `oxirush_s1ap::decode::<T>(&octets)`, `oxirush_s1ap::encode(&value)` (a `Vec<u8>`: `.into()` gives `Octets`) |
| `MME_UE_S1AP_ID(42)` (the type) | `MMEUES1APID(42)` |
| `ErrorKind::NoSuchMember`, `NothingSelected`, `NotOpened` | `ErrorKind::Path` |
| `ErrorKind::Form`, `Edit`, `Limit` | `ErrorKind::Value` (`Path` for the limits of a path) |
| `plmn_from(&identity)` | `Plmn::try_from(identity)`, its `mcc()` and its `mnc()` |
| `global_enb_id(plmn, EnbKind::Macro, id)` | `GlobalENBID { plmn_identity: plmn, enb_id: ENBID::macroENB_ID(Bits::new(id, 20)?), ie_extensions: None }` |
| `eutran_cgi(plmn, EnbKind::Macro, id, cell)` | `eutran_cgi(plmn, &ENBID::macroENB_ID(Bits::new(id, 20)?), cell)` |
| `TAI::new(plmn, tac, None)` | `TAI { plmn_identity: plmn, tac, ie_extensions: None }` |
| `inspect::message_named(name)` gives `("InitiatingMessage", code)` | it gives `(Direction::Initiating, code)` |
| `tree["message"] == "..."` | `tree["message"] == tree!("...")` (oxirush-utils 0.2.0) |
| `build_s1ap!(InitiatingMessage, UEContextReleaseRequest, UEContextReleaseRequest, ...)` | `build_s1ap!(UEContextReleaseRequest, ...)` |
| `build_s1ap!(SuccessfulOutcome, Proc, IGNORE, Response, ...)` | `build_s1ap!(IGNORE Response, ...)` |
| `S1apPduKind::Initiating_Proc` | `S1apPduKind::` and the name of the type of the message |
| `with_s1ap_ie_mut!(m, M, X(id) = v)` | `m.set_ie::<ie::X>(v)` |
| `build_s1ap_ie!(REJECT X(v))` | `ie::X::field(Criticality::Reject, v)` |
| `build_s1ap_ie!(Message, X(v))` | `ie::X::field_of::<Message>(v)` |
| `inspect::message_ies(name)` | `S1ap::object_set_named(name)`, and its `ies` |
| `inspect::message_names()`, `message_named(name)` | `S1ap::object_sets()`, `S1ap::object_set_named(name)` |
| `inspect::message_name(&pdu)` | `S1ap::object_set(pdu.direction(), pdu.procedure_code())`, and its `name` |

### Added

- At a handover to 5GS, the transparent containers carry those of TS 38.413
  (TS 36.413 clauses 9.2.1.56 and 9.2.1.57): a tree writes them by the name
  of their type, `SourceNGRANNode-ToTargetNGRANNode-TransparentContainer` and
  `TargetNGRANNode-ToSourceNGRANNode-TransparentContainer`, and a view opens,
  reads and edits them by the tables of oxirush-ngap, once a program calls
  `oxirush_ngap::inspect::register()`. `inspect::register()` registers the
  tables of S1AP for oxirush-ngap in the same way. Without the call, the name
  is refused with the call to make. The feature `inspect` turns on the
  register of oxirush-aper, which needs the standard library.
- The keywords take `epc` and `mme` in the place of `asn1` and `telecom`, and the description says 4G LTE/EPS, eNB and MME, as those who look for the crate search.

## 0.3.0 - 2026-10-24

The bindings come from TS 36.413 V19.2.0 (Release 19) instead of V16.6.0, and
their codec is [`oxirush-aper`](https://github.com/linouxis9/oxirush-aper),
aligned PER by ITU-T X.691, where 0.2.0 derived the codec of rasn 0.28. It is
not source compatible with 0.2.0.

### Breaking changes relative to 0.2.0

- `Display` of a PDU is the name of its message, its procedure code and its
  direction, then the identifiers of the UE that it has, by the names of
  their IEs: `UEContextReleaseRequest (procedure 18, initiating message),
  MME-UE-S1AP-ID 42, eNB-UE-S1AP-ID 7`, where 0.2.0 printed
  `InitiatingMessage UEContextReleaseRequest (code=18)`. The identifiers that
  UE-S1AP-IDs holds, as in a UE CONTEXT RELEASE COMMAND, are shown alike: the
  pair, or the MME-UE-S1AP-ID alone. `Debug` prints octets in hexadecimal in
  lower case.
- The crate no longer depends on `rasn`, and `oxirush_s1ap::rasn` is gone.
  The bindings implement `AsnType`, `Decode` and `Encode` of
  `oxirush-aper`, which the crate re-exports as `oxirush_s1ap::aper`, and
  have aligned PER alone.
- A field has the words of the ASN.1 identifier of its member: `p_lmnidentity`
  is `plmn_identity`, `i_e_extensions` is `ie_extensions`, `e_nb_id` is
  `enb_id`, `m_me_code` is `mme_code`, `g_tp_teid` is `gtp_teid`. `build/fields.tsv` of the
  repository has each field with its name in 0.2.0.
- The values of the bindings are native numbers, arrays and the types of
  `oxirush-aper`:

  | 0.2.0 | Now | To migrate |
  | --- | --- | --- |
  | A native number, for an `INTEGER` whose constraint has no extension marker | The same number | As before |
  | `Integer`, for an `INTEGER` whose constraint has an extension marker | `u64`, or `i64` for a type whose root goes below zero | `ERABID(n)` for `(0..15, ...)`, where `n` is a `u64`: no `.into()`, and no `Integer` to convert. A number that 64 bits do not hold does not decode: see below |
  | `FixedOctetString<N>` | `[u8; N]` | `TBCDSTRING([0x02, 0xf8, 0x39])` |
  | `OctetString`, `Any` | `Octets<'a>` | `Octets::from(vec)`, `Octets::from(&slice)`, or `.into()` of either. It derefs to `[u8]`: `.as_bytes()` is gone |
  | `FixedBitString<N>` and `BitString` of 64 bits at most, without an extension marker | `Bits`, a number with a length | `CellIdentity::new(0x1234507)?`, which a type of one size has, or `CellIdentity(Bits::new(0x1234507, 28)?)`, `bits.value()` and `bits.len()` |
  | Another `BitString` | `BitString<'a>` | `BitString::from([0xe0, 0x00])`, `BitString::new(octets, length)?`, `.as_octets()` and `.len()` |
  | `PrintableString`, `VisibleString`, `Utf8String` | `Cow<'a, str>` | `MMEname("mme-1".into())`, and `&name.0` is a `&str` |
  | `SequenceOf<T>` | `Vec<T>` | As before |
  | The `LazyLock<Integer>` statics of the value assignments, as `MAXNOOF…` | `pub const … : u64` | The name is the same |

- A type that holds octets has a lifetime, as `NASPDU<'a>`, and borrows them
  from what it was decoded from: `into_owned()` gives the `'static` value to
  keep. A type that holds none, as `Cause`, `TAI` or `GUMMEI`, has no
  lifetime, as the extension container of a value owns its octets.
  `S1AP_PDU` is `S1APPDU<'static>`, and a pattern on a PDU that borrows names
  `S1APPDU`. The message of `decode_value` borrows from the PDU, which is a
  variable of its own: `let pdu = S1AP_PDU::decode(&octets)?;`.
- `decode_value::<T>()` takes the type of a message, and refuses a PDU that
  holds another one (`WrongMessage`): 0.2.0 decoded the octets as any type.
- `decode_open_type` takes `&[u8]` and `encode_open_type` gives `Octets`.
  The `value` of an IE, of an extension and of a PDU is an `Octets`, where it
  was an `Any`: `&ie.value` is what `ie.value.as_bytes()` was.
- One error type, `oxirush_s1ap::Error`, whose `kind()` is matched on, for
  every call and every macro, where they returned the errors of rasn.
  `build_s1ap!` and `build_s1ap_ie!` give a `Result`, where they panicked
  for a value that does not encode. `with_s1ap_ie_mut!` gives a
  `Result<bool, Error>`. `extract_s1ap_ies!` returns an `Error` from the
  enclosing function, and `MissingIeError` is gone: a `req` field whose IE
  is missing is `MissingIe`, and an IE that does not decode is
  `UndecodableIe` for a `req` and for an `opt` field, where 0.2.0 called it
  missing or `None`. A field that is to go on without an IE that does not
  decode is `lax`.
- When a message has an IE twice, `extract_s1ap_ies!` and `with_s1ap_ie_mut!`
  take the first one, and do not decode the second; `check()` reports it as
  `Deviation::RepeatedIe`, and `ies::<T>()` gives each.
- The macros, the readers, `check()` and the registry are those of
  `oxirush_aper::protocol`, which oxirush-ngap and oxirush-s1ap share:
  `Message` is `oxirush_aper::protocol::Message`, re-exported, with its
  `Protocol`, and the hidden `registry::ies::NAME` modules are the marker
  types of the module `ie`: `ie::NAME::ID` is `<ie::NAME as Ie>::ID`.
- Each protocol IE and extension container is one type, `ProtocolIEContainer`
  of `ProtocolIEField` and `ProtocolExtensionContainer` of
  `ProtocolExtensionField`, in place of one for each use, as
  `InitialUEMessageProtocolIEs`. The `id` of a field is a `ProtocolIEID` or
  a `ProtocolExtensionID`.
- The macros name an IE by its identifier, and a type name stands for the IE
  of that type only where one IE has it, where 0.2.0's `GUMMEI` built
  id-GUMMEI-ID in a PATH SWITCH REQUEST, which carries id-SourceMME-GUMMEI.
  `build_s1ap!` takes the message that the procedure has in that direction,
  and a criticality that a call leaves out is the one that the specification
  assigns: that of the procedure for the PDU (`Message::CRITICALITY`), and that
  of the object set of the message for an IE, which an IE that the set does
  not have has not, an error of the kind `Argument`.
- No helper panics, and none cuts a number short. `plmn`, `eutran_cgi` and
  `global_enb_id` return a `Result` with an error of the kind `Argument`;
  `tai` takes `[u8; 2]`, `ue_security_capabilities` its two
  octets, and `plmn_from` returns an `Option`. `int_to_bitvec` and
  `bytes_to_bitvec` are gone: `Bits::new(value, width)?` and
  `BitString::from(octets)` take their place.
- An `INTEGER` is held in 64 bits. A larger number, which only an extension
  marker lets a later release send, is an error of its IE, and the rest of the
  PDU is read.
- `ECGI-List`, of at most 256 cells, is `ECGI_List`, where it was one type
  with `ECGIList`, of at most 65535: the cell list of a warning request had a
  count of one octet.
- The variants of `S1apPduKind` are in the order of the procedure codes, and `Other` has the `direction` that `direction()`
  gives, as `"UnsuccessfulOutcome"`.
- `PLMNidentity` prints as its MCC and its MNC, `PLMNidentity(208-93)`, and a
  type of a fixed number of octets prints them in hexadecimal,
  `TAC(000001)`. `Debug` prints the length of a key, not its octets.
- `helpers::plmn` writes another order for an MNC of three digits: see Fixed.
- The examples are named after the protocol: `s1ap_build_pdu`.
- The minimum supported Rust version is 1.88: 0.2.0 declared 1.85.
- The crate depends on one version of `oxirush-aper` exactly, `=0.1.0`: it
  builds on the module `inspect` of `oxirush-aper`, which is hidden and changes
  with the two protocol crates, and the three are released in step.
- `MDTModeExtension`, `SONInformationExtension` and `SourceNodeIDExtension`, the
  single containers of the extensions of three CHOICE types, are gone: the
  alternative holds a `ProtocolIEField`, as every single container does.
- `global_enb_id` and `eutran_cgi` take the kind of the eNB ID, `EnbKind`:
  macro (20 bits), home (28), short macro (18) or long macro (21), where 0.2.0
  had the macro eNB ID alone, and `eutran_cgi` takes the cell in the bits after
  it as a `u16`.

### Added

- `ie`: a marker type for each IE, by the names that the macros take it by,
  and `ies::<T>()`, `ie::<T>()` and `items::<L>()` of a PDU, of a message and
  of a transfer: every occurrence of an IE in order, decoded alone, the first,
  and the items of a list of single containers, walked and not built. On a
  PDU they read the octets of its message where they are, without
  `decode_value` and without allocating.
- `extract_s1ap_ies!` takes a PDU as well as a message, as the message that it
  holds: it walks the IEs once and decodes those that a field asks for.
- `check_structure()` of a PDU: the object set of its message, the presence,
  the order, the repetition and the criticality of its IEs, without decoding a
  value, for a receiver that reads the IEs that it needs; `check()` decodes
  every value too.
- `pdu.check()`: how a PDU deviates from the object set of its message, as
  `Deviation`s, refusing nothing. `deviation.cause()` and
  `deviation.diagnostics_item()` give what a receiver answers, as TS 36.413
  clause 10 has it, `pdu.criticality_diagnostics(&deviations)` the whole
  Criticality Diagnostics IE of an ERROR INDICATION, with the procedure code
  and the triggering message, `pdu.criticality_diagnostics_for_response(&deviations)`
  that of a response of the procedure, without them (clause
  9.2.1.21), `pdu.criticality()`, and `error.ie()` the IE of an
  error of the macros. A value outside the root of an extensible constraint
  and one in another form than the canonical one are reported too, and
  `aper::encode_strict` and `aper::decode_strict` refuse them. A procedure
  that the specification does not have is the deviation `UnknownProcedure`,
  which a receiver answers by the criticality of the PDU (clause 10.3.4.1).
- `pdu.protocol_ies()` and `pdu.set_protocol_ies(ies)`: the IEs of any
  message, and `S1AP_PDU::of_message(&message, criticality)`, the PDU of a
  message that is at hand.
- `S1AP_PDU::decode_borrowed`, which copies nothing, `pdu.message()`, which
  lives as long as what the PDU borrows, and `pdu.encode_into(&mut octets)`.
- A field of `extract_s1ap_ies!` of the type of the one value of its IE,
  `req id: u32 = MME_UE_S1AP_ID(id)`, and `build_s1ap_ie!` without the name
  of a message, for an IE that is in none.
- `oxirush_s1ap::Message`, which each message of a procedure implements.
- The `S1_Message` IE name, for the IE whose type is an inline `OCTET STRING`,
  and `ENB_UE_S1AP_ID`, the alias of the type of id-eNB-UE-S1AP-ID, as the
  name of that IE.
- `PLMNidentity` converts to and from the `Plmn` of `oxirush-utils`, which
  the crate has as `oxirush_s1ap::identity`.
- Each type of the bindings says in its documentation what it is in ASN.1:
  `MME-UE-S1AP-ID ::= INTEGER (0..4294967295)`, and what the comments of the ASN.1 say of it, without the
  banners of the sections of the modules. The pages of the macros show the
  forms that a caller writes, with an example of `=>`.
- The `inspect` feature: a PDU as JSON, the trees of `oxirush-utils`
  (`oxirush_s1ap::tree`), as its tree (`to_tree`, `from_tree`) and as its view
  (`to_view`, `with_view`, `from_view`), with the paths of the values of a
  view (`select`, `paths`, `set`, `insert`, `remove`, `open`, `check_path`,
  `enumerated_at`) and what the ASN.1 says of each message (`message_ies`,
  `message_ie_criticalities`, `item_criticality` and the others). A
  transfer and the container of a handover are read as their types, the
  well-known values as they are usually written, wherever they are, and what
  is not edited keeps the octets received. Every PDU that decodes has a
  tree, which is the same PDU again: a procedure that the protocol does not
  have by its procedure code and its direction, a message that has no list
  of IEs or does not decode by its octets, and an IE or a transfer whose
  typed value has no place for what a later release added by its octets. An
  alternative of the type `NULL` is `{"pLMNWide": null}`, whose `null` a
  path lists and selects. The trees and the paths are those of
  `oxirush-aper`, which oxirush-ngap shares, on `oxirush-utils`: octets in
  lower case, members in the order of the specification, and the JSON of the
  program that uses the crate as the program has it.
- The `serde` feature: `Serialize` and `Deserialize` through the trees, a
  PDU as the tree of its message and each other type as the tree of a PDU
  shows it, read back in it or in the form of its ASN.1 type. A name that an
  object has twice is refused.
- Tests on 101 messages and 284 vectors that pycrate 0.8.1 encoded, on 81
  PDUs that the MME of Open5GS sent, against the codec of
  rasn 0.29 in `oracle/`, with three fuzz targets, and from the published
  package alone. `build/asn1.sha256` records the digests of the ASN.1 modules
  that the bindings are generated from.
- The tools, the programs and the jobs that oxirush-ngap and oxirush-s1ap
  share are those of the repository of `oxirush-aper`, at the revision of
  `build/revisions`, from a checkout beside this repository: its
  `protocols/` has the oracle and its tests, the fuzz targets, the
  benchmarks, the scripts of the fixtures, of the package and of the
  revisions, and the writer of the test of the README, and its workflows
  the jobs of CI and of Reproduce. CONTRIBUTING.md says where each lives.
- `helpers::enb_id`, the eNB ID of a kind.
- In a tree and through serde, a string of decimal digits is never read as
  octets: a PLMN identity or a number written so (`"310410"`, a TAC
  `"123456"`) is refused with the two values that it could be, and octets whose
  hexadecimal has no letter are written after `0x`. A YAML `true` is the name
  `true` of an ENUMERATED that has it. A member of a tree written twice in two
  spellings is refused. A refusal of a key shows nothing of what is written,
  and a misspelled name of an IE or of a message is refused with the nearest
  names.

### Changed

- The codec reads what a peer may send as rasn read it where the value is the
  same, as padding bits that are not zero or a length in a longer form than
  it needs, and refuses a number of no octet.
- The crate compiles to less code: a type reads and writes its fields with
  the functions of `oxirush-aper`, compiled once, where the derived codec of
  rasn was compiled for each type in each of its encoding rules. It no longer
  depends on `paste`.

### Fixed

- A view and a tree read the value of an IE, and a transfer, alike. Octets in
  another form than the one of X.691, as padding bits that are not zero, are
  their octets with a `_decode_error` in a view, which a path list lists, where
  the view had the value that a tolerant reader gives and the tree the octets.
  A path list lists `_lossy`, where a typed value has no place for all its octets.
- The octets of a PLMN identity with an MNC of three digits. TS 36.413 clause
  9.2.3.8 has the digits of the MCC, then those of the MNC: 310-410 is
  `13 40 01`. `helpers::plmn` wrote `13 00 14`, the order of TS 24.008,
  which a decoder of S1AP reads as 310-041. An MNC of two digits has the same
  octets in both orders.
- Aligned PER of constrained `SEQUENCE OF` lists, among them the ten
  `E-RAB-IE-ContainerList` lists, which get back their bound of 256, of
  fixed-size `BIT STRING` values longer than 16 bits, of strings with a
  two-octet length, and of empty open types.
- Decoding skips the unknown additions of an extensible `SEQUENCE`, refuses
  incomplete values and trailing octets, and reserves no memory for a count
  that a length claims before its elements are read.

The earlier release is described by its tag.
