use crate::action::{Action, ActionSequence, ACTIONS};
use crate::board::Board;
use std::collections::HashSet;

struct SolveStep {
    board: Board,
    seq: ActionSequence,
}

impl SolveStep {
    pub fn next_steps(&self) -> Vec<SolveStep> {
        ACTIONS
            .into_iter()
            .filter_map(|action| {
                self.board.action(action).map(|board| SolveStep {
                    board,
                    seq: self.seq.add(action),
                })
            })
            .collect()
    }
}

/**
Perform a breadth-first search to find the shortest path of actions where `board.is_won()`.
Besides pruning `board.is_lost()` this is a brute force search.
*/
pub fn solve_board(board: &Board, max_moves: usize) -> Option<Vec<Action>> {
    assert!(max_moves <= ActionSequence::MAX_LENGTH);
    let mut steps = vec![SolveStep {
        board: *board,
        seq: ActionSequence::new(),
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
            return Some(solution.seq.into());
        }
        for step in steps.iter() {
            visited.insert(step.board);
        }
        steps = next_steps;
    }
    None
}
