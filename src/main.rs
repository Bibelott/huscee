mod board;
mod coord;
mod moves;

use board::Board;
use coord::Coord;

use std::{error::Error, fmt::Display};

use crate::moves::MoveDict;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    White = 0,
    Black = 1,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::White,
            1 => Self::Black,
            _ => panic!("AAAAAAAAAAAAAAAA"),
        }
    }
}

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
        ((self as u8) | ((color as u8) << 3)).into()
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

fn main() {
    let board = Board::start_pos();

    let move_dict = MoveDict::gen_moves(&board);

    println!("{move_dict:?}");
}
