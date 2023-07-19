<script lang="ts">
  import {
    charFromCol,
    get_index,
    get_row_col,
    svgFromPieceInfo,
  } from "./utils";
  import { PieceType, type Board, type Move, type Piece } from "./models";
  import {
    api_get_board,
    api_get_is_check,
    api_get_piece,
    api_reset_board,
    api_update_board,
  } from "./api";

  const WIDTH = 600;
  const HEIGHT = 600;
  const BLACK_COLOR = "#BE6735";
  const WHITE_COLOR = "#EDCAA1";
  const COORD_COLOR = "#FFFFFF";
  const MOVE_COLOR = "#A38A6E";
  const HIGHLIGHT_SELF_COLOR = "#F3CE68";
  const CELL_SIZE = WIDTH / 8;
  const DEBUG_FLAG = true;
  const AUDIO_CAPTURE = new Audio("sounds/capture.mp3");
  const AUDIO_CASTLE = new Audio("sounds/castle.mp3");
  const AUDIO_MOVE_CHECK = new Audio("sounds/move-check.mp3");
  const AUDIO_MOVE_SELF = new Audio("sounds/move-self.mp3");
  const AUDIO_PROMOTE = new Audio("sounds/promote.mp3");

  let board: Board;
  let board_check: boolean;
  let gameCanvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;
  let selected_moves: Array<Move> | null;
  let highlight_self_index: number | null;

  window.onload = async () => {
    board = await api_get_board();
    ctx = gameCanvas.getContext("2d");
    selected_moves = null;
    highlight_self_index = null;

    gameCanvas.onclick = handleClick;
    gameCanvas.onselectstart = () => false;

    render();
  };

  export const reset_board = async () => {
    await api_reset_board();
    board = await api_get_board();
    render();
  };

  const get_piece_moves = (piece: Piece | null): Array<Move> | null => {
    if (!piece || piece?.team != board.current_player) {
      return null;
    }

    return board.available_moves.filter((mv) => mv.from == piece.index);
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
    highlight_self_index = null;

    let target_move: Move | undefined = selected_moves?.find(
      (mv) => mv.to == index
    );

    if (target_move) {
      move_piece(target_move);
      return;
    }

    let piece = await api_get_piece(index);
    selected_moves = get_piece_moves(piece);

    if (
      piece &&
      piece.team == board.current_player &&
      selected_moves.length > 0
    ) {
      highlight_self_index = index;
    }

    render();
  };

  const move_piece = async (mv: Move) => {
    let target_square = board.pieces[mv.to];
    let source_piece_type = board.pieces[mv.from].piece_type;

    board = await api_update_board(mv);
    board_check = await api_get_is_check();

    if (board_check) {
      AUDIO_MOVE_CHECK.play();
    } else if (target_square && target_square.piece_type != PieceType.Empty) {
      AUDIO_CAPTURE.play();
    } else if (
      source_piece_type.valueOf() == PieceType.Pawn.valueOf() &&
      (mv.from - mv.to) % 2 != 0
    ) {
      AUDIO_CAPTURE.play();
    } else {
      AUDIO_MOVE_SELF.play();
    }

    selected_moves = null;
    render();
  };

  const render = () => {
    drawGrid();
    drawPieces();
    drawPieceMoves();
  };

  const drawGrid = () => {
    let blackSquare = false;
    for (let row = 0; row < 8; row++) {
      for (let col = 0; col < 8; col++) {
        ctx.fillStyle = blackSquare ? BLACK_COLOR : WHITE_COLOR;
        if (get_index(row, col) == highlight_self_index) {
          ctx.fillStyle = HIGHLIGHT_SELF_COLOR;
        }

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
        CELL_SIZE / 6,
        CELL_SIZE / 6,
        0,
        0,
        Math.PI * 2
      );

      ctx.fill();
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
    <audio src="sounds/capture.mp3" />
    <audio src="sounds/castle.mp3" />
    <audio src="sounds/move-check.mp3" />
    <audio src="sounds/move-self.mp3" />
    <audio src="sounds/promote.mp3" />
  </div>
</div>

<style>
  #game-canvas {
    background-color: white;
  }
</style>
