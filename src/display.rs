use crate::board::{Chess, Board};

pub fn display(board: &Board) {
    let n = board.size();
    for i in 0..n {
        for j in 0..n {
            print!(" ");
            match board.get_chess(i, j) {
                Chess::EMPTY => print!("."),
                Chess::BLACK => print!("o"),
                Chess::WHITE => print!("x"),
                Chess::INVALID => print!("I"),
            }
        }
        println!("");
    }
}