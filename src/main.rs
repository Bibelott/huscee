use std::io;

use huscee::{
    moves::{Move, gen_moves},
    *,
};

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
                            board.make_move(Move::new(
                                Coord::from_alg(&mov[..2]).unwrap(),
                                Coord::from_alg(&mov[2..4]).unwrap(),
                                if mov.len() == 4 {
                                    None
                                } else {
                                    Piece::from_char(
                                        mov.chars().nth(4).unwrap().to_ascii_lowercase(),
                                    )
                                    .ok()
                                },
                            ));
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

                        for (mov, n) in split_map {
                            println!(
                                "{}{}{}: {}",
                                mov.orig.to_alg(),
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

                        let moves = gen_moves(&board);

                        let mut best = f32::NEG_INFINITY;
                        let mut best_move = None;

                        for &mov in moves.iter() {
                            let mut board = board.clone();
                            board.make_move(mov);

                            let value =
                                -negamax(f32::NEG_INFINITY, f32::INFINITY, depth - 1, &board);

                            if value > best {
                                best = value;
                                best_move = Some(mov);
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
                            best_move.orig.to_alg(),
                            best_move.dst.to_alg(),
                            best_move.prom_tgt.map_or_else(
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
