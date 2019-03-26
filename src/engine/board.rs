const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
}

impl Board {

    pub fn new(_fen: &str) -> Board {
        // TODO parse fen
        Board {
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            kings: 0,
        }
    }

    pub fn start_pos() -> Board {
        Board::new(START_POS)
    }
}
