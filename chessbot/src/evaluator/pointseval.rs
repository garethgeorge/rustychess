use super::Evaluator;

pub struct PointsEval {}

impl PointsEval {
    pub fn new() -> PointsEval {
        return PointsEval {};
    }
}

impl Evaluator for PointsEval {
    fn evaluate(&self, board: &chess::Board) -> anyhow::Result<f32> {
        let mut points = 0;
        let white = board.color_combined(chess::Color::White);
        let black = board.color_combined(chess::Color::Black);

        points += (board.pieces(chess::Piece::Pawn) & white).popcnt();
        points -= (board.pieces(chess::Piece::Pawn) & black).popcnt();
        points += (board.pieces(chess::Piece::Knight) & white).popcnt() * 3;
        points -= (board.pieces(chess::Piece::Knight) & black).popcnt() * 3;
        points += (board.pieces(chess::Piece::Bishop) & white).popcnt() * 3;
        points -= (board.pieces(chess::Piece::Bishop) & black).popcnt() * 3;
        points += (board.pieces(chess::Piece::Rook) & white).popcnt() * 5;
        points -= (board.pieces(chess::Piece::Rook) & black).popcnt() * 5;
        points += (board.pieces(chess::Piece::Queen) & white).popcnt() * 9;
        points -= (board.pieces(chess::Piece::Queen) & black).popcnt() * 9;
        points += (board.pieces(chess::Piece::King) & white).popcnt() * 100;
        points -= (board.pieces(chess::Piece::King) & black).popcnt() * 100;

        return Ok(points as f32);
    }
}
