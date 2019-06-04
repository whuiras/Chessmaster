use crate::game_piece::*;

#[derive(PartialEq, Copy, Clone)]
pub struct Square {
    pub x: i32,
    pub y: i32,
    pub game_piece: Option<GamePiece>,
}

impl Square {
    pub fn new(x: i32, y: i32) -> Square {
        Square {
            x,
            y,
            game_piece: None,
        }
    }

    pub fn print(self) {
        match self.game_piece {
            None => print!("."),
            Some(n) => n.print(),
        }
    }
}
