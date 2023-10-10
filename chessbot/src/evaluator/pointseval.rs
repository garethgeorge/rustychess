use super::Evaluator;

pub struct PointsEval {}

impl PointsEval {
    pub fn new() -> PointsEval {
        return PointsEval {};
    }
}

impl Evaluator for PointsEval {
    fn evaluate(&self, board: &chess::Board) -> anyhow::Result<f32> {
        let mut points: i32 = 0;
        let white = board.color_combined(chess::Color::White);
        let black = board.color_combined(chess::Color::Black);

        points += (board.pieces(chess::Piece::Pawn) & white).popcnt() as i32;
        points -= (board.pieces(chess::Piece::Pawn) & black).popcnt() as i32;
        points += (board.pieces(chess::Piece::Knight) & white).popcnt() as i32 * 3;
        points -= (board.pieces(chess::Piece::Knight) & black).popcnt() as i32 * 3;
        points += (board.pieces(chess::Piece::Bishop) & white).popcnt() as i32 * 3;
        points -= (board.pieces(chess::Piece::Bishop) & black).popcnt() as i32 * 3;
        points += (board.pieces(chess::Piece::Rook) & white).popcnt() as i32 * 5;
        points -= (board.pieces(chess::Piece::Rook) & black).popcnt() as i32 * 5;
        points += (board.pieces(chess::Piece::Queen) & white).popcnt() as i32 * 9;
        points -= (board.pieces(chess::Piece::Queen) & black).popcnt() as i32 * 9;
        points += (board.pieces(chess::Piece::King) & white).popcnt() as i32 * 100;
        points -= (board.pieces(chess::Piece::King) & black).popcnt() as i32 * 100;

        return Ok(match board.side_to_move() {
            chess::Color::White => points,
            chess::Color::Black => -points,
        } as f32);
    }
}
