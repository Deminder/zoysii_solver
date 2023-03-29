use crate::action::{Action, ActionSequence, ACTIONS};
use crate::board::Board;
use crate::values::Point;
use itertools::chain;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy)]
struct SolveStep {
    board: Board,
    seq: ActionSequence,
    zero_path_end: Option<Point>,
}
enum Choice {
    Free(Action),
    ZeroPath(Point),
}

impl SolveStep {
    pub fn next_choices(&self) -> impl Iterator<Item = Choice> {
        let mut iters = (None, None, None);
        if self.board.at_zero() {
            // Walk on a zero path to some non-zero cell
            if let Some(end) = self.zero_path_end {
                // Follow each SolveStep on their path
                iters.0 = Some([Choice::ZeroPath(end)].into_iter());
            } else {
                // New SolveStep for each path
                iters.1 = Some(
                    self.board
                        .zero_walk_endings()
                        .into_iter()
                        .map(|end| Choice::ZeroPath(end)),
                );
            }
        } else {
            // Walk freely
            iters.2 = Some(ACTIONS.into_iter().map(|action| Choice::Free(action)))
        }
        chain!(
            iters.0.into_iter().flatten(),
            iters.1.into_iter().flatten(),
            iters.2.into_iter().flatten(),
        )
    }
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
        zero_path_end: None,
    }];
    let mut moves_remaining = max_moves;
    let mut visited: HashSet<Board> = HashSet::new();
    while steps.len() > 0 && moves_remaining > 0 {
        moves_remaining -= 1;
        let mut next_steps: Vec<SolveStep> = Vec::with_capacity(steps.len() * ACTIONS.len());
        next_steps.par_extend(
            steps
                .par_iter()
                .copied()
                .flat_map_iter(move |step| {
                    step.next_choices()
                        .into_iter()
                        .filter_map(move |choice| match choice {
                            Choice::Free(action) => {
                                step.board.action(action).map(|board| SolveStep {
                                    board,
                                    seq: step.seq.add(action),
                                    zero_path_end: None,
                                })
                            }
                            Choice::ZeroPath(end) => {
                                step.board
                                    .move_towards(end)
                                    .map(|(action, board)| SolveStep {
                                        board,
                                        seq: step.seq.add(action),
                                        zero_path_end: if board.at_point(end) {
                                            None
                                        } else {
                                            Some(end)
                                        },
                                    })
                            }
                        })
                })
                .filter(|step| !visited.contains(&step.board) && !step.board.is_lost()),
        );

        if let Some(solution) = next_steps.iter().find(|step| step.board.is_won()) {
            println!("Visited: {}", visited.len());
            return Some(solution.seq.into());
        }
        for step in steps.iter() {
            visited.insert(step.board);
        }
        steps = next_steps;
    }
    None
}
