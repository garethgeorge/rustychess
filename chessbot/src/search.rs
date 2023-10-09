use chess::{Board, ChessMove, Color, MoveGen};

use crate::evaluator::Evaluator;

pub struct Move {
    pub chessmove: ChessMove,
    pub score: f32,
}

pub fn move_search(
    board: &Board,
    depth: i32,
    eval: &dyn Evaluator,
) -> anyhow::Result<Option<Move>> {
    let mut best_score: f32 = -100000.0;
    let mut best: Option<ChessMove> = None;

    let movegen = MoveGen::new_legal(board);
    for maybe_move in movegen {
        let b = board.make_move_new(maybe_move);
        let score = -eval.evaluate(&b)?;

        if score > best_score {
            best = Some(maybe_move);
            best_score = score;
        }
    }

    match best {
        Some(chessmove) => {
            return Ok(Some(Move {
                chessmove,
                score: best_score,
            }))
        }
        None => return Ok(None),
    };
}

fn scorer(board: &Board, depth: i32, eval: &dyn Evaluator) -> anyhow::Result<f32> {
    let movegen = MoveGen::new_legal(board);
    let mut moveiter: Box<dyn Iterator<Item = ChessMove>> = Box::new(movegen);

    if depth < -4 {
        return Ok(eval.evaluate(board)?);
    } else if depth <= 0 {
        // examine captures only.
        moveiter = Box::new(
            moveiter.filter(|chessmove: &ChessMove| board.piece_on(chessmove.get_dest()).is_some()),
        );
    }

    let scores = moveiter
        .map(|chessmove| -> anyhow::Result<f32> {
            let b = board.make_move_new(chessmove);
            return Ok(-scorer(&b, depth - 1, eval)?);
        })
        .reduce(|a, b| -> anyhow::Result<f32> {
            return Ok(a?.max(b?));
        });

    match scores {
        Some(Ok(score)) => return Ok(score),
        Some(Err(err)) => return Err(err),
        None => {
            return Ok(eval.evaluate(board)?);
        }
    };
}
