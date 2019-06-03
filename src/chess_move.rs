use crate::square::*;

pub struct ChessMove {
    pub to_square: Square,
    pub from_square: Square,
}

impl ChessMove {
    pub fn new(from_square: Square, to_square: Square) -> ChessMove {
        ChessMove {
            to_square,
            from_square,
        }
    }
}
