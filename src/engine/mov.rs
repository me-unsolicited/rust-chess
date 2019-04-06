use crate::engine::square::*;
use crate::engine::piece::*;

const PROMOTIONS: [PieceType; 4] = [
    PieceType::KNIGHT,
    PieceType::BISHOP,
    PieceType::ROOK,
    PieceType::QUEEN,
];

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: &'static Square,
    pub to: &'static Square,
    pub promotion: Option<&'static PieceType>,
}

impl Move {
    pub fn parse(algebra: &str) -> Option<Move> {
        let mut from: Option<&Square> = None;
        let mut to: Option<&Square> = None;
        let mut promotion: Option<&PieceType> = None;

        let len = algebra.len();
        if [4, 5].contains(&len) {
            from = Square::parse(&algebra[0..2]);
            to = Square::parse(&algebra[2..4]);

            if len == 5 {
                promotion = PieceType::parse(&algebra[4..5]);
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

    pub fn enumerate_promotions(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(PROMOTIONS.len());
        for promotion in PROMOTIONS.iter() {
            let mut mov = *self;
            mov.promotion = Option::from(promotion);
            moves.push(mov);
        }

        moves
    }

    pub fn mirror(&self) -> Move {
        Move {
            from: self.from.mirror(),
            to: self.to.mirror(),
            promotion: self.promotion
        }
    }
}
