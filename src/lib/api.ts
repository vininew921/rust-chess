import { invoke } from "@tauri-apps/api";
import type { Board, Move, Piece } from "./models";

export const api_get_board = async (): Promise<Board> => {
  let result: Board;
  await invoke("get_board").then((res: Board) => {
    result = res;
  });

  return result;
};

export const api_get_piece = async (index: number): Promise<Piece | null> => {
  let result: Piece | null = null;
  await invoke("get_piece", { index: index }).then(
    (res: Piece | null) => (result = res)
  );

  return result;
};

export const api_get_position = async (index: number): Promise<String> => {
  let result: String | null = null;
  await invoke("get_position", { index: index }).then((res: String) => {
    result = res;
  });

  return result;
};

export const api_reset_board = async () => {
  await invoke("reset_board");
};

export const api_make_move = async (mv: Move): Promise<Board> => {
  let result: Board | null = null;
  await invoke("make_move", { mv: mv }).then((res: Board) => {
    result = res;
  });

  return result;
};
