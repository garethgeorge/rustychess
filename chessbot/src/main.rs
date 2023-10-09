use std::str::FromStr;

use chess::{ChessMove, MoveGen};

use crate::evaluator::Evaluator;

mod evaluator;
mod search;
mod ui;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

fn main() {
    let mut board = chess::Board::default();
    let model = include_bytes!("../model.safetensors");
    let evaluator = evaluator::NnetEval::new(model, "seq.linear-").unwrap();

    for _ in 0..100 {
        let chessmove = search::move_search(&board, 1, &evaluator).unwrap().unwrap();
        board = board.make_move_new(chessmove.chessmove);
        println!("score: {}", chessmove.score);
        println!("{}\n", ui::display_board(&board, false));
    }
}

#[wasm_bindgen]
pub struct ChessEngine {
    evaluator: Box<dyn Evaluator>,
}

impl ChessEngine {
    pub fn new() -> Self {
        let model = include_bytes!("../model.safetensors");
        let evaluator = evaluator::NnetEval::new(model, "seq.linear-").unwrap();

        return ChessEngine {
            evaluator: Box::new(evaluator),
        };
    }

    pub fn select_move(&self, fen: &str) -> Result<String, JsError> {
        let board = match chess::Board::from_str(fen) {
            Ok(board) => board,
            Err(err) => return Err(JsError::new(&format!("failed to load board {}", err))),
        };
        match search::move_search(&board, 1, self.evaluator.as_ref()) {
            Ok(Some(chessmove)) => {
                println!(
                    "generated move with score {} move {}",
                    chessmove.score,
                    chessmove.chessmove.to_string()
                );
                return Ok(chessmove.chessmove.to_string());
            }
            Ok(None) => return Err(JsError::new("No move found")),
            Err(err) => return Err(JsError::new(&format!("failed to generate move {}", err))),
        };
    }
}
