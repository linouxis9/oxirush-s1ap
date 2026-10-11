# Changelog

All notable changes to `oxirush-s1ap` are recorded here.

## Unreleased (0.3.0)

The bindings come from TS 36.413 V19.2.0 (Release 19) instead of V16.6.0,
and the APER codec follows ITU-T X.691 where rasn 0.28 departs from it. It
is not source compatible with 0.2.0.

### Breaking changes relative to 0.2.0

- The bindings are generated from the Release 19 ASN.1 (`36413-j20`), with
  every procedure, IE and extension up to Release 19.
- Each protocol IE container, each extension container and their fields are
  one type: `ProtocolIEContainer` of `ProtocolIEField`, and
  `ProtocolExtensionContainer` of `ProtocolExtensionField`. The types that
  each use of one had, such as `InitialUEMessageProtocolIEs` and
  `AnonymousInitialUEMessageProtocolIEs`, are gone. The `id` of a field is a
  `ProtocolIEID` or a `ProtocolExtensionID`, where extension fields and the
  items of the single-container lists, such as `AnonymousERABList`, had a
  `u16`, and every `criticality` is a `Criticality`.
- `ECGI-List`, of at most 256 cells, is `ECGI_List`. It was one type with
  `ECGIList`, of at most 65535, so the cell list of a warning request had a
  one-octet count. This changes the type of `SynchronisationInformation`'s
  `aggressore_cgi_list`.
- The macros name each IE by its own identifier before any type alias, and a
  type name is an alias only for the one IE of that type: `GUMMEI` built
  id-GUMMEI-ID where a PATH SWITCH REQUEST carries id-SourceMME-GUMMEI.
- The minimum supported Rust version is 1.88, which rasn 0.28.15 needs:
  0.2.0 declared 1.85.
- `helpers::plmn` writes other octets for an MNC of three digits, and
  `helpers::plmn_from` reads them: see the first entry of Fixed.
- `helpers::plmn_from` returns an `Option<(String, String)>`, where it
  returned the two strings: `None` for octets that are not a PLMN identity,
  a half octet being no digit where it is not the filler before an MNC of
  two digits. `ab cd ef` was the MCC `111013` and the MNC `151412`.
- One error type, `oxirush_s1ap::Error`. `S1AP_PDU::decode`, `encode` and
  `decode_value`, `s1ap::decode_open_type` and `s1ap::encode_open_type` return
  it where they returned a `rasn::error::DecodeError` or an `EncodeError`,
  and so do the four macros. A function that returned one of those errors
  returns `Result<_, oxirush_s1ap::Error>`, or keeps a
  `Box<dyn std::error::Error>`. `error.kind()` is an `ErrorKind` to match on,
  and the message says what is wrong without the wording of rasn: `these
  octets do not decode as S1AP-PDU: the octets end before the value does`.
  The error of rasn is the `source()`, for code that read its `kind`:
  `source.downcast_ref::<rasn::error::DecodeError>()`. `Error` has a `From`
  of both, for code that calls `rasn::aper` itself.
- `decode_value::<T>()` takes the type of a message alone, and checks that
  the PDU holds that message: on another one the error has the kind
  `WrongMessage`, where 0.2.0 decoded the octets as whatever type was named,
  and a UE CONTEXT RELEASE REQUEST read as a setup request with three IEs.
  A type that is no message, as that of an IE, does not compile: decode it
  with `s1ap::decode_open_type::<T>(&any)`. Generic code that calls
  `decode_value` asks for `T: oxirush_s1ap::Message` where it asked for
  `T: rasn::Decode`.
- `build_s1ap!` gives a `Result<S1AP_PDU, Error>` and `build_s1ap_ie!` a
  `Result<ProtocolIEField, Error>`: write `?` after the call, or `.unwrap()`
  where the values are known to encode. A value that does not encode, as an
  integer outside its constraint, is an error of the kind `Encode`, where
  the macros panicked with `failed to APER-encode S1AP IE open type`.
- `extract_s1ap_ies!` returns an `Error` from the enclosing function, and
  `macros::MissingIeError` is gone. A `req` field whose IE is not in the
  message is an error of the kind `MissingIe`, whose message names the IE and
  the field, where `MissingIeError` had the name of the field as `ie_name`.
  An IE that is in the message and does not decode is an error of the kind
  `UndecodableIe`, for a `req` and for an `opt` field: 0.2.0 said that a
  `req` field was missing, and gave `None` for an `opt` one. The macro
  converts the error as `?` converts one, which 0.2.0 documented and did
  not do, so the enclosing function returns a `Result` whose error is
  `Error`, `Box<dyn std::error::Error>` or a type with a
  `From<oxirush_s1ap::Error>`, where it returned `MissingIeError` itself. A
  call with `opt` fields alone returns the error too: it no longer compiles
  in a function that returns nothing.
- `with_s1ap_ie_mut!` gives a `Result<bool, Error>` where it gave a `bool`:
  what the expression says, `Ok(false)` for an IE that is not in the message,
  and an error for an IE that does not decode (`UndecodableIe`) or a changed
  value that does not encode (`Encode`), which were both `false`. Write
  `with_s1ap_ie_mut!(…)?` where the `bool` was read.
- When a message has the same IE twice, `extract_s1ap_ies!` reads the last
  one and `with_s1ap_ie_mut!` changes the last one, and any of them that does
  not decode is an error for both. 0.2.0 read the last one that decoded,
  and changed the first one or, when that did not decode, nothing.
- No function of `helpers` panics, and none cuts a number short. Each of
  these returns a `Result` with an `Error` of the kind `Argument`, so a call
  takes a `?` or an `.unwrap()`:
  - `plmn(mcc, mnc) -> Result<PLMNidentity, Error>`, for an MCC that is not
    three digits or an MNC that is not two or three, where it panicked;
  - `int_to_bitvec(value, width) -> Result<BitString, Error>`, for a width
    over 64 bits, where it panicked, and for a value that does not fit in the
    width, whose high bits it dropped;
  - `eutran_cgi(…) -> Result<EUTRANCGI, Error>` and
    `global_enb_id(…) -> Result<GlobalENBID, Error>`, for a macro eNB ID over
    20 bits, which they masked: `global_enb_id(plmn, 0x1fffff)` built the
    eNB `0xfffff`.
- `helpers::tai` takes the TAC as `[u8; 2]`, where it took a slice and
  panicked unless it had two octets: `tai(plmn, [0, 1])`.
- `helpers::ue_security_capabilities(encryption, integrity)` takes the two
  octets, where it took a slice, read its first two octets and took a
  missing one for zero: `ue_security_capabilities(0xe0, 0xe0)`.

### Added

- The optional `inspect` feature: `pdu.to_view()` gives a decoded PDU as its
  view, a `serde_json` value in the ASN.1 JSON encoding, which is edited and
  which `pdu.with_view(&view)` encodes again; `S1AP_PDU::from_view(&view)`
  gives the PDU of any view, and `with_view` refuses the view of another
  message than the PDU, with an error of the kind `Form`. What is not edited
  keeps the octets received, and an IE is added by its typed value or by its
  octets. The two JSON forms are methods of the PDU, by the names that
  oxirush-nas has for them. Before the release, the revisions of this version
  had them as functions of `inspect`: `message_view`, `message_from_view`,
  `message_tree` and `message_from_tree`, and before that `inspect_pdu` and
  `encode_pdu`, which called the view a tree.
  A PLMN identity, a transport layer address, an IMSI and the usual
  identifiers read and write as they are usually written, and an `ENUMERATED`
  value is named in any case.
- With the `inspect` feature, the values of a view by their paths:
  `inspect::paths` gives each value with the path that selects it,
  `inspect::select` the values at a path, and `inspect::set`, `remove` and
  `insert` edit a view at a path. `/s1ap` stands for the IEs of the message,
  and an IE goes by its name, as in `/s1ap/eNB-UE-S1AP-ID/value`, by its
  position or by its identifier.
- With the `inspect` feature, `inspect::message_name` gives the name that
  ASN.1 has for the message of a PDU, such as `E-RABSetupRequest`,
  `inspect::message_named` the direction and the procedure code of the
  message of a name, in any case, and `inspect::message_names` all of them.
- The `S1_Message` IE name, for the one IE whose type is an inline
  `OCTET STRING`.
- The macros take `ENB_UE_S1AP_ID`, the alias that the crate gives the type
  of id-eNB-UE-S1AP-ID, wherever they take `eNB_UE_S1AP_ID`, the name of the
  IE: an alias of a type names the IE that the name of the type names. The
  alias did not compile as an IE name.
- The `sized` module: the `OCTET STRING` and `BIT STRING` types whose length
  determinant takes two octets.
- `oxirush_s1ap::rasn`, the `rasn` that the crate is built with: a crate that
  names the values of the bindings, such as `FixedBitString`, or the error
  of the codec that an `Error` has as its source, takes them from there,
  where it had to depend on the same `rasn` itself. The examples do.
- `oxirush_s1ap::Message`, which the type of each message of a procedure
  implements: its `NAME`, as ASN.1 gives it, its `DIRECTION` and its
  `PROCEDURE_CODE`.
- With the `inspect` feature, `inspect::check_path`: whether a path can select
  anything in a message of a name, without a view. The IE has to be one of
  the object set of the message, and each segment under it a member that the
  type of the value has. `src/inspect_registry.rs` lists what each type has,
  from the bindings: one line for each SEQUENCE and each CHOICE.
- With the `inspect` feature, `inspect::message_ie_criticalities` and
  `inspect::message_criticality`: the criticality that ASN.1 assigns to each
  IE of the object set of a message, and to the procedure of the message.
  The list of `src/registry.rs` has both on the line of each procedure.
- With the `inspect` feature, `inspect::item_criticality`: the criticality
  that ASN.1 assigns to an IE that a value holds alone, in a
  `ProtocolIE-SingleContainer`, by its identifier, when its object sets give
  it one. The list of `src/registry.rs` has it on the line of the IE.
- With the `inspect` feature, `inspect::enumerated_at` gives the names that
  the value at a path of a message can have when its type is an `ENUMERATED`,
  from the bindings, without a view: a caller can refuse a name that no
  message has before it compares one.
- With the `inspect` feature, the tree of a PDU: the short form, in which a
  message is shown and written as its name and its IEs by name.
  `S1AP_PDU::from_tree(&tree)` makes a PDU from `{"message": NAME, "ies":
  [{NAME: VALUE}, …]}`, with the procedure code, the direction and the
  criticalities that the specification assigns, and `pdu.to_tree()` shows a
  PDU in that form.
- With the `inspect` feature, `value` may be left out. In a path, a segment
  that names no member of an IE is taken in its value:
  `/s1ap/Cause/radioNetwork` is `/s1ap/Cause/value/radioNetwork`.
  `to_tree` shows each IE that a value holds as its name with its value, to
  any depth, as in `{"E-RABToBeSetupListBearerSUReq":
  [{"E-RABToBeSetupItemBearerSUReq": {"e-RAB-ID": 5, …}}]}`, and `from_tree`
  reads that form. What is written with the word is taken as it was.
- With the `inspect` feature, the content of a handover's transparent
  container, which TS 36.413 gives by reference where the ASN.1 has an
  `OCTET STRING`. The octets of `Source-ToTarget-TransparentContainer` and
  `Target-ToSource-TransparentContainer` are written by the name of the eNB
  type that they carry, as in `{"Source-ToTarget-TransparentContainer":
  {"SourceeNB-ToTargeteNB-TransparentContainer": {…}}}`, and a path goes
  through that name. `inspect::open` decodes, in a view, the octets that a
  path takes so, and nothing else does: a view shows them as octets until
  then, as nothing in a message says what they carry.
- With the `inspect` feature, a path may say which IE an entry is. After an
  entry of a list of IEs, or an IE that a value holds alone, the name of that
  IE stays on it, so the names and the positions that `to_tree` shows,
  joined, are the path of a value, as in
  `/s1ap/E-RABToBeSetupListCtxtSUReq/0/E-RABToBeSetupItemCtxtSUReq/e-RAB-ID`.
  The name of another IE there is an error that says which IE the entry is,
  and `inspect::check_path` reads what follows a position and a name against
  the type of that IE.
- With the `inspect` feature, every function that can fail returns the
  `Error` of the crate, and its kind says what a caller can do: `Path` for a
  path that is not well formed, `NoSuchMember` for a name that the type at
  its place cannot have, `NothingSelected` for an edit that selects nothing,
  `NotOpened` for a path through the type that octets carry before
  `inspect::open` decoded them, `Value` for a JSON value that is not a value
  of its ASN.1 type, `Form` for JSON that is no tree and no view, `Edit` for
  an edit that is refused, `Limit` for what is over the limits of the
  inspection, and `UnknownMessage` and `UnknownIe`. The message of a `Value`
  says at which member the value is wrong, by the names that the value has:
  `this is not a value of Cause, at radioNetwork: "no-such-cause" is no
  value of the ENUMERATED`. The revisions of this version before the release
  returned a `String`, which for such a value was the text of rasn.

### Changed

- The macros and the `inspect` feature read one generated list,
  `src/registry.rs`, with one line for each procedure and for each IE. An IE
  or a procedure that a macro does not know is reported by its name, with
  the names that are close to it, where the error named the first IE of the
  specification. The variants of `S1apPduKind` are in the order of the procedure
  codes, and each says its message.
- The crate no longer depends on `paste`: the macros paste no name.
- A call of `build_s1ap!` or `build_s1ap_ie!` that leaves out a criticality is
  a compile error that says which one: `the criticality of the IE … is
  missing: write REJECT, IGNORE or NOTIFY before its name`, where the compiler
  said ``no rules expected `(` ``. A word that is no criticality is named too.
- `build_s1ap!` takes the message that the procedure has in that direction, and
  the other macros a type that has IEs: another name does not compile, where
  it was not looked at. `S1apPduKind::Other` has the `direction` that
  `direction()` gives, as `"UnsuccessfulOutcome"`, where it had
  `"Unsuccessful"`.
- With the `inspect` feature, `inspect::message_ies`: the IEs that a message
  can have, as its ASN.1 object set lists them, each with its identifier and
  whether its presence is mandatory. The list of `src/registry.rs` has them
  on the line of each message.
- An edit at a path of an inspection view changes what is sent or is refused.
  `set` with `null` takes an optional member out, as its documentation said,
  where it wrote a `null` that did not encode. The value of an IE is not
  taken out: that edit returned without an error and sent the octets
  received. The `octets` of an IE whose value an edit changed are an error to
  select, and `*` leaves out the members that start with `_`.
- The error of an identifier that no object set has is `S1AP IE N is unknown`,
  where it said `is unknown or has several types`: no identifier of TS 36.413
  has two.
- The name of an ENUMERATED value of the PDU itself, its `criticality`, is
  taken whatever its case, as those of its IEs are.
- What is nested deeper than 64 levels stays as its octets beside a
  `_decode_error`: `to_view` refused the whole PDU.
- A path of an inspection view selects nothing for an OPTIONAL member that
  is absent and for another alternative of a CHOICE, and a name that the type
  of a value cannot have is an error that lists the members of the type: both
  were the same error. What follows an absent value in a path is checked
  against the type, `*` on a value without members selects nothing, and after
  `*` a segment is an error when none of the values can have it. The name of
  an IE in a list whose entries are no IEs is an error, where it selected
  nothing, and so is a path into a value that did not decode.
- One rule for the names of a path: a name is its letters and its digits,
  whatever their case and whatever is between them. The root, the members of
  a value and the names of ENUMERATED values are taken as the names of IEs
  and of messages were: `/s1ap/cause/value/radionetwork`. A position is a
  number as decimal writes it: `+1` and `01` are not.
- `set` refuses a name that the type of a value does not have, where the
  error came from `from_view`, and writes a member by the name that
  ASN.1 gives it. A member that its type always has is not taken out, and an
  alternative of a CHOICE takes the place of the one that is there.
- An IE is written with the name of its IE as `id` and with its `octets`
  wherever a view has IEs: `from_view` takes them in any entry, where
  only `set` and `insert` on a path under `/s1ap` did, and the `octets` of an
  IE that was added are set at its path. An `_original_id` that is no number
  is refused.
- `paths` lists the `octets` of an IE that has no value beside them.
- An IE whose value JER does not read back as it is stays its octets beside
  a `_decode_error`. rasn's JER writes a `BIT STRING` of one extensible size
  as its octets, without its length: an algorithm mask of 20 bits and one of
  24 were shown as the same value, an edit of their IE was refused, and the
  tree of their PDU was not written back.
- The examples of the documentation and of the README are compiled and run
  by the documentation tests.

### Fixed

- The octets of a PLMN identity with an MNC of three digits, which is a fix
  of the wire format of 0.2.0. TS 36.413 clause 9.2.3.8 has the three digits
  of the MCC, then those of the MNC, two in each octet, the second over the
  first: 310-410 is `13 40 01`. `helpers::plmn` wrote `13 00 14`, the order
  that TS 24.008 gives a NAS message, which a decoder of S1AP reads as
  310-041, and `helpers::plmn_from` read 310-410 in those octets. Both have
  the order of the clause now, and so has the `"310-410"` that the `inspect`
  feature writes and shows. An MNC of two digits has the same octets in both
  orders: 208-93 is `02 f8 39` as before.
- APER encoding and decoding of constrained `SEQUENCE OF` lists, among them
  the ten `E-RAB-IE-ContainerList` lists, which get back their bound of 256;
  of fixed-size `BIT STRING` values longer than 16 bits; of strings with a
  two-octet length; and of empty open type values.
- Decoding consumes the unknown extension additions of an extensible
  `SEQUENCE`, and rejects incomplete values and trailing octets.
- A claimed list length no longer reserves memory before its elements are
  read.

The earlier release is described by its tag.
