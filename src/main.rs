use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dxf_table::run()?;
    Ok(())
}
