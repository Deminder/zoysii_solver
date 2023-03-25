use std::fmt;
// Board size: 4x4
pub const N: usize = 4;
// Cell value range: 0-255
pub type CellNumber = u8;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point(u8);
impl Point {
    pub fn from(row: usize, column: usize) -> Self {
        Point(if column < N { column + row * N } else { N * N } as u8)
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn row(&self) -> usize {
        self.index() / N
    }

    pub fn column(&self) -> usize {
        self.index() % N
    }

    pub fn inside(&self) -> bool {
        self.index() < N * N
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point[{},{}]", self.row(), self.column())
    }
}
