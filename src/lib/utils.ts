import type { PieceType, Team } from "./models";

export const svgFromPieceInfo = (
  type: PieceType,
  team: Team
): HTMLImageElement => {
  let img = new Image();
  img.src = `pieces/${type.toString().toLowerCase()}_${team
    .toString()
    .toLowerCase()}.svg`;

  return img;
};
