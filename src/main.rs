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

#[cfg(test)]
mod test {
    use crate::state::State;

    #[test]
    fn my_test() {
        let state = State::new();

        assert_eq!(4, 2 + 2);

    }
}