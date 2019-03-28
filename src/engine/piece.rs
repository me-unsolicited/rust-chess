#[derive(Debug)]
pub enum Piece {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

impl Piece {
    pub fn parse(algebra: &str) -> Option<Self> {
        match algebra {
            "p" => Option::from(Piece::PAWN),
            "n" => Option::from(Piece::KNIGHT),
            "b" => Option::from(Piece::BISHOP),
            "r" => Option::from(Piece::ROOK),
            "q" => Option::from(Piece::QUEEN),
            "k" => Option::from(Piece::KING),
            _ => None,
        }
    }
}
