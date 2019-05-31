extern crate chessmaster;

use crate::state::*;

#[test]
fn test_move() {
    let state = State::new();
    state.print_board();
    assert_eq!(0,0);
}
