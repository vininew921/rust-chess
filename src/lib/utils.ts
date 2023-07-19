import { PieceType, type Team } from "./models";

export const svgFromPieceInfo = (
  type: PieceType,
  team: Team
): HTMLImageElement => {
  let img = new Image();
  img.src = `pieces/${PieceType[type].toLowerCase()}_${team
    .toString()
    .toLowerCase()}.svg`;

  return img;
};

export const charFromCol = (col: number): string => {
  return String.fromCharCode("A".charCodeAt(0) + col);
};

export const get_row_col = (index: number): [number, number] => {
  let row = Math.floor(index / 8);
  let col = index % 8;

  return [row, col];
};

export const get_index = (row: number, col: number): number => {
  return row * 8 + col;
};
