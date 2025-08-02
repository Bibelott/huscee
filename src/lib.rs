pub mod board;
pub mod coord;
pub mod moves;

pub use board::Board;
pub use coord::Coord;

use std::{error::Error, fmt::Display};

use crate::moves::{gen_at_coord_illegal, gen_moves};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    fn flip(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::White,
            1 => Self::Black,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Piece {
    Empty = 0,

    PawnW = 1,
    RookW = 2,
    KnightW = 3,
    BishopW = 4,
    QueenW = 5,
    KingW = 6,

    PawnB = 9,
    RookB = 10,
    KnightB = 11,
    BishopB = 12,
    QueenB = 13,
    KingB = 14,
}

impl Piece {
    pub fn from_char(value: char) -> Result<Self, ()> {
        let p = match value {
            'P' => Piece::PawnW,
            'R' => Piece::RookW,
            'N' => Piece::KnightW,
            'B' => Piece::BishopW,
            'Q' => Piece::QueenW,
            'K' => Piece::KingW,

            'p' => Piece::PawnB,
            'r' => Piece::RookB,
            'n' => Piece::KnightB,
            'b' => Piece::BishopB,
            'q' => Piece::QueenB,
            'k' => Piece::KingB,

            _ => return Err(()),
        };
        Ok(p)
    }

    pub fn is_white(self) -> bool {
        self != Piece::Empty && ((self as u8) & 8) == 0
    }

    pub fn is_black(self) -> bool {
        self != Piece::Empty && !self.is_white()
    }

    pub fn get_color(self) -> Color {
        (((self as u8) & 8) >> 3).into()
    }

    pub fn to_color(self, color: Color) -> Self {
        ((self as u8 & 0b111) | ((color as u8) << 3)).into()
    }
}

impl From<u8> for Piece {
    fn from(value: u8) -> Self {
        match value {
            1 => Piece::PawnW,
            2 => Piece::RookW,
            3 => Piece::KnightW,
            4 => Piece::BishopW,
            5 => Piece::QueenW,
            6 => Piece::KingW,

            9 => Piece::PawnB,
            10 => Piece::RookB,
            11 => Piece::KnightB,
            12 => Piece::BishopB,
            13 => Piece::QueenB,
            14 => Piece::KingB,

            _ => Piece::Empty,
        }
    }
}

impl From<char> for Piece {
    fn from(value: char) -> Self {
        Self::from_char(value).unwrap()
    }
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        match value {
            Piece::PawnW => 'P',
            Piece::RookW => 'R',
            Piece::KnightW => 'N',
            Piece::BishopW => 'B',
            Piece::QueenW => 'Q',
            Piece::KingW => 'K',

            Piece::PawnB => 'p',
            Piece::RookB => 'r',
            Piece::KnightB => 'n',
            Piece::BishopB => 'b',
            Piece::QueenB => 'q',
            Piece::KingB => 'k',

            Piece::Empty => ' ',
        }
    }
}

pub struct AbortedError;

pub fn negamax(
    alpha: f32,
    beta: f32,
    depth: u8,
    board: &Board,
    start: &std::time::Instant,
    max_time: u128,
) -> Result<f32, AbortedError> {
    if start.elapsed().as_millis() > max_time {
        return Err(AbortedError);
    }

    let my_color = board.to_move;

    if depth == 0 {
        let mut heuristic = 0.0;

        for rank in 0..8 {
            for file in 0..8 {
                let coord = Coord::from_rf(rank, file).unwrap();

                let piece = board[coord];

                let mut value = match piece {
                    Piece::PawnW | Piece::PawnB => 1.0,

                    Piece::KnightW | Piece::KnightB | Piece::BishopW | Piece::BishopB => 3.0,

                    Piece::RookW | Piece::RookB => 5.0,

                    Piece::QueenW | Piece::QueenB => 9.0,

                    _ => continue,
                };

                if piece.to_color(Color::White) == Piece::KingW {
                    value += file.abs_diff(4) as f32 / 5.0;

                    if rank != 0 && rank != 7 {
                        value -= 3.0;
                    }
                } else if piece == Piece::PawnW {
                    value += coord.rank() as f32 / 7.0;
                } else if piece == Piece::PawnB {
                    value += (7 - coord.rank()) as f32 / 7.0
                } else {
                    let mut moves = Vec::new();
                    gen_at_coord_illegal(board, coord, &mut moves);
                    value += moves.len() as f32 / 10.0;
                }

                if piece.get_color() != my_color {
                    value = -value;
                }

                heuristic += value;
            }
        }

        return Ok(heuristic);
    }
    let moves = gen_moves(board);

    if moves.is_empty() {
        let mut board = board.clone();
        board.to_move = board.to_move.flip();
        let moves = gen_moves(&board);

        return if board.check_check(&moves, my_color.flip()) {
            Ok(f32::NEG_INFINITY)
        } else {
            Ok(0.0)
        };
    }

    let mut alpha = alpha;

    for mov in moves {
        let mut board = board.clone();
        board.make_move(mov);

        let value = -negamax(-beta, -alpha, depth - 1, &board, start, max_time)?;

        if value >= beta {
            return Ok(beta);
        }

        if value > alpha {
            alpha = value;
        }
    }

    Ok(alpha)
}
