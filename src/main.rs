mod board;
mod coord;
mod moves;

use board::Board;
use coord::Coord;

use std::{error::Error, fmt::Display, io};

use crate::moves::{Move, MoveDict};

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
        let mut response = String::from("");
        while let Some(first) = line_iter.next() {
            response = match first {
                "uci" => String::from("uciok"),
                "ucinewgame" => {
                    board = Board::start_pos();
                    "".into()
                }
                "isready" => "readyok".into(),
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

                    "".into()
                }
                "go" => match line_iter.next().unwrap_or("") {
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
                        "".into()
                    }
                    depth => {
                        let depth = depth.parse::<u8>().unwrap();
                        let start = std::time::Instant::now();

                        let move_dict = MoveDict::gen_moves(&board);

                        let mut best = f32::NEG_INFINITY;
                        let mut best_move = None;

                        for (orig, moves) in move_dict.0 {
                            for mov in moves {
                                let mut board = board.clone();
                                board.make_move(orig, mov);

                                let value =
                                    -negamax(f32::NEG_INFINITY, f32::INFINITY, depth - 1, &board);

                                if value > best {
                                    best = value;
                                    best_move = Some((orig, mov));
                                }
                            }
                        }
                        let best_move = best_move.expect("No moves :(");

                        println!(
                            "Found move with {} valuation in {}ms",
                            best,
                            start.elapsed().as_millis()
                        );

                        format!(
                            "bestmove {}{}{}",
                            best_move.0.to_alg(),
                            best_move.1.dst.to_alg(),
                            best_move.1.prom_tgt.map_or_else(
                                || "",
                                |p| match p {
                                    Piece::QueenW | Piece::QueenB => "q",
                                    Piece::KnightW | Piece::KnightB => "n",
                                    Piece::RookW | Piece::RookB => "r",
                                    Piece::BishopW | Piece::BishopB => "b",
                                    _ => unreachable!(),
                                }
                            )
                        )
                    }
                },
                "quit" => break 'line_loop,
                _ => continue,
            };
            break;
        }

        println!("{response}");

        line.clear();
    }
}

fn negamax(alpha: f32, beta: f32, depth: u8, board: &Board) -> f32 {
    let my_color = board.to_move;

    let move_dict = MoveDict::gen_moves(board);

    if depth == 0 {
        let mut heuristic = 0.0;

        for rank in 0..8 {
            for file in 0..8 {
                let coord = Coord::from_rf(rank, file).unwrap();

                let mut value = match board[coord] {
                    Piece::PawnW | Piece::PawnB => 1.0,

                    Piece::KnightW | Piece::KnightB | Piece::BishopW | Piece::BishopB => 3.0,

                    Piece::RookW | Piece::RookB => 5.0,

                    Piece::QueenW | Piece::QueenB => 9.0,

                    _ => continue,
                };

                value += if let Some(moves) = move_dict.0.get(&coord) {
                    (moves.len() as f32) / 10.0
                } else {
                    0.0
                };

                if board[coord] == Piece::PawnW {
                    value += coord.rank() as f32 / 10.0;
                } else if board[coord] == Piece::PawnB {
                    value += (7 - coord.rank()) as f32 / 10.0
                }

                if board[coord].get_color() != my_color {
                    value = -value;
                }

                heuristic += value;
            }
        }

        return heuristic;
    }

    if move_dict.0.is_empty() {
        return if board.check_check(&move_dict, my_color.flip()) {
            f32::NEG_INFINITY
        } else {
            0.0
        };
    }

    let mut alpha = alpha;

    for (orig, moves) in move_dict.0 {
        for mov in moves {
            let mut board = board.clone();
            board.make_move(orig, mov);

            let value = -negamax(-beta, -alpha, depth - 1, &board);

            if value >= beta {
                return beta;
            }

            if value > alpha {
                alpha = value;
            }
        }
    }

    alpha
}
