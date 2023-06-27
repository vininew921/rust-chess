use super::piece::{Piece, PieceType, Team};

pub struct Board {
    pieces: [Option<Piece>; 64],
}

//Starting board:
//rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR

impl Board {
    pub fn new(fen_string: &str) -> Self {
        let mut board = Board { pieces: [None; 64] };

        let mut fen_chars: Vec<char> = Vec::new();

        for (i, substring) in fen_string.split('/').rev().enumerate() {
            for (j, ch) in substring.chars().enumerate() {
                let digit = ch.to_digit(10);

                match digit {
                    Some(x) => {
                        for _ in 0..x {
                            fen_chars.insert(Board::get_index(i, j), '.');
                        }
                    }
                    _ => fen_chars.insert(Board::get_index(i, j), ch),
                }
            }
        }

        for (i, ch) in fen_chars.iter().enumerate() {
            board.pieces[i] = match ch {
                'p' => Option::from(Piece::new(PieceType::Pawn, Team::Black)),
                'P' => Option::from(Piece::new(PieceType::Pawn, Team::White)),
                'r' => Option::from(Piece::new(PieceType::Rook, Team::Black)),
                'R' => Option::from(Piece::new(PieceType::Rook, Team::White)),
                'n' => Option::from(Piece::new(PieceType::Knight, Team::Black)),
                'N' => Option::from(Piece::new(PieceType::Knight, Team::White)),
                'b' => Option::from(Piece::new(PieceType::Bishop, Team::Black)),
                'B' => Option::from(Piece::new(PieceType::Bishop, Team::White)),
                'q' => Option::from(Piece::new(PieceType::Queen, Team::Black)),
                'Q' => Option::from(Piece::new(PieceType::Queen, Team::White)),
                'k' => Option::from(Piece::new(PieceType::King, Team::Black)),
                'K' => Option::from(Piece::new(PieceType::King, Team::White)),
                '.' => None,
                _ => panic!("Invalid FEN character: {}", ch),
            }
        }

        board
    }

    pub fn get_index(row: usize, col: usize) -> usize {
        row * 8 + col
    }

    pub fn get_piece(&self, row: usize, col: usize) -> Option<Piece> {
        self.pieces[row * 8 + col]
    }

    pub fn print(&self) {
        for i in (0..8).rev() {
            for j in 0..8 {
                let piece = self.get_piece(i, j);
                match piece {
                    Some(x) => print!(
                        "{}{}:{:?}->{:?} ",
                        ('a' as u8 + j as u8) as char,
                        i + 1,
                        x.get_team(),
                        x.get_piece_type()
                    ),
                    _ => print!("{:?} ", PieceType::Empty),
                }
            }
            println!();
        }
    }
}
