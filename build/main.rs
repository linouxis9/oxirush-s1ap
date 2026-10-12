mod aper_fix;
mod asn1;
mod containers;
mod inspection;
mod registry;
mod s1ap;

fn main() -> anyhow::Result<()> {
    s1ap::generate_s1ap()
}
