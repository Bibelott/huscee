use super::*;

use std::error::Error;

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
pub struct InvalidFenStringError {
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    board: [Piece; 128],
    to_move: Turn,
    castling: [bool; 4],
    en_pass_tgt: Option<Coord>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [Piece::Empty; 128],
            to_move: Turn::White,
            castling: [true; 4],
            en_pass_tgt: None,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, InvalidFenStringError> {
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

    pub fn start_pos() -> Self {
        Self::from_fen(START_POS).unwrap()
    }
}
