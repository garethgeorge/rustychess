use chess::ChessMove;
use chess::MoveGen;

use crate::evaluator::Evaluator;

use super::MoveSearch;
use super::ScoredMove;

pub struct AlphaBeta {
    pub max_depth: i32,
    pub eval: Box<dyn Evaluator>,
}

impl SimpleMinMax {
    pub fn new(max_depth: i32, max_capture_depth: i32, eval: Box<dyn Evaluator>) -> SimpleMinMax {
        SimpleMinMax { max_depth, eval }
    }

    fn score(&self, board: &chess::Board, depth: i32) -> anyhow::Result<f32> {
        let scored_move = self.search(board, depth)?;
        return match scored_move {
            Some(scored_move) => Ok(scored_move.score),
            // evaluate board state with the model, happens when:
            // - we've exceeded max_capture_depth.
            // - we're past max_depth but there are no captures.
            // - we've reached the end of the game.
            None => Ok(self.eval.evaluate(board)?),
        };
    }

    fn search(&self, board: &chess::Board, depth: i32) -> anyhow::Result<Option<ScoredMove>> {
        let movegen = MoveGen::new_legal(board);

        if depth <= 0 {
            // we've exceeded the maximum depth.
            return Ok(None);
        }

        let mut best_move: Option<ScoredMove> = None;

        for m in moveiter {
            // we negate the score b/c it is relative to the player whose turn it is.
            let score = -self.score(&board.make_move_new(m), depth - 1)?;

            match &best_move {
                Some(best_move_val) => {
                    if score > best_move_val.score {
                        best_move = Some(ScoredMove {
                            chessmove: m,
                            score,
                        });
                    }
                }
                None => {
                    best_move = Some(ScoredMove {
                        chessmove: m,
                        score,
                    });
                }
            }
        }

        return Ok(best_move);
    }
}

impl MoveSearch for SimpleMinMax {
    fn search(&self, board: &chess::Board) -> anyhow::Result<Option<ScoredMove>> {
        return self.search(board, self.max_depth);
    }
}