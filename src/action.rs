use crate::values::Point;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    START,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Action {
    pub fn cell_offset(&self) -> Point {
        match self {
            Action::UP => Point(0, -1),
            Action::DOWN => Point(0, 1),
            Action::LEFT => Point(-1, 0),
            Action::RIGHT => Point(1, 0),
            _ => Point(0, 0),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::START => write!(f, "Start"),
            Action::UP => write!(f, "Up"),
            Action::DOWN => write!(f, "Down"),
            Action::LEFT => write!(f, "Left"),
            Action::RIGHT => write!(f, "Right"),
        }
    }
}
