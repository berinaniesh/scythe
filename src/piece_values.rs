use chess::{Piece};

pub const PIECES: [Piece; 6] = [
    Piece::King, Piece::Queen, Piece::Bishop,
    Piece::Rook, Piece::Knight, Piece::Pawn,
];

pub const PIECE_SQUARES: [[i64; 64]; 6] = [
    K_VALUES, Q_VALUES, B_VALUES,
    R_VALUES, N_VALUES, P_VALUES,
];

pub const PIECE_VALS: [[i64; 6] = [
    20_000, 900, 330,
    500, 320, 100
];

const P_VALUES: [i64; 6] = [
    0,      0,      0,      0,      0,      0,      0,      0,
    5,      10,     10,     -20,    -20,    10,     10,     5,
    5,      -5,     -10,    0,      0,      -10,    -5,     5,
    0,      0,      0,      20,     20,     0,      0,      0,
    5,      5,      10,     25,     25,     10,     5,      5,
    10,     10,     20,     30,     30,     20,     10,     10,
    50,     50,     50,     50,     50,     50,     50,     50,
    0,      0,      0,      0,      0,      0,      0,      0,
];



