mod aper_fix;
mod s1ap;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/s1ap.rs");
    println!("cargo:rerun-if-changed=build/aper_fix.rs");
    println!("cargo:rerun-if-changed=s1ap");

    // docs.rs uses the checked-in generated module and does not need a compiler run.
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    s1ap::generate_s1ap()?;
    Ok(())
}
