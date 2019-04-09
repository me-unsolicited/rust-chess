use crate::engine::piece::*;
use crate::engine::square::*;

const PROMOTIONS: [PieceType; 4] = [
    PieceType::KNIGHT,
    PieceType::BISHOP,
    PieceType::ROOK,
    PieceType::QUEEN,
];

pub const QUEENSIDE_CASTLE_W: ((&Square, &Square), (&Square, &Square)) = ((&Square::E1, &Square::C1), (&Square::A1, &Square::D1));
pub const KINGSIDE_CASTLE_W: ((&Square, &Square), (&Square, &Square)) = ((&Square::E1, &Square::G1), (&Square::H1, &Square::F1));
pub const QUEENSIDE_CASTLE_B: ((&Square, &Square), (&Square, &Square)) = ((&Square::E8, &Square::C8), (&Square::A8, &Square::D8));
pub const KINGSIDE_CASTLE_B: ((&Square, &Square), (&Square, &Square)) = ((&Square::E8, &Square::G8), (&Square::H8, &Square::F8));

pub const CASTLES: [((&Square, &Square), (&Square, &Square)); 4] = [
    QUEENSIDE_CASTLE_W,
    KINGSIDE_CASTLE_W,
    QUEENSIDE_CASTLE_B,
    KINGSIDE_CASTLE_B,
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
            promotion: self.promotion,
        }
    }

    pub fn get_castling_rook(&self) -> Option<Move> {
        for castle in CASTLES.iter() {
            let king = castle.0;
            let rook = castle.1;
            if king.0.idx == self.from.idx && king.1.idx == self.to.idx {
                return Some(Move {
                    from: rook.0,
                    to: rook.1,
                    promotion: None,
                });
            }
        }

        None
    }

    pub fn uci(&self) -> String {
        let mut repr = String::new();
        repr.push_str(self.from.symbol);
        repr.push_str(self.to.symbol);

        if self.promotion.is_some() {
            repr.push_str(self.promotion.unwrap().symbol);
        }

        repr
    }
}
