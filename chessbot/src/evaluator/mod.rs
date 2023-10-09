use chess::Board;

mod nneteval;
mod pointseval;
pub use nneteval::NnetEval;
pub use pointseval::PointsEval;

pub trait Evaluator {
    // Evaluate returns a score for the board where positive is good for white and negative is good for black.
    fn evaluate(&self, board: &Board) -> anyhow::Result<f32>;
}
