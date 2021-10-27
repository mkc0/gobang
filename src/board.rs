pub enum Chess {
    EMPTY,
    WHITE,
    BLACK,
    INVALID,
}

pub struct Board {
    n: usize,
    board: Vec<Vec<u8>>,
    turn: u8,
}

impl Board {
    pub fn new(n: usize) -> Board {
        Board {
            n: n,
            board: (0..n).map(|_i| vec![0u8; n]).collect(),
            turn: 1u8,
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

    #[allow(dead_code)]
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
        true
    }

    pub fn size(self: &Self) -> usize {
        self.n
    }

    #[allow(dead_code)]
    pub fn whose_turn(self: &Self) -> Chess {
        match self.turn {
            1u8 => return Chess::BLACK,
            2u8 => return Chess::WHITE,
            _ => return Chess::INVALID,
        }
    }
}
