mod board;
mod coord;
mod moves;

use board::Board;
use coord::Coord;

use std::{error::Error, fmt::Display, io};

use crate::moves::Move;

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
            _ => panic!("AAAAAAAAAAAAAAAA"),
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

fn main() {
    let mut board = Board::start_pos();

    let stdin = io::stdin();

    let mut line = String::new();

    // TODO: Add better error handling
    //
    // TODO: Clean this up
    //
    'line_loop: while stdin.read_line(&mut line).is_ok() {
        let mut line_iter = line.split_whitespace();
        let mut response = "";
        while let Some(first) = line_iter.next() {
            response = match first {
                "uci" => "uciok",
                "ucinewgame" => {
                    board = Board::start_pos();
                    ""
                }
                "isready" => "readyok",
                "position" => {
                    match line_iter.next().expect("Unknown command") {
                        "startpos" => board = Board::start_pos(),
                        "fen" => {
                            let mut fen: String = String::new();
                            for _ in 0..6 {
                                fen.push_str(line_iter.next().unwrap());
                                fen.push(' ');
                            }
                            board = Board::from_fen(&fen).unwrap()
                        }
                        _ => panic!("Unknown command"),
                    }

                    if line_iter.next().is_some_and(|txt| txt == "moves") {
                        for mov in line_iter {
                            board.make_move(
                                Coord::from_alg(&mov[..2]).unwrap(),
                                Move::new(
                                    Coord::from_alg(&mov[2..4]).unwrap(),
                                    if mov.len() == 4 {
                                        None
                                    } else {
                                        Piece::from_char(
                                            mov.chars().nth(4).unwrap().to_ascii_lowercase(),
                                        )
                                        .ok()
                                    },
                                ),
                            );
                        }
                    }

                    ""
                }
                "go" => {
                    match line_iter.next().expect("Unknown command") {
                        "perft" => {
                            let depth = line_iter.next().unwrap().parse().unwrap();
                            let start = std::time::Instant::now();
                            let split_map = board.perft_split(depth);
                            let time = start.elapsed().as_millis();

                            let mut nodes = 0;

                            for ((orig, mov), n) in split_map {
                                println!(
                                    "{}{}{}: {}",
                                    orig.to_alg(),
                                    mov.dst.to_alg(),
                                    mov.prom_tgt.map_or_else(
                                        || "",
                                        |p| match p {
                                            Piece::QueenW | Piece::QueenB => "q",
                                            Piece::KnightW | Piece::KnightB => "n",
                                            Piece::RookW | Piece::RookB => "r",
                                            Piece::BishopW | Piece::BishopB => "b",
                                            _ => unreachable!(),
                                        }
                                    ),
                                    n
                                );
                                nodes += n;
                            }

                            println!("\nSearched {nodes} nodes in {time}ms");
                        }
                        _ => panic!("Unknown command"),
                    }
                    ""
                }
                "quit" => break 'line_loop,
                _ => continue,
            };
            break;
        }

        println!("{response}");

        line.clear();
    }
}
