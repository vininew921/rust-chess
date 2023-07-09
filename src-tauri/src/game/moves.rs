use serde::Serialize;

use super::{board::Board, piece::Piece};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn bishop(_piece: &Piece, _board: &Board) -> Vec<Move> {
        Vec::new()
    }

    pub fn king(piece: &Piece, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let index = piece.get_index();
        let (row, col) = Board::get_row_col(index as i32);

        let targets = vec![
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ];

        for (t_row, t_col) in targets.into_iter() {
            if Move::out_of_bounds(t_row, t_col) {
                continue;
            }

            let target_index = Board::get_index(t_row as usize, t_col as usize);
            if index == 1 {
                println!(" target index: {}", target_index);
            }

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    continue;
                }
            }

            result.push(Move {
                from: index,
                to: target_index,
            });
        }

        result
    }

    pub fn knight(piece: &Piece, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let index = piece.get_index();
        let (row, col) = Board::get_row_col(index as i32);

        let targets = vec![
            (row + 2, col + 1),
            (row + 2, col - 1),
            (row + 1, col + 2),
            (row + 1, col - 2),
            (row - 1, col - 2),
            (row - 1, col + 2),
            (row - 2, col - 1),
            (row - 2, col + 1),
        ];

        for (t_row, t_col) in targets.into_iter() {
            if Move::out_of_bounds(t_row, t_col) {
                continue;
            }

            let target_index = Board::get_index(t_row as usize, t_col as usize);
            if index == 1 {
                println!(" target index: {}", target_index);
            }

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    continue;
                }
            }

            result.push(Move {
                from: index,
                to: target_index,
            });
        }

        result
    }

    pub fn pawn(_piece: &Piece, _board: &Board) -> Vec<Move> {
        Vec::new()
    }

    pub fn queen(_piece: &Piece, _board: &Board) -> Vec<Move> {
        Vec::new()
    }

    pub fn rook(_piece: &Piece, _board: &Board) -> Vec<Move> {
        Vec::new()
    }

    fn out_of_bounds(row: i32, col: i32) -> bool {
        row < 0 || row > 7 || col < 0 || col > 7
    }
}
