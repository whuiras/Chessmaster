use crate::chess_move::*;
use crate::game_piece::*;
use crate::square::*;

pub struct State {
    move_num: u64,
    taken: String,
    turn: char,
    pub board: [[Square; 5]; 6],
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

    pub fn new_none() -> State {
        State {
            move_num: 0,
            taken: "".to_string(),
            turn: 'W',
            board: Self::init_none(),
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

    pub fn make_move(&mut self, chess_move: ChessMove) -> &State {
        let from = self.board[(chess_move.from_square.x as usize)]
            [(chess_move.from_square.y as usize)]
            .game_piece;
        let to = self.board[(chess_move.to_square.x as usize)][(chess_move.to_square.y as usize)]
            .game_piece;

        // check that the piece exists:
        match from {
            Some(from_piece) => {
                match to {
                    // check if we are taking a piece
                    Some(to_piece) => self.taken.push(to_piece.encode_piece()),
                    None => {}
                }
                // Move the piece
                self.board[(chess_move.from_square.x as usize)]
                    [(chess_move.from_square.y as usize)]
                    .game_piece = self.board[(chess_move.to_square.x as usize)]
                    [(chess_move.to_square.y as usize)]
                    .game_piece;
            }
            None => {
                print!("move error");
                panic!();
            }
        }
        return self;
    }

    fn movescan(
        &self,
        x0: i32,
        y0: i32,
        dx: i32,
        dy: i32,
        color: Color,
        capture: Capture,
        mut stop_short: bool,
    ) -> Vec<ChessMove> {
        let mut x = x0;
        let mut y = y0;
        let mut moves = Vec::new();

        'outer: while !stop_short {
            x = x + dx as i32;
            y = y + dy as i32;
            println!("x is: {}, y is: {}", x, y);
            if !Self::in_bounds(x, y) {
                break;
            }
            match self.board[(x as usize)][(y as usize)].game_piece {
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

            let new_move = ChessMove::new(
                self.board[(x0 as usize)][(y0 as usize)],
                self.board[(x as usize)][(y as usize)],
            );
            moves.push(new_move)
        }
        return moves;
    }

    fn symmscan(
        &self,
        x0: i32,
        y0: i32,
        mut dx: i32,
        mut dy: i32,
        color: Color,
        capture: Capture,
        stop_short: bool,
    ) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

        for _i in 0..3 {
            moves.append(&mut Self::movescan(
                &self, x0, y0, dx, dy, color, capture, stop_short,
            ));
            std::mem::swap(&mut dx, &mut dy);
            dy = dy * (-1);
        }
        return moves;
    }

    pub fn movelist(&self, x0: i32, y0: i32) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let target = self.board[(x0 as usize)][(y0 as usize)].game_piece;
        match target {
            Some(GamePiece {
                piece: Piece::King,
                color,
            }) => {
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    0,
                    1,
                    color,
                    Capture::True,
                    true,
                ));
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    1,
                    1,
                    color,
                    Capture::True,
                    true,
                ));
            }
            Some(GamePiece {
                piece: Piece::Queen,
                color,
            }) => {
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    0,
                    1,
                    color,
                    Capture::True,
                    false,
                ));
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    1,
                    1,
                    color,
                    Capture::True,
                    false,
                ));
            }
            Some(GamePiece {
                piece: Piece::Rook,
                color,
            }) => {
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    0,
                    1,
                    color,
                    Capture::True,
                    false,
                ));
            }
            Some(GamePiece {
                piece: Piece::Bishop,
                color,
            }) => {
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    1,
                    1,
                    color,
                    Capture::True,
                    false,
                ));
            }
            Some(GamePiece {
                piece: Piece::Knight,
                color,
            }) => {
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    1,
                    2,
                    color,
                    Capture::True,
                    true,
                ));
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    -1,
                    2,
                    color,
                    Capture::True,
                    true,
                ));
            }
            Some(GamePiece {
                piece: Piece::Pawn,
                color: Color::White,
            }) => {
                let dir = 1;
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    -1,
                    dir,
                    Color::White,
                    Capture::Only,
                    true,
                ));
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    1,
                    dir,
                    Color::White,
                    Capture::Only,
                    true,
                ));
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    0,
                    dir,
                    Color::White,
                    Capture::False,
                    true,
                ));
            }
            Some(GamePiece {
                piece: Piece::Pawn,
                color: Color::Black,
            }) => {
                let dir = -1;
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    -1,
                    dir,
                    Color::Black,
                    Capture::Only,
                    true,
                ));
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    1,
                    dir,
                    Color::Black,
                    Capture::Only,
                    true,
                ));
                moves.append(&mut Self::symmscan(
                    &self,
                    x0,
                    y0,
                    0,
                    dir,
                    Color::Black,
                    Capture::False,
                    true,
                ));
            }

            None => {}
        }

        return moves;
    }

    pub fn in_bounds(x: i32, y: i32) -> bool {
        if x < 5 && x > -1 && y < 6 && y > -1 {
            return true;
        }
        return false;
    }

    pub fn is_legal(&mut self, chess_move: ChessMove) -> bool {
        match chess_move.from_square.game_piece {
            // Check that the piece we are trying to move exists and is the correct color.
            Some(piece) => {
                if !Self::check_turn(&self, piece.color) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        let mut moves: Vec<ChessMove> = Vec::new();
        for i in 0..6 {
            for j in 0..5 {
                moves.append(&mut Self::movelist(&self, i, j));
            }
        }

        println!("number of possible moves is: {}", moves.len());

        for move1 in moves {
            &self.make_move(move1);
            Self::print_board(&self);
        }

        /*
              if moves.contains(&chess_move) {
                  return true;
              }
        */

        return false;
    }

    fn check_turn(&self, color: Color) -> bool {
        if self.turn == 'W' && color == Color::Black {
            return false;
        }
        if self.turn == 'B' && color == Color::White {
            return false;
        }
        return true;
    }

    pub fn init_board() -> [[Square; 5]; 6] {
        let mut board = [[Square::new(0, 0); 5]; 6];
        for i in 0..6 {
            for j in 0..5 {
                board[i][j] = Square::new((i as i32), (j as i32))
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

    pub fn init_none() -> [[Square; 5]; 6] {
        let mut board = [[Square::new(0, 0); 5]; 6];
        for i in 0..6 {
            for j in 0..5 {
                board[i][j] = Square::new((i as i32), (j as i32));
            }
        }
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
