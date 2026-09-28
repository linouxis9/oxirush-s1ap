use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, ensure};
use rasn_compiler::OutputMode;
use rasn_compiler::prelude::{Compiler, RasnBackend, RasnConfig};
use regex::{Captures, Regex};

pub fn generate_s1ap() -> Result<()> {
    let mut files: Vec<PathBuf> = fs::read_dir("s1ap")
        .context("read S1AP ASN.1 source directory")?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<_>>()?;
    files.sort();

    let config = RasnConfig {
        // rasn-compiler's typed open types are experimental and currently emit
        // invalid bindings for TS 36.413. Stable opaque open types are wrapped
        // by the generated macros below, retaining a typed public API.
        opaque_open_types: true,
        generate_from_impls: true,
        ..RasnConfig::default()
    };
    let output = Path::new("src/s1ap.rs");

    let warnings = Compiler::<RasnBackend, _>::new_with_config(config)
        .add_asn_sources_by_path(files.iter())
        .set_output_mode(OutputMode::SingleFile(output.into()))
        .compile()
        .map_err(|error| anyhow!("compile S1AP ASN.1 definitions: {error}"))?;

    for warning in warnings {
        println!("cargo:warning={warning}");
    }

    post_process(output, &files)
}

fn post_process(path: &Path, asn_files: &[PathBuf]) -> Result<()> {
    let mut generated = fs::read_to_string(path).context("read generated S1AP bindings")?;
    generated.insert_str(0, "#![allow(clippy::large_enum_variant)]\n\n");

    // Plain bracketed specification references are otherwise parsed as rustdoc links.
    generated = generated.replace("[16]", "(reference 16)");

    // rasn-compiler already resolves every parameterized container invocation to
    // a concrete anonymous type. These now-unused imports refer to parameterized
    // definitions that intentionally have no standalone Rust representation.
    let container_imports = Regex::new(r"(?ms)^    use super::s1_ap_containers::\{.*?^    \};\n")?;
    generated = container_imports.replace_all(&generated, "").into_owned();

    // TS 36.413 declares both ECGIList, SIZE(1..maxnoofCellID), and ECGI-List,
    // SIZE(1..maxnoofCellsineNB), which rasn-compiler both names `ECGIList`.
    // ECGI-List becomes `ECGI_List`, as its one user, the aggressoreCGI-List
    // of SynchronisationInformation, refers to it.
    let ecgi_list = "    #[rasn(delegate, size(\"1..=256\"), identifier = \"ECGI-List\")]\n    pub struct ECGIList(";
    ensure!(
        generated.matches(ecgi_list).count() == 1,
        "no unique ECGI-List declaration"
    );
    generated = generated.replacen(ecgi_list, &ecgi_list.replace("ECGIList(", "ECGI_List("), 1);
    let aggressor_list = "aggressore_cgi_list: Option<ECGIList>";
    ensure!(
        generated.matches(aggressor_list).count() == 2,
        "unexpected aggressoreCGI-List declarations"
    );
    generated = generated.replace(aggressor_list, "aggressore_cgi_list: Option<ECGI_List>");

    generated = constrain_container_lists(&generated, asn_files)?;
    generated = crate::aper_fix::fix_constrained_sequences(&generated)?;
    generated = crate::aper_fix::fix_utf8_strings(&generated)?;
    generated = crate::aper_fix::fix_fixed_bit_strings(&generated)?;
    generated = crate::aper_fix::fix_long_inline_strings(&generated)?;

    // The concrete private/extension containers below use these common types,
    // but rasn-compiler omits both from the generated module import list.
    generated = generated.replacen(
        "    use super::s1_ap_common_data_types::{Criticality, Presence, ProcedureCode, ProtocolIEID};",
        "    use super::s1_ap_common_data_types::{\n        Criticality, Presence, PrivateIEID, ProcedureCode, ProtocolExtensionID, ProtocolIEID,\n    };",
        1,
    );

    // Some resolved ProtocolIE containers use primitive/anonymous field types
    // while equivalent containers use the named common types. Normalize only
    // message ProtocolIE entries; their APER representations are identical.
    let protocol_ie_block = Regex::new(
        r"(?ms)(    pub struct Anonymous[A-Za-z0-9_]+ProtocolIEs \{.*?^    \}\n    impl Anonymous[A-Za-z0-9_]+ProtocolIEs \{.*?^    \}\n)",
    )?;
    let anonymous_criticality = Regex::new(r"Anonymous[A-Za-z0-9_]+ProtocolIEsCriticality")?;
    generated = protocol_ie_block
        .replace_all(&generated, |captures: &Captures<'_>| {
            let block = captures[1]
                .replace("pub id: u16", "pub id: ProtocolIEID")
                .replace("id: u16,", "id: ProtocolIEID,");
            anonymous_criticality
                .replace_all(&block, "Criticality")
                .into_owned()
        })
        .into_owned();

    let support = generate_support(&generated, asn_files)?;
    generated.push_str(&support);
    fs::write(path, generated).context("write post-processed S1AP bindings")
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

fn generate_support(generated: &str, asn_files: &[PathBuf]) -> Result<String> {
    let mut asn = String::new();
    for file in asn_files {
        asn.push_str(&fs::read_to_string(file)?);
        asn.push('\n');
    }

    let generated_type = Regex::new(r"(?m)^    pub (?:struct|enum|type) ([A-Za-z][A-Za-z0-9_]*)")?;
    let mut rust_types = BTreeMap::new();
    for captures in generated_type.captures_iter(generated) {
        let name = captures[1].to_string();
        if !name.starts_with("Anonymous") {
            rust_types.entry(canonical(&name)).or_insert(name);
        }
    }

    let newtype = Regex::new(r"(?m)^    pub struct ([A-Za-z][A-Za-z0-9_]*)\(pub ([^;\n]+)\);$")?;
    let mut newtypes = BTreeMap::new();
    for captures in newtype.captures_iter(generated) {
        let name = captures[1].to_string();
        let inner = captures[2].to_string();
        if !inner.contains(", pub ") {
            newtypes.entry(name).or_insert(inner);
        }
    }

    let ie_constant =
        Regex::new(r"(?m)^\s*(id-[A-Za-z][A-Za-z0-9-]*)\s+ProtocolIE-ID\s+::=\s+(\d+)")?;
    let ie_constants: BTreeMap<String, u16> = ie_constant
        .captures_iter(&asn)
        .map(|captures| Ok((captures[1].to_string(), captures[2].parse()?)))
        .collect::<Result<_>>()?;

    let ie_object = Regex::new(
        r"(?s)\{\s*ID\s+(id-[A-Za-z][A-Za-z0-9-]*)\s+CRITICALITY\s+[A-Za-z-]+\s+TYPE\s+(OCTET\s+STRING|[A-Za-z][A-Za-z0-9-]*)\s+PRESENCE",
    )?;
    let mut ies: BTreeMap<String, (u16, String)> = BTreeMap::new();
    let mut type_aliases: BTreeMap<String, BTreeSet<u16>> = BTreeMap::new();
    for captures in ie_object.captures_iter(&asn) {
        let id_name = &captures[1];
        let asn_type = &captures[2];
        let Some(id) = ie_constants.get(id_name).copied() else {
            continue;
        };
        let ie_name = macro_ident(id_name.trim_start_matches("id-"));
        // id-S1-Message has the inline type OCTET STRING, which has neither a
        // generated type nor an alias.
        if asn_type.starts_with("OCTET") {
            ies.entry(ie_name)
                .or_insert((id, "$crate::__rasn::types::OctetString".into()));
            continue;
        }
        let Some(rust_type) = rust_types.get(&canonical(asn_type)).cloned() else {
            continue;
        };

        // The IE-name spelling used by TS 36.413 names its own IE. The rasn
        // type name is a concise alias, added below, unless an IE has that
        // name: id-UERadioCapability-NR-Format is a UERadioCapability, but
        // `UERadioCapability` names id-UERadioCapability.
        ies.entry(ie_name)
            .or_insert((id, format!("$crate::s1ap::{rust_type}")));
        type_aliases.entry(rust_type).or_default().insert(id);
    }
    // A type name is an alias only for the one IE of that type: the type of
    // id-SourceMME-GUMMEI and id-GUMMEI-ID could otherwise address either.
    // Nor is it one when an IE outside the macros' reach, an extension IE,
    // has that name.
    let ie_names: BTreeSet<String> = ie_constants
        .keys()
        .map(|id_name| macro_ident(id_name.trim_start_matches("id-")))
        .collect();
    for (rust_type, ids) in type_aliases {
        if let [id] = ids.into_iter().collect::<Vec<_>>()[..]
            && !ie_names.contains(&rust_type)
        {
            ies.entry(rust_type.clone())
                .or_insert((id, format!("$crate::s1ap::{rust_type}")));
        }
    }

    let procedure_constant =
        Regex::new(r"(?m)^\s*(id-[A-Za-z][A-Za-z0-9-]*)\s+ProcedureCode\s+::=\s+(\d+)")?;
    let mut procedures = BTreeMap::new();
    let mut procedure_ids = BTreeMap::new();
    for captures in procedure_constant.captures_iter(&asn) {
        let id_name = captures[1].to_string();
        let code: u8 = captures[2].parse()?;
        let name = macro_ident(id_name.trim_start_matches("id-"));
        procedures.insert(name.clone(), code);
        procedure_ids.insert(id_name, name);
    }

    let procedure_block =
        Regex::new(r"(?ms)^[A-Za-z][A-Za-z0-9-]*\s+S1AP-ELEMENTARY-PROCEDURE\s+::=\s*\{(.*?)^\}")?;
    let procedure_ref = Regex::new(r"PROCEDURE CODE\s+(id-[A-Za-z][A-Za-z0-9-]*)")?;
    let mut directions: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    for captures in procedure_block.captures_iter(&asn) {
        let body = &captures[1];
        let Some(id) = procedure_ref
            .captures(body)
            .map(|capture| capture[1].to_string())
        else {
            continue;
        };
        let Some(name) = procedure_ids.get(&id).cloned() else {
            continue;
        };
        for (label, direction) in [
            ("INITIATING MESSAGE", "Initiating"),
            ("SUCCESSFUL OUTCOME", "Successful"),
            ("UNSUCCESSFUL OUTCOME", "Unsuccessful"),
        ] {
            if body.contains(label) {
                directions
                    .entry(direction)
                    .or_default()
                    .insert(name.clone());
            }
        }
    }

    let mut out = String::new();
    writeln!(
        out,
        "\n// Auto-generated S1AP compatibility surface and ASN.1-derived lookups."
    )?;
    writeln!(out, "pub use s1_ap_common_data_types::*;")?;
    writeln!(out, "pub use s1_ap_constants::*;")?;
    writeln!(out, "pub use s1_ap_ies::*;")?;
    writeln!(out, "pub use s1_ap_pdu_contents::*;")?;
    writeln!(out, "pub use s1_ap_pdu_descriptions::*;")?;
    writeln!(out, "#[allow(non_camel_case_types)]")?;
    writeln!(out, "pub type S1AP_PDU = S1APPDU;")?;
    if rust_types.values().any(|name| name == "MMEUES1APID") {
        writeln!(out, "#[allow(non_camel_case_types)]")?;
        writeln!(out, "pub type MME_UE_S1AP_ID = MMEUES1APID;")?;
    }
    if rust_types.values().any(|name| name == "ENBUES1APID") {
        writeln!(out, "#[allow(non_camel_case_types)]")?;
        writeln!(out, "pub type ENB_UE_S1AP_ID = ENBUES1APID;")?;
    }

    writeln!(
        out,
        "use rasn::prelude::{{BitString, FixedBitString, FixedOctetString, Integer, OctetString, PrintableString, SequenceOf, VisibleString}};"
    )?;
    writeln!(
        out,
        "\n// Auto-generated newtype conversions used by the builder macros."
    )?;
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

    writeln!(out, "#[macro_export]")?;
    writeln!(out, "#[doc(hidden)]")?;
    writeln!(out, "macro_rules! __s1ap_ie_id {{")?;
    for (name, (id, _)) in &ies {
        writeln!(out, "    ({name}) => {{ {id}u16 }};")?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "#[macro_export]")?;
    writeln!(out, "#[doc(hidden)]")?;
    writeln!(out, "macro_rules! __s1ap_encode_ie {{")?;
    for (name, (_, type_path)) in &ies {
        writeln!(
            out,
            "    ({name}, $value:expr) => {{ {{ let value: {type_path} = ($value).into(); $crate::s1ap::encode_open_type(&value) }} }};"
        )?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "#[macro_export]")?;
    writeln!(out, "#[doc(hidden)]")?;
    writeln!(out, "macro_rules! __s1ap_decode_ie {{")?;
    for (name, (_, type_path)) in &ies {
        writeln!(
            out,
            "    ({name}, $value:expr) => {{ $crate::s1ap::decode_open_type::<{type_path}>($value) }};"
        )?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "#[macro_export]")?;
    writeln!(out, "#[doc(hidden)]")?;
    writeln!(out, "macro_rules! __s1ap_proc_code {{")?;
    for (name, code) in &procedures {
        writeln!(out, "    ({name}) => {{ {code}u8 }};")?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "#[allow(non_camel_case_types)]")?;
    writeln!(out, "#[derive(Clone, Debug, PartialEq, Eq)]")?;
    writeln!(out, "#[non_exhaustive]")?;
    writeln!(out, "pub enum S1apPduKind {{")?;
    for direction in ["Initiating", "Successful", "Unsuccessful"] {
        if let Some(names) = directions.get(direction) {
            for name in names {
                writeln!(out, "    {direction}_{name},")?;
            }
        }
    }
    writeln!(
        out,
        "    Other {{ direction: &'static str, procedure_code: u8 }},"
    )?;
    writeln!(out, "}}")?;

    writeln!(out, "impl S1APPDU {{")?;
    writeln!(out, "    /// Encode this PDU using Aligned PER.")?;
    writeln!(
        out,
        "    pub fn encode(&self) -> Result<Vec<u8>, rasn::error::EncodeError> {{"
    )?;
    writeln!(out, "        rasn::aper::encode(self)")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// Decode an S1AP PDU from Aligned PER bytes.")?;
    writeln!(
        out,
        "    pub fn decode(bytes: &[u8]) -> Result<Self, rasn::error::DecodeError> {{"
    )?;
    writeln!(out, "        rasn::aper::decode(bytes)")?;
    writeln!(out, "    }}")?;
    writeln!(
        out,
        "    /// Decode the typed message held by this PDU's open type."
    )?;
    writeln!(
        out,
        "    pub fn decode_value<T: rasn::Decode>(&self) -> Result<T, rasn::error::DecodeError> {{"
    )?;
    writeln!(out, "        let value = match self {{")?;
    writeln!(
        out,
        "            S1APPDU::initiatingMessage(message) => &message.value,"
    )?;
    writeln!(
        out,
        "            S1APPDU::successfulOutcome(message) => &message.value,"
    )?;
    writeln!(
        out,
        "            S1APPDU::unsuccessfulOutcome(message) => &message.value,"
    )?;
    writeln!(out, "        }};")?;
    writeln!(out, "        rasn::aper::decode(value.as_bytes())")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// Return the procedure code of this PDU.")?;
    writeln!(out, "    pub fn procedure_code(&self) -> u8 {{")?;
    writeln!(out, "        match self {{")?;
    writeln!(
        out,
        "            S1APPDU::initiatingMessage(message) => message.procedure_code.0,"
    )?;
    writeln!(
        out,
        "            S1APPDU::successfulOutcome(message) => message.procedure_code.0,"
    )?;
    writeln!(
        out,
        "            S1APPDU::unsuccessfulOutcome(message) => message.procedure_code.0,"
    )?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(
        out,
        "    /// Return the PDU direction as a human-readable string."
    )?;
    writeln!(out, "    pub fn direction(&self) -> &'static str {{")?;
    writeln!(out, "        match self {{")?;
    writeln!(
        out,
        "            S1APPDU::initiatingMessage(_) => \"InitiatingMessage\","
    )?;
    writeln!(
        out,
        "            S1APPDU::successfulOutcome(_) => \"SuccessfulOutcome\","
    )?;
    writeln!(
        out,
        "            S1APPDU::unsuccessfulOutcome(_) => \"UnsuccessfulOutcome\","
    )?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// Returns `true` for an initiating message.")?;
    writeln!(
        out,
        "    pub fn is_initiating(&self) -> bool {{ matches!(self, S1APPDU::initiatingMessage(_)) }}"
    )?;
    writeln!(out, "    /// Returns `true` for a successful outcome.")?;
    writeln!(
        out,
        "    pub fn is_successful(&self) -> bool {{ matches!(self, S1APPDU::successfulOutcome(_)) }}"
    )?;
    writeln!(out, "    /// Returns `true` for an unsuccessful outcome.")?;
    writeln!(
        out,
        "    pub fn is_unsuccessful(&self) -> bool {{ matches!(self, S1APPDU::unsuccessfulOutcome(_)) }}"
    )?;
    writeln!(out, "    /// Return the ASN.1 procedure name.")?;
    writeln!(out, "    pub fn procedure_name(&self) -> &'static str {{")?;
    writeln!(out, "        match self.procedure_code() {{")?;
    for (name, code) in &procedures {
        writeln!(out, "            {code} => \"{name}\",")?;
    }
    writeln!(out, "            _ => \"Unknown\",")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(
        out,
        "    /// Return the canonical direction/procedure kind."
    )?;
    writeln!(out, "    pub fn kind(&self) -> S1apPduKind {{")?;
    writeln!(out, "        let code = self.procedure_code();")?;
    writeln!(out, "        match self {{")?;
    for (variant, direction) in [
        ("initiatingMessage", "Initiating"),
        ("successfulOutcome", "Successful"),
        ("unsuccessfulOutcome", "Unsuccessful"),
    ] {
        writeln!(out, "            S1APPDU::{variant}(_) => match code {{")?;
        if let Some(names) = directions.get(direction) {
            for name in names {
                if let Some(code) = procedures.get(name) {
                    writeln!(
                        out,
                        "                {code} => S1apPduKind::{direction}_{name},"
                    )?;
                }
            }
        }
        writeln!(
            out,
            "                _ => S1apPduKind::Other {{ direction: \"{direction}\", procedure_code: code }},"
        )?;
        writeln!(out, "            }},")?;
    }
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    writeln!(out, "impl S1apPduKind {{")?;
    writeln!(out, "    /// Return this kind's S1AP procedure code.")?;
    writeln!(out, "    pub fn procedure_code(&self) -> u8 {{")?;
    writeln!(out, "        match self {{")?;
    for direction in ["Initiating", "Successful", "Unsuccessful"] {
        if let Some(names) = directions.get(direction) {
            for name in names {
                if let Some(code) = procedures.get(name) {
                    writeln!(
                        out,
                        "            S1apPduKind::{direction}_{name} => {code},"
                    )?;
                }
            }
        }
    }
    writeln!(
        out,
        "            S1apPduKind::Other {{ procedure_code, .. }} => *procedure_code,"
    )?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    writeln!(out, "impl std::fmt::Display for S1APPDU {{")?;
    writeln!(
        out,
        "    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
    )?;
    writeln!(
        out,
        "        write!(formatter, \"{{}} {{}} (code={{}})\", self.direction(), self.procedure_name(), self.procedure_code())"
    )?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    writeln!(
        out,
        "/// Encode a typed ASN.1 value for an S1AP open type using APER."
    )?;
    writeln!(out, "///")?;
    writeln!(
        out,
        "/// An open type holds a complete encoding, in which an empty encoding"
    )?;
    writeln!(
        out,
        "/// becomes one zero octet (X.691 (07/2002) §10.1.4, §10.2.1)."
    )?;
    writeln!(
        out,
        "pub fn encode_open_type<T: rasn::Encode>(value: &T) -> Result<rasn::types::Any, rasn::error::EncodeError> {{"
    )?;
    writeln!(out, "    let mut bytes = rasn::aper::encode(value)?;")?;
    writeln!(out, "    if bytes.is_empty() {{")?;
    writeln!(out, "        bytes.push(0);")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    Ok(rasn::types::Any::new(bytes))")?;
    writeln!(out, "}}")?;
    writeln!(
        out,
        "/// Decode a typed ASN.1 value from an S1AP open type using APER."
    )?;
    writeln!(
        out,
        "pub fn decode_open_type<T: rasn::Decode>(value: &rasn::types::Any) -> Result<T, rasn::error::DecodeError> {{"
    )?;
    writeln!(out, "    rasn::aper::decode(value.as_bytes())")?;
    writeln!(out, "}}")?;

    Ok(out)
}

fn canonical(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn macro_ident(value: &str) -> String {
    value.replace('-', "_")
}
