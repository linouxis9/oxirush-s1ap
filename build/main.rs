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

/// The examples of the types that a program meets first, which the documentation of each
/// has after what the type is in ASN.1.
const EXAMPLES: &[(&str, &str)] = &[(
    "ERABToBeSetupItemCtxtSUReq",
    r#"An E-RAB of an INITIAL CONTEXT SETUP REQUEST: the item of a list of single containers,
which `items()` of a PDU walks.

```
use oxirush_s1ap::aper::BitString;
use oxirush_s1ap::{build_s1ap, build_s1ap_ie, ie, s1ap::*};

// The default bearer: E-RAB 5 of QCI 9, and the S-GW's end of its tunnel
let priority = AllocationAndRetentionPriority::new(PriorityLevel(15),
    PreEmptionCapability::shall_not_trigger_pre_emption,
    PreEmptionVulnerability::not_pre_emptable, None);
let qos = ERABLevelQoSParameters::new(QCI(9), priority, None, None);
let bearer = ERABToBeSetupItemCtxtSUReq::new(ERABID(5), qos,
    TransportLayerAddress(BitString::from([10, 0, 0, 1])), GTPTEID([0, 0, 0, 1]), None, None);
let request = build_s1ap!(InitiatingMessage, InitialContextSetup, InitialContextSetupRequest,
    MME_UE_S1AP_ID(1u32),
    eNB_UE_S1AP_ID(7u32),
    E_RABToBeSetupListCtxtSUReq(ERABToBeSetupListCtxtSUReq(vec![
        build_s1ap_ie!(REJECT E_RABToBeSetupItemCtxtSUReq(bearer))?,
    ])),
)?;

// Each item, where it is in the octets of the PDU
for item in request.items::<ie::E_RABToBeSetupListCtxtSUReq>() {
    let item = item?;
    assert_eq!((item.e_rab_id, item.gtp_teid), (ERABID(5), GTPTEID([0, 0, 0, 1])));
}
# Ok::<(), oxirush_s1ap::Error>(())
```"#,
)];

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
        release_display: "UEContextReleaseRequest (procedure 18, initiating message), MME-UE-S1AP-ID 42, eNB-UE-S1AP-ID 7",
        other_message: "S1SetupRequest",
        no_message: "ERABToBeSetupItemCtxtSUReq",
        examples: EXAMPLES,
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
