extern crate rand;

pub mod board;
pub mod magic;

use board::Board;
use magic::MagicBoards;

pub fn run() {
    let magic_boards = MagicBoards::gen_magics();
    let board_state = Board::start_position(&magic_boards).unwrap();
    println!("{:?}", board_state);
}
