use crate::evaluator::Evaluator;

mod evaluator;

fn main() {
    let board = chess::Board::default();
    let evaluator = evaluator::SimpleEval::new();
    let score = evaluator.evaluate(&board);
    println!("Score: {:?}", score);
}
