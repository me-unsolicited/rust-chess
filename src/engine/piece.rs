use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct PieceType {
    pub symbol: &'static str,
}

lazy_static! {
    static ref SYMBOL_MAP: HashMap<&'static str, &'static PieceType> = {
        let mut map = HashMap::with_capacity(PieceType::PIECE_TYPES.len());
        for piece_type in PieceType::PIECE_TYPES.iter() {
            map.insert(&piece_type.symbol[..], *piece_type);
        }

        map
    };
}

impl PieceType {
    pub const PAWN: Self = Self { symbol: "p" };
    pub const KNIGHT: Self = Self { symbol: "n" };
    pub const BISHOP: Self = Self { symbol: "b" };
    pub const ROOK: Self = Self { symbol: "r" };
    pub const QUEEN: Self = Self { symbol: "q" };
    pub const KING: Self = Self { symbol: "k" };

    pub const PIECE_TYPES: [&'static PieceType; 6] = [
        &Self::PAWN,
        &Self::KNIGHT,
        &Self::BISHOP,
        &Self::ROOK,
        &Self::QUEEN,
        &Self::KING
    ];

    pub fn parse(symbol: &str) -> Option<&'static Self> {
        match SYMBOL_MAP.get(&symbol.to_lowercase()[..]) {
            Some(piece_type) => Option::from(*piece_type),
            None => None,
        }
    }
}
