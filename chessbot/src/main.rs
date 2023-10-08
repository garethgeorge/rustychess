use chess::{ChessMove, MoveGen};

use crate::evaluator::Evaluator;

mod evaluator;
mod search;
mod ui;

fn main() {
    let model = include_bytes!("../model.safetensors");
    let mut board = chess::Board::default();
    let evaluator = evaluator::NnetEval::new(model, "seq.linear-").unwrap();

    for _ in 0..100 {
        let chessmove = search::move_search(&board, 1, &evaluator).unwrap().unwrap();
        board = board.make_move_new(chessmove.chessmove);
        println!("{}\n", ui::display_board(&board, false))
    }
}
