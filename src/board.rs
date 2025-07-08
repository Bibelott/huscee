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
    pub to_move: Turn,
    pub castling: [bool; 4],
    pub en_pass_tgt: Option<Coord>,
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
            let piece = if let Ok(p) = Piece::from_char(p) {
                p
            } else {
                match p {
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
                }
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

impl std::ops::Index<Coord> for Board {
    type Output = Piece;
    fn index(&self, index: Coord) -> &Self::Output {
        &self.board[index.0 as usize]
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

        assert_eq!(board.to_move, Turn::White);

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
            Turn::Black
        );
    }
}
