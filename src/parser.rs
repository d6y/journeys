use nom;
use nom::character::complete::digit1;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::multi::many0;
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

#[derive(Debug, PartialEq)]
enum Movement {
    F,
    R,
    L,
}

struct UnrecognizedMovement;

impl Movement {
    fn from(ch: char) -> Result<Movement, UnrecognizedMovement> {
        match ch {
            'F' => Ok(Movement::F),
            'R' => Ok(Movement::R),
            'L' => Ok(Movement::L),
            _ => Err(UnrecognizedMovement),
        }
    }
}

fn robot_state(input: &str) -> IResult<&str, RobotState> {
    // Basic parsing
    let direction = map_res(one_of("NESW"), Direction::from);
    let location = map_res(separated_pair(digit1, space1, digit1), |(x, y)| {
        Location::from(x, y)
    });

    let parse = tuple((location, space1, direction));

    let (remainder, (at, _, facing)) = parse(input)?;
    Ok((remainder, RobotState { at, facing }))
}

fn movements(input: &str) -> IResult<&str, Vec<Movement>> {
    many0( map_res(one_of("LRF"), Movement::from) )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_robot_state() {
        let at = Location { x: 1, y: 2 };
        let facing = Direction::N;
        assert_eq!(robot_state("1 2 N"), Ok(("", RobotState { at, facing })));
    }

    #[test]
    fn test_parse_movements() {
        let expected: Vec<Movement> =
            "FRRFLFFL".chars().flat_map(Movement::from).collect();

        assert_eq!(movements("FRRFLFFL"), Ok(("", expected)));
    }

}
