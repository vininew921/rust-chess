// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rust_chess::game::{board::Board, moves::Move, piece::Piece};

#[tauri::command]
fn get_board(board: tauri::State<'_, MutexBoard>) -> Board {
    board.0.lock().unwrap().to_owned()
}

#[tauri::command]
fn get_position(index: usize) -> String {
    Board::coordinates_from_index(index)
}

#[tauri::command]
fn get_piece(index: usize, board: tauri::State<'_, MutexBoard>) -> Option<Piece> {
    board.0.lock().unwrap().get_piece(index).to_owned()
}

#[tauri::command]
fn update_board(mv: Move, board: tauri::State<'_, MutexBoard>) -> Board {
    board.0.lock().unwrap().update(mv)
}

#[tauri::command]
fn reset_board(board: tauri::State<'_, MutexBoard>) {
    board.0.lock().unwrap().reset();
}

struct MutexBoard(Mutex<Board>);

fn main() {
    tracing_subscriber::fmt().pretty().init();

    let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    tracing::info!("Initializing with FEN: {}", fen_string);

    let board = Board::from_fen(fen_string);

    tauri::Builder::default()
        .manage(MutexBoard(Mutex::from(board)))
        .invoke_handler(tauri::generate_handler![
            get_board,
            get_position,
            get_piece,
            update_board,
            reset_board
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
