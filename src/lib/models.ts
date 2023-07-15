export enum PieceType {
  Empty = 0,
  Pawn = 0b0001,
  Bishop = 0b0010,
  Knight = 0b0011,
  Rook = 0b0100,
  Queen = 0b0101,
  King = 0b0110,
}

export enum Team {
  White = 0b0000,
  Black = 0b1000,
}

export type Board = {
  pieces: Array<Piece | null>;
  available_moves: Array<Move>;
  current_player: Team;
  turn: number;
  en_passant: boolean;
  last_moved_piece: number;
};

export type Piece = {
  piece_type: PieceType;
  team: Team;
  value: number;
  index: number;
};

export type Move = {
  from: number;
  to: number;
  en_passant: boolean;
};
