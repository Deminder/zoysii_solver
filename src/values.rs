use std::ops;

// Board size: 4x4
pub const N: usize = 4;
// Cell value range: 0-255
pub type CellNumber = u8;

#[derive(Eq, Hash, Clone, Copy, Debug)]
pub struct Point(pub i8, pub i8);
impl Point {
    pub fn inside_board(&self) -> bool {
        let n = N as i8;
        self.0 >= 0 && self.0 < n && self.1 >= 0 && self.1 < n
    }
}

impl ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
