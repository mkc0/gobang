mod board;
mod display;

fn main() {
    println!("Hello, world!");
    let mut board = board::Board::new(15);
    board.play(1,1);
    board.play(1,2);
    display::display(&board);
}
