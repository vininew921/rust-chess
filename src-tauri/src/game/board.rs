use std::ops::Index;

use serde::Serialize;

use super::{
    moves::Move,
    piece::{Piece, PieceType, Team},
};

#[derive(Clone, Serialize)]
pub struct Board {
    fen: String,
    pieces: Vec<Option<Piece>>,
    available_moves: Vec<Move>,
    current_player: Team,
    turn: u16,
    en_passant: bool,
    last_moved_piece: usize,
}

//Starting board:
//rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR

impl Board {
    pub fn from_fen(fen_string: &str) -> Self {
        let mut board = Board {
            fen: String::from(fen_string),
            pieces: vec![None; 64],
            available_moves: Vec::new(),
            current_player: Team::White,
            turn: 1,
            en_passant: false,
            last_moved_piece: 0,
        };

        board.initialize();
        board.generate_moves();
        board
    }

    fn initialize(&mut self) {
        let mut fen_chars: Vec<char> = vec!['.'; 64];

        let mut count = 0;
        for ch in self.fen.chars() {
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
            self.pieces[i] = match ch {
                'p' => Option::from(Piece::new(PieceType::Pawn, Team::Black, i)),
                'P' => Option::from(Piece::new(PieceType::Pawn, Team::White, i)),
                'r' => Option::from(Piece::new(PieceType::Rook, Team::Black, i)),
                'R' => Option::from(Piece::new(PieceType::Rook, Team::White, i)),
                'n' => Option::from(Piece::new(PieceType::Knight, Team::Black, i)),
                'N' => Option::from(Piece::new(PieceType::Knight, Team::White, i)),
                'b' => Option::from(Piece::new(PieceType::Bishop, Team::Black, i)),
                'B' => Option::from(Piece::new(PieceType::Bishop, Team::White, i)),
                'q' => Option::from(Piece::new(PieceType::Queen, Team::Black, i)),
                'Q' => Option::from(Piece::new(PieceType::Queen, Team::White, i)),
                'k' => Option::from(Piece::new(PieceType::King, Team::Black, i)),
                'K' => Option::from(Piece::new(PieceType::King, Team::White, i)),
                '.' => None,
                _ => panic!("Invalid FEN character: {}", ch),
            }
        }
    }

    pub fn reset(&mut self) {
        println!("Reseting board...");

        self.fen = String::from(self.fen.clone());
        self.pieces = vec![None; 64];
        self.available_moves = Vec::new();
        self.current_player = Team::White;
        self.turn = 1;
        self.last_moved_piece = 0;
        self.en_passant = false;

        self.initialize();
        self.generate_moves();

        println!("Done!");
    }

    pub fn generate_moves(&mut self) {
        self.available_moves.clear();

        for i in 0..self.pieces.len() {
            if let Some(piece) = self.get_piece(i) {
                let available_moves = match piece.get_piece_type() {
                    PieceType::Bishop => Move::bishop(piece, self),
                    PieceType::King => Move::king(piece, self),
                    PieceType::Knight => Move::knight(piece, self),
                    PieceType::Pawn => Move::pawn(piece, self),
                    PieceType::Queen => Move::queen(piece, self),
                    PieceType::Rook => Move::rook(piece, self),
                    _ => Vec::new(),
                };

                for am in available_moves {
                    self.available_moves.push(am);
                }
            }
        }
    }

    pub fn update(&mut self, mv: Move) -> Self {
        //Double checking if the move provided is actually one of the available moves
        if !self
            .available_moves
            .iter()
            .any(|x| x.from == mv.from && x.to == mv.to && x.en_passant == mv.en_passant)
        {
            return self.clone();
        }

        let from_piece = self.get_piece(mv.from).unwrap();

        //Double checking if the move provided is actually from the current player
        if from_piece.get_team() != self.current_player {
            return self.clone();
        }

        println!(
            "MOVE -> Turn {}, {:?} | {} to {}",
            self.turn,
            self.current_player,
            Board::coordinates_from_index(mv.from),
            Board::coordinates_from_index(mv.to)
        );

        let mut new_piece = Piece::new(from_piece.get_piece_type(), from_piece.get_team(), mv.to);
        new_piece.moved(true);

        self.pieces[mv.to] = Some(new_piece);
        self.pieces[mv.from] = None;

        //Capture the last moved piece if the current move is an en passant
        if mv.en_passant {
            self.pieces[self.last_moved_piece] = None;
        }

        self.en_passant = false;

        //Check if an en passant is possible on the next turn
        if from_piece.get_piece_type() == PieceType::Pawn {
            if i8::abs(mv.to as i8 - mv.from as i8) == 16 {
                self.en_passant = true;
            }
        }

        self.last_moved_piece = mv.to;

        self.generate_moves();

        self.current_player = match self.current_player {
            Team::White => Team::Black,
            Team::Black => {
                self.turn = self.turn + 1;
                Team::White
            }
        };

        self.clone()
    }

    pub fn get_pieces(&self) -> Vec<Option<Piece>> {
        self.pieces.clone()
    }

    pub fn get_piece(&self, index: usize) -> &Option<Piece> {
        self.pieces.index(index)
    }

    pub fn en_passant_possible(&self) -> bool {
        self.en_passant
    }

    pub fn get_last_move(&self) -> usize {
        self.last_moved_piece
    }

    pub fn get_piece_moves(&self, index: usize) -> Vec<Move> {
        self.available_moves
            .iter()
            .filter(|&mov| mov.from == index)
            .cloned()
            .collect()
    }

    pub fn print(&self) {
        for i in 0..64 {
            let piece = self.get_piece(i);
            match piece {
                Some(x) => print!("{:?}->{:?} ", x.get_team(), x.get_piece_type()),
                _ => print!("{:?} ", PieceType::Empty),
            }

            if (i + 1) % 8 == 0 {
                println!("");
            }
        }
    }

    pub fn get_index(row: usize, col: usize) -> usize {
        row * 8 + col
    }

    pub fn get_row_col(index: i32) -> (i32, i32) {
        let row = index / 8;
        let col = index % 8;

        (row, col)
    }

    pub fn coordinates_from_index(index: usize) -> String {
        let (row, col) = Board::get_row_col(index as i32);

        let col_str = (8 - row).to_string();
        let row_str = ('A' as u8 + col as u8) as char;

        let result = format!("{}{}", row_str, col_str);

        result
    }
}
