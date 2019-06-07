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
        print!("Make a move\n");

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let cleaned_line = line.trim().to_string();

        if !validate_coordinates(&cleaned_line) {
            print!("Invalid input\n");
            continue;
        }

        let encoded: Vec<i32> = encode_move(&cleaned_line);
        print!("length of input encoded = {}\n", encoded.len());
        print!("encoded[0] is = {}\n", encoded[0]);
        print!("encoded[1] is = {}\n", encoded[1]);

        print!("encoded[2] is = {}\n", encoded[2]);

        print!("encoded[3] is = {}\n", encoded[3]);

        let player_move = ChessMove::new(
            state.board[(encoded[0] as usize)][(encoded[1] as usize)],
            state.board[(encoded[2] as usize)][(encoded[3] as usize)],
        );

        if !state.is_legal(player_move) {
            print!("Illegal move\n");
            continue;
        }

        print!("You did it!");
        break;
    }
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
    fn my_test() {
        println!("Testing board instantiation: ");

        let state = State::new();
        state.print_board();

    }

    #[test]
    fn rook_test() {
        let mut state = State::new_none();

        state.board[2][2].game_piece = Some(GamePiece::new(Piece::Rook, Color::White));
        state.board[2][3].game_piece = Some(GamePiece::new(Piece::Rook, Color::Black));

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

    }

    #[test]
    fn bishop_test() {
        let mut state = State::new_none();

        state.board[2][2].game_piece = Some(GamePiece::new(Piece::Bishop, Color::White));
        state.board[3][3].game_piece = Some(GamePiece::new(Piece::Rook, Color::Black));

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

    }

}
