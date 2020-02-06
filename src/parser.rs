use nom;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::*;

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
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

#[derive(Debug, PartialEq)]
struct Location {
    x: u16,
    y: u16,
}

impl Location {
    fn from(x: &str, y: &str) -> Result<Location, ParseIntError> {
        let x: u16 = x.parse()?;
        let y: u16 = y.parse()?;
        Ok(Location { x, y })
    }
}

#[derive(Debug, PartialEq)]
struct RobotState {
    at: Location,
    facing: Direction,
}

impl RobotState {
    fn new(x: u16, y: u16, facing: Direction) -> RobotState {
        RobotState {
            at: Location { x, y },
            facing,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Journey {
    start: RobotState,
    moves: Vec<Movement>,
    end: RobotState,
}

#[derive(Debug, PartialEq)]
enum Movement {
    F,
    R,
    L,
}

struct UnrecognizedMovement;

impl Movement {
    fn lookup(ch: char) -> Result<Movement, UnrecognizedMovement> {
        match ch {
            'F' => Ok(Movement::F),
            'R' => Ok(Movement::R),
            'L' => Ok(Movement::L),
            _ => Err(UnrecognizedMovement),
        }
    }

    fn from(str: &str) -> Vec<Movement> {
        str.chars().flat_map(Movement::lookup).collect()
    }
}

fn robot_state(input: &str) -> IResult<&str, RobotState> {
    // Basic parsing
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
}
