use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum PieceType {
    Empty = 1 << 0,
    Pawn = 1 << 1,
    Bishop = 1 << 2,
    Knight = 1 << 3,
    Rook = 1 << 4,
    Queen = 1 << 5,
    King = 1 << 6,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Team {
    White = 0,
    Black = 1 << 7,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Piece {
    piece_type: PieceType,
    team: Team,
    value: u8,
}

impl Piece {
    pub fn new(piece_type: PieceType, team: Team) -> Self {
        Piece {
            piece_type: piece_type,
            team: team,
            value: piece_type as u8 | team as u8,
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

    pub fn team_from_value(value: u8) -> Team {
        match value & (1 << 7) {
            0 => Team::White,
            _ => Team::Black,
        }
    }
}
