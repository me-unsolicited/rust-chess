use crate::engine::square::*;

#[derive(Debug)]
pub struct Move {
    from: Square,
    to: Square,
}

impl Move {
    pub fn parse(algebra: &str) -> Option<Move> {
        let mut from: Option<Square> = None;
        let mut to: Option<Square> = None;

        let len = algebra.len();
        if [4, 5].contains(&len) {
            from = Square::parse(&algebra[0..2]);
            to = Square::parse(&algebra[2..4]);
        }

        if from.is_none() || to.is_none() {
            return None;
        }

        Option::from(Move {
            from: from.unwrap(),
            to: to.unwrap(),
        })
    }
}
