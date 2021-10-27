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
            turn: 1u8,
            history: Vec::<(usize, usize)>::new(),
        }
    }

    pub fn get_chess(self: &Self, i: usize, j: usize) -> Chess {
        match self.board[i][j] {
            0u8 => return Chess::EMPTY,
            1u8 => return Chess::BLACK,
            2u8 => return Chess::WHITE,
            _ => return Chess::INVALID,
        }
    }

    // ret: Success or not
    pub fn play(self: &mut Self, i: usize, j: usize) -> bool {
        if i >= self.n || j >= self.n {
            return false;
        }
        if self.board[i][j] != 0 {
            return false;
        }
        self.board[i][j] = self.turn;
        self.turn = 3 - self.turn;
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
            self.turn = 3 - self.turn;
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

    pub fn size(self: &Self) -> usize {
        self.n
    }

    pub fn current_turn(self: &Self) -> Chess {
        match self.turn {
            1u8 => return Chess::BLACK,
            2u8 => return Chess::WHITE,
            _ => return Chess::INVALID,
        }
    }
}
