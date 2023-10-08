use super::Evaluator;

pub struct SimpleEval {}

impl SimpleEval {
    pub fn new() -> SimpleEval {
        return SimpleEval {};
    }
}

impl Evaluator for SimpleEval {
    fn evaluate(&self, _board: &chess::Board) -> anyhow::Result<f32> {
        return Ok(0.0);
    }
}
