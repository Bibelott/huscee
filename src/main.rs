mod board;

use std::{error::Error, fmt::Display};

use board::Board;

#[derive(Debug)]
enum CoordRepr<'a> {
    Num(usize),
    Alg(&'a str),
}

#[derive(Debug)]
struct InvalidCoordinateError<'a> {
    coord: CoordRepr<'a>,
}

impl<'a> InvalidCoordinateError<'a> {
    fn new(coord: usize) -> Self {
        Self {
            coord: CoordRepr::Num(coord),
        }
    }

    fn new_alg(alg: &'a str) -> Self {
        Self {
            coord: CoordRepr::Alg(alg),
        }
    }
}

impl<'a> Display for InvalidCoordinateError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.coord {
            CoordRepr::Num(coord) => write!(f, "Invalid Coordinate: {:#x}", coord),
            CoordRepr::Alg(alg) => write!(f, "Invalid Coordinate: {}", alg),
        }
    }
}

impl<'a> Error for InvalidCoordinateError<'a> {}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coord(u8);

impl<'a> Coord {
    fn from_rf(rank: usize, file: usize) -> Result<Self, InvalidCoordinateError<'a>> {
        let val = ((rank) << 4) + (file);
        if val & 0x88 != 0 || val >= 128 {
            return Err(InvalidCoordinateError::new(val));
        }
        Ok(Self(val as u8))
    }

    fn to_rf(self) -> (usize, usize) {
        ((self.0 >> 4) as usize, (self.0 & 7) as usize)
    }

    fn from_alg(alg: &str) -> Result<Self, InvalidCoordinateError> {
        if alg.len() != 2 {
            return Err(InvalidCoordinateError::new_alg(alg));
        }
        let mut chars = alg.chars();
        let file = (chars.next().unwrap() as usize) - ('a' as usize);
        let rank = (chars.next().unwrap() as usize) - ('1' as usize);

        Self::from_rf(rank, file)
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self::from_rf(value.0, value.1).unwrap()
    }
}

impl From<Coord> for (usize, usize) {
    fn from(value: Coord) -> (usize, usize) {
        value.to_rf()
    }
}

fn main() {
    let board = Board::start_pos();
    println!("{board:?}");
}
