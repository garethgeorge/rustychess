use chess::CacheTable;
use chess::ChessMove;
use chess::MoveGen;

use crate::evaluator::Evaluator;


use super::MoveSearch;
use super::ScoredMove;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum TransTableEntryFlag {
    NONE,
    EXACT,
    LOWERBOUND,
    UPPERBOUND,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct TransTableEntry {
    depth: i32,
    score: f32,
    flag: TransTableEntryFlag,
}

pub struct AlphaBeta {
    max_depth: i32,
    max_capture_depth: i32, // an additional depth on top of max depth to which captures are evaluated
    eval: Box<dyn Evaluator>,
    cache: CacheTable<TransTableEntry>,
}

impl AlphaBeta {
    // new creates a new instance of AlphaBeta
    // generally I observe good gameplay when max_depth is even and max capture depth is odd.
    pub fn new(max_depth: i32, max_capture_depth: i32, eval: Box<dyn Evaluator>) -> AlphaBeta {
        AlphaBeta {
            max_depth,
            max_capture_depth,
            eval,
            // note size must be 2^x (i.e. only 1 bit set).
            // 1048576 is 2^20, which is ~1M entries. This takes up (8 + 4) * 1M = 12MB of memory.
            cache: CacheTable::new(
                1048576,
                TransTableEntry {
                    depth: 0,
                    score: 0.0,
                    flag: TransTableEntryFlag::NONE,
                },
            ),
        }
    }

    fn score(
        &mut self,
        board: &chess::Board,
        depth: i32,
        alpha: f32,
        beta: f32,
    ) -> anyhow::Result<f32> {
        let mut alpha = alpha;
        let mut beta = beta;
        let hash = board.get_hash();
        let entry = self.cache.get(hash);
        if let Some(entry) = entry {
            if entry.depth >= depth {
                match entry.flag {
                    TransTableEntryFlag::NONE => {}
                    TransTableEntryFlag::EXACT => {
                        return Ok(entry.score);
                    }
                    TransTableEntryFlag::LOWERBOUND => {
                        alpha = alpha.max(entry.score);
                    }
                    TransTableEntryFlag::UPPERBOUND => {
                        beta = beta.min(entry.score);
                    }
                }
                if alpha >= beta {
                    return Ok(entry.score);
                }
            }
        }

        let scored_move = self.search(board, depth, alpha, beta)?;
        let score = match scored_move {
            Some(scored_move) => scored_move.score,
            // evaluate board state with the model, happens when:
            // - we've exceeded max_capture_depth.
            // - we're past max_depth but there are no captures.
            // - we've reached the end of the game.
            None => self.eval.evaluate(board)?,
        };

        return Ok(score);
    }

    fn search(
        &mut self,
        board: &chess::Board,
        depth: i32,
        alpha_orig: f32,
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

        let mut alpha = alpha_orig;
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

        if let Some(score) = &best_move {
            self.cache.add(
                board.get_hash(),
                TransTableEntry {
                    depth,
                    score: score.score,
                    flag: if score.score <= alpha_orig {
                        TransTableEntryFlag::UPPERBOUND
                    } else if score.score >= beta {
                        TransTableEntryFlag::LOWERBOUND
                    } else {
                        TransTableEntryFlag::EXACT
                    },
                },
            );
        }

        return Ok(best_move);
    }
}

impl MoveSearch for AlphaBeta {
    fn search(&mut self, board: &chess::Board) -> anyhow::Result<Option<ScoredMove>> {
        return self.search(board, self.max_depth, f32::MIN, f32::MAX);
    }
}
