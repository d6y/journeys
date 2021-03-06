#[derive(Debug, PartialEq)]
pub struct Journey {
    pub start: RobotState,
    pub moves: Vec<Movement>,
    pub end: RobotState,
}

#[derive(Debug, PartialEq)]
pub enum Movement {
    F,
    R,
    L,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RobotState {
    pub at: Location,
    pub facing: Direction,
}

impl RobotState {
    pub fn new(x: i16, y: i16, facing: Direction) -> RobotState {
        RobotState {
            at: Location { x, y },
            facing,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

// This is the plan for working out the end state of a robot:
// 1. Imagine we have a function that can take one step: given a robot state and a move (like forward),
//    it can calculate a new robot state.
// 2. With that, we take the starting point and for each move crank the step function to compute each new state.
//    That's the `fold` function on a collection.
pub fn run(journey: &Journey) -> RobotState {
    let step = |state: RobotState, movement: &Movement| {
        use Direction::{E, N, S, W};
        use Movement::{F, L, R};
        match (movement, state.facing, state.at.x, state.at.y) {
            (F, N, x, y) => RobotState::new(x, y + 1, N),
            (F, E, x, y) => RobotState::new(x + 1, y, E),
            (F, S, x, y) => RobotState::new(x, y - 1, S),
            (F, W, x, y) => RobotState::new(x - 1, y, W),
            (L, N, x, y) => RobotState::new(x, y, W),
            (L, E, x, y) => RobotState::new(x, y, N),
            (L, S, x, y) => RobotState::new(x, y, E),
            (L, W, x, y) => RobotState::new(x, y, S),
            (R, N, x, y) => RobotState::new(x, y, E),
            (R, E, x, y) => RobotState::new(x, y, S),
            (R, S, x, y) => RobotState::new(x, y, W),
            (R, W, x, y) => RobotState::new(x, y, N),
        }
    };

    journey.moves.iter().fold(journey.start.clone(), step)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Movement::{F, R};

    #[test]
    fn follow_journey_up_and_right() {
        let start = RobotState::new(0, 0, Direction::N);
        let end = RobotState::new(1, 1, Direction::E);
        let moves = vec![F, R, F];
        let journey = Journey { start, moves, end };
        assert_eq!(run(&journey), journey.end);
    }

    #[test]
    fn follow_journey_round_the_block() {
        let start = RobotState::new(1, 1, Direction::E);
        let end = RobotState::new(1, 1, Direction::E);
        let moves = vec![R, F, R, F, R, F, R, F];
        let journey = Journey { start, moves, end };
        assert_eq!(run(&journey), journey.end);
    }
}
