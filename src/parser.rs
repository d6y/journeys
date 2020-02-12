use nom;
use nom::character::complete::{digit1, newline, one_of, space1};
use nom::combinator::{all_consuming, cut, map, map_res, opt};
use nom::multi::{many1, separated_list};
use nom::sequence::{separated_pair, tuple};

use std::num::ParseIntError;

use super::robot::{Direction, Journey, Location, Movement, RobotState};

type Res<'a, T> = nom::IResult<&'a str, T, nom::error::VerboseError<&'a str>>;

pub fn journeys(input: &str) -> Res<Vec<Journey>> {
    all_consuming(separated_list(newline, journey))(input)
}

fn journey(input: &str) -> Res<Journey> {
    let movement = map(one_of("FRL"), |ch| match ch {
        'F' => Movement::F,
        'R' => Movement::R,
        'L' => Movement::L,
        _ => unreachable!(),
    });

    // NB: `cut` prevents backtracking.
    // This gives a more precise error for an unrecognized movement.
    let movements = cut(many1(movement));

    let sequence = (
        robot_state,
        newline,
        movements,
        newline,
        robot_state,
        opt(newline),
    );
    map(tuple(sequence), |(start, _, moves, _, end, _)| Journey {
        start,
        moves,
        end,
    })(input)
}

fn robot_state(input: &str) -> Res<RobotState> {
    let direction = cut(map(one_of("NESW"), |ch| match ch {
        'N' => Direction::N,
        'E' => Direction::E,
        'S' => Direction::S,
        'W' => Direction::W,
        _ => unreachable!(),
    }));

    let location = map_res(separated_pair(digit1, space1, digit1), |(x, y)| {
        Location::from(x, y)
    });

    let sequence = (location, space1, direction);

    map(tuple(sequence), |(at, _, facing)| RobotState { at, facing })(input)
}

impl Location {
    fn from(x: &str, y: &str) -> Result<Location, ParseIntError> {
        Ok(Location {
            x: x.parse()?,
            y: y.parse()?,
        })
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
    fn parse_journey() {
        let input = "0 3 W\n\
            LLFFFLFLFL\n\
            2 4 S";

        use Movement::{F, L};
        let moves = vec![L, L, F, F, F, L, F, L, F, L];

        let expected = Journey {
            start: RobotState::new(0, 3, Direction::W),
            moves,
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
