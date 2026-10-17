//! The generator of the bindings of the crate: the generator of oxirush-aper, which
//! oxirush-ngap and oxirush-s1ap share, with the names of S1AP. It runs rasn-compiler on
//! the ASN.1 of the specification, and writes the bindings, the registry, the inspection
//! and the oracle of the tests.

use anyhow::{Result, ensure};
use generator::protocol::{Protocol, generate};

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

fn main() -> Result<()> {
    generate(&Protocol {
        name: "S1AP",
        modules: "s1_ap",
        carried: CARRIED,
        keys: &["SecurityKey"],
        plmn: "PLMNidentity",
        aliases: &[
            ("MME_UE_S1AP_ID", "MMEUES1APID"),
            ("ENB_UE_S1AP_ID", "ENBUES1APID"),
        ],
        release_request: "0012401500000300000002002a000800020007000240020280",
        other_message: "S1SetupRequest",
        no_message: "ERABToBeSetupItemCtxtSUReq",
        correct,
    })
}

/// What the bindings that rasn-compiler resolves of TS 36.413 need.
fn correct(mut resolved: String) -> Result<String> {
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
    Ok(resolved)
}
