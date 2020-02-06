use super::journey::{Journey, Movement};

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub x: u16,
    pub y: u16,
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
                y: self.x + 1,
                ..self
            },
            Direction::W => Location {
                y: self.x - 1,
                ..self
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RobotState {
    pub at: Location,
    pub facing: Direction,
}

impl RobotState {
    pub fn new(x: u16, y: u16, facing: Direction) -> RobotState {
        RobotState {
            at: Location { x, y },
            facing,
        }
    }

    fn step(self, movement: Movement) -> RobotState {
        match movement {
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

pub fn run(journey: &Journey) -> RobotState {
    RobotState {
        at: journey.start.at.clone(),
        facing: journey.start.facing.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_move_forward() {
        let start = RobotState::new(0, 0, Direction::N);
        let end = RobotState::new(0, 1, Direction::N);
        assert_eq!(start.step(Movement::F), end);
    }

    #[test]
    fn one_turn() {
        let start = RobotState::new(0, 0, Direction::N);
        let end = RobotState::new(0, 0, Direction::E);
        assert_eq!(start.step(Movement::R), end);
    }
}
