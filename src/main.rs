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
    let state = State::new();
    state.print_board();

    while !state.game_over() {
        print!("Make a move\n");

        let mut line = String::new();
        let input = std::io::stdin().read_line(&mut line).unwrap();
        if !validate_coordinates(input.to_string()) {
            print!("Invalid input\n");
            continue;
        }

        let encoded: Vec<usize> = encode_move(input.to_string());

        let player_move = ChessMove::new(
            state.board[encoded[0]][encoded[1]],
            state.board[encoded[3]][encoded[4]],
        );

        print!("You did it!");
        break;
    }
}

fn validate_coordinates(input: String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[abcde][123456]-[abcde][123456]$").unwrap();
    }
    RE.is_match(&input)
}

fn encode_move(input: String) -> Vec<usize> {
    let mut encoded: Vec<usize> = Vec::new();
    for char in input.chars() {
        encoded.push(encode_letter(char));
    }
    return encoded;
}

pub fn encode_letter(letter: char) -> usize {
    match letter {
        'a' => return 0,
        'b' => return 1,
        'c' => return 2,
        'd' => return 3,
        'e' => return 4,
        '-' => return 99,
        _ => {
            print!("encode letter error");
            panic!();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::state::State;

    #[test]
    fn my_test() {
        let state = State::new();

        assert_eq!(4, 2 + 2);
    }
}
