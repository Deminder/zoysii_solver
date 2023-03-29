mod action;
mod board;
mod solve;
mod marks;
mod values;

use board::Board;
use action::ActionSequence;
use clap::Parser;
use itertools::join;
use solve::solve_board;
use std::io;
use std::process::exit;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Max number of moves
    #[arg(short, long, default_value_t = 20)]
    moves: usize,

    /// Read boards as lines from stdin
    #[arg(short, long)]
    stdin: bool,

    #[arg(help = "Example: \"18 9 6 0|0 9 3 0|33 18 18 3|0 0 15 0\"")]
    board: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    if args.moves > ActionSequence::MAX_LENGTH {
        eprintln!("Invalid: Max supported moves: {}", ActionSequence::MAX_LENGTH);
        exit(1);
    }
    if args.stdin {
        let lines = io::stdin().lines();
        for line_r in lines {
            match line_r {
                Ok(line) => {
                    if let Ok(board) = line.trim().parse::<Board>() {
                        if let Some(actions) = solve_board(&board, args.moves) {
                            let action_str = join(&actions, ",");
                            println!("{action_str}");
                        } else {
                            println!("X");
                        }
                    } else {
                        eprintln!("Invalid: Failed to parse board!");
                        exit(2);
                    }
                }
                Err(e) => eprintln!("Error: {e}"),
            }
        }
    } else if args.board.len() > 0 {
        for board_str in args.board {
            if let Ok(board) = board_str.parse::<Board>() {
                if let Some(actions) = solve_board(&board, args.moves) {
                    let action_str = join(&actions, ", ");
                    println!("Solution with {} moves: {action_str}", actions.len());
                } else {
                    println!("No solution!");
                }
            } else {
                eprintln!("Invalid: Failed to parse board!");
                exit(2);
            }
        }
    } else {
        println!("No board to solve. Try --help.");
        exit(3);
    }
}
