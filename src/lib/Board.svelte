<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { svgFromPieceInfo } from "./utils";
  import type { Board, Move, Piece } from "./models";

  const WIDTH = 600;
  const HEIGHT = 600;
  const BLACK_COLOR = "#053363";
  const WHITE_COLOR = "#0D698B";
  const COORD_COLOR = "#FFFFFF";
  const MOVE_COLOR = "#4B2D0B";
  const CELL_SIZE = WIDTH / 8;
  const DEBUG_FLAG = true;

  let board: Board;
  let gameCanvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;
  let selected_moves: Array<Move> | null;

  window.onload = async () => {
    board = await api_get_board();
    ctx = gameCanvas.getContext("2d");
    selected_moves = null;

    gameCanvas.onclick = handleClick;
    gameCanvas.onselectstart = () => false;

    render();
  };

  export const reset_board = async () => {
    await api_reset_board();
    board = await api_get_board();
    render();
  };

  const render = () => {
    drawGrid();
    drawPieces();
    drawPieceMoves();
  };

  const move_piece = async (mv: Move) => {
    board = await api_update_board(mv);
    selected_moves = null;
    render();
  };

  const api_get_board = async (): Promise<Board> => {
    let result: Board;
    await invoke("get_board").then((res: Board) => {
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

  const api_reset_board = async () => {
    await invoke("reset_board");
  };

  const api_update_board = async (mv: Move): Promise<Board> => {
    let result: Board | null = null;
    await invoke("update_board", { mv: mv }).then((res: Board) => {
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
    if (DEBUG_FLAG) {
      ctx.fillStyle = "black";
      ctx.fillText(
        (row * 8 + col).toString(),
        col * CELL_SIZE + 1,
        row * CELL_SIZE + CELL_SIZE
      );
    }

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
        let piece = board.pieces[row * 8 + col];
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

  const drawPieceMoves = () => {
    if (!selected_moves) {
      return;
    }

    for (let i = 0; i < selected_moves.length; i++) {
      let mv = selected_moves[i];

      let [row, col] = get_row_col(mv.to);

      ctx.fillStyle = MOVE_COLOR;
      ctx.beginPath();
      ctx.ellipse(
        col * CELL_SIZE + CELL_SIZE / 2,
        row * CELL_SIZE + CELL_SIZE / 2,
        CELL_SIZE / 5,
        CELL_SIZE / 5,
        0,
        0,
        Math.PI * 2
      );

      ctx.fill();
    }
  };

  const get_piece_moves = (piece: Piece | null): Array<Move> | null => {
    if (!piece || piece?.team != board.current_player) {
      return null;
    }

    return board.available_moves.filter((mv) => mv.from == piece.index);
  };

  const get_row_col = (index: number): [number, number] => {
    let row = Math.floor(index / 8);
    let col = index % 8;

    return [row, col];
  };

  const get_index = (row: number, col: number): number => {
    return row * 8 + col;
  };

  const get_index_from_mousepos = (x: number, y: number) => {
    let rect = gameCanvas.getBoundingClientRect();

    let mouseX = x - rect.left;
    let mouseY = y - rect.top;

    let indexX = Math.floor(mouseX / (WIDTH / 8));
    let indexY = Math.floor(mouseY / (HEIGHT / 8));

    return get_index(indexY, indexX);
  };

  const handleClick = async (ev: MouseEvent) => {
    let index = get_index_from_mousepos(ev.clientX, ev.clientY);

    let target_move: Move | undefined = selected_moves?.find(
      (mv) => mv.to == index
    );

    if (target_move) {
      move_piece(target_move);
      return;
    }

    let piece = await api_get_piece(index);

    selected_moves = get_piece_moves(piece);

    render();
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
