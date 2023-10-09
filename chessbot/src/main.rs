use std::str::FromStr;

use chess::{ChessMove, MoveGen};

use crate::evaluator::Evaluator;

mod evaluator;
mod search;
mod ui;

extern crate wasm_bindgen;

fn main() {
    let mut board = chess::Board::default();
    let model = include_bytes!("../model.safetensors");
    let evaluator = evaluator::NnetEval::new(model, "seq.linear-").unwrap();
    let searcher = search::simpleminmax::SimpleMinMax::new(3, Box::new(evaluator));

    for _ in 0..100 {
        let chessmove = searcher.(&board, 1, &evaluator).unwrap().unwrap();
        board = board.make_move_new(chessmove.chessmove);
        println!("score: {}", chessmove.score);
        println!("{}\n", ui::display_board(&board, false));
    }
}
