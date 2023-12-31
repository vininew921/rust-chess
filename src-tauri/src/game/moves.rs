use serde::{Deserialize, Serialize};

use super::{
    board::Board,
    piece::{Piece, PieceType, Team},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub en_passant: bool,
    pub promotion_piece: Option<PieceType>,
}

impl Move {
    pub fn basic(from: usize, to: usize) -> Self {
        Move {
            from: from,
            to: to,
            en_passant: false,
            promotion_piece: None,
        }
    }

    pub fn en_passant(from: usize, to: usize) -> Self {
        Move {
            from: from,
            to: to,
            en_passant: true,
            promotion_piece: None,
        }
    }

    pub fn promotion(from: usize, to: usize, promotion_piece: PieceType) -> Self {
        Move {
            from: from,
            to: to,
            en_passant: false,
            promotion_piece: Some(promotion_piece),
        }
    }
}

impl Move {
    pub fn bishop(piece: &Piece, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let index = piece.get_index();
        let (row, col) = Board::get_row_col(index as i32);

        //Left-up diagonal
        let (mut t_row, mut t_col) = (row - 1, col - 1);
        while !Move::out_of_bounds(t_row, t_col) {
            let target_index = Board::get_index(t_row as usize, t_col as usize);

            t_row -= 1;
            t_col -= 1;

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    break;
                }

                result.push(Move::basic(index, target_index));
                break;
            }

            result.push(Move::basic(index, target_index));
        }

        //Right-up diagonal
        let (mut t_row, mut t_col) = (row - 1, col + 1);
        while !Move::out_of_bounds(t_row, t_col) {
            let target_index = Board::get_index(t_row as usize, t_col as usize);

            t_row -= 1;
            t_col += 1;

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    break;
                }

                result.push(Move::basic(index, target_index));
                break;
            }

            result.push(Move::basic(index, target_index));
        }

        //Left-down diagonal
        let (mut t_row, mut t_col) = (row + 1, col - 1);
        while !Move::out_of_bounds(t_row, t_col) {
            let target_index = Board::get_index(t_row as usize, t_col as usize);

            t_row += 1;
            t_col -= 1;

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    break;
                }

                result.push(Move::basic(index, target_index));
                break;
            }

            result.push(Move::basic(index, target_index));
        }

        //Right-down diagonal
        let (mut t_row, mut t_col) = (row + 1, col + 1);
        while !Move::out_of_bounds(t_row, t_col) {
            let target_index = Board::get_index(t_row as usize, t_col as usize);

            t_row += 1;
            t_col += 1;

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    break;
                }

                result.push(Move::basic(index, target_index));
                break;
            }

            result.push(Move::basic(index, target_index));
        }

        result
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

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    continue;
                }
            }

            result.push(Move::basic(index, target_index));
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

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    continue;
                }
            }

            result.push(Move::basic(index, target_index));
        }

        result
    }

    pub fn pawn(piece: &Piece, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();

        let index = piece.get_index();
        let (row, col) = Board::get_row_col(index as i32);

        //if it's a white pawn, we want to subtract 1 from the row and vice-versa
        let team_modifier = match piece.get_team() {
            Team::Black => 1,
            Team::White => -1,
        };

        let mut targets = vec![(row + team_modifier, col)];

        //pawn can move 2 squares if it hasn't moved yet
        if !piece.has_moved() {
            targets.push((row + (2 * team_modifier), col));
        }

        for (t_row, t_col) in targets {
            if Move::out_of_bounds(t_row, t_col) {
                continue;
            }

            let target_index = Board::get_index(t_row as usize, t_col as usize);

            //pawns can't capture normally, so break out of the loop if there's a piece in front of it
            if let Some(_target_piece) = board.get_piece(target_index) {
                break;
            }

            result.push(Move::basic(index, target_index));
        }

        //Check diagonals for captures
        let capture_targets = vec![
            (row + team_modifier, col + 1),
            (row + team_modifier, col - 1),
        ];

        for (t_row, t_col) in capture_targets {
            if Move::out_of_bounds(t_row, t_col) {
                continue;
            }

            let target_index = Board::get_index(t_row as usize, t_col as usize);

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    continue;
                }

                result.push(Move::basic(index, target_index));
            }
        }

        //En passant
        if board.en_passant_possible() {
            let ep_index = board.get_last_move();
            let (ep_row, ep_col) = Board::get_row_col(ep_index as i32);

            if ep_index == index
                || row != ep_row
                || board.get_piece(ep_index).unwrap().get_team() == piece.get_team()
            {
                return result;
            }

            let diff = ep_col - col;

            result.push(Move::en_passant(
                index,
                Board::get_index((row + team_modifier) as usize, (col + diff) as usize),
            ));
        }

        result
    }

    pub fn queen(piece: &Piece, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();

        for mv in Move::rook(piece, board) {
            result.push(mv);
        }

        for mv in Move::bishop(piece, board) {
            result.push(mv);
        }

        result
    }

    pub fn rook(piece: &Piece, board: &Board) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();

        let index = piece.get_index();
        let (row, col) = Board::get_row_col(index as i32);

        //Rows above
        for t_row in (0..(row)).rev() {
            let mut stop = false;
            let target_index = Board::get_index(t_row as usize, col as usize);

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    break;
                }

                stop = true;
            }

            result.push(Move::basic(index, target_index));

            if stop {
                break;
            }
        }

        //Rows below
        for t_row in (row + 1)..8 {
            let mut stop = false;
            let target_index = Board::get_index(t_row as usize, col as usize);

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    break;
                }

                stop = true;
            }

            result.push(Move::basic(index, target_index));

            if stop {
                break;
            }
        }

        //Cols to the left
        for t_col in (0..(col)).rev() {
            let mut stop = false;
            let target_index = Board::get_index(row as usize, t_col as usize);

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    break;
                }

                stop = true;
            }

            result.push(Move::basic(index, target_index));

            if stop {
                break;
            }
        }

        //Cols to the right
        for t_col in (col + 1)..8 {
            let mut stop = false;
            let target_index = Board::get_index(row as usize, t_col as usize);

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    break;
                }

                stop = true;
            }

            result.push(Move::basic(index, target_index));

            if stop {
                break;
            }
        }
        result
    }

    pub fn possible_mate(mv: Move, mut cloned_board: Board) -> bool {
        cloned_board.make_move(mv, true);

        //If the king would be in check after the move,
        //return true so the move can be filtered out
        for mv in cloned_board.get_possible_moves() {
            if cloned_board.get_piece_type_by_index(mv.to) == PieceType::King {
                return true;
            }
        }

        false
    }

    fn out_of_bounds(row: i32, col: i32) -> bool {
        row < 0 || row > 7 || col < 0 || col > 7
    }
}
