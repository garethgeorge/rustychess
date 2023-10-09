use chess::ChessMove;
use chess::MoveGen;

use crate::evaluator::Evaluator;

use super::MoveSearch;
use super::ScoredMove;

pub struct AlphaBeta {
    pub max_depth: i32,
    pub max_capture_depth: i32, // an additional depth on top of max depth to which captures are evaluated
    pub eval: Box<dyn Evaluator>,
}

impl AlphaBeta {
    pub fn new(max_depth: i32, max_capture_depth: i32, eval: Box<dyn Evaluator>) -> AlphaBeta {
        AlphaBeta {
            max_depth,
            max_capture_depth,
            eval,
        }
    }

    fn score(
        &self,
        board: &chess::Board,
        depth: i32,
        alpha: f32,
        beta: f32,
    ) -> anyhow::Result<f32> {
        let scored_move = self.search(board, depth, alpha, beta)?;
        return match scored_move {
            Some(scored_move) => Ok(scored_move.score),
            // evaluate board state with the model, happens when:
            // - we've exceeded max_capture_depth.
            // - we're past max_depth but there are no captures.
            // - we've reached the end of the game.
            None => Ok(self.eval.evaluate(board)?),
        };
    }

    fn search(
        &self,
        board: &chess::Board,
        depth: i32,
        alpha: f32,
        beta: f32,
    ) -> anyhow::Result<Option<ScoredMove>> {
        if depth <= -self.max_capture_depth {
            // we've exceeded the maximum depth and capture depth
            return Ok(None);
        }

        // create possible iterators
        let mut capture_movegen: MoveGen = MoveGen::new_legal(board);
        capture_movegen.set_iterator_mask(*board.color_combined(!board.side_to_move()));
        let mut other_movegen: MoveGen = MoveGen::new_legal(board);
        other_movegen.set_iterator_mask(!*board.color_combined(!board.side_to_move()));

        let moves: Box<dyn Iterator<Item = ChessMove>> = if depth <= 0 {
            Box::new(capture_movegen)
        } else {
            Box::new(capture_movegen.chain(other_movegen))
        };

        let mut alpha = alpha;
        let mut best_move: Option<ScoredMove> = None;
        for m in moves {
            // we negate the score b/c it is relative to the player whose turn it is.
            let score = -self.score(&board.make_move_new(m), depth - 1, -beta, -alpha)?;

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

            alpha = alpha.max(score);
            if alpha > beta {
                break;
            }
        }

        return Ok(best_move);
    }
}

impl MoveSearch for AlphaBeta {
    fn search(&self, board: &chess::Board) -> anyhow::Result<Option<ScoredMove>> {
        return self.search(board, self.max_depth, f32::MIN, f32::MAX);
    }
}
