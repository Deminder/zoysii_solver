use crate::action::Action;
use crate::values::{CellNumber, Point, N};
use itertools::Itertools;
use std::cmp::{max, min};
use std::fmt;
use std::str::FromStr;

fn cell_num_diff(num: CellNumber, origin: CellNumber) -> CellNumber {
    if num == origin {
        0
    } else {
        let low = min(num, origin);
        let high = max(num, origin);
        if high > low + 1 {
            high - low
        } else {
            high + low
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Board {
    pos: Point,
    cells: u128,
}

impl Board {
    fn cell(&self, p: Point) -> CellNumber {
        (self.cells >> (p.index() * 8)) as u8
    }

    fn set_cell(&mut self, p: Point, v: CellNumber) {
        self.cells ^= ((self.cell(p) ^ v) as u128) << (p.index() * 8)
    }

    fn apply_action(&mut self, p: Point, action: Action) -> u8 {
        let mut clears: u8 = 0;
        let origin = self.cell(p);
        if origin > 0 {
            let mut pos = p + action;
            while pos.inside() {
                let v = self.cell(pos);
                if v > 0 {
                    let nv = cell_num_diff(v, origin);
                    clears += (nv == 0) as u8;
                    self.set_cell(pos, nv);
                }
                pos = pos + action;
            }
            if clears > 0 {
                self.set_cell(p, 0);
                clears += 1;
            }
        }
        clears
    }

    pub fn action(&self, action: Action) -> Option<Self> {
        let pos = self.pos + action;
        if pos.inside() {
            let mut next_board = Self {
                pos,
                cells: self.cells,
            };
            next_board.apply_action(self.pos, action);
            Some(next_board)
        } else {
            None
        }
    }

    fn row(&self, row: usize) -> u32 {
        (self.cells >> (row * N * 8)) as u32
    }

    fn col(&self, col: usize) -> u32 {
        (0..N)
            .into_iter()
            .map(|r| (self.cell(Point::from(r, col)) as u32) << (r * 8))
            .reduce(|acc, e| acc | e)
            .unwrap()
    }

    /**
       A cell is dead if both its the last in its column and row.
    */
    fn dead_cell(&self, p: Point) -> bool {
        let r = p.row();
        let c = p.column();
        self.cell(p) != 0
            && // Row is dead
                (self.row(r) & !(0xFF << (c * 8))) == 0
            && // Column is dead
                (self.col(c) & !(0xFF << (r * 8))) == 0
    }

    /**
       The board is lost if it contains any dead cell.
    */
    pub fn is_lost(&self) -> bool {
        (0..N)
            .into_iter()
            .flat_map(|r| (0..N).into_iter().map(move |c| Point::from(r, c)))
            .any(|p| self.dead_cell(p))
    }

    pub fn is_won(&self) -> bool {
        // Board is won if all cells are 0
        self.cells == 0
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #![allow(unstable_name_collisions)]
        (0..N)
            .into_iter()
            .map(|r| {
                (0..N)
                    .into_iter()
                    .map(|c| self.cell(Point::from(r, c)).to_string())
                    .intersperse(" ".into())
                    .collect_vec()
            })
            .intersperse(vec!["|".into()])
            .flatten()
            .map(|s| write!(f, "{}", s))
            .find(|r| r.is_err())
            .unwrap_or(Ok(()))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBoardError;

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<_> = s
            .splitn(N, '|')
            .flat_map(|r| {
                r.splitn(N, ' ')
                    .map(|c| c.parse::<CellNumber>().map_err(|_| ParseBoardError))
            })
            .try_collect()?;
        if numbers.len() == N * N {
            Ok(Board {
                pos: Point::from(0, 0),
                cells: numbers
                    .into_iter()
                    .map(|c| c as u128)
                    .enumerate()
                    .reduce(|(_, acc), (i, c)| (0, acc | (c << (i * 8))))
                    .unwrap()
                    .1,
            })
        } else {
            Err(ParseBoardError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn board_to_string() {
        let board_str = "18 255 6 0|0 9 3 0|33 18 18 3|0 0 15 0";
        let board = board_str.parse::<Board>();
        assert!(board.is_ok(), "should parse valid board");
        assert_eq!(board.unwrap().to_string(), board_str);
        assert!(
            "18 9 6 0|0 255 3 0|33 18 18 3|0 0 15"
                .parse::<Board>()
                .is_err(),
            "should be 4x4"
        );
        assert!(
            "18 9 6 0|0 256 3 0|33 18 18 3|0 0 15 0"
                .parse::<Board>()
                .is_err(),
            "should be max 255"
        );
    }

    #[test]
    fn check_lost_won() -> Result<(), ParseBoardError> {
        let alive: Board = "18 9 6 0|0 9 3 0|33 18 18 3|0 0 15 0".parse()?;
        assert!(!alive.is_lost(), "should not be lost");
        assert!(!alive.is_won(), "should not be won");

        let lost: Board = "18 9 0 0|0 9 0 0|33 18 0 3|0 0 15 0".parse()?;
        println!("cells num: 0x{:032X}", lost.cells);
        let dead_point = Point::from(3, 2);
        let row_num = lost.row(dead_point.row());
        println!("dead row num: 0x{row_num:08X}");
        let col_num = lost.col(dead_point.column());
        println!("dead col num: 0x{col_num:08X}");
        assert!(lost.dead_cell(dead_point), "should have dead cell");
        assert!(lost.is_lost(), "should be lost");
        assert!(!lost.is_won(), "should not be won");

        let won: Board = "0 0 0 0|0 0 0 0|0 0 0 0|0 0 0 0".parse()?;
        assert!(!won.is_lost(), "should not be lost");
        assert!(won.is_won(), "should be won");
        Ok(())
    }

    #[test]
    fn apply_action() -> Result<(), ParseBoardError> {
        let alive: Board = "18 9 6 0|0 9 3 0|33 18 18 3|0 0 15 0".parse()?;
        println!("Alive: {}", alive);
        assert!(alive.pos == Point::from(0, 0), "should start at 0,0");
        {
            let alive2 = alive.action(Action::UP);
            assert!(alive2.is_none(), "should be none for invalid action");
        }
        if let Some(alive3) = alive.action(Action::DOWN) {
            println!("Alive: {}", alive);
            println!("Alive3: {}", alive3);
            assert!(alive != alive3, "should change for valid action");
            assert!(
                alive3.to_string() == "18 9 6 0|0 9 3 0|15 18 18 3|0 0 15 0",
                "should change for valid action"
            );
            let alive_n = alive3
                .action(Action::DOWN)
                .unwrap()
                .action(Action::RIGHT)
                .unwrap()
                .action(Action::LEFT)
                .unwrap()
                .action(Action::RIGHT)
                .unwrap()
                .action(Action::UP)
                .unwrap();
            assert!(alive_n.pos == Point::from(1, 1));
            assert!(
                alive_n.to_string() == "18 0 6 0|0 0 3 0|0 0 9 0|0 0 15 0",
                "should play correctly"
            );
        } else {
            panic!("should be some for valid action");
        }
        Ok(())
    }
}
