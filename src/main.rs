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

    let simulate = |journeys: &Vec<Journey>| journeys.iter().map(robot::run).collect();

    let report = |journeys: &Vec<Journey>, results: &Vec<RobotState>| {
        for (i, (journey, result)) in journeys.iter().zip(results.iter()).enumerate() {
            let success = &journey.end == result;
            if success {
                println!("Journey {} 👍", i);
            } else {
                println!(
                    "Journey {} 👎- ended up at {:?}, facing {:?}",
                    i, result.at, result.facing
                );
            }
        }
    };

    match parser::journeys(&input) {
        Ok(("", journeys)) => {
            let end_states: Vec<RobotState> = simulate(&journeys);
            report(&journeys, &end_states);
        }
        Ok((text, _)) => eprintln!("Could not read whole journeys file. Left with: {}", text),
        Err(err) => eprintln!("Problem inside the journeys file: {}", err),
    }

    Ok(())
}
