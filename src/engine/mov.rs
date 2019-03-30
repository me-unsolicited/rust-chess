use crate::engine::square::*;
use crate::engine::piece::*;

#[derive(Debug)]
pub struct Move {
    from: &'static Square,
    to: &'static Square,
    promotion: Option<Piece>,
}

impl Move {
    pub fn parse(algebra: &str) -> Option<Move> {
        let mut from: Option<&Square> = None;
        let mut to: Option<&Square> = None;
        let mut promotion: Option<Piece> = None;

        let len = algebra.len();
        if [4, 5].contains(&len) {
            from = Square::parse(&algebra[0..2]);
            to = Square::parse(&algebra[2..4]);

            if len == 5 {
                promotion = Piece::parse(&algebra[4..5]);
            }
        }

        if from.is_none() || to.is_none() {
            return None;
        }

        Option::from(Move {
            from: from.unwrap(),
            to: to.unwrap(),
            promotion,
        })
    }
}
