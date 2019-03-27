use crate::engine::square::*;

#[derive(Debug)]
pub struct Move {
    from: Square,
    to: Square,
}

impl Move {
    pub fn parse(algebra: &str) -> Option<Move> {

        println!("move: {}", algebra);

        // TODO support all the moves
        if algebra != "a1a1" {
            return Option::None;
        }

        Option::from(Move {
            from: Square::A1,
            to: Square::A1,
        })
    }
}
