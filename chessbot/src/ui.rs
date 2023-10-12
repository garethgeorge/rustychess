use chess::{Board, Color, File, Rank, Square};
use colored::Colorize;

pub fn display_board(board: &Board, ascii: bool) -> String {
    let mut out = Vec::new();
    for rank in 0..8 {
        for file in 0..8 {
            let sq = Square::make_square(Rank::from_index(rank), File::from_index(file));
            let piece = board.piece_on(sq);
            let color = board.color_on(sq).unwrap_or(Color::White);
            let symbol = match piece {
                Some(piece) => piece.to_string(color),
                None => String::from(" "),
            };

            if !ascii {
                if color == Color::White {
                    out.push(symbol.white().to_string());
                } else {
                    out.push(symbol.red().to_string());
                }
            } else {
                out.push(symbol);
            }
        }
        out.push(String::from("\n"));
    }

    return out.join("");
}
