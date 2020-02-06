mod journey;
mod parser;
mod robot;

use anyhow::Result;
use journey::Journey;
use robot::RobotState;
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

    let simulate = |journeys: Vec<Journey>| journeys.iter().map(robot::run).collect();

    match parser::journeys(&input) {
        Err(err) => eprintln!("Problem inside the journeys file: {}", err),
        Ok((_text, journeys)) => {
            let end_states: Vec<RobotState> = simulate(journeys);
            println!("{:?}", end_states);
            // for (journey, run) in journeys.iter().zip(robot::run)
        }
    }

    Ok(())
}
