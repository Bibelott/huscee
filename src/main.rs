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

#[derive(Clone, PartialEq, Eq, Debug)]
struct Board {
    board: [Piece; 128],
    to_move: Turn,
    castling: [bool; 4],
    en_pass_tgt: Option<u8>,
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

    fn from_fen(fen: &str) -> Self {
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

                _ => panic!("Invalid FEN string"),
            };

            board[idx] = piece;
            idx += 1;
        }

        let turn = it.next().unwrap();

        let turn = if turn == "w" {
            Turn::White
        } else if turn == "b" {
            Turn::Black
        } else {
            panic!("Invalid FEN string");
        };

        let mut castling = [false; 4];

        for c in it.next().unwrap().chars() {
            match c {
                '-' => break,
                'K' => castling[0] = true,
                'Q' => castling[1] = true,
                'k' => castling[2] = true,
                'q' => castling[3] = true,

                _ => panic!("Invalid FEN string"),
            }
        }

        todo!();
    }
}

fn main() {
    let board = Board::new();
    println!("{:?}", board);
}
