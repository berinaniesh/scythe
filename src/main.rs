#![allow(unused_imports)]

use chess::{Board, MoveGen, Color, BoardStatus, ChessMove, ALL_RANKS, Piece, get_rank};
use std::env;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::{Duration, Instant};
use args::{Args, ArgsError};
use getopts::Occur;
use colored::{ColoredString, Colorize};
mod piece_values;
mod benchmarks;


const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const TEST_FEN: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
const DEFAULT_DEPTH: i64 = 7;
const PROGRAM_DESC: &'static str = "A basic chess engine written in rust";
const PROGRAM_NAME: &'static str = "Scythe";

fn calc_piece_value(pc_idx: usize, sq_idx: usize, color: Option<Color>) -> i64 {
    match color {
        Some(Color::White) => {
            let sq_value = piece_values::PIECE_SQUARES[pc_idx][sq_idx];
            return -(piece_values::PIECE_VALS[pc_idx]+sq_value);
        },
        Some(Color::Black) => {
            let sq_value = piece_values::PIECE_SQUARES[pc_idx][63 - sq_idx];
            return piece_values::PIECE_VALS[pc_idx] + sq_value;
        },
        None => {
            return 0;
        }
    }
}

fn calc_pieces_value(board: &Board) -> i64 {
    let mut result = 0;
    for pc_idx in 0..6 {
        let pc_type = piece_values::PIECES[pc_idx];
        let bboard = *board.pieces(pc_type);
        for square in bboard {
            let sq_idx = square.to_index();
            result += calc_piece_value(pc_idx, sq_idx, board.color_on(square));
        }
    }
    return result;
}

fn calc_board_value(board: &Board) -> i64 {
    let w_move = board.side_to_move() == Color::White;
    let result = match board.status() {
        BoardStatus::Checkmate => if w_move {20000} else {-20000},
        BoardStatus::Stalemate => 0,
        BoardStatus::Ongoing => calc_pieces_values(board);
    };
    return result;
}

fn alpha_beta (
    board: &Board, depth: i8,
    is_max: bool, alpha: i64,
    beta: i64, totatl: &mut i64) -> i64 {
    if (depth == 0) || (board.status() != BoardStatus::Ongoing) {
        *total += 1;
        let val = calc_board_value(board);
        return val;
    }
    let mut alpha = alpha;
    let mut beta = beta;
    if is_max {
        let mut best_value = i64::MIN;
        let moves = MoveGen::new_legal(&board);
        let mut result_board = chess::Board::default();
        for mv in moves {
            board.make_move(mv, &mut result_board);
            let value = alpha_beta(&result_board, depth-1, false, alpha, beta, best_val = std::cmp::max(alpha, best_val);
            alpha = std::cmp::max(alpha, best_val);
            if beta <= alpha {
                break;
            }
        }
        return best_val;
        } 
    else {
        let mut best_val = i64::MAX;
        let moves = MoveGen::new_legal(&board);
        let mut result_board = chess::Board::default();
        for mv in moves {
            board.make_move(mv, &mut result_board);
            let value  = alpha_beta(&result_board, depth-1, true, alpha, beta, best_val = std::cmp::min(value, best_val);
            beta = std::cmp::min(beta, best_val);
            if beta <= alpha {
                break;
            }
        }
        return best_val;
    }
}

fn show_board(board: Board) {
    for(&rank, lbl) in ALL_RANKS.iter().zip("12345678".chars()) {
        print!("{}", lbl);
        print!(" ");
        for sq in get_rank(rank) {


fn main() {
    println!("Hello");
}
