use super::*;

// TODO: Use a non-crypto hashmap
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub dst: Coord,
    pub prom_tgt: Option<Piece>,
}

impl Move {
    pub fn new(dst: Coord, prom_tgt: Option<Piece>) -> Self {
        Self { dst, prom_tgt }
    }
}

impl From<Coord> for Move {
    fn from(value: Coord) -> Self {
        Self {
            dst: value,
            prom_tgt: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveDict(pub HashMap<Coord, Box<[Move]>>);

impl MoveDict {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn gen_moves(board: &Board) -> Self {
        let orig_board = board.clone();

        let dict = Self::gen_moves_illegal(board);

        let mut legal_dict = HashMap::new();

        for (orig, moves) in dict.0 {
            let mut legal_moves = Vec::new();
            for mov in moves {
                let mut board = orig_board.clone();
                board.make_move(orig, mov);
                let dict = Self::gen_moves_illegal(&board);
                if board.check_check(&dict, board.to_move) {
                    continue;
                }

                legal_moves.push(mov);
            }
            legal_dict.insert(orig, legal_moves.into_boxed_slice());
        }

        Self(legal_dict)
    }

    /// Generates all possible moves of all pieces. Includes moves that expose your king to check (thus the 'illegal' part).
    fn gen_moves_illegal(board: &Board) -> Self {
        let mut dict: HashMap<Coord, Box<[Move]>> = HashMap::new();

        for rank in 0..8 {
            for file in 0..8 {
                let coord = Coord::try_from((rank, file)).unwrap();

                let piece = board[coord];

                if piece == Piece::Empty || piece.get_color() != board.to_move {
                    continue;
                }

                let moves = Self::gen_at_coord_illegal(board, coord);

                dict.insert(coord, moves);
            }
        }

        Self(dict)
    }

    /// Generates all possible moves of a piece on the given square. Includes moves that expose
    /// your king to check (thus the 'illegal' part). Does not consider turn order.
    fn gen_at_coord_illegal(board: &Board, coord: Coord) -> Box<[Move]> {
        let mut moves: Vec<Move> = Vec::new();

        let piece = board[coord];

        match piece {
            Piece::PawnW | Piece::PawnB => Self::add_pawn_moves(board, coord, &mut moves),

            Piece::RookW | Piece::RookB => Self::add_rook_moves(board, coord, &mut moves),

            Piece::BishopW | Piece::BishopB => Self::add_bishop_moves(board, coord, &mut moves),

            Piece::QueenW | Piece::QueenB => {
                Self::add_rook_moves(board, coord, &mut moves);
                Self::add_bishop_moves(board, coord, &mut moves);
            }

            Piece::KnightW | Piece::KnightB => Self::add_knight_moves(board, coord, &mut moves),

            Piece::KingW | Piece::KingB => Self::add_king_moves(board, coord, &mut moves),

            Piece::Empty => {}
        }

        moves.into_boxed_slice()
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
                    moves.push(Move::new(c, Some(p.to_color(board[c].get_color()))))
                }
            } else {
                moves.push(c.into());
            }

            // first move by 2
            if orig.rank() == if board[orig].is_white() { 1 } else { 6 }
                && let Ok(c2) = orig.add((row_add * 2, 0))
                && board[c2] == Piece::Empty
            {
                moves.push(c2.into())
            }
        }

        // capture left/right
        for file_add in [-1, 1] {
            if let Ok(c) = orig.add((row_add, file_add))
                && (((board[c] != Piece::Empty)
                    && (board[c].get_color() != board[orig].get_color()))
                    || board.en_pass_tgt.is_some_and(|tgt| c == tgt))
            {
                if c.rank() == 0 || c.rank() == 7 {
                    for p in [Piece::QueenW, Piece::KnightW, Piece::BishopW, Piece::RookW] {
                        moves.push(Move::new(c, Some(p.to_color(board[c].get_color()))))
                    }
                } else {
                    moves.push(c.into());
                }
            }
        }
    }

    fn add_rook_moves(board: &Board, orig: Coord, moves: &mut Vec<Move>) {
        // TODO: figure out how to DRY this
        for i in 1..8 {
            let err = Self::add_move(board, moves, orig, (i, 0));
            // break on capture
            if err == Ok(true) || err.is_err() {
                break;
            }
        }

        for i in 1..8 {
            let err = Self::add_move(board, moves, orig, (-i, 0));
            // break on capture
            if err == Ok(true) || err.is_err() {
                break;
            }
        }

        for j in 1..8 {
            let err = Self::add_move(board, moves, orig, (0, j));
            // break on capture
            if err == Ok(true) || err.is_err() {
                break;
            }
        }

        for j in 1..8 {
            let err = Self::add_move(board, moves, orig, (0, -j));
            // break on capture
            if err == Ok(true) || err.is_err() {
                break;
            }
        }
    }

    fn add_bishop_moves(board: &Board, orig: Coord, moves: &mut Vec<Move>) {
        for i in 1..8 {
            let err = Self::add_move(board, moves, orig, (i, i));
            // break on capture
            if err == Ok(true) || err.is_err() {
                break;
            }
        }

        for i in 1..8 {
            let err = Self::add_move(board, moves, orig, (i, -i));
            // break on capture
            if err == Ok(true) || err.is_err() {
                break;
            }
        }

        for i in 1..8 {
            let err = Self::add_move(board, moves, orig, (-i, i));
            // break on capture
            if err == Ok(true) || err.is_err() {
                break;
            }
        }

        for i in 1..8 {
            let err = Self::add_move(board, moves, orig, (-i, -i));
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
            _ = Self::add_move(board, moves, orig, (i, j));
        }
    }

    fn add_king_moves(board: &Board, orig: Coord, moves: &mut Vec<Move>) {
        for i in -1..=1 {
            for j in -1..=1 {
                let _ = Self::add_move(board, moves, orig, (i, j));
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
            Self::add_move(board, moves, orig, (0, 2))
                .expect("Castling destination was empty, but it also wasn't?");
        }

        // Castle queenside
        if board.castling[color_int * 2 + 1]
            && board[Coord::try_from((rank, 0)).unwrap()] == Piece::RookW.to_color(color)
            && board[Coord::try_from((rank, 1)).unwrap()] == Piece::Empty
            && board[Coord::try_from((rank, 2)).unwrap()] == Piece::Empty
            && board[Coord::try_from((rank, 3)).unwrap()] == Piece::Empty
        {
            Self::add_move(board, moves, orig, (0, -2))
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
            moves.push(c.into());
            Ok(board[c] != Piece::Empty)
        } else {
            Err(())
        }
    }
}
