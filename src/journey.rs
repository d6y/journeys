use super::robot::RobotState;

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
