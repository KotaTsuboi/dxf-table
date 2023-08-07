use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    stdr_table::run()?;
    Ok(())
}
