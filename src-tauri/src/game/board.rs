use super::piece::{Piece, PieceType, Team};

pub struct Board {
    pub pieces: [Option<Piece>; 64],
}

//Starting board:
//rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR

impl Board {
    pub fn from_fen(fen_string: &str) -> Self {
        let mut board = Board { pieces: [None; 64] };

        let mut fen_chars: Vec<char> = vec!['.'; 64];

        let mut count = 0;
        for ch in fen_string.chars() {
            if ch == '/' {
                continue;
            }

            let digit = ch.to_digit(10);

            match digit {
                Some(x) => {
                    for _ in 0..x {
                        fen_chars[count] = '.';
                        count = count + 1;
                    }
                }
                _ => {
                    fen_chars[count] = ch;
                    count = count + 1;
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

    pub fn get_row_col(index: usize) -> (usize, usize) {
        let row = index / 8;
        let col = index % 8;

        (row, col)
    }

    pub fn coordinates_from_index(index: usize) -> String {
        let (row, col) = Board::get_row_col(index);

        let col_str = (8 - row).to_string();
        let row_str = ('A' as u8 + col as u8) as char;

        let result = format!("{}{}", row_str, col_str);

        result
    }

    pub fn print(&self) {
        for i in 0..64 {
            let piece = self.pieces[i];
            match piece {
                Some(x) => print!("{:?}->{:?} ", x.get_team(), x.get_piece_type()),
                _ => print!("{:?} ", PieceType::Empty),
            }

            if (i + 1) % 8 == 0 {
                println!("");
            }
        }
    }
}
