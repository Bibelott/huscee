use std::io;
use std::thread;

use huscee::{
    moves::{Move, gen_moves},
    *,
};

fn main() {
    let mut board = Board::start_pos();

    let stdin = io::stdin();

    let mut line = String::new();

    let mut is_searching = false;
    let mut search_thread: Option<thread::JoinHandle<()>> = None;
    let mut search_time_max = 0;

    // TODO: Add better error handling
    //
    // TODO: Clean this up
    //
    'line_loop: while stdin.read_line(&mut line).is_ok() {
        let mut line_iter = line.split_whitespace().peekable();
        while let Some(first) = line_iter.next() {
            match first {
                "uci" => println!("uciok"),
                "ucinewgame" => {
                    board = Board::start_pos();
                }
                "isready" => {
                    let thread = search_thread.take_if(|t| t.is_finished());
                    if let Some(thread) = thread {
                        thread.join().unwrap();
                        is_searching = false;
                    }

                    println!("readyok");
                }
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
                }
                "go" => {
                    let thread = search_thread.take_if(|t| t.is_finished());
                    if let Some(thread) = thread {
                        thread.join().unwrap();
                        is_searching = false;
                    }

                    if is_searching {
                        continue 'line_loop;
                    }

                    let mut wtime = 0;
                    let mut winc = 0;

                    let mut btime = 0;
                    let mut binc = 0;

                    while line_iter.peek().is_some() {
                        match line_iter.next().unwrap() {
                            "perft" => {
                                let depth = line_iter.next().unwrap().parse().unwrap();
                                let start = std::time::Instant::now();
                                let split_map = board.perft_split(depth);
                                let time = start.elapsed().as_millis();

                                let mut nodes = 0;

                                for (mov, n) in split_map {
                                    println!("{mov}: {n}");
                                    nodes += n;
                                }

                                println!("info string Searched {nodes} nodes in {time}ms");
                                break;
                            }

                            "wtime" => {
                                wtime = line_iter.next().unwrap().parse().unwrap();
                            }

                            "winc" => {
                                winc = line_iter.next().unwrap().parse().unwrap();
                            }

                            "btime" => {
                                btime = line_iter.next().unwrap().parse().unwrap();
                            }

                            "binc" => {
                                binc = line_iter.next().unwrap().parse().unwrap();
                            }

                            _ => break,
                        }
                    }
                    let board = board.clone();

                    let mytime;
                    let myinc;

                    if board.to_move == Color::White {
                        mytime = wtime;
                        myinc = winc;
                    } else {
                        mytime = btime;
                        myinc = binc;
                    }

                    search_time_max = mytime / 20 + myinc / 2;
                    if search_time_max == 0 {
                        break;
                    }

                    println!("info string Searching with max time of {search_time_max}ms");
                    search_thread = Some(thread::spawn(move || {
                        find_best_move(&board, search_time_max);
                    }));
                    is_searching = true;
                }
                "quit" => break 'line_loop,
                _ => continue,
            };
            break;
        }
        line.clear();
    }

    if let Some(thread) = search_thread {
        thread.join().unwrap();
    }
}

fn find_best_move(board: &Board, max_time_ms: u128) {
    let start = std::time::Instant::now();

    let moves = gen_moves(board);

    let mut curr_best = None;

    let mut depth = 1;

    let mut mate_in: i16 = 0;

    // TODO: Draw by 3-fold repetition

    'depth_loop: while start.elapsed().as_millis() < max_time_ms {
        let mut best = f32::NEG_INFINITY;
        let mut best_move = None;

        for &mov in moves.iter() {
            let mut board = board.clone();
            board.make_move(mov);

            if let Ok(value) = negamax(
                f32::NEG_INFINITY,
                f32::INFINITY,
                depth - 1,
                &board,
                &start,
                max_time_ms,
            ) {
                let value = -value;
                if value >= best {
                    best = value;
                    best_move = Some(mov);
                }
            } else {
                println!("info string Search aborted at depth {depth}");
                break 'depth_loop;
            }
        }

        // TODO: Clean this up
        if best == f32::NEG_INFINITY {
            if mate_in == 0 {
                mate_in = depth as i16 / 2;
            }
            println!(
                "info depth {} score mate {} pv {}",
                depth,
                -mate_in,
                best_move.unwrap()
            );
        } else if best == f32::INFINITY {
            if mate_in == 0 {
                mate_in = depth as i16 / 2;
            }
            println!(
                "info depth {} score mate {} pv {}",
                depth,
                mate_in,
                best_move.unwrap()
            );
        } else {
            println!(
                "info depth {} score cp {} pv {}",
                depth,
                (best * 100.0) as i64,
                best_move.unwrap()
            );
        }

        curr_best = best_move;

        depth += 1;
    }

    println!("bestmove {}", curr_best.unwrap());
}
