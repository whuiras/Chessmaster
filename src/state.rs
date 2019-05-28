use crate::square::*;
use crate::game_piece::*;


pub struct State {
    move_num:   u64,
    taken:      String,
    turn:       char,
    board:      [[Square; 5] ;6]
}

impl State {
    pub fn new() -> State {
        State {
            move_num: 0,
            taken: "".to_string(),
            turn: 'W',
            board: Self::init_board(),
        }
    }

    pub fn init_board() -> [[Square; 5] ;6] {
        let mut board= [[Square::new(0,0); 5]; 6];
        for i in 0..6 {
            for j in 0..5 {
                board[i][j] = Square::new(i,j)
            }
        }
        board[0][0].game_piece = Some(GamePiece::new(Piece::Rook, Color::White));
        // place rest of pieces...
        return board;
    }

    pub fn print_board(self) {
        for i in 0..6 {
            for j in 0..5 {
                self.board[i][j].print();
            }
            println!();
        }
    }
}