use crate::moves::{Move, MoveDict};

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
    pub board: [Piece; 128],
    pub to_move: Color,
    pub castling: [bool; 4],
    pub en_pass_tgt: Option<Coord>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [Piece::Empty; 128],
            to_move: Color::White,
            castling: [true; 4],
            en_pass_tgt: None,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, InvalidFenStringError> {
        let mut board = [Piece::Empty; 128];

        let mut idx: usize = 119;

        let mut it = fen.split_ascii_whitespace();

        for p in it.next().unwrap().chars() {
            let piece = if let Ok(p) = Piece::from_char(p) {
                p
            } else {
                match p {
                    '0'..='8' => {
                        idx -= p.to_digit(10).unwrap() as usize;
                        continue;
                    }

                    '/' => {
                        idx -= 8;
                        continue;
                    }

                    ' ' => break,

                    _ => return Err(InvalidFenStringError::new(fen)),
                }
            };

            board[idx] = piece;

            if idx > 0 {
                idx -= 1;
            } else {
                break;
            }
        }

        let to_move = it.next().unwrap();

        let to_move = if to_move == "w" {
            Color::White
        } else if to_move == "b" {
            Color::Black
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

    pub fn make_move(&mut self, orig: Coord, mov: Move) {
        let piece = self[orig];

        self[mov.dst] = mov.prom_tgt.unwrap_or(self[orig]);
        self[orig] = Piece::Empty;

        if let Some(tgt) = self.en_pass_tgt
            && mov.dst == tgt
        {
            if piece == Piece::PawnW {
                self[mov.dst.add((-1, 0)).unwrap()] = Piece::Empty;
            } else if piece == Piece::PawnB {
                self[mov.dst.add((1, 0)).unwrap()] = Piece::Empty;
            }
        }

        let (r, f) = orig.to_rf();

        // Move the rook when castling
        if piece.to_color(Color::White) == Piece::KingW && f == 4 {
            if mov.dst.file() == 2 {
                self.make_move(
                    (r, 0).try_into().unwrap(),
                    Move::new((r, 3).try_into().unwrap(), None),
                );
            } else if mov.dst.file() == 6 {
                self.make_move(
                    (r, 7).try_into().unwrap(),
                    Move::new((r, 5).try_into().unwrap(), None),
                );
            }
        }

        // Update castling possibilty array
        if piece.to_color(Color::White) == Piece::KingW {
            self.castling[2 * piece.get_color() as usize] = false;
            self.castling[2 * piece.get_color() as usize + 1] = false;
        } else if piece.to_color(Color::White) == Piece::RookW {
            if f == 7 {
                self.castling[2 * piece.get_color() as usize] = false;
            } else if f == 0 {
                self.castling[2 * piece.get_color() as usize + 1] = false;
            }
        }
    }

    pub fn check_check(&self, move_dict: &MoveDict, color: Color) -> bool {
        for moves in move_dict.0.values() {
            for mov in moves {
                if self[mov.dst] == Piece::KingW.to_color(color) {
                    return true;
                }
            }
        }

        false
    }
}

impl std::ops::Index<Coord> for Board {
    type Output = Piece;
    fn index(&self, index: Coord) -> &Self::Output {
        &self.board[index.0 as usize]
    }
}

impl std::ops::IndexMut<Coord> for Board {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.board[index.0 as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_pos() {
        let board = Board::start_pos();

        assert_eq!(
            board.board,
            [
                Piece::RookB,
                Piece::KnightB,
                Piece::BishopB,
                Piece::QueenB,
                Piece::KingB,
                Piece::BishopB,
                Piece::KnightB,
                Piece::RookB,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::PawnB,
                Piece::PawnB,
                Piece::PawnB,
                Piece::PawnB,
                Piece::PawnB,
                Piece::PawnB,
                Piece::PawnB,
                Piece::PawnB,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::PawnW,
                Piece::PawnW,
                Piece::PawnW,
                Piece::PawnW,
                Piece::PawnW,
                Piece::PawnW,
                Piece::PawnW,
                Piece::PawnW,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::RookW,
                Piece::KnightW,
                Piece::BishopW,
                Piece::QueenW,
                Piece::KingW,
                Piece::BishopW,
                Piece::KnightW,
                Piece::RookW,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty, // thank you rustfmt for this incredibly readable piece of code
            ]
        );

        assert_eq!(board.to_move, Color::White);

        assert_eq!(board.castling, [true, true, true, true]);

        assert_eq!(board.en_pass_tgt, None);
    }

    #[test]
    fn test_castle_array() {
        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qkq - 0 1")
                .unwrap()
                .castling,
            [false, true, true, true]
        );

        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kkq - 0 1")
                .unwrap()
                .castling,
            [true, false, true, true]
        );

        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQq - 0 1")
                .unwrap()
                .castling,
            [true, true, false, true]
        );

        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQk - 0 1")
                .unwrap()
                .castling,
            [true, true, true, false]
        );

        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1")
                .unwrap()
                .castling,
            [false, false, false, false]
        );
    }

    #[test]
    fn test_en_passant() {
        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - e3 0 1")
                .unwrap()
                .en_pass_tgt,
            Some(Coord(0x24))
        );
    }

    #[test]
    fn test_turn() {
        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - e3 0 1")
                .unwrap()
                .to_move,
            Color::Black
        );
    }
}
