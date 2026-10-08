mod aper_fix;
mod containers;
mod inspection;
mod registry;
mod s1ap;

fn main() -> anyhow::Result<()> {
    s1ap::generate_s1ap()
}
