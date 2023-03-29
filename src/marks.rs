use crate::action::{Action, ACTIONS};
use crate::values::{BoardLike, Point, Sym, Transform, N};
use itertools::join;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MarkBoard(u16);

fn fmt_board(f: &mut fmt::Formatter, cell_fn: impl Fn(Point) -> String) -> fmt::Result {
    (0..N)
        .into_iter()
        .map(|r| join((0..N).into_iter().map(|c| cell_fn(Point::from(r, c))), ""))
        .map(|s| write!(f, "\n{}", s))
        .find(|r| r.is_err())
        .unwrap_or(Ok(()))
}

impl fmt::Display for MarkBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_board(f, |p| if self.marked(p) { "|#|" } else { "| |" }.into())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct ActionBoard {
    end: Point,
    actions: u32,
    starts: MarkBoard,
}

impl fmt::Display for ActionBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_board(f, |p| {
            if let Some(a) = self.action_by_pos(p) {
                format!("|{a}|")
            } else if p == self.end {
                "|X|".into()
            } else {
                "| |".into()
            }
        })
    }
}

type ActionBoardMap = HashMap<Point, ActionBoard>;

type MarkBoardMap = HashMap<MarkBoard, ActionBoardMap>;

static ACTION_BOARDS: Lazy<MarkBoardMap> = Lazy::new(|| {
    let mut seen = HashSet::<MarkBoard>::new();
    let b = (0..((1 << 16) - 1) as usize)
        .into_iter()
        .filter_map(|i| {
            let marks = MarkBoard(i as u16);
            if seen.insert(marks) {
                for sym in marks.all_symmeries() {
                    seen.insert(marks.symmetry(sym));
                }
                Some((
                    marks,
                    Point::iter_all()
                        .filter(|&p| {
                            // Check if point is a valid end point
                            marks.marked(p)
                                // The end point should be reachable from zero cells
                                && ACTIONS.into_iter().any(|a| {
                                    let neigh = p + a;
                                    neigh.inside() && !marks.marked(neigh)
                                })
                        })
                        .map(|end| (end, ActionBoard::from(marks, end)))
                        .collect(),
                ))
            } else {
                None
            }
        })
        .collect();
    println!("Initialized action boards.");
    b
});

impl ActionBoard {
    fn from(marks: MarkBoard, end: Point) -> Self {
        assert!(marks.marked(end), "end must be marked");
        let mut actions: u32 = 0;
        let mut starts = MarkBoard(0);
        let mut points = vec![end];
        while points.len() > 0 {
            points = ACTIONS
                .into_iter()
                .cartesian_product(points)
                .map(|(a, point)| (a, point + a))
                .filter_map(|(a, point)| {
                    if point.inside() && !marks.marked(point) && !starts.marked(point) {
                        // Set action
                        actions |= (a.reverse().index() as u32) << (point.index() * 2);
                        starts.mark(point);
                        Some(point)
                    } else {
                        None
                    }
                })
                .collect_vec();
        }
        ActionBoard {
            actions,
            starts,
            end,
        }
    }

    pub fn action_by_pos(&self, pos: Point) -> Option<Action> {
        if self.starts.marked(pos) {
            Some(ACTIONS[((self.actions >> (pos.index() * 2)) & 0x3) as usize])
        } else {
            None
        }
    }
}

impl MarkBoard {
    pub fn from(board: &impl BoardLike) -> Self {
        Self(
            Point::iter_all()
                .map(|p| ((board.cell(p) != 0) as u16) << p.index())
                .reduce(|acc, m| acc | m)
                .unwrap(),
        )
    }

    fn marked(&self, p: Point) -> bool {
        debug_assert!(p.inside());
        ((self.0 >> p.index()) & 0x1) != 0
    }

    fn mark(&mut self, p: Point) {
        debug_assert!(p.inside());
        self.0 |= (1 as u16) << p.index()
    }

    #[allow(dead_code)]
    fn unmark(&mut self, p: Point) {
        debug_assert!(p.inside());
        self.0 &= !((1 as u16) << p.index())
    }

    fn all_symmeries(&self) -> impl Iterator<Item = Sym> {
        [
            Transform::Mirror,
            Transform::Deg90,
            Transform::Deg180,
            Transform::Deg270,
        ]
        .into_iter()
        .cartesian_product([true, false].into_iter())
        .map(|(transform, mirror)| Sym { transform, mirror })
    }

    pub fn transform(&self, sym: Transform) -> Self {
        Self(
            Point::iter_all()
                .map(|p| (self.marked(p) as u16) << p.transform(sym).index())
                .reduce(|acc, m| acc | m)
                .unwrap(),
        )
    }

    fn symmetry(&self, sym: Sym) -> Self {
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

    fn action_board_map(&self) -> (Sym, &ActionBoardMap) {
        let (sym, marks) = self
            .all_symmeries()
            .map(|sym| (sym, self.symmetry(sym)))
            .reduce(|(msym, mmarks), (sym, marks)| {
                if marks.0 < mmarks.0 {
                    (sym, marks)
                } else {
                    (msym, mmarks)
                }
            })
            .unwrap();

        (sym, ACTION_BOARDS.get(&marks).unwrap())
    }

    pub fn action_towards(&self, pos: Point, end: Point) -> Option<Action> {
        let (sym, ab) = self.action_board_map();

        ab.get(&end.symmetry(sym))
            .and_then(|b| b.action_by_pos(pos.symmetry(sym)))
            .map(|action| action.reverse_symmetry(sym))
    }

    pub fn find_all_ends_for(&self, pos: Point) -> impl Iterator<Item = Point> + '_ {
        let (sym, ab) = self.action_board_map();

        ab.into_iter().filter_map(move |(p, b)| {
            if b.starts.marked(pos.symmetry(sym)) {
                Some(p.reverse_symmetry(sym))
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn set_and_get_marks() {
        let mut marks = MarkBoard(0);
        let point = Point::from(1, 1);
        assert!(!marks.marked(point));
        marks.mark(point);
        assert!(marks.marked(point));
        marks.unmark(point);
        assert!(!marks.marked(point));
    }

    #[test]
    fn transform_mark_board() {
        let mut marks = MarkBoard(0);
        assert_eq!(marks, marks.transform(Transform::Deg90));
        marks.mark(Point::from(1, 1));
        marks.mark(Point::from(0, 0));
        marks.mark(Point::from(0, 1));
        marks.mark(Point::from(3, 3));
        marks.mark(Point::from(2, 3));
        marks.mark(Point::from(3, 2));
        println!("marks {marks}");
        let rot90 = marks.transform(Transform::Deg90);
        println!("rot90 {rot90}");
        assert_ne!(marks, rot90);
        assert!(!marks.marked(Point::from(2, 1)));
        assert!(
            rot90.marked(Point::from(2, 1)),
            "should rotate point in counter clockwise direction"
        );
        let rot180 = marks.transform(Transform::Deg180);
        println!("rot180 {rot180}");
        assert!(rot180.marked(Point::from(2, 2)));
        let rot360 = marks
            .transform(Transform::Deg270)
            .transform(Transform::Deg90);
        println!("rot360 {rot360}");
        assert_eq!(marks, rot360);

        assert_eq!(
            marks,
            marks
                .transform(Transform::Mirror)
                .transform(Transform::Mirror)
        );
    }

    #[test]
    fn action_board() {
        let mut marks = MarkBoard(0);
        let start = Point::from(0, 0);
        let mid = Point::from(2, 2);
        let top = mid + Action::UP;
        let top_right = top + Action::RIGHT;
        let right = mid + Action::RIGHT;
        let down = mid + Action::DOWN;
        let left = mid + Action::LEFT;

        marks.mark(top);
        let ab1 = ActionBoard::from(marks, top);
        println!("ab1 {}", ab1);
        marks.mark(top_right);
        println!("ab2 {}", ActionBoard::from(marks, top));
        marks.mark(right);
        println!("ab3 {}", ActionBoard::from(marks, top));
        marks.mark(down);
        println!("ab4 {}", ActionBoard::from(marks, top));
        marks.mark(left);
        println!("ab5 {}", ActionBoard::from(marks, top));
        let ab = ActionBoard::from(marks, top);
        let mut pos = start;
        pos = pos + ab.action_by_pos(pos).unwrap();
        pos = pos + ab.action_by_pos(pos).unwrap();
        pos = pos + ab.action_by_pos(pos).unwrap();
        assert_eq!(pos, top, "should reach top in three moves");
        assert_eq!(
            ab.action_by_pos(Point::from(0, 3)),
            Some(Action::LEFT),
            "should choose left"
        );
        assert_eq!(ab.action_by_pos(mid), Some(Action::UP), "should choose up");
    }

    #[test]
    fn action_board_find_ends() {
        for i in 0..(1 << 16) as usize {
            let marks = MarkBoard(i as u16);
            let mut true_end_points = HashSet::<Point>::new();
            let end_points = Point::iter_all()
                .filter(|&p| {
                    true_end_points.insert(p);
                    !marks.marked(p)
                })
                .flat_map(|p| marks.find_all_ends_for(p))
                .collect::<HashSet<Point>>();
            if true_end_points.len() > 0 && true_end_points.len() < 16 {
                assert!(end_points.len() > 0);
            }
            assert!(end_points.is_subset(&true_end_points));
        }
    }

    #[test]
    fn action_board_lookups() {
        println!("Cached ActionBoardMaps: {}", ACTION_BOARDS.len());
        let mut marks = MarkBoard(0);
        let start = Point::from(0, 0);
        let mid = Point::from(2, 2);
        let top = mid + Action::UP;
        let top_right = top + Action::RIGHT;
        let right = mid + Action::RIGHT;
        let down = mid + Action::DOWN;
        let left = mid + Action::LEFT;
        println!("{top},{right}, {top_right}, {right}, {down}, {left}");

        assert_eq!(
            marks.action_towards(start, top),
            None,
            "should not have action board for 0 < 2 marks"
        );
        marks.mark(top);
        println!("marks {marks}");
        let (sym, ab) = marks.action_board_map();
        println!(
            "ActionBoardMap {sym}: {}",
            ab.get(&top.symmetry(sym)).unwrap()
        );

        let mut pos = start;
        pos = pos + marks.action_towards(pos, top).unwrap();
        pos = pos + marks.action_towards(pos, top).unwrap();
        pos = pos + marks.action_towards(pos, top).unwrap();
        assert_eq!(pos, top, "should reach top in three moves");
        marks.mark(top_right);

        pos = start;
        pos = pos + marks.action_towards(pos, top_right).unwrap();
        pos = pos + marks.action_towards(pos, top_right).unwrap();
        pos = pos + marks.action_towards(pos, top_right).unwrap();
        pos = pos + marks.action_towards(pos, top_right).unwrap();
        assert_eq!(pos, top_right, "should reach top-right in four moves");
        assert_eq!(
            marks.action_towards(top, top),
            None,
            "should not be able to put start into end"
        );
        marks.mark(right);
        marks.mark(down);

        println!("marks2 {marks}");
        let (sym, ab) = marks.action_board_map();
        println!("markssym {}", marks.symmetry(sym));
        for (p, a) in Point::iter_all().filter_map(|p| ab.get(&p).map(|a| (p, a))) {
            println!("{p} {a}");
        }
        println!(
            "ActionBoardMap2 {sym}: {}",
            ab.get(&top.symmetry(sym)).unwrap()
        );
        assert_eq!(marks.action_towards(mid, top), Some(Action::UP));
        assert_eq!(marks.action_towards(mid, down), Some(Action::DOWN));
        assert_eq!(marks.action_towards(mid, right), Some(Action::RIGHT));
        assert_eq!(
            marks.action_towards(mid, top_right),
            Some(Action::LEFT),
            "should choose left (around)"
        );
        marks.mark(left);
        assert_eq!(
            marks.action_towards(mid, top_right),
            None,
            "should have no path to top_right (from mid)"
        );
    }
}
