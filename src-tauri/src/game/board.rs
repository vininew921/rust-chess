use serde::Serialize;

use super::piece::{Piece, PieceType, Team};

#[derive(Serialize)]
pub struct Board {
    #[serde(serialize_with = "serialize_pieces")]
    pub pieces: [Option<Piece>; 64],
}

// Custom serializer for the pieces array
fn serialize_pieces<S>(pieces: &[Option<Piece>; 64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let piece_list: Vec<&Option<Piece>> = pieces.iter().collect();
    piece_list.serialize(serializer)
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
