mod chess_move;
mod game_piece;
mod square;
mod state;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use crate::chess_move::ChessMove;
use crate::state::State;
use regex::Regex;

fn main() {
    println!("Welcome to Chessmaster");
    println!("Starting a new game: ");
    let mut state = State::new();
    state.print_board();

    while !state.game_over() {
        println!("Make a move\n");

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let cleaned_line = line.trim().to_string();

        if !validate_coordinates(&cleaned_line) {
            println!("Invalid input\n");
            continue;
        }

        let encoded: Vec<i32> = encode_move(&cleaned_line);

        let player_move = ChessMove::new(
            state.board[(encoded[0] as usize)][(encoded[1] as usize)],
            state.board[(encoded[2] as usize)][(encoded[3] as usize)],
        );

        if !state.is_legal(&player_move) {
            println!("Illegal move\n");
            continue;
        }

        state.make_move(player_move);
        state.print_board();
    }

    println!("Game Over!");
}

fn validate_coordinates(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[abcde][123456]-[abcde][123456]$").unwrap();
    }
    RE.is_match(input)
}

fn encode_move(input: &str) -> Vec<i32> {
    let mut encoded: Vec<i32> = Vec::new();
    for char in input.chars() {
        if char == '-' {
            continue;
        }
        encoded.push(encode_letter(char));
    }
    return encoded;
}

pub fn encode_letter(letter: char) -> i32 {
    match letter {
        'a' | '1' => return 0,
        'b' | '2' => return 1,
        'c' | '3' => return 2,
        'd' | '4' => return 3,
        'e' | '5' => return 4,
        '6' => return 5,
        _ => {
            print!("encode letter error");
            panic!();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game_piece::*;
    use crate::state::State;

    #[test]
    fn instantiation_test() {
        println!("Testing board instantiation: ");

        let state = State::new();
        state.print_board();

        println!("=======================================");
    }

    #[test]
    fn rook_test() {
        let mut state = State::new_none();

        state.board[2][2].game_piece = Some(GamePiece::new(Piece::Rook, Color::White));
        state.board[2][3].game_piece = Some(GamePiece::new(Piece::Rook, Color::Black));
        state.board[1][2].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));

        println!();
        println!("Testing Rook movement: ");

        state.print_board();

        let moves = state.movelist(2, 2);

        println!();
        println!("Possible moves are: ");

        for move1 in moves {
            let mut new_state = state.clone();
            new_state.make_move(move1);
            new_state.print_board();
        }

        println!("=======================================");
    }

    #[test]
    fn bishop_test() {
        let mut state = State::new_none();

        state.board[2][2].game_piece = Some(GamePiece::new(Piece::Bishop, Color::White));
        state.board[3][3].game_piece = Some(GamePiece::new(Piece::Rook, Color::Black));
        state.board[1][1].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));

        println!();
        println!("Testing Bishop movement: ");

        state.print_board();

        let moves = state.movelist(2, 2);

        println!();
        println!("Possible moves are: ");

        for move1 in moves {
            let mut new_state = state.clone();
            new_state.make_move(move1);
            new_state.print_board();
        }

        println!("=======================================");
    }

    #[test]
    fn pawn_test() {
        let mut state = State::new_none();

        state.board[2][2].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));
        state.board[2][3].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        state.board[3][3].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        state.board[0][2].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));

        println!();
        println!("Testing Pawn movement: ");

        state.print_board();

        let mut moves = state.movelist(2, 2);
        moves.append(&mut state.movelist(0, 2));

        println!();
        println!("Possible moves are: ");

        for move1 in moves {
            let mut new_state = state.clone();
            new_state.make_move(move1);
            new_state.print_board();
        }

        println!("=======================================");
    }

    #[test]
    fn king_test() {
        let mut state = State::new_none();

        state.board[2][2].game_piece = Some(GamePiece::new(Piece::King, Color::White));
        state.board[2][3].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        state.board[3][3].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        state.board[1][2].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));

        println!();
        println!("Testing King movement: ");

        state.print_board();

        let moves = state.movelist(2, 2);

        println!();
        println!("Possible moves are: ");

        for move1 in moves {
            let mut new_state = state.clone();
            new_state.make_move(move1);
            new_state.print_board();
        }

        println!("=======================================");
    }

    #[test]
    fn queen_test() {
        let mut state = State::new_none();

        state.board[2][2].game_piece = Some(GamePiece::new(Piece::Queen, Color::White));
        state.board[2][3].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        state.board[3][3].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        state.board[1][2].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));

        println!();
        println!("Testing Queen movement: ");

        state.print_board();

        let moves = state.movelist(2, 2);

        println!();
        println!("Possible moves are: ");

        for move1 in moves {
            let mut new_state = state.clone();
            new_state.make_move(move1);
            new_state.print_board();
        }

        println!("=======================================");
    }

    #[test]
    fn knight_test() {
        let mut state = State::new_none();

        state.board[2][2].game_piece = Some(GamePiece::new(Piece::Knight, Color::White));
        state.board[3][4].game_piece = Some(GamePiece::new(Piece::Pawn, Color::Black));
        state.board[1][4].game_piece = Some(GamePiece::new(Piece::Pawn, Color::White));

        println!();
        println!("Testing Knight movement: ");

        state.print_board();

        let moves = state.movelist(2, 2);

        println!();
        println!("Possible moves are: ");

        for move1 in moves {
            let mut new_state = state.clone();
            new_state.make_move(move1);
            new_state.print_board();
        }

        println!("=======================================");
    }

}
