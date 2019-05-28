use crate::game_piece::*;

#[derive(PartialEq, Copy, Clone)]
pub struct Square {
    x: usize,
    y: usize,
    pub game_piece: Option<GamePiece>
}

impl Square {
    pub fn new(x: usize, y: usize) -> Square {
        Square {
            x,
            y,
            game_piece: None,
        }
    }

    pub fn print(self) {
        match self.game_piece {
            None    => print!("."),
            Some(n) => n.print()
        }
    }
}