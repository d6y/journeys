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
