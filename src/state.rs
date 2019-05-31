use crate::square::*;
use crate::game_piece::*;
use crate::chess_move::*;


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

    pub fn make_move(&mut self, chess_move:ChessMove) -> &State {
        let x_from = chess_move.from_square.x;
        let y_from = chess_move.from_square.y;
        let x_to = chess_move.to_square.x;
        let y_to = chess_move.to_square.y;

        if self.board[x_from][y_from].game_piece != None {
            if self.board[x_from][y_from].game_piece != None {
                //update taken
            }
            self.board[x_to][y_to].game_piece = self.board[x_from][y_from].game_piece;

        }

        return self;
    }

    pub fn init_board() -> [[Square; 5] ;6] {
        let mut board= [[Square::new(0,0); 5]; 6];
        for i in 0..6 {
            for j in 0..5 {
                board[i][j] = Square::new(i,j)
            }
        }
        board[0][0].game_piece = Some(GamePiece::new(Piece::King,   Color::White));
        board[0][1].game_piece = Some(GamePiece::new(Piece::Queen,  Color::White));
        board[0][2].game_piece = Some(GamePiece::new(Piece::Bishop, Color::White));
        board[0][3].game_piece = Some(GamePiece::new(Piece::Knight, Color::White));
        board[0][4].game_piece = Some(GamePiece::new(Piece::Rook,   Color::White));

        board[1][0].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::White));
        board[1][1].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::White));
        board[1][2].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::White));
        board[1][3].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::White));
        board[1][4].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::White));

        board[4][0].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::Black));
        board[4][1].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::Black));
        board[4][2].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::Black));
        board[4][3].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::Black));
        board[4][4].game_piece = Some(GamePiece::new(Piece::Pawn,   Color::Black));

        board[5][0].game_piece = Some(GamePiece::new(Piece::King,   Color::Black));
        board[5][1].game_piece = Some(GamePiece::new(Piece::Queen,  Color::Black));
        board[5][2].game_piece = Some(GamePiece::new(Piece::Bishop, Color::Black));
        board[5][3].game_piece = Some(GamePiece::new(Piece::Knight, Color::Black));
        board[5][4].game_piece = Some(GamePiece::new(Piece::Rook,   Color::Black));

        return board;
    }

    pub fn print_board(self) {
        println!();
        for i in 0..6 {
            print!("{} ", 6-i);
            for j in 0..5 {
                // Print backwards so board[0][0] can be the south-west corner
                self.board[5-i][j].print();
            }
            println!();
        }
        println!();
        println!("  abcde");
        println!();
    }
}