use chess::{ChessMove, MoveGen};

use crate::evaluator::Evaluator;

mod evaluator;
mod ui;

fn main() {
    let model = include_bytes!("../model.safetensors");
    let mut board = chess::Board::default();
    let evaluator = evaluator::NnetEval::new(model, "seq.linear-").unwrap();

    for _ in 0..100 {
        let movegen = MoveGen::new_legal(&board);
        let mut best_score: f32 = -100000.0;
        let mut best: Option<ChessMove> = None;

        for maybe_move in movegen {
            let b = board.make_move_new(maybe_move);
            let mut score = evaluator.evaluate(&b).expect("expected a score");
            if board.side_to_move() == chess::Color::Black {
                score = -score;
            }
            if score > best_score {
                best = Some(maybe_move);
                best_score = score;
            }
        }

        board = board.make_move_new(best.unwrap());
        println!("{}\n", ui::display_board(&board, false))
    }
}
