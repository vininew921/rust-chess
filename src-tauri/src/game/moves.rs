use serde::{Deserialize, Serialize};

use super::{
    board::Board,
    piece::{Piece, Team},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub en_passant: bool,
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

                result.push(Move {
                    from: index,
                    to: target_index,
                    en_passant: false,
                });
                break;
            }

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });
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

                result.push(Move {
                    from: index,
                    to: target_index,
                    en_passant: false,
                });
                break;
            }

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });
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

                result.push(Move {
                    from: index,
                    to: target_index,
                    en_passant: false,
                });
                break;
            }

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });
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

                result.push(Move {
                    from: index,
                    to: target_index,
                    en_passant: false,
                });
                break;
            }

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });
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

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
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

            if let Some(target_piece) = board.get_piece(target_index) {
                if target_piece.get_team() == piece.get_team() {
                    continue;
                }
            }

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });
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

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });
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

                result.push(Move {
                    from: index,
                    to: target_index,
                    en_passant: false,
                });
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

            result.push(Move {
                from: index,
                to: Board::get_index((row + team_modifier) as usize, (col + diff) as usize),
                en_passant: true,
            })
        }

        result
    }

    pub fn queen(_piece: &Piece, _board: &Board) -> Vec<Move> {
        Vec::new()
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

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });

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

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });

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

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });

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

            result.push(Move {
                from: index,
                to: target_index,
                en_passant: false,
            });

            if stop {
                break;
            }
        }
        result
    }

    fn out_of_bounds(row: i32, col: i32) -> bool {
        row < 0 || row > 7 || col < 0 || col > 7
    }
}
