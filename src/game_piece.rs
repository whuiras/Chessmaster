#[derive(PartialEq, Copy, Clone)]
pub struct GamePiece {
    piece: Piece,
    color: Color,
}

impl GamePiece {
    pub fn new(piece: Piece, color: Color) -> GamePiece {
        GamePiece {
            piece,
            color,
        }
    }

    pub fn print(self) {
        let piece:char = self.encode_piece();
        if !self.is_white() {
            print!("{}", piece.to_ascii_lowercase());
        } else {
            print!("{}", piece);
        }
    }

    fn is_white(self) -> bool {
        if self.color == Color::White {
            return true;
        }
        return false;
    }

    fn encode_piece(self) -> char {
        return {
            match self.piece {
                Piece::King => 'K',
                Piece::Queen => 'Q',
                Piece::Rook => 'R',
                Piece::Bishop => 'B',
                Piece::Knight => 'N',
                Piece::Pawn => 'P',
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}