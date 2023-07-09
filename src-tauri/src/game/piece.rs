use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum PieceType {
    Empty = 0,
    Pawn = 0b0001,
    Bishop = 0b0010,
    Knight = 0b0011,
    Rook = 0b0100,
    Queen = 0b0101,
    King = 0b0110,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum Team {
    White = 0b0000,
    Black = 0b1000,
}

#[derive(Debug, Clone, Serialize)]
pub struct Piece {
    piece_type: PieceType,
    team: Team,
    value: u8,
    index: usize,
}

impl Piece {
    pub fn new(piece_type: PieceType, team: Team, index: usize) -> Self {
        Piece {
            piece_type,
            team,
            value: piece_type as u8 | team as u8,
            index,
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn get_team(&self) -> Team {
        self.team
    }

    pub fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn team_from_value(value: u8) -> Team {
        match value & (7) {
            0 => Team::White,
            _ => Team::Black,
        }
    }
}
