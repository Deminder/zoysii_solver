use crate::action::{Action, ActionSequence, ACTIONS};
use crate::board::Board;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct SolveStep {
    board: Board,
    seq: ActionSequence,
}

/**
Perform a breadth-first search to find the shortest path of actions where `board.is_won()`.
Besides pruning `board.is_lost()` this is a brute force search.
*/
pub fn solve_board(board: &Board, max_moves: usize) -> Option<Vec<Action>> {
    assert!(max_moves <= ActionSequence::MAX_LENGTH);
    if board.is_won() {
        return Some(vec![]);
    }
    let mut steps = vec![SolveStep {
        board: *board,
        seq: ActionSequence::new(),
    }];
    let mut moves_remaining = max_moves;
    let mut visited: HashSet<Board> = HashSet::new();
    while steps.len() > 0 && moves_remaining > 0 {
        moves_remaining -= 1;
        let mut next_steps: Vec<SolveStep> = Vec::with_capacity(steps.len() * ACTIONS.len());
        next_steps.par_extend(
            steps
                .par_chunks(10000)
                .flatten()
                .copied()
                .flat_map_iter(|step| {
                    ACTIONS.into_iter().filter_map(move |action| {
                        step.board.action(action).map(|board| SolveStep {
                            board,
                            seq: step.seq.add(action),
                        })
                    })
                })
                .filter(|step| !visited.contains(&step.board) && !step.board.is_lost()),
        );

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
