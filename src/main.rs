mod state;
mod square;
mod game_piece;
mod chess_move;
use crate::state::State;

fn main() {
    println!("Welcome to Chessmaster");
    let state = State::new();
    state.print_board();
}


