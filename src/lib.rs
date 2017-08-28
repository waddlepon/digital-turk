pub mod board;
pub mod magic;

use board::Board;

pub fn run() {
    let board_state = Board::start_position().unwrap();
    println!("{:?}", board_state);
}
