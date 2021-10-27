use crate::board::{Board, Chess};

pub fn display(board: &Board) {
    let n = board.size();
    let coord = board.last_coord();
    print!("  ");
    for i in 0..n {
        print!("{:>2}", i as i32 - (n / 2) as i32);
    }
    println!();
    for i in 0..n {
        print!("{:>2}", i as i32 - (n / 2) as i32);
        for j in 0..n {
            if coord == Some((i, j)) {
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
