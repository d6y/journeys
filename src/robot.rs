#[derive(Debug, PartialEq)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq)]
pub struct Location {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq)]
pub struct RobotState {
    pub at: Location,
    pub facing: Direction,
}

