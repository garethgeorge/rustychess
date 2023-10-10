use std::str::FromStr;

use chessbot::{
    evaluator::{self, Evaluator},
    search::simpleminmax::SimpleMinMax,
    search::{alphabeta::AlphaBeta, MoveSearch},
};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str(
        "Hello world from WASM!! Loaded the chess engine.",
    ));

    Ok(())
}

#[wasm_bindgen]
pub struct ChessEngine {
    searcher: Box<dyn MoveSearch>,
}

#[wasm_bindgen]
impl ChessEngine {
    pub fn new() -> Self {
        let model = include_bytes!("../model.safetensors");
        let evaluator = evaluator::NnetEval::new(model, "seq.linear-").unwrap();
        // let evaluator = evaluator::PointsEval::new();
        let searcher = Box::new(AlphaBeta::new(2, 7, Box::new(evaluator)));

        return ChessEngine { searcher };
    }

    pub fn select_move(&mut self, fen: &str) -> Result<String, JsError> {
        let board = match chess::Board::from_str(fen) {
            Ok(board) => board,
            Err(err) => {
                return Err(JsError::new(&format!(
                    "failed to load board (fen probably corrupt): {}",
                    err
                )))
            }
        };
        match self.searcher.search(&board) {
            Ok(Some(chessmove)) => {
                println!(
                    "generated move with score {} move {}",
                    chessmove.score,
                    chessmove.chessmove.to_string()
                );
                return Ok(chessmove.chessmove.to_string());
            }
            Ok(None) => return Err(JsError::new("No move found")),
            Err(err) => return Err(JsError::new(&format!("failed to generate move: {}", err))),
        };
    }
}
