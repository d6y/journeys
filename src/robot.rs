use super::journey::{Journey, Movement};

#[derive(Debug, PartialEq, Clone)]
pub struct RobotState {
    pub at: Location,
    pub facing: Direction,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub x: u16,
    pub y: u16,
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
    let step = |state: RobotState, movement| state.step(movement);
    journey.moves.iter().fold(journey.start.clone(), step)
}

impl RobotState {
    #[cfg(test)]
    pub fn new(x: u16, y: u16, facing: Direction) -> RobotState {
        RobotState {
            at: Location { x, y },
            facing,
        }
    }

    // Take one step
    fn step(self, movement: &Movement) -> RobotState {
        match *movement {
            Movement::F => RobotState {
                at: self.at.step(self.facing.clone()),
                ..self
            },
            Movement::L => RobotState {
                facing: self.facing.left(),
                ..self
            },
            Movement::R => RobotState {
                facing: self.facing.right(),
                ..self
            },
        }
    }
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        }
    }

    fn right(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}
impl Location {
    fn step(self, dir: Direction) -> Location {
        match dir {
            Direction::N => Location {
                y: self.y + 1,
                ..self
            },
            Direction::S => Location {
                y: self.y - 1,
                ..self
            },
            Direction::E => Location {
                x: self.x + 1,
                ..self
            },
            Direction::W => Location {
                x: self.x - 1,
                ..self
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_move_forward() {
        let start = RobotState::new(0, 0, Direction::N);
        let end = RobotState::new(0, 1, Direction::N);
        assert_eq!(start.step(&Movement::F), end);
    }

    #[test]
    fn one_turn() {
        let start = RobotState::new(0, 0, Direction::N);
        let end = RobotState::new(0, 0, Direction::E);
        assert_eq!(start.step(&Movement::R), end);
    }

    #[test]
    fn follow_journey_up_and_right() {
        let start = RobotState::new(0, 0, Direction::N);
        let end = RobotState::new(1, 1, Direction::E);
        let moves = Movement::from("FRF");
        let journey = Journey { start, moves, end };
        assert_eq!(run(&journey), journey.end);
    }

    #[test]
    fn follow_journey_round_the_block() {
        let start = RobotState::new(1, 1, Direction::E);
        let end = RobotState::new(1, 1, Direction::E);
        let moves = Movement::from("RFRFRFRF");
        let journey = Journey { start, moves, end };
        assert_eq!(run(&journey), journey.end);
    }
}
