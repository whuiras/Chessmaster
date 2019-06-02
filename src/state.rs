use crate::chess_move::*;
use crate::game_piece::*;
use crate::square::*;

pub struct State {
    move_num: u64,
    taken: String,
    turn: char,
    pub board: [[Square; 5]; 6],
    moves: Vec<ChessMove>,
}

impl State {
    pub fn new() -> State {
        State {
            move_num: 0,
            taken: "".to_string(),
            turn: 'W',
            board: Self::init_board(),
            moves: Vec::new(), //TODO: update
        }
    }

    pub fn game_over(&self) -> bool {
        if self.taken.contains("K") || self.taken.contains("k") {
            return true;
        } else {
            return false;
        }
    }

    fn change_turn(&mut self) {
        if self.turn == 'W' {
            self.turn = 'B';
        } else {
            self.turn = 'W'
        }
    }

    /*
    pub fn make_move(&mut self, chess_move:ChessMove) -> &State {

        // check that the piece exists:
        if self.board[chess_move.from_square.x][chess_move.from_square.y].game_piece != None {
            if self.board[chess_move.to_square.x][chess_move.to_square.y].game_piece != None {
                //TODO: update taken here
            }
            self.board[x_to][y_to].game_piece = self.board[x_from][y_from].game_piece;
        } else {
            print!("move error");
            panic!();
        }
        return self;
    }
    */

    pub fn movescan(
        &self,
        x0: usize,
        y0: usize,
        dx: usize,
        dy: usize,
        color:Color,
        mut capture: Capture,
        mut stop_short: bool,
    ) -> Vec<ChessMove> {
        let mut x = x0;
        let mut y = y0;
        let target = self.board[x][y].game_piece;
        let mut moves: Vec<ChessMove> = Vec::new();

        'outer: while !stop_short {
            x = x + dx;
            y = y + dy;
            if !Self::in_bounds(x, y) {
                break;
            }
            match self.board[x][y].game_piece {
                Some(piece) => {
                    // If we run into our own piece or we try to move
                    // a pawn vertically into another, break.
                    if piece.color == color || capture == Capture::False {
                        break 'outer;
                    }
                    // Otherwise, take the piece and set stop-short
                    stop_short = true;
                }
                None => {
                    if capture == Capture::Only {
                        break 'outer;
                    }
                }
            }
            // Add the move

            let new_move = ChessMove::new(self.board[x0][y0], self.board[x][y]);
            moves.push(new_move)
        }
        return moves;
    }

    pub fn in_bounds(x: usize, y: usize) -> bool {
        if x < 5 && y < 6 {
            return true;
        }
        return false;
    }

    //TODO: Update
    pub fn is_legal(self, chess_move: ChessMove) -> bool {
        if true {
            return true;
        } else {
            return false;
        }
    }

    pub fn init_board() -> [[Square; 5]; 6] {
        let mut board = [[Square::new(0, 0); 5]; 6];
        for i in 0..6 {
            for j in 0..5 {
                board[i][j] = Square::new(i, j)
            }
        }
        board[0][0].game_piece = Some(GamePiece::new(Piece::King, Color::White));
        board[0][1].game_piece = Some(GamePiece::new(Piece::Queen, Color::White));
        board[0][2].game_piece = Some(GamePiece::new(Piece::Bishop, Color::White));
        board[0][3].game_piece = Some(GamePiece::new(Piece::Knight, Color::White));
        board[0][4].game_piece = Some(GamePiece::new(Piece::Rook, Color::White));

        board[1][0].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));
        board[1][1].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));
        board[1][2].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));
        board[1][3].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));
        board[1][4].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));

        board[4][0].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        board[4][1].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        board[4][2].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        board[4][3].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        board[4][4].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));

        board[5][0].game_piece = Some(GamePiece::new(Piece::King, Color::Black));
        board[5][1].game_piece = Some(GamePiece::new(Piece::Queen, Color::Black));
        board[5][2].game_piece = Some(GamePiece::new(Piece::Bishop, Color::Black));
        board[5][3].game_piece = Some(GamePiece::new(Piece::Knight, Color::Black));
        board[5][4].game_piece = Some(GamePiece::new(Piece::Rook, Color::Black));

        return board;
    }

    pub fn print_board(&self) {
        println!();
        for i in 0..6 {
            print!("{} ", 6 - i);
            for j in 0..5 {
                // Print backwards so board[0][0] can be the south-west corner
                self.board[5 - i][j].print();
            }
            println!();
        }
        println!();
        println!("  abcde");
        println!();
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Capture {
    True,
    False,
    Only,
}
