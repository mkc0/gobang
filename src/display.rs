use crate::board::{Board, Chess};

pub fn display(board: &Board) {
    let n = board.size();
    let last_coord = board.last_coord();
    for i in 0..n {
        for j in 0..n {
            if last_coord == Some((i, j)) {
                print!(">");
            } else {
                print!(" ");
            }
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
