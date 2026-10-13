use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, ensure};
use rasn_compiler::OutputMode;
use rasn_compiler::prelude::{Compiler, RasnBackend, RasnConfig};
use regex::Regex;

/// The IEs whose octets ASN.1 leaves open, each with the type of this module that TS
/// 36.413 gives them for a handover between eNBs: clauses 9.2.1.56 and 9.2.1.57. For
/// another target system the octets are those of its own specification, and so are
/// those of the secondary containers, which a handover to GERAN has.
const CARRIED: &[(&str, &str)] = &[
    (
        "Source-ToTarget-TransparentContainer",
        "SourceeNB-ToTargeteNB-TransparentContainer",
    ),
    (
        "Target-ToSource-TransparentContainer",
        "TargeteNB-ToSourceeNB-TransparentContainer",
    ),
];

/// The keys of the protocol, of which `Debug` prints no octet: the type of a key, K_gNB
/// or K_eNB and NH, and the types whose open type holds the value of an IE or a message,
/// each with what says whether that value may hold a key.
const KEYS: generator::bindings::Keys<'static> = generator::bindings::Keys {
    types: &["SecurityKey"],
    holders: &[
        (
            "ProtocolIEField",
            "crate::registry::KEY_IES.contains(&self.id.0)",
        ),
        (
            "ProtocolExtensionField",
            "crate::registry::KEY_IES.contains(&self.id.0)",
        ),
        (
            "InitiatingMessage",
            "crate::pdu::holds_key(\"InitiatingMessage\", self.procedure_code.0)",
        ),
        (
            "SuccessfulOutcome",
            "crate::pdu::holds_key(\"SuccessfulOutcome\", self.procedure_code.0)",
        ),
        (
            "UnsuccessfulOutcome",
            "crate::pdu::holds_key(\"UnsuccessfulOutcome\", self.procedure_code.0)",
        ),
    ],
};

pub fn generate_s1ap() -> Result<()> {
    // The modules that `build/asn1.sha256` records, and no other.
    let mut files = generator::asn1::modules()?;
    files.sort();

    let config = RasnConfig {
        // rasn-compiler's typed open types are experimental and currently emit
        // invalid bindings for TS 36.413. An open type is opaque: the macros over the
        // generated registry give it its type.
        opaque_open_types: true,
        generate_from_impls: true,
        ..RasnConfig::default()
    };
    // What rasn-compiler resolves is written as bindings for rasn, which become the
    // oracle of the tests.
    let output = Path::new(ORACLE);

    let warnings = Compiler::<RasnBackend, _>::new_with_config(config)
        .add_asn_sources_by_path(files.iter())
        .set_output_mode(OutputMode::SingleFile(output.into()))
        .compile()
        .map_err(|error| anyhow!("compile S1AP ASN.1 definitions: {error}"))?;

    for warning in warnings {
        eprintln!("warning: {warning}");
    }

    post_process(output, &files)
}

/// The bindings of the oracle, and those of the crate.
const ORACLE: &str = "oracle/src/bindings.rs";
const BINDINGS: &str = "src/s1ap.rs";

fn post_process(path: &Path, asn_files: &[PathBuf]) -> Result<()> {
    let mut resolved = fs::read_to_string(path).context("read the resolved S1AP bindings")?;
    resolved.insert_str(0, "#![allow(clippy::large_enum_variant)]\n\n");

    // Plain bracketed specification references are otherwise parsed as rustdoc links.
    resolved = resolved.replace("[16]", "(reference 16)");

    resolved = generator::containers::share(&resolved, "s1_ap_containers")?;

    // TS 36.413 declares both ECGIList, SIZE(1..maxnoofCellID), and ECGI-List,
    // SIZE(1..maxnoofCellsineNB), which rasn-compiler both names `ECGIList`.
    // ECGI-List becomes `ECGI_List`, as its one user, the aggressoreCGI-List
    // of SynchronisationInformation, refers to it.
    let ecgi_list = "    #[rasn(delegate, size(\"1..=256\"), identifier = \"ECGI-List\")]\n    pub struct ECGIList(";
    ensure!(
        resolved.matches(ecgi_list).count() == 1,
        "no unique ECGI-List declaration"
    );
    resolved = resolved.replacen(ecgi_list, &ecgi_list.replace("ECGIList(", "ECGI_List("), 1);
    let aggressor_list = "aggressore_cgi_list: Option<ECGIList>";
    ensure!(
        resolved.matches(aggressor_list).count() == 2,
        "unexpected aggressoreCGI-List declarations"
    );
    resolved = resolved.replace(aggressor_list, "aggressore_cgi_list: Option<ECGI_List>");

    resolved = constrain_container_lists(&resolved, asn_files)?;

    // The private IE container uses PrivateIE-ID, which rasn-compiler omits
    // from the import list of its module.
    resolved = resolved.replacen(
        "    use super::s1_ap_common_data_types::{Criticality, Presence, ProcedureCode, ProtocolIEID};",
        "    use super::s1_ap_common_data_types::{\n        Criticality, Presence, PrivateIEID, ProcedureCode, ProtocolIEID,\n    };",
        1,
    );

    // The bindings of the crate, on oxirush-aper.
    let bindings = generator::bindings::bindings(
        &resolved,
        KEYS,
        &["PLMNidentity"],
        Some("crate::message::__Field"),
    )?;

    // The oracle of the tests: the same types, with the codec of rasn and the
    // corrections that it needs.
    let mut oracle = resolved;
    oracle = generator::rasn_fix::fix_constrained_sequences(&oracle)?;
    oracle = generator::rasn_fix::fix_utf8_strings(&oracle)?;
    oracle = generator::rasn_fix::fix_fixed_bit_strings(&oracle)?;
    oracle = generator::rasn_fix::fix_long_inline_strings(&oracle)?;
    oracle = generator::rasn_fix::fix_sequences(&oracle)?;
    oracle.push_str(&oracle_support(&oracle)?);

    let asn = asn_files
        .iter()
        .map(fs::read_to_string)
        .collect::<std::io::Result<Vec<_>>>()?
        .join("\n");
    // The procedures and the IEs, once for the macros and for the inspection.
    let registry = generator::registry::Registry::read(
        "S1AP",
        &oracle,
        &asn,
        bindings.borrowing.clone(),
        bindings.keys.clone(),
    )?;
    let list = registry.list()?;
    let inspection = generator::inspection::generate("S1AP", &oracle, &asn, &registry, CARRIED)?;
    // The types that the crate decodes by a name or an identifier: the oracle decodes
    // the same.
    let named = Regex::new(r"crate::s1ap::([A-Za-z][A-Za-z0-9_]*)")?;
    let mut types: BTreeSet<&str> = BTreeSet::from(["S1APPDU"]);
    for text in [&list, &inspection] {
        types.extend(
            named
                .captures_iter(text)
                .map(|c| c.get(1).expect("a name").as_str()),
        );
    }
    let mut oracle_types = String::from(
        "// Auto-generated by build/s1ap.rs from ASN.1; do not edit.\ncrate::oracle_types! {\n",
    );
    for name in &types {
        writeln!(oracle_types, "    {name}")?;
    }
    oracle_types.push_str("}\n");
    fs::write("oracle/src/types.rs", oracle_types)?;
    fs::write("src/registry.rs", list)?;
    fs::write("src/inspect_registry.rs", inspection)?;
    fs::write(path, oracle).context("write the bindings of the oracle")?;
    let mut text = bindings.text;
    text.push_str(&support(&bindings.borrowing)?);
    fs::write(BINDINGS, text).context("write the S1AP bindings")
}

/// Restore the SIZE constraint of the lists declared through a parameterized
/// `ProtocolIE-ContainerList` or `ProtocolIE-ContainerPairList`.
///
/// rasn-compiler drops the bounds these take as value parameters, so
/// `E-RABAdmittedList ::= E-RAB-IE-ContainerList { {...} }`, a
/// `SEQUENCE (SIZE(1..maxnoofE-RABs)) OF ProtocolIE-SingleContainer`, would
/// get an unconstrained length determinant.
fn constrain_container_lists(generated: &str, asn_files: &[PathBuf]) -> Result<String> {
    let mut asn = String::new();
    for file in asn_files {
        asn.push_str(&fs::read_to_string(file)?);
        asn.push('\n');
    }

    let constant = Regex::new(r"(?m)^([A-Za-z][A-Za-z0-9-]*)\s+INTEGER\s*::=\s*(\d+)")?;
    let constants: BTreeMap<&str, &str> = constant
        .captures_iter(&asn)
        .map(|captures| {
            let (_, [name, value]) = captures.extract();
            (name, value)
        })
        .collect();
    let bound = |value: &str| -> Result<String> {
        if value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Ok(value.to_string());
        }
        constants
            .get(value)
            .map(|value| value.to_string())
            .ok_or_else(|| anyhow!("unknown container list bound {value}"))
    };

    let list_definition = Regex::new(
        r"(?m)^([A-Za-z][A-Za-z0-9-]*)\s*\{[^}]*\}\s*::=\s*ProtocolIE-Container(?:Pair)?List\s*\{\s*([A-Za-z0-9-]+)\s*,\s*([A-Za-z0-9-]+)\s*,",
    )?;
    let mut generated = generated.to_string();
    let mut constrained = 0usize;
    for captures in list_definition.captures_iter(&asn) {
        let (_, [list, lower, upper]) = captures.extract();
        let size = format!("size(\"{}..={}\")", bound(lower)?, bound(upper)?);
        let list_use = Regex::new(&format!(
            r"(?m)^([A-Za-z][A-Za-z0-9-]*)\s*::=\s*{}\s*\{{",
            regex::escape(list)
        ))?;
        for captures in list_use.captures_iter(&asn) {
            // Each of these ASN.1 names needs a rasn `identifier`.
            let name = &captures[1];
            let declaration = format!("    #[rasn(delegate, identifier = \"{name}\")]\n");
            ensure!(
                generated.matches(&declaration).count() == 1,
                "no unique declaration of {name}"
            );
            generated = generated.replacen(
                &declaration,
                &format!("    #[rasn(delegate, {size}, identifier = \"{name}\")]\n"),
                1,
            );
            constrained += 1;
        }
    }
    ensure!(constrained > 0, "no parameterized container lists found");
    Ok(generated)
}

/// What follows the modules in the bindings of the oracle: their names at the root,
/// the aliases of the crate, and the conversions that its tests build values with.
fn oracle_support(generated: &str) -> Result<String> {
    let generated_type = Regex::new(r"(?m)^    pub (?:struct|enum|type) ([A-Za-z][A-Za-z0-9_]*)")?;
    let rust_types: BTreeSet<&str> = generated_type
        .captures_iter(generated)
        .map(|captures| captures.get(1).expect("a name").as_str())
        .collect();

    let newtype = Regex::new(r"(?m)^    pub struct ([A-Za-z][A-Za-z0-9_]*)\(pub ([^;\n]+)\);$")?;
    let mut newtypes = BTreeMap::new();
    for captures in newtype.captures_iter(generated) {
        let name = captures[1].to_string();
        let inner = captures[2].to_string();
        if !inner.contains(", pub ") {
            newtypes.entry(name).or_insert(inner);
        }
    }

    let mut out = String::new();
    writeln!(
        out,
        "\n// The names of the modules at the root, and the aliases of the crate."
    )?;
    writeln!(out, "pub use s1_ap_common_data_types::*;")?;
    writeln!(out, "pub use s1_ap_constants::*;")?;
    writeln!(out, "pub use s1_ap_containers::*;")?;
    writeln!(out, "pub use s1_ap_ies::*;")?;
    writeln!(out, "pub use s1_ap_pdu_contents::*;")?;
    writeln!(out, "pub use s1_ap_pdu_descriptions::*;")?;
    writeln!(out, "#[allow(non_camel_case_types)]")?;
    writeln!(out, "pub type S1AP_PDU = S1APPDU;")?;
    if rust_types.contains("MMEUES1APID") {
        writeln!(out, "#[allow(non_camel_case_types)]")?;
        writeln!(out, "pub type MME_UE_S1AP_ID = MMEUES1APID;")?;
    }
    if rust_types.contains("ENBUES1APID") {
        writeln!(out, "#[allow(non_camel_case_types)]")?;
        writeln!(out, "pub type ENB_UE_S1AP_ID = ENBUES1APID;")?;
    }

    writeln!(out, "#[allow(unused_imports)]")?;
    writeln!(
        out,
        "use rasn::prelude::{{BitString, FixedBitString, FixedOctetString, Integer, OctetString, PrintableString, SequenceOf, Utf8String, VisibleString}};"
    )?;
    writeln!(out, "\n// The conversions of the types that have a name.")?;
    for (name, inner) in &newtypes {
        writeln!(
            out,
            "impl From<{inner}> for {name} {{ fn from(value: {inner}) -> Self {{ Self(value) }} }}"
        )?;
        if inner == "OctetString" {
            writeln!(
                out,
                "impl From<Vec<u8>> for {name} {{ fn from(value: Vec<u8>) -> Self {{ Self(value.into()) }} }}"
            )?;
        } else if let Some(size) = inner
            .strip_prefix("FixedOctetString<")
            .and_then(|value| value.strip_suffix('>'))
        {
            writeln!(
                out,
                "impl From<[u8; {size}]> for {name} {{ fn from(value: [u8; {size}]) -> Self {{ Self(value.into()) }} }}"
            )?;
        }
    }
    Ok(out)
}

/// What follows the modules in the bindings of the crate: their names at the root, the
/// aliases, and what a PDU does.
fn support(borrowing: &BTreeSet<String>) -> Result<String> {
    let mut out = String::new();
    writeln!(
        out,
        "\n// The names of the modules at the root, and the aliases."
    )?;
    writeln!(out, "pub use s1_ap_common_data_types::*;")?;
    writeln!(out, "pub use s1_ap_constants::*;")?;
    writeln!(out, "pub use s1_ap_containers::*;")?;
    writeln!(out, "pub use s1_ap_ies::*;")?;
    writeln!(out, "pub use s1_ap_pdu_contents::*;")?;
    writeln!(out, "pub use s1_ap_pdu_descriptions::*;")?;
    writeln!(out, "use alloc::vec::Vec;")?;
    writeln!(out, "use oxirush_aper::{{Decode, Encode, Octets}};")?;
    ensure!(borrowing.contains("S1APPDU"), "a PDU that borrows nothing");
    for (alias, name) in [
        ("MME_UE_S1AP_ID", "MMEUES1APID"),
        ("ENB_UE_S1AP_ID", "ENBUES1APID"),
    ] {
        ensure!(!borrowing.contains(name), "{name} borrows");
        writeln!(out, "#[allow(non_camel_case_types)]")?;
        writeln!(out, "pub type {alias} = {name};")?;
    }
    out.push_str(SUPPORT);
    Ok(out)
}

/// What a PDU does. The same text serves oxirush-ngap, with the names of its protocol.
const SUPPORT: &str = r####"/// A PDU that owns its octets: what `decode` gives, what the macros build, and what
/// is kept and sent. It borrows nothing, and so has no lifetime to write.
#[allow(non_camel_case_types)]
pub type S1AP_PDU = S1APPDU<'static>;
pub use crate::registry::S1apPduKind;
impl S1APPDU<'static> {
    /// The PDU that `bytes` are, in aligned PER: the octets of one PDU, whole, and no
    /// octet after it. The PDU owns the octets of its message, which are copied once.
    pub fn decode(bytes: &[u8]) -> Result<Self, crate::Error> {
        match S1APPDU::usual(bytes) {
            Some((alternative, code, criticality, message)) => Ok(Self::of_alternative(
                alternative,
                code,
                criticality,
                Octets::owned(message.to_vec()),
            )),
            None => oxirush_aper::decode(bytes).map(S1APPDU::into_owned),
        }
    }
    /// The PDU that `bytes` are, as `decode` reads it, with the octets of its message
    /// borrowed from `bytes`: nothing is copied, and the PDU, a `S1APPDU<'_>`, lives as
    /// long as they do. `into_owned()` gives the PDU that `decode` gives.
    ///
    /// [`message`](Self::message) gives the message of the PDU, which borrows from
    /// `bytes` as the PDU does and is kept after it: a function that is given octets
    /// returns their message.
    ///
    /// ```
    /// use oxirush_s1ap::s1ap::*;
    ///
    /// // A UE CONTEXT RELEASE REQUEST, read where it was received
    /// let received = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    /// let pdu = S1AP_PDU::decode_borrowed(&received)?;
    ///
    /// // Its message borrows from `received` too, and not from the PDU
    /// let request: UEContextReleaseRequest = pdu.message()?;
    ///
    /// // A PDU that is kept after `received` is gone
    /// let kept: S1AP_PDU = pdu.into_owned();
    /// assert_eq!(request.protocol_ies.0.len(), 3);
    /// # let _ = kept;
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    pub fn decode_borrowed<'b>(bytes: &'b [u8]) -> Result<S1APPDU<'b>, crate::Error> {
        match S1APPDU::usual(bytes) {
            Some((alternative, code, criticality, message)) => Ok(S1APPDU::of_alternative(
                alternative,
                code,
                criticality,
                Octets::borrowed(message),
            )),
            None => oxirush_aper::decode(bytes),
        }
    }
}
impl<'a> S1APPDU<'a> {
    /// A PDU of the usual layout, read octet by octet: its alternative, its procedure
    /// code and its criticality, in an octet each, then its message in one piece up to
    /// the last octet. `None` for octets of another layout and for octets that are no
    /// PDU, which the codec of the type reads or refuses: this is a shorter way to
    /// what it gives, and the tests hold the two to the same PDU.
    #[inline]
    fn usual(bytes: &'a [u8]) -> Option<(u8, u8, Criticality, &'a [u8])> {
        let [alternative, code, criticality, ref rest @ ..] = *bytes else {
            return None;
        };
        // No extension bit, and one of the three alternatives.
        if alternative & 0x80 != 0 || alternative >> 5 > 2 {
            return None;
        }
        let criticality = *Criticality::VALUES.get(usize::from(criticality >> 6))?;
        // The length of the message in one octet or in two, and no fragment.
        let message = match *rest {
            [length @ 1..=0x7f, ref message @ ..] if message.len() == usize::from(length) => {
                message
            }
            [high @ 0x80..=0xbf, low, ref message @ ..]
                if !message.is_empty()
                    && message.len() == usize::from(high & 0x3f) << 8 | usize::from(low) =>
            {
                message
            }
            _ => return None,
        };
        Some((alternative >> 5, code, criticality, message))
    }
    /// The PDU of an alternative, which is one of the three.
    #[inline]
    fn of_alternative(alternative: u8, code: u8, criticality: Criticality, value: Octets<'a>) -> Self {
        let procedure_code = ProcedureCode(code);
        match alternative {
            0 => Self::initiatingMessage(InitiatingMessage {
                procedure_code,
                criticality,
                value,
            }),
            1 => Self::successfulOutcome(SuccessfulOutcome {
                procedure_code,
                criticality,
                value,
            }),
            _ => Self::unsuccessfulOutcome(UnsuccessfulOutcome {
                procedure_code,
                criticality,
                value,
            }),
        }
    }
    /// The octets of this PDU, in aligned PER.
    pub fn encode(&self) -> Result<Vec<u8>, crate::Error> {
        let mut octets = Vec::with_capacity(self.value().len() + 6);
        self.encode_into(&mut octets)?;
        Ok(octets)
    }
    /// Append the octets of this PDU, in aligned PER, to `octets`: a buffer that is
    /// kept from one PDU to the next is allocated once. After an error, `octets` are as
    /// they were.
    pub fn encode_into(&self, octets: &mut Vec<u8>) -> Result<(), crate::Error> {
        // Octet by octet, as `decode` reads the usual layout: the alternative, the
        // procedure code and the criticality, then the message in its open type. It
        // is what the codec of the type writes, which the tests hold it to.
        let (alternative, code, criticality, value) = match self {
            S1APPDU::initiatingMessage(message) => {
                (0, message.procedure_code.0, message.criticality, &message.value)
            }
            S1APPDU::successfulOutcome(message) => {
                (1, message.procedure_code.0, message.criticality, &message.value)
            }
            S1APPDU::unsuccessfulOutcome(message) => {
                (2, message.procedure_code.0, message.criticality, &message.value)
            }
        };
        octets.extend_from_slice(&[alternative << 5, code, (criticality as u8) << 6]);
        oxirush_aper::Writer::new(octets).open_type(value);
        Ok(())
    }
    /// The message that this PDU holds, as its type `T`. What the message holds of
    /// octets is borrowed from the PDU: the PDU is a variable of its own, which the
    /// message does not outlive. [`message`](Self::message) gives one that is kept
    /// after the PDU, without a copy when the PDU itself borrows its octets.
    ///
    /// `T` is the type of the message that `kind()` says. For the type of another
    /// message the error has the kind `WrongMessage`, as the octets of one message
    /// may decode as another:
    ///
    /// ```
    /// use oxirush_s1ap::{ErrorKind, s1ap::*};
    ///
    /// // A UE CONTEXT RELEASE REQUEST
    /// let octets = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    /// let pdu = S1AP_PDU::decode(&octets)?;
    /// let request: UEContextReleaseRequest = pdu.decode_value()?;
    /// assert_eq!(request.protocol_ies.0.len(), 3);
    ///
    /// let other = pdu.decode_value::<S1SetupRequest>().unwrap_err();
    /// assert_eq!(other.kind(), ErrorKind::WrongMessage);
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    ///
    /// A type that is no message, as that of a transfer or an IE, does not compile.
    /// `decode_open_type` decodes any type from an open type.
    ///
    /// ```compile_fail
    /// use oxirush_s1ap::s1ap::*;
    ///
    /// fn held(pdu: &S1AP_PDU) {
    ///     let _ = pdu.decode_value::<ERABToBeSetupItemCtxtSUReq>();
    /// }
    /// ```
    pub fn decode_value<'s, T: crate::Message<'s>>(&'s self) -> Result<T, crate::Error> {
        let (direction, code) = (self.direction(), self.procedure_code());
        if (direction, code) != (T::DIRECTION, T::PROCEDURE_CODE) {
            return Err(crate::error::wrong_message(T::NAME, direction, code));
        }
        oxirush_aper::decode(self.value())
    }
    /// The message that this PDU holds, as its type `T`, which is kept after the PDU:
    /// it lives as long as the octets that the PDU was decoded from.
    ///
    /// The message of a PDU that borrows its octets, as `decode_borrowed` gives it,
    /// borrows from them too. Nothing is copied, and a function returns the message:
    ///
    /// ```
    /// use oxirush_s1ap::{Error, s1ap::*};
    ///
    /// // The message borrows from `received`, and the PDU is gone
    /// fn read(received: &[u8]) -> Result<UEContextReleaseRequest<'_>, Error> {
    ///     S1AP_PDU::decode_borrowed(received)?.message()
    /// }
    ///
    /// let received = hex::decode("0012401500000300000002002a000800020007000240020280").unwrap();
    /// assert_eq!(read(&received)?.protocol_ies.0.len(), 3);
    /// # Ok::<(), oxirush_s1ap::Error>(())
    /// ```
    ///
    /// The message of a PDU that owns its octets, as `decode` gives it, owns its own:
    /// the value of each IE is copied, as `into_owned()` of the message copies it.
    /// `decode_value` borrows from the PDU instead, and copies nothing. A PDU whose
    /// message came in fragments, which one of 16384 octets or more does, owns its
    /// octets however it was decoded.
    ///
    /// The errors are those of `decode_value`.
    pub fn message<T: crate::Message<'a>>(&self) -> Result<T, crate::Error> {
        let (direction, code) = (self.direction(), self.procedure_code());
        if (direction, code) != (T::DIRECTION, T::PROCEDURE_CODE) {
            return Err(crate::error::wrong_message(T::NAME, direction, code));
        }
        match self.value().as_borrowed() {
            Some(octets) => oxirush_aper::decode(octets),
            None => T::__decode_owned(self.value()),
        }
    }
    /// Return the procedure code of this PDU.
    pub fn procedure_code(&self) -> u8 {
        match self {
            S1APPDU::initiatingMessage(message) => message.procedure_code.0,
            S1APPDU::successfulOutcome(message) => message.procedure_code.0,
            S1APPDU::unsuccessfulOutcome(message) => message.procedure_code.0,
        }
    }
    /// Return the PDU direction as a human-readable string.
    pub fn direction(&self) -> &'static str {
        match self {
            S1APPDU::initiatingMessage(_) => "InitiatingMessage",
            S1APPDU::successfulOutcome(_) => "SuccessfulOutcome",
            S1APPDU::unsuccessfulOutcome(_) => "UnsuccessfulOutcome",
        }
    }
    /// Returns `true` for an initiating message.
    pub fn is_initiating(&self) -> bool {
        matches!(self, S1APPDU::initiatingMessage(_))
    }
    /// Returns `true` for a successful outcome.
    pub fn is_successful(&self) -> bool {
        matches!(self, S1APPDU::successfulOutcome(_))
    }
    /// Returns `true` for an unsuccessful outcome.
    pub fn is_unsuccessful(&self) -> bool {
        matches!(self, S1APPDU::unsuccessfulOutcome(_))
    }
}
impl core::fmt::Display for S1APPDU<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "{} {} (code={})",
            self.direction(),
            self.procedure_name(),
            self.procedure_code()
        )
    }
}
/// The open type of a value: its complete encoding in aligned PER, in which an empty
/// encoding becomes one zero octet (X.691 (02/2021) §11.1.4, §11.2.1).
pub fn encode_open_type<T: Encode>(value: &T) -> Result<Octets<'static>, crate::Error> {
    oxirush_aper::encode(value).map(Octets::owned)
}
/// The value of the type `T` that the octets of an open type hold, in aligned PER: one
/// value, whole, and no octet after it. What the value holds of octets is borrowed
/// from `value`.
pub fn decode_open_type<'a, T: Decode<'a>>(value: &'a [u8]) -> Result<T, crate::Error> {
    oxirush_aper::decode(value)
}
"####;
