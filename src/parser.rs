use nom;
use nom::character::complete::digit1;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::*;

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum Bearing {
    N,
    E,
    S,
    W,
}

struct UnrecognizedBearing;

impl Bearing {
    fn from(ch: char) -> Result<Bearing, UnrecognizedBearing> {
        match ch {
            'N' => Ok(Bearing::N),
            'E' => Ok(Bearing::E),
            'S' => Ok(Bearing::S),
            'W' => Ok(Bearing::W),
            _ => Err(UnrecognizedBearing),
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

fn location(input: &str) -> IResult<&str, Location> {
    map_res(separated_pair(digit1, space1, digit1), |(x, y)| {
        Location::from(x, y)
    })(input)
}

fn bearing(input: &str) -> IResult<&str, Bearing> {
    map_res(one_of("NESW"), Bearing::from)(input)
}

fn movement(input: &str) -> IResult<&str, Movement> {
    map_res(one_of("LRF"), Movement::from)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_location() {
        assert_eq!(location("1 2"), Ok(("", Location { x: 1, y: 2 })));
    }

    #[test]
    fn test_parse_compass_bearing() {
        assert_eq!(bearing("E"), Ok(("", Bearing::E)));
    }

    #[test]
    fn test_parse_motion() {
        assert_eq!(movement("L"), Ok(("", Movement::L)));
    }
}
