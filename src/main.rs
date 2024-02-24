use std::path::PathBuf;

use clap::Parser;

use component_opt::{optimize_file, ComponentOptError};

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(short, long)]
    input: PathBuf,
    #[clap(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), ComponentOptError> {
    let args = Arguments::parse();
    optimize_file(&args.input, &args.output)
}
