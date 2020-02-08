use nom;
use nom::character::complete::{digit1, newline, one_of, space1};
use nom::combinator::{map_res, opt};
use nom::multi::{many0, separated_list};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;

use std::num::ParseIntError;

use super::robot::{Direction, Journey, Location, Movement, RobotState};

pub fn journeys(input: &str) -> IResult<&str, Vec<Journey>> {
    separated_list(newline, journey)(input)
}

fn journey(input: &str) -> IResult<&str, Journey> {
    let grammar = tuple((
        robot_state,
        newline,
        movements,
        newline,
        robot_state,
        opt(newline),
    ));

    let (remainder, (start, _, moves, _, end, _)) = grammar(input)?;
    Ok((remainder, Journey { start, moves, end }))
}

fn robot_state(input: &str) -> IResult<&str, RobotState> {
    let direction = map_res(one_of("NESW"), Direction::from);
    let location = map_res(separated_pair(digit1, space1, digit1), |(x, y)| {
        Location::from(x, y)
    });

    let grammar = tuple((location, space1, direction));

    let (remainder, (at, _, facing)) = grammar(input)?;
    Ok((remainder, RobotState { at, facing }))
}

fn movements(input: &str) -> IResult<&str, Vec<Movement>> {
    many0(map_res(one_of("LRF"), Movement::lookup))(input)
}

struct UnrecognizedDirection;

impl Direction {
    fn from(ch: char) -> Result<Direction, UnrecognizedDirection> {
        match ch {
            'N' => Ok(Direction::N),
            'E' => Ok(Direction::E),
            'S' => Ok(Direction::S),
            'W' => Ok(Direction::W),
            _ => Err(UnrecognizedDirection),
        }
    }
}

impl Location {
    fn from(x: &str, y: &str) -> Result<Location, ParseIntError> {
        Ok(Location {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

pub struct UnrecognizedMovement;

impl Movement {
    pub fn lookup(ch: char) -> Result<Movement, UnrecognizedMovement> {
        match ch {
            'F' => Ok(Movement::F),
            'R' => Ok(Movement::R),
            'L' => Ok(Movement::L),
            _ => Err(UnrecognizedMovement),
        }
    }

    #[cfg(test)]
    pub fn from(str: &str) -> Vec<Movement> {
        str.chars().flat_map(Movement::lookup).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_robot_state() {
        let at = Location { x: 1, y: 2 };
        let facing = Direction::N;
        assert_eq!(robot_state("1 2 N"), Ok(("", RobotState { at, facing })));
    }

    #[test]
    fn parse_movements() {
        let input = "FRRFLFFL";
        let expected = Movement::from(input);
        assert_eq!(expected.len(), 8);
        assert_eq!(movements(input), Ok(("", expected)));
    }

    #[test]
    fn parse_journey() {
        let input = "0 3 W\n\
            LLFFFLFLFL\n\
            2 4 S";

        let expected = Journey {
            start: RobotState::new(0, 3, Direction::W),
            moves: Movement::from("LLFFFLFLFL"),
            end: RobotState::new(2, 4, Direction::S),
        };

        assert_eq!(journey(input), Ok(("", expected)));
    }

    #[test]
    fn parse_journeys() {
        let input = include_str!("../input.txt");
        let actual = journeys(input);

        let num_journeys = actual.map(|(_, js)| js.len());
        assert_eq!(num_journeys, Ok(3));
    }
}
