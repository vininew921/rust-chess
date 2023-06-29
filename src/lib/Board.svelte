<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Piece } from "./models";
  import { svgFromPieceInfo } from "./utils";

  var DOMURL = window.URL || window.webkitURL || window;

  const WIDTH = 600;
  const HEIGHT = 600;
  const BLACK_COLOR = "#053363";
  const WHITE_COLOR = "#0D698B";
  const COORD_COLOR = "#FFFFFF";

  let cellSize = WIDTH / 8;
  let pieces: Array<Piece | null>;

  let gameCanvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  const get_board = async (): Promise<Array<Piece | null>> => {
    let result: Array<Piece | null> = new Array(64);
    await invoke("get_board").then((res: Array<Piece | null>) =>
      res.map((piece, i) => {
        result[i] = piece;
      })
    );

    return result;
  };

  window.onload = async () => {
    pieces = await get_board();
    ctx = gameCanvas.getContext("2d");
    drawGrid();
    drawPieces();
  };

  const charFromCol = (col: number): string => {
    return String.fromCharCode("A".charCodeAt(0) + col);
  };

  const drawGrid = () => {
    let blackSquare = false;
    for (let row = 0; row < 8; row++) {
      for (let col = 0; col < 8; col++) {
        ctx.fillStyle = blackSquare ? BLACK_COLOR : WHITE_COLOR;
        blackSquare = !blackSquare;
        ctx.fillRect(col * cellSize, row * cellSize, cellSize, cellSize);

        if (col == 0) {
          ctx.fillStyle = COORD_COLOR;
          ctx.fillText(
            (8 - row).toString(),
            col * cellSize + 1,
            row * cellSize + cellSize / 2
          );
        }

        if (row == 7) {
          ctx.fillStyle = COORD_COLOR;
          ctx.fillText(
            charFromCol(col),
            col * cellSize + 1 + cellSize / 2,
            row * cellSize + cellSize - 2
          );
        }
      }
      blackSquare = !blackSquare;
    }
  };

  const drawPieces = () => {
    for (let row = 0; row < 8; row++) {
      for (let col = 0; col < 8; col++) {
        let piece = pieces[row * 8 + col];
        if (!piece) {
          continue;
        }

        let img = svgFromPieceInfo(piece.piece_type, piece.team);
        ctx.drawImage(
          img,
          col * cellSize + 2,
          row * cellSize - 2,
          cellSize - 4,
          cellSize - 4
        );
      }
    }
  };
</script>

<div>
  <h1>Chess</h1>
  <canvas
    bind:this={gameCanvas}
    id="game-canvas"
    width={WIDTH}
    height={HEIGHT}
  />

  <div hidden>
    <img alt="bishop black" src="pieces/bishop_black.svg" />
    <img alt="king black" src="pieces/king_black.svg" />
    <img alt="knight black" src="pieces/knight_black.svg" />
    <img alt="pawn black" src="pieces/pawn_black.svg" />
    <img alt="queen black" src="pieces/queen_black.svg" />
    <img alt="rook black" src="pieces/rook_black.svg" />
    <img alt="bishop white" src="pieces/bishop_white.svg" />
    <img alt="king white" src="pieces/king_white.svg" />
    <img alt="knight white" src="pieces/knight_white.svg" />
    <img alt="pawn white" src="pieces/pawn_white.svg" />
    <img alt="queen white" src="pieces/queen_white.svg" />
    <img alt="rook white" src="pieces/rook_white.svg" />
  </div>
</div>

<style>
  #game-canvas {
    background-color: white;
  }
</style>
