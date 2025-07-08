mod board;
mod coord;
mod moves;

use board::Board;
use coord::Coord;

use std::{error::Error, fmt::Display};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    fn from_char(value: char) -> Result<Self, ()> {
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

    fn is_white(self) -> bool {
        self != Piece::Empty && ((self as u8) & 8) == 0
    }

    fn is_black(self) -> bool {
        self != Piece::Empty && !self.is_white()
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

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Turn {
    White = 0,
    Black = 1,
}

fn main() {
    let board = Board::start_pos();
    println!("{board:?}");
}
