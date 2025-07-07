use std::{error::Error, fmt::Display};

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
enum Piece {
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
    fn is_white(self) -> bool {
        self != Piece::Empty && ((self as u8) & 8) == 0
    }

    fn is_black(self) -> bool {
        self != Piece::Empty && !self.is_white()
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
        let rank = (chars.next().unwrap() as usize) - ('0' as usize);
        let file = (chars.next().unwrap() as usize) - ('a' as usize);

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

#[derive(Debug)]
struct InvalidFenStringError {
    fen: String,
}

impl InvalidFenStringError {
    fn new(fen: &str) -> Self {
        Self {
            fen: String::from(fen),
        }
    }
}

impl Error for InvalidFenStringError {}

impl Display for InvalidFenStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid FEN String {}", self.fen)
    }
}

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, PartialEq, Eq, Debug)]
struct Board {
    board: [Piece; 128],
    to_move: Turn,
    castling: [bool; 4],
    en_pass_tgt: Option<Coord>,
}

impl Board {
    fn new() -> Self {
        Self {
            board: [Piece::Empty; 128],
            to_move: Turn::White,
            castling: [true; 4],
            en_pass_tgt: None,
        }
    }

    fn from_fen(fen: &str) -> Result<Self, InvalidFenStringError> {
        let mut board = [Piece::Empty; 128];

        let mut idx: usize = 0;

        let mut it = fen.split_ascii_whitespace();

        for p in it.next().unwrap().chars() {
            let piece = match p {
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

                '0'..='8' => {
                    idx += p.to_digit(10).unwrap() as usize;
                    continue;
                }

                '/' => {
                    idx += 8;
                    continue;
                }

                ' ' => break,

                _ => return Err(InvalidFenStringError::new(fen)),
            };

            board[idx] = piece;
            idx += 1;
        }

        let to_move = it.next().unwrap();

        let to_move = if to_move == "w" {
            Turn::White
        } else if to_move == "b" {
            Turn::Black
        } else {
            return Err(InvalidFenStringError::new(fen));
        };

        let mut castling = [false; 4];

        for c in it.next().unwrap().chars() {
            match c {
                '-' => break,
                'K' => castling[0] = true,
                'Q' => castling[1] = true,
                'k' => castling[2] = true,
                'q' => castling[3] = true,

                _ => return Err(InvalidFenStringError::new(fen)),
            }
        }

        let en_pass_tgt = Coord::from_alg(it.next().unwrap()).ok();

        Ok(Self {
            board,
            to_move,
            castling,
            en_pass_tgt,
        })
    }

    fn start_pos() -> Self {
        Self::from_fen(START_POS).unwrap()
    }
}

fn main() {
    let board = Board::start_pos();
    println!("{:?}", board);
}
