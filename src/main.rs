mod journey;
mod parser;
mod robot;

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Settings {
    /// File containing journeys to be validated
    file: PathBuf,
}

fn main() -> Result<()> {
    let settings = Settings::from_args();

    let input = fs::read_to_string(settings.file)?;

    match parser::journeys(&input) {
        Err(err) => eprintln!("Problem inside the journeys file: {}", err),
        Ok((_text, journeys)) => println!("{:?}", journeys),
    }

    Ok(())
}
