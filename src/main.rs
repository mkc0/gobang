mod board;
mod display;
mod game;
mod player;

fn main() {
    let mut game = game::Game::new();
    game.start();
}
