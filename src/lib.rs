extern crate rand;

pub mod board;
pub mod magic;

use board::Board;
use magic::MagicBoards;

pub fn run() {
    let board_state = Board::start_position().unwrap();
    println!("{:?}", board_state);
    MagicBoards::gen_magics();
}
