use gdal::Dataset;
use std::path::PathBuf;

use clap::Parser;

fn main() {
    let args = CliArgs::parse();
    let _ds = Dataset::open(args.path).unwrap();
}

#[derive(Parser)]
struct CliArgs {
    path: PathBuf,
}
