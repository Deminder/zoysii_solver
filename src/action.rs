use crate::values::{Point, N};
use std::fmt;
use std::mem;
use std::ops;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Action {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub const ACTIONS: [Action; 4] = [Action::UP, Action::DOWN, Action::LEFT, Action::RIGHT];

impl ops::Add<Action> for Point {
    type Output = Self;

    fn add(self, rhs: Action) -> Self {
        let row = self.row();
        let col = self.column();
        Point::from(
            match rhs {
                Action::LEFT | Action::RIGHT => row,
                Action::UP if row > 0 => row - 1,
                Action::DOWN => row + 1,
                _ => N,
            },
            match rhs {
                Action::UP | Action::DOWN => col,
                Action::LEFT if col > 0 => col - 1,
                Action::RIGHT => col + 1,
                _ => N,
            },
        )
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Action::UP => "Up",
            Action::DOWN => "Down",
            Action::LEFT => "Left",
            Action::RIGHT => "Right",
        };
        write!(f, "{name}")
    }
}

type Seq = u64;
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ActionSequence(Seq);
const SEQ_BITS: usize = mem::size_of::<ActionSequence>() * 8;
const LEN_BITS: usize = 6;
const ACTION_BITS: usize = 2;
const ACTION_MASK: Seq = 0x03;

impl ActionSequence {
    pub const MAX_LENGTH: usize = (SEQ_BITS - LEN_BITS) / ACTION_BITS;

    pub fn new() -> Self {
        Self(0)
    }

    pub fn length(&self) -> usize {
        (0x3F & self.0) as usize
    }

    pub fn add(self, action: Action) -> ActionSequence {
        let len = self.length();
        debug_assert!(
            len + 1 <= Self::MAX_LENGTH,
            " should be lower than max length"
        );
        Self(
            self.0 ^ ((len ^ (len + 1)) as Seq)
                | match action {
                    Action::UP => 0,
                    Action::DOWN => 1,
                    Action::LEFT => 2,
                    Action::RIGHT => 3,
                } << (ACTION_BITS * len + LEN_BITS),
        )
    }

    pub fn get(&self, index: usize) -> Action {
        ACTIONS[((self.0 >> (index * ACTION_BITS + LEN_BITS)) & ACTION_MASK) as usize]
    }
}

impl From<ActionSequence> for Vec<Action> {
    fn from(value: ActionSequence) -> Self {
        (0..value.length())
            .into_iter()
            .map(|i| value.get(i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn add_and_get_actions() {
        let mut seq = ActionSequence::new();
        let actions = vec![
            Action::DOWN,
            Action::DOWN,
            Action::UP,
            Action::UP,
            Action::LEFT,
            Action::RIGHT,
        ];
        // should be immutable sequence
        seq.add(Action::RIGHT);
        assert_eq!(seq, ActionSequence::new());

        for a in actions.iter() {
            println!("adding {} to seq of length: {}", a, seq.length());
            seq = seq.add(*a);
        }
        let seq_actions: Vec<Action> = seq.into();
        assert_eq!(seq_actions, actions);

        seq = ActionSequence::new();
        for _ in 0..ActionSequence::MAX_LENGTH {
            seq = seq.add(Action::RIGHT);
        }
        let long_seq: Vec<Action> = seq.into();
        assert_eq!(
            long_seq,
            (0..ActionSequence::MAX_LENGTH)
                .into_iter()
                .map(|_| Action::RIGHT)
                .collect_vec()
        );
    }

    #[test]
    fn add_action_to_point() {
        let point = Point::from(1, 1);
        println!("{point}");
        assert!(point.inside());
        assert_eq!(point + Action::DOWN, Point::from(2, 1));
        assert_eq!(point + Action::UP, Point::from(0, 1));
        assert_eq!(point + Action::RIGHT, Point::from(1, 2));
        assert_eq!(point + Action::LEFT, Point::from(1, 0));
        assert!(!(point + Action::LEFT + Action::LEFT).inside());
        assert!(!(point + Action::UP + Action::UP).inside());

        let point2 = Point::from(2, 2);
        println!("{point2}");
        assert!(point2.inside());
        assert_eq!(point2 + Action::DOWN, Point::from(3, 2));
        assert_eq!(point2 + Action::UP, Point::from(1, 2));
        assert_eq!(point2 + Action::RIGHT, Point::from(2, 3));
        assert_eq!(point2 + Action::LEFT, Point::from(2, 1));
        assert!(!(point2 + Action::RIGHT + Action::RIGHT).inside());
        assert!(!(point2 + Action::DOWN + Action::DOWN).inside());
    }
}
