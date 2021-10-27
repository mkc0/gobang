use crate::board::Board;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

pub trait Player {
    fn play(self: &Self, board: &Board) -> Option<(u8, u8)>;
}

pub struct HumanPlayer {}

impl Player for HumanPlayer {
    fn play(self: &Self, board: &Board) -> Option<(u8, u8)> {
        let n = board.size() as i32;
        let halfn = n / 2;
        loop {
            print!("put> ");
            io::stdout().flush().unwrap();
            let line = io::stdin().lock().lines().next().unwrap().unwrap();
            let mut iter = line.split_ascii_whitespace();
            let s = iter.next();
            if s.is_none() {
                continue;
            }
            let r:i32 = FromStr::from_str(s.unwrap()).unwrap_or(n) + halfn;
            let s = iter.next();
            if s.is_none() {
                continue;
            }
            let c:i32 = FromStr::from_str(s.unwrap()).unwrap_or(n) + halfn;
            if r < n && c < n && r >= 0 && c >= 0{
                return Some((r as u8, c as u8));
            }
        }
    }
}
