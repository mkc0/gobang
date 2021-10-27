use crate::board::{self, Board};
use crate::display;
use crate::player::{HumanPlayer, Player};
use std::io::{self, BufRead, Write};

const BOARD_SIZE: usize = 15;

#[allow(dead_code)]
pub struct Game {
    board: Board,
    winner: board::Chess,
    p1: Box<dyn Player>,
    p2: Box<dyn Player>,
}

#[allow(dead_code)]
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(BOARD_SIZE),
            winner: board::Chess::EMPTY,
            p1: Box::new(HumanPlayer {}) as Box<dyn Player>,
            p2: Box::new(HumanPlayer {}) as Box<dyn Player>,
        }
    }

    pub fn start(self: &mut Self) {
        display::display(&self.board);
        self.process_cmd();
    }

    fn match_cmd(cmd: &str, full_cmd: &str) -> bool {
        if cmd.len() > full_cmd.len() {
            return false;
        }
        *cmd == full_cmd[0..cmd.len()]
    }

    fn check_win(self: &mut Self) {
        if self.winner != board::Chess::EMPTY {
            return;
        }
        let b = &self.board;
        let turn = b.last_turn();
        if turn == board::Chess::EMPTY {
            return;
        }
        let n = b.size() as i8;
        let coord = b.last_coord().unwrap();
        let coord = (coord.0 as i8, coord.1 as i8);
        let dirs:Vec<(i8, i8)> = vec![(1, 0), (0, 1), (1, 1), (1, -1)];
        let max_in_a_line = dirs.iter()
            .map(|(d0, d1)| {
                let mut cnt:u8 = 1;
                let mut c = coord;
                loop {
                    c.0 += d0;
                    c.1 += d1;
                    if c.0 >= 0
                        && c.0 < n
                        && c.1 >= 0
                        && c.1 < n
                        && b.get_chess(c.0 as usize, c.1 as usize) == turn
                    {
                        cnt += 1
                    } else {
                        break;
                    }
                }
                let mut c = coord;
                loop {
                    c.0 -= d0;
                    c.1 -= d1;
                    if c.0 >= 0
                        && c.0 < n
                        && c.1 >= 0
                        && c.1 < n
                        && b.get_chess(c.0 as usize, c.1 as usize) == turn
                    {
                        cnt += 1
                    } else {
                        break;
                    }
                }
                cnt
            })
            .fold(0, |mx, x| std::cmp::max(mx, x));
        if max_in_a_line >= 5 {
            self.winner = turn;
        }
    }

    fn process_cmd(self: &mut Self) {
        loop {
            print!("cmd> ");
            io::stdout().flush().unwrap();
            let line = io::stdin().lock().lines().next().unwrap().unwrap();
            let mut cmditer = line.split_ascii_whitespace();
            let cmd0 = cmditer.next();
            if cmd0.is_none() {
                continue;
            }
            let cmd0 = cmd0.unwrap();
            if Game::match_cmd(cmd0, "continue") {
                println!("continue");
                if self.winner != board::Chess::EMPTY {
                    println!("{} already win", self.winner);
                    continue;
                }
                if self.board.current_turn() == board::Chess::BLACK {
                    if let Some((r, c)) = self.p1.play(&self.board) {
                        self.board.play(r as usize, c as usize);
                        display::display(&self.board);
                        self.check_win();
                        if self.winner != board::Chess::EMPTY {
                            println!("{} win", self.winner);
                        }
                    }
                } else {
                    if let Some((r, c)) = self.p2.play(&self.board) {
                        self.board.play(r as usize, c as usize);
                        display::display(&self.board);
                        self.check_win();
                        if self.winner != board::Chess::EMPTY {
                            println!("{} win", self.winner);
                        }
                    }
                }
            } else if Game::match_cmd(cmd0, "exit") {
                break;
            } else if Game::match_cmd(cmd0, "new") {
                println!("new");
                self.board = board::Board::new(BOARD_SIZE);
                self.winner = board::Chess::EMPTY;
                display::display(&self.board);
                continue;
            } else if Game::match_cmd(cmd0, "set") {
                println!("set");
            } else if Game::match_cmd(cmd0, "help") {
                println!("help:");
                println!("1. new: New Game");
                println!("2. exit: Exit Game");
                println!("3. continue: Next Player");
            } else {
                continue;
            }
        }
    }
}
