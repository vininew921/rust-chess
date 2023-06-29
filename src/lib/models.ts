export enum PieceType {
  Empty = 1 << 0,
  Pawn = 1 << 1,
  Bishop = 1 << 2,
  Knight = 1 << 3,
  Rook = 1 << 4,
  Queen = 1 << 5,
  King = 1 << 6,
}

export enum Team {
  White = 0,
  Black = 1 << 7,
}

export type Piece = {
  piece_type: PieceType;
  team: Team;
  value: number;
};
