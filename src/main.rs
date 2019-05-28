mod state;
mod square;
mod game_piece;
use crate::state::State;

fn main() {
    println!("Hello, world!");
    let state = State::new();
    state.print_board();
}


