use serde::Serialize;

use super::{board::Board, piece::Piece};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn knight(piece: &Piece, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let index = piece.get_index();
        let (py, px) = Board::get_row_col(index as i32);

        if index == 1 {
            println!("Looking at index {}, px {} py {}", index, px, py);
        }

        let targets = vec![
            (px + 2, py + 1),
            (px + 2, py - 1),
            (px + 1, py + 2),
            (px + 1, py - 2),
            (px - 1, py - 2),
            (px - 1, py + 2),
            (px - 2, py - 1),
            (px - 2, py + 1),
        ];

        for (tx, ty) in targets.into_iter() {
            if Move::out_of_bounds(tx, ty) {
                continue;
            }

            if index == 1 {
                print!("Looking at tx {} ty {}", tx, ty);
            }

            let target_index = Board::get_index(ty as usize, tx as usize);
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

    fn out_of_bounds(row: i32, col: i32) -> bool {
        row < 0 || row > 7 || col < 0 || col > 7
    }
}
