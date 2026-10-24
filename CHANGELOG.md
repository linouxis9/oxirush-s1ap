# Changelog

All notable changes to `oxirush-s1ap` are recorded here.

## Unreleased (0.3.0)

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
