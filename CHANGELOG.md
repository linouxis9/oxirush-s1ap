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

### Added

- The optional `inspect` feature: a decoded PDU as a `serde_json` tree in the
  ASN.1 JSON encoding, which can be edited and encoded again. What is not
  edited keeps the octets received, and an IE is added by its typed value or
  by its octets. A PLMN identity, a transport layer address, an IMSI and the
  usual identifiers read and write as they are usually written, and an
  `ENUMERATED` value is named in any case.
- With the `inspect` feature, the values of a tree by their paths:
  `inspect::paths` gives each value with the path that selects it,
  `inspect::select` the values at a path, and `inspect::set`, `remove` and
  `insert` edit a tree at a path. `/s1ap` stands for the IEs of the message,
  and an IE goes by its name, as in `/s1ap/eNB-UE-S1AP-ID/value`, by its
  position or by its identifier.
- With the `inspect` feature, `inspect::message_name` gives the name that
  ASN.1 has for the message of a PDU, such as `E-RABSetupRequest`,
  `inspect::message_named` the direction and the procedure code of the
  message of a name, in any case, and `inspect::message_names` all of them.
- The `S1_Message` IE name, for the one IE whose type is an inline
  `OCTET STRING`.
- The `sized` module: the `OCTET STRING` and `BIT STRING` types whose length
  determinant takes two octets.

### Changed

- The macros and the `inspect` feature read one generated list,
  `src/registry.rs`, with one line for each procedure and for each IE. An IE
  or a procedure that a macro does not know is reported by its name, with
  the names that are close to it, where the error named the first IE of the
  specification. The variants of `S1apPduKind` are in the order of the procedure
  codes, and each says its message.
- The crate no longer depends on `paste`: the macros paste no name.
- An edit at a path of an inspection tree changes what is sent or is refused.
  `set` with `null` takes an optional member out, as its documentation said,
  where it wrote a `null` that did not encode. The value of an IE is not
  taken out: that edit returned without an error and sent the octets
  received. The `octets` of an IE whose value an edit changed are an error to
  select, and `*` leaves out the members that start with `_`.
- The name of an ENUMERATED value of the PDU itself, its `criticality`, is
  taken whatever its case, as those of its IEs are.
- What is nested deeper than 64 levels stays as its octets beside a
  `_decode_error`: `inspect_pdu` refused the whole PDU.

### Fixed

- APER encoding and decoding of constrained `SEQUENCE OF` lists, among them
  the ten `E-RAB-IE-ContainerList` lists, which get back their bound of 256;
  of fixed-size `BIT STRING` values longer than 16 bits; of strings with a
  two-octet length; and of empty open type values.
- Decoding consumes the unknown extension additions of an extensible
  `SEQUENCE`, and rejects incomplete values and trailing octets.
- A claimed list length no longer reserves memory before its elements are
  read.

The earlier release is described by its tag.
