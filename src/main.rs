pub mod deser;
pub mod model;
pub mod ser;

use crate::{deser::deser_json, ser::ser_json};
use clap::Parser;
use std::path::PathBuf;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: PathBuf,
    output: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let p = args.input;
    println!("reading from {}", &p.canonicalize()?.display());
    let data = deser_json(&p)?;
    println!("{:?}", &data);
    ser_json(data, args.output)?;
    Ok(())
}
