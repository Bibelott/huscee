use super::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MoveDict(HashMap<Coord, Box<[Coord]>>);

impl MoveDict {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn gen_moves(board: &Board) -> Self {
        let moves: HashMap<Coord, Box<[Coord]>> = HashMap::new();

        todo!();
    }

    fn gen_at_coord(board: &Board, coord: Coord) -> Box<[Coord]> {
        todo!();
    }

    fn get_at_coord_illegal(board: &Board, coord: Coord) -> Vec<Coord> {
        let mut moves: Vec<Coord> = Vec::new();

        let piece = board[coord];

        match piece {
            Piece::PawnW => {
                // normal move forward
                if let Ok(c) = coord.add((1, 0))
                    && board[c] == Piece::Empty
                {
                    moves.push(c);

                    // first move by 2
                    if coord.rank() == 1
                        && let Ok(c2) = coord.add((2, 0))
                        && board[c2] == Piece::Empty
                    {
                        moves.push(c2)
                    }
                }

                // capture left
                if let Ok(c) = coord.add((1, -1))
                    && (board[c].is_black() || board.en_pass_tgt.is_some_and(|tgt| c == tgt))
                {
                    moves.push(c);
                }

                // capture right
                if let Ok(c) = coord.add((1, 1))
                    && (board[c].is_black() || board.en_pass_tgt.is_some_and(|tgt| c == tgt))
                {
                    moves.push(c);
                }
            }

            Piece::PawnB => {
                // normal move forward
                if let Ok(c) = coord.add((-1, 0))
                    && board[c] == Piece::Empty
                {
                    moves.push(c);

                    // first move by 2
                    if coord.rank() == 6
                        && let Ok(c2) = coord.add((-2, 0))
                        && board[c2] == Piece::Empty
                    {
                        moves.push(c2)
                    }
                }

                // capture left
                if let Ok(c) = coord.add((-1, -1))
                    && (board[c].is_white() || board.en_pass_tgt.is_some_and(|tgt| c == tgt))
                {
                    moves.push(c);
                }

                // capture right
                if let Ok(c) = coord.add((-1, 1))
                    && (board[c].is_white() || board.en_pass_tgt.is_some_and(|tgt| c == tgt))
                {
                    moves.push(c);
                }
            }
        }

        todo!();
    }
}

fn add_move(moves: &mut Vec<Coord>, c1: Coord, c2: (isize, isize)) {
    if let Ok(c) = c1.add(c2) {
        moves.push(c);
    }
}
