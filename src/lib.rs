use clap::Parser;
use std::error::Error;

pub mod args;
pub mod input;
pub mod output;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();

    let input = input::read_input(&args.input_file)?;

    output::write(input, args.output_file)?;

    Ok(())
}
