use crate::action::Action;
use std::rc::Rc;
use std::collections::HashSet;
use crate::board::Board;

#[derive(Clone, Debug)]
struct ActionSequence {
    prev: Option<Rc<ActionSequence>>,
    action: Action,
}

impl From<&ActionSequence> for Vec<Action> {
    fn from(value: &ActionSequence) -> Self {
        let mut v = value.clone();
        let mut actions = vec![v.action];
        while let Some(z) = v.prev {
            actions.push(z.action);
            v = z.as_ref().clone();
        }
        actions.reverse();
        actions
    }
}

struct SolveStep {
    board: Box<Board>,
    seq: Rc<ActionSequence>,
}

impl SolveStep {
    pub fn next_steps(&self) -> Vec<SolveStep> {
        [Action::UP, Action::DOWN, Action::LEFT, Action::RIGHT]
            .into_iter()
            .filter_map(|action| {
                self.board.action(action).map(|b| SolveStep {
                    board: Box::new(b),
                    seq: Rc::new(ActionSequence {
                        prev: Some(Rc::clone(&self.seq)),
                        action,
                    }),
                })
            })
            .collect()
    }
}


/**
Perform a breadth-first search to find the shortest path of actions where `board.is_won()`.
Besides pruning `board.is_lost()` this is a brute force search.
*/
pub fn solve_board(board: &Board, max_moves: u8) -> Option<Vec<Action>> {
    let mut steps = vec![SolveStep {
        board: Box::new(*board),
        seq: Rc::new(ActionSequence {
            prev: None,
            action: Action::START,
        }),
    }];
    let mut moves_remaining = max_moves;
    let mut visited: HashSet<Board> = HashSet::new();
    while steps.len() > 0 && moves_remaining > 0 {
        moves_remaining -= 1;
        let next_steps: Vec<_> = steps
            .iter()
            .flat_map(SolveStep::next_steps)
            .filter(|step| !visited.contains(&step.board) && !step.board.is_lost())
            .collect();
        if let Some(solution) = next_steps.iter().find(|step| step.board.is_won()) {
            let sboard = Rc::clone(&solution.seq);
            let actions: Vec<Action> = sboard.as_ref().into();
            return Some(actions);
        }
        for step in steps.iter() {
            visited.insert(*step.board);
        }
        steps = next_steps;
    }
    None
}
