// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rust_chess::game::{board::Board, piece::Piece};

#[tauri::command]
fn get_board(board: tauri::State<'_, MutexBoard>) -> Vec<Option<Piece>> {
    Vec::from(board.0.lock().unwrap().pieces)
}

#[tauri::command]
fn get_position(index: usize) -> String {
    Board::coordinates_from_index(index)
}

#[tauri::command]
fn get_piece(index: usize, board: tauri::State<'_, MutexBoard>) -> Option<Piece> {
    board.0.lock().unwrap().pieces[index]
}

struct MutexBoard(Mutex<Board>);

fn main() {
    let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    println!("Initializing with FEN: {}", fen_string);

    let board = Board::from_fen(fen_string);

    tauri::Builder::default()
        .manage(MutexBoard(Mutex::from(board)))
        .invoke_handler(tauri::generate_handler![get_board, get_position, get_piece])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
