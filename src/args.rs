use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Input file path
    #[arg(short, long)]
    pub input_file: String,

    /// Output file path
    #[arg(short, long)]
    pub output_file: String,
}
