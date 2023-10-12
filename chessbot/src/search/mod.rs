use chess::{Board};

pub mod alphabeta;
pub mod simpleminmax;

pub struct ScoredMove {
    pub chessmove: chess::ChessMove,
    pub score: f32,
}

pub trait MoveSearch {
    fn search(&mut self, board: &Board) -> anyhow::Result<Option<ScoredMove>>;
}
