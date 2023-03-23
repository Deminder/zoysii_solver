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

#[derive(Eq, Hash, Clone, Copy, Debug)]
pub struct BoardRow {
    cells: [CellNumber; N],
}

impl PartialEq for BoardRow {
    fn eq(&self, other: &Self) -> bool {
        self.cells.iter().eq(other.cells.iter())
    }
}

#[derive(Eq, Hash, Clone, Copy, Debug)]
pub struct Board {
    pos: Point,
    rows: [BoardRow; N],
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.rows.iter().eq(other.rows.iter())
    }
}

impl Board {
    fn cell(&self, p: &Point) -> CellNumber {
        self.rows[p.1 as usize].cells[p.0 as usize]
    }

    fn set_cell(&mut self, p: &Point, v: CellNumber) {
        let row = &mut self.rows[p.1 as usize];
        row.cells[p.0 as usize] = v;
    }

    fn apply_action(&mut self, p: &Point, action: Action) -> u8 {
        let mut clears: u8 = 0;
        let direction = action.cell_offset();
        let origin = self.cell(p);
        if origin > 0 {
            let mut pos = *p + direction;
            while pos.inside_board() {
                let v = self.cell(&pos);
                if v > 0 {
                    let nv = cell_num_diff(v, origin);
                    clears += (nv == 0) as u8;
                    self.set_cell(&pos, nv);
                }
                pos = pos + direction;
            }
            if clears > 0 {
                self.set_cell(p, 0);
                clears += 1;
            }
        }
        clears
    }

    pub fn action(&self, action: Action) -> Option<Self> {
        let offset = action.cell_offset();
        let pos = self.pos + offset;
        if pos.inside_board() {
            let mut next_board = Self {
                pos,
                rows: self.rows.clone(),
            };
            next_board.apply_action(&self.pos, action);
            Some(next_board)
        } else {
            None
        }
    }

    /**
       A row/column is dead if it contains exactly one non-zero cell.
       A cell is dead if both its column and row are dead.
       The board is lost if there is a dead cell.
    */
    pub fn is_lost(&self) -> bool {
        self.rows
            .into_iter()
            .filter_map(|r| {
                // Detect a dead row
                let candidate: Vec<usize> = r
                    .cells
                    .into_iter()
                    .enumerate()
                    .filter(|(_, c)| *c != 0)
                    .map(|(i, _)| i)
                    .take(2)
                    .collect();
                if candidate.len() == 1 {
                    Some(candidate[0])
                } else {
                    None
                }
            })
            .any(|column| {
                // Check for dead columns in dead rows
                self.rows
                    .into_iter()
                    .map(|r| r.cells[column])
                    .filter(|c| *c != 0)
                    .take(2)
                    .count()
                    == 1
            })
    }

    pub fn is_won(&self) -> bool {
        // Board is won if all cells are 0
        self.rows
            .into_iter()
            .all(|r| r.cells.into_iter().all(|c| c == 0))
    }
}

impl fmt::Display for BoardRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #![allow(unstable_name_collisions)]
        self.cells
            .into_iter()
            .map(|c| c.to_string())
            .intersperse(" ".into())
            .map(|s| write!(f, "{}", s))
            .find(|r| r.is_err())
            .unwrap_or(Ok(()))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #![allow(unstable_name_collisions)]
        self.rows
            .into_iter()
            .map(|r| r.to_string())
            .intersperse("|".into())
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
        let rows: Vec<_> = s
            .splitn(N, '|')
            .map(|s| s.parse::<BoardRow>().map_err(|_| ParseBoardError))
            .try_collect()?;
        if rows.len() == N {
            Ok(Board {
                pos: Point(0, 0),
                rows: core::array::from_fn(|i| rows[i]),
            })
        } else {
            Err(ParseBoardError)
        }
    }
}

impl FromStr for BoardRow {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<_> = s
            .splitn(N, ' ')
            .map(|s| s.parse::<CellNumber>().map_err(|_| ParseBoardError))
            .try_collect()?;
        if numbers.len() == N {
            Ok(BoardRow {
                cells: core::array::from_fn(|i| numbers[i]),
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
    fn check_lost_won() -> Result<(), ParseBoardError> {
        let alive: Board = "18 9 6 0|0 9 3 0|33 18 18 3|0 0 15 0".parse()?;
        assert!(!alive.is_lost(), "should not be lost");
        assert!(!alive.is_won(), "should not be won");

        let lost: Board = "18 9 0 0|0 9 0 0|33 18 0 3|0 0 15 0".parse()?;
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
        assert!(alive.pos == Point(0, 0), "should start at 0,0");
        {
            let alive2 = alive.action(Action::UP);
            assert!(alive2.is_none(), "should be none for invalid action");
        }
        if let Some(alive3) = alive.action(Action::DOWN) {
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
            assert!(alive_n.pos == Point(1, 1));
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
