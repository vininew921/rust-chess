<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Piece } from "./models";
  import { svgFromPieceInfo } from "./utils";

  const WIDTH = 600;
  const HEIGHT = 600;
  const BLACK_COLOR = "#053363";
  const WHITE_COLOR = "#0D698B";
  const COORD_COLOR = "#FFFFFF";
  const CELL_SIZE = WIDTH / 8;

  let pieces: Array<Piece | null>;
  let gameCanvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  window.onload = async () => {
    pieces = await map_pieces();
    ctx = gameCanvas.getContext("2d");
    drawGrid();
    drawPieces();

    gameCanvas.onclick = handleClick;
  };

  const map_pieces = async (): Promise<Array<Piece | null>> => {
    let result: Array<Piece | null> = new Array(64);
    await api_get_board().then((res) =>
      res.map((piece, i) => {
        result[i] = piece;
      })
    );

    return result;
  };

  const api_get_board = async (): Promise<Array<Piece | null> | null> => {
    let result: Array<Piece | null> | null = null;
    await invoke("get_board").then((res: Array<Piece | null>) => {
      result = res;
    });

    return result;
  };

  const api_get_piece = async (index: number): Promise<Piece | null> => {
    let result: Piece | null = null;
    await invoke("get_piece", { index: index }).then(
      (res: Piece | null) => (result = res)
    );

    return result;
  };

  const api_get_position = async (index: number): Promise<String> => {
    let result: String | null = null;
    await invoke("get_position", { index: index }).then((res: String) => {
      result = res;
    });

    return result;
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
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

        drawCoordinates(row, col);
      }
      blackSquare = !blackSquare;
    }
  };

  const drawCoordinates = (row: number, col: number) => {
    if (col == 0) {
      ctx.fillStyle = COORD_COLOR;
      ctx.fillText(
        (8 - row).toString(),
        col * CELL_SIZE + 1,
        row * CELL_SIZE + CELL_SIZE / 2
      );
    }

    if (row == 7) {
      ctx.fillStyle = COORD_COLOR;
      ctx.fillText(
        charFromCol(col),
        col * CELL_SIZE + 1 + CELL_SIZE / 2,
        row * CELL_SIZE + CELL_SIZE - 2
      );
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
          col * CELL_SIZE + 2,
          row * CELL_SIZE - 2,
          CELL_SIZE - 4,
          CELL_SIZE - 4
        );
      }
    }
  };

  const get_index = (row: number, col: number): number => {
    return row * 8 + col;
  };

  const index_from_mousepos = (x: number, y: number) => {
    var rect = gameCanvas.getBoundingClientRect();

    let mouseX = x - rect.left;
    let mouseY = y - rect.top;

    let indexX = Math.floor(mouseX / (WIDTH / 8));
    let indexY = Math.floor(mouseY / (HEIGHT / 8));

    return get_index(indexY, indexX);
  };

  const handleClick = async (ev: MouseEvent) => {
    let index = index_from_mousepos(ev.clientX, ev.clientY);
    console.log(await api_get_piece(index));
    console.log(await api_get_position(index));
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
