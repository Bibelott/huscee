use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub orig: Coord,
    pub dst: Coord,
    pub prom_tgt: Option<Piece>,
}

impl Move {
    pub fn new(orig: Coord, dst: Coord, prom_tgt: Option<Piece>) -> Self {
        Self {
            orig,
            dst,
            prom_tgt,
        }
    }
}

pub fn gen_moves(board: &Board) -> Vec<Move> {
    let orig_board = board.clone();

    let moves = gen_moves_illegal(board);

    let mut legal_moves = Vec::with_capacity(moves.len());

    for mov in moves {
        let orig = mov.orig;
        let dst = mov.dst;

        let mut board = orig_board.clone();

        if board[orig].to_color(Color::White) == Piece::KingW {
            let orig_file = orig.file();
            let dst_file = dst.file();

            if orig_file.abs_diff(dst_file) == 2 {
                board.to_move = board.to_move.flip();
                let dict = gen_moves_illegal(&board);
                if board.check_check(&dict, board.to_move) {
                    continue;
                }
                board.to_move = board.to_move.flip();

                let file = orig_file.midpoint(dst_file);
                let mut board = board.clone();
                board.make_move(Move::new(
                    orig,
                    Coord::from_rf(mov.dst.rank() as usize, file as usize).unwrap(),
                    None,
                ));
                let new_moves = gen_moves_illegal(&board);
                if board.check_check(&new_moves, board.to_move) {
                    continue;
                }
            }
        }
        board.make_move(mov);
        let dict = gen_moves_illegal(&board);
        if board.check_check(&dict, board.to_move) {
            continue;
        }

        legal_moves.push(mov);
    }

    legal_moves
}

/// Generates all possible moves of all pieces. Includes moves that expose your king to check (thus the 'illegal' part).
pub fn gen_moves_illegal(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for rank in 0..8 {
        for file in 0..8 {
            let coord = Coord::try_from((rank, file)).unwrap();

            let piece = board[coord];

            if piece == Piece::Empty || piece.get_color() != board.to_move {
                continue;
            }

            gen_at_coord_illegal(board, coord, &mut moves);
        }
    }

    moves
}

/// Generates all possible moves of a piece on the given square. Includes moves that expose
/// your king to check (thus the 'illegal' part). Does not consider turn order.
pub fn gen_at_coord_illegal(board: &Board, coord: Coord, moves: &mut Vec<Move>) {
    let piece = board[coord];

    match piece {
        Piece::PawnW | Piece::PawnB => add_pawn_moves(board, coord, moves),

        Piece::RookW | Piece::RookB => add_rook_moves(board, coord, moves),

        Piece::BishopW | Piece::BishopB => add_bishop_moves(board, coord, moves),

        Piece::QueenW | Piece::QueenB => {
            add_rook_moves(board, coord, moves);
            add_bishop_moves(board, coord, moves);
        }

        Piece::KnightW | Piece::KnightB => add_knight_moves(board, coord, moves),

        Piece::KingW | Piece::KingB => add_king_moves(board, coord, moves),

        Piece::Empty => {}
    }
}

fn add_pawn_moves(board: &Board, orig: Coord, moves: &mut Vec<Move>) {
    let row_add = if board[orig].is_white() { 1 } else { -1 };
    // normal move forward
    if let Ok(c) = orig.add((row_add, 0))
        && board[c] == Piece::Empty
    {
        // promotion
        if c.rank() == 0 || c.rank() == 7 {
            for p in [Piece::QueenW, Piece::KnightW, Piece::BishopW, Piece::RookW] {
                moves.push(Move::new(
                    orig,
                    c,
                    Some(p.to_color(board[orig].get_color())),
                ))
            }
        } else {
            moves.push(Move::new(orig, c, None));
        }

        // first move by 2
        if orig.rank() == if board[orig].is_white() { 1 } else { 6 }
            && let Ok(c2) = orig.add((row_add * 2, 0))
            && board[c2] == Piece::Empty
        {
            moves.push(Move::new(orig, c2, None));
        }
    }

    // capture left/right
    for file_add in [-1, 1] {
        if let Ok(c) = orig.add((row_add, file_add))
            && (((board[c] != Piece::Empty) && (board[c].get_color() != board[orig].get_color()))
                || board.en_pass_tgt.is_some_and(|tgt| c == tgt))
        {
            if c.rank() == 0 || c.rank() == 7 {
                for p in [Piece::QueenW, Piece::KnightW, Piece::BishopW, Piece::RookW] {
                    moves.push(Move::new(
                        orig,
                        c,
                        Some(p.to_color(board[orig].get_color())),
                    ))
                }
            } else {
                moves.push(Move::new(orig, c, None));
            }
        }
    }
}

fn add_rook_moves(board: &Board, orig: Coord, moves: &mut Vec<Move>) {
    // TODO: figure out how to DRY this
    for i in 1..8 {
        let err = add_move(board, moves, orig, (i, 0));
        // break on capture
        if err == Ok(true) || err.is_err() {
            break;
        }
    }

    for i in 1..8 {
        let err = add_move(board, moves, orig, (-i, 0));
        // break on capture
        if err == Ok(true) || err.is_err() {
            break;
        }
    }

    for j in 1..8 {
        let err = add_move(board, moves, orig, (0, j));
        // break on capture
        if err == Ok(true) || err.is_err() {
            break;
        }
    }

    for j in 1..8 {
        let err = add_move(board, moves, orig, (0, -j));
        // break on capture
        if err == Ok(true) || err.is_err() {
            break;
        }
    }
}

fn add_bishop_moves(board: &Board, orig: Coord, moves: &mut Vec<Move>) {
    for i in 1..8 {
        let err = add_move(board, moves, orig, (i, i));
        // break on capture
        if err == Ok(true) || err.is_err() {
            break;
        }
    }

    for i in 1..8 {
        let err = add_move(board, moves, orig, (i, -i));
        // break on capture
        if err == Ok(true) || err.is_err() {
            break;
        }
    }

    for i in 1..8 {
        let err = add_move(board, moves, orig, (-i, i));
        // break on capture
        if err == Ok(true) || err.is_err() {
            break;
        }
    }

    for i in 1..8 {
        let err = add_move(board, moves, orig, (-i, -i));
        // break on capture
        if err == Ok(true) || err.is_err() {
            break;
        }
    }
}

fn add_knight_moves(board: &Board, orig: Coord, moves: &mut Vec<Move>) {
    let jumps = [
        (2, 1),
        (2, -1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
    ];

    for (i, j) in jumps {
        _ = add_move(board, moves, orig, (i, j));
    }
}

fn add_king_moves(board: &Board, orig: Coord, moves: &mut Vec<Move>) {
    for i in -1..=1 {
        for j in -1..=1 {
            let _ = add_move(board, moves, orig, (i, j));
        }
    }

    let color = board[orig].get_color();
    let color_int = color as usize;
    let rank = orig.rank() as usize;

    // Castle kingside
    if board.castling[color_int * 2]
        && board[Coord::try_from((rank, 7)).unwrap()] == Piece::RookW.to_color(color)
        && board[Coord::try_from((rank, 6)).unwrap()] == Piece::Empty
        && board[Coord::try_from((rank, 5)).unwrap()] == Piece::Empty
    {
        add_move(board, moves, orig, (0, 2))
            .expect("Castling destination was empty, but it also wasn't?");
    }

    // Castle queenside
    if board.castling[color_int * 2 + 1]
        && board[Coord::try_from((rank, 0)).unwrap()] == Piece::RookW.to_color(color)
        && board[Coord::try_from((rank, 1)).unwrap()] == Piece::Empty
        && board[Coord::try_from((rank, 2)).unwrap()] == Piece::Empty
        && board[Coord::try_from((rank, 3)).unwrap()] == Piece::Empty
    {
        add_move(board, moves, orig, (0, -2))
            .expect("Castling destination was empty, but it also wasn't?");
    }
}

/// Returns Ok(true) if the move is a capture, Ok(false) if it isn't and Err(()) if the move is
/// invalid.
fn add_move(
    board: &Board,
    moves: &mut Vec<Move>,
    orig: Coord,
    add: (isize, isize),
) -> Result<bool, ()> {
    if let Ok(c) = orig.add(add)
        && (board[c] == Piece::Empty || board[c].get_color() != board[orig].get_color())
    {
        moves.push(Move::new(orig, c, None));
        Ok(board[c] != Piece::Empty)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use paste::paste;

    macro_rules! test_battery {
        ($({$name:ident, $fen:expr, $expected:tt}),*) => {
            $(
                huscee_macros::perft_test!($name, $fen, $expected);
            )*
        }
    }

    // NOTE: Positions taken from the Chess Programming Wiki
    //
    test_battery!(
        {
            startpos,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            [20, 400, 8_902, 197_281, 4_865_609, 119_060_324, 3_195_901_860]
        },
        {
            kiwipete,
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            [48, 2039, 97_862, 4_085_603, 193_690_690, 8_031_647_685]
        },
        {
            eppin,
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            [14, 191, 2_812, 43_238, 674_624, 11_030_083, 178_633_661, 3_009_794_393]
        },
        {
            checkstart,
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            [6, 264, 9_467, 422_333, 15_833_292, 706_045_033]
        },
        {
            checkstart_mirrored,
            "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            [6, 264, 9_467, 422_333, 15_833_292, 706_045_033]
        },
        {
            gold,
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            [44, 1_486, 62_379, 2_103_487, 89_941_194]
        },
        {
            edwards,
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            [46, 2_079, 89_890, 3_894_594, 164_075_551, 6_923_051_137]
        }

    );
}
