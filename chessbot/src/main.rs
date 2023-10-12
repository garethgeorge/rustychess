


use chessbot::search::{alphabeta::AlphaBeta, MoveSearch};

mod evaluator;
mod search;
mod ui;

extern crate wasm_bindgen;

fn main() {
    let model = include_bytes!("../../web/model.safetensors");
    // let evaluator = evaluator::PointsEval::new();
    let evaluator = chessbot::evaluator::NnetEval::new(model, "seq.linear-").unwrap();
    // let evaluator = evaluator::PointsEval::new();
    let mut searcher = Box::new(AlphaBeta::new(3, 0, Box::new(evaluator)));

    let mut board = chess::Board::default();
    for _ in 0..100 {
        let chessmove = searcher.search(&board).unwrap().unwrap();
        board = board.make_move_new(chessmove.chessmove);
        println!("score: {}", chessmove.score);
        println!("{}\n", ui::display_board(&board, false));
    }
}
