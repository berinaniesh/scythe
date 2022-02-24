#![allow(unused_imports)]

us chess::{Board, MoveGen, Color, BoardStatus, ChessMove, ALL_RANKS, Piece, get_rank};
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

fn calc_piece_value(pc_idx: usize, sq_idx: usize, color: Option<Color> -> i64
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

fn main() {
    
}
