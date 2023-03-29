use std::fmt;
// Board size: 4x4
pub const N: usize = 4;
// Cell value range: 0-255
pub type CellNumber = u8;

#[derive(Clone, Copy, Debug)]
pub enum Transform {
    /// Mirror rows
    Mirror,
    /// Counter clockwise rotation by 90 degrees
    Deg90,
    /// Rotation by 180 degrees
    Deg180,
    /// Clockwise rotation by 90 degrees
    Deg270,
}

#[derive(Clone, Copy, Debug)]
pub struct Sym {
    pub transform: Transform,
    pub mirror: bool,
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.transform {
                Transform::Mirror if !self.mirror => "Mirror",
                _ if self.mirror => "Mirror-",
                _ => ""
            },
            match self.transform {
                Transform::Mirror if self.mirror => "Identity",
                Transform::Deg90 => "Deg90",
                Transform::Deg180 => "Deg180",
                Transform::Deg270 => "Deg270",
                _ => ""
            },
        )
    }
}

impl Transform {
    pub fn reverse(&self) -> Self {
        match self {
            Transform::Mirror => Transform::Mirror,
            Transform::Deg90 => Transform::Deg270,
            Transform::Deg270 => Transform::Deg90,
            Transform::Deg180 => Transform::Deg180,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point(u8);
impl Point {
    pub fn from(row: usize, column: usize) -> Self {
        Point(if column < N { column + row * N } else { N * N } as u8)
    }

    pub fn iter_all() -> impl Iterator<Item = Self> {
        (0..N * N).into_iter().map(|i| Point(i as u8))
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

    pub fn reverse_symmetry(&self, sym: Sym) -> Self {
        match sym.transform {
            Transform::Mirror if sym.mirror => *self,
            _ => {
                let v = self.transform(sym.transform.reverse());
                if sym.mirror {
                    v.transform(Transform::Mirror)
                } else {
                    v
                }
            }
        }
    }

    pub fn symmetry(&self, sym: Sym) -> Self {
        match sym.transform {
            Transform::Mirror if sym.mirror => *self,
            _ => if sym.mirror {
                self.transform(Transform::Mirror)
            } else {
                *self
            }
            .transform(sym.transform),
        }
    }

    pub fn transform(&self, t: Transform) -> Self {
        debug_assert!(self.inside());
        match t {
            Transform::Mirror => Point::from(N - 1 - self.row(), self.column()),
            Transform::Deg90 => Point::from(N - 1 - self.column(), self.row()),
            Transform::Deg270 => Point::from(self.column(), N - 1 - self.row()),
            Transform::Deg180 => Point::from(N - 1 - self.row(), N - 1 - self.column()),
        }
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

pub trait BoardLike {
    fn cell(&self, p: Point) -> CellNumber;
}
