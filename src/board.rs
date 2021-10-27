pub enum Chess {
    EMPTY,
    WHITE,
    BLACK,
    INVALID,
}

#[allow(dead_code)]
pub struct Board {
    n: usize,
    board: Vec<Vec<u8>>,
    turn: u8,
    history: Vec<(usize, usize)>,
}

#[allow(dead_code)]
impl Board {
    pub fn new(n: usize) -> Board {
        Board {
            n: n,
            board: (0..n).map(|_i| vec![0u8; n]).collect(),
            turn: 0,
            history: Vec::<(usize, usize)>::new(),
        }
    }

    fn turn_to_chess(x: u8) -> Chess {
        match x {
            0u8 => return Chess::EMPTY,
            1u8 => return Chess::BLACK,
            2u8 => return Chess::WHITE,
            _ => return Chess::INVALID,
        }
    }

    fn valid_coord(self: &Self, i: usize, j: usize) -> bool {
        i < self.n && j < self.n
    }

    pub fn next_turn(self: &Self) -> u8 {
        if self.turn == 0u8 {
            return 1u8;
        }
        3 - self.turn
    }

    pub fn prev_turn(self: &Self) -> u8 {
        if self.turn == 0u8 || self.history.len() == 0 {
            return 0u8;
        }
        3 - self.turn
    }

    pub fn get_chess(self: &Self, i: usize, j: usize) -> Chess {
        if !self.valid_coord(i, j) {
            return Chess::INVALID;
        }
        Board::turn_to_chess(self.board[i][j])
    }

    // ret: Success or not
    pub fn play(self: &mut Self, i: usize, j: usize) -> bool {
        if !self.valid_coord(i, j) {
            return false;
        }
        if self.board[i][j] != 0 {
            return false;
        }
        self.turn = self.next_turn();
        self.board[i][j] = self.turn;
        self.history.push((i, j));
        true
    }

    pub fn backward(self: &mut Self, nsteps: usize) -> bool {
        if self.history.len() < nsteps {
            return false;
        }
        for _ in 0..nsteps {
            let (r, c) = self.history.pop().unwrap();
            self.board[r][c] = 0;
            self.turn = self.prev_turn();
        }
        true
    }

    pub fn last_coord(self: &Self) -> Option<(usize, usize)> {
        let nhis = self.history.len();
        if nhis == 0 {
            return None;
        }
        Some(self.history[nhis - 1])
    }

    // pub fn win(self: &Self) -> bool {
    //     let turn = self.turn;
    //     if turn == 0 {
    //         return false;
    //     }
    //     let coord = self.last_coord().unwrap();
        
    //     true
    // }

    pub fn size(self: &Self) -> usize {
        self.n
    }

    pub fn current_turn(self: &Self) -> Chess {
        Board::turn_to_chess(self.next_turn())
    }
}
