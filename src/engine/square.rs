use std::collections::HashMap;

#[derive(Debug)]
pub struct Square {
    pub idx: u8,
    pub symbol: &'static str,
}

lazy_static! {
    static ref SYMBOL_MAP: HashMap<&'static str, &'static Square> = {
        let mut map = HashMap::with_capacity(Square::SQUARES.len());
        for square in Square::SQUARES.iter() {
            map.insert(&square.symbol[..], *square);
        }

        map
    };
}


impl Square {
    pub const A1: Self = Self { idx: 0, symbol: "a1" };
    pub const B1: Self = Self { idx: 1, symbol: "b1" };
    pub const C1: Self = Self { idx: 2, symbol: "c1" };
    pub const D1: Self = Self { idx: 3, symbol: "d1" };
    pub const E1: Self = Self { idx: 4, symbol: "e1" };
    pub const F1: Self = Self { idx: 5, symbol: "f1" };
    pub const G1: Self = Self { idx: 6, symbol: "g1" };
    pub const H1: Self = Self { idx: 7, symbol: "h1" };
    pub const A2: Self = Self { idx: 8, symbol: "a2" };
    pub const B2: Self = Self { idx: 9, symbol: "b2" };
    pub const C2: Self = Self { idx: 10, symbol: "c2" };
    pub const D2: Self = Self { idx: 11, symbol: "d2" };
    pub const E2: Self = Self { idx: 12, symbol: "e2" };
    pub const F2: Self = Self { idx: 13, symbol: "f2" };
    pub const G2: Self = Self { idx: 14, symbol: "g2" };
    pub const H2: Self = Self { idx: 15, symbol: "h2" };
    pub const A3: Self = Self { idx: 16, symbol: "a3" };
    pub const B3: Self = Self { idx: 17, symbol: "b3" };
    pub const C3: Self = Self { idx: 18, symbol: "c3" };
    pub const D3: Self = Self { idx: 19, symbol: "d3" };
    pub const E3: Self = Self { idx: 20, symbol: "e3" };
    pub const F3: Self = Self { idx: 21, symbol: "f3" };
    pub const G3: Self = Self { idx: 22, symbol: "g3" };
    pub const H3: Self = Self { idx: 23, symbol: "h3" };
    pub const A4: Self = Self { idx: 24, symbol: "a4" };
    pub const B4: Self = Self { idx: 25, symbol: "b4" };
    pub const C4: Self = Self { idx: 26, symbol: "c4" };
    pub const D4: Self = Self { idx: 27, symbol: "d4" };
    pub const E4: Self = Self { idx: 28, symbol: "e4" };
    pub const F4: Self = Self { idx: 29, symbol: "f4" };
    pub const G4: Self = Self { idx: 30, symbol: "g4" };
    pub const H4: Self = Self { idx: 31, symbol: "h4" };
    pub const A5: Self = Self { idx: 32, symbol: "a5" };
    pub const B5: Self = Self { idx: 33, symbol: "b5" };
    pub const C5: Self = Self { idx: 34, symbol: "c5" };
    pub const D5: Self = Self { idx: 35, symbol: "d5" };
    pub const E5: Self = Self { idx: 36, symbol: "e5" };
    pub const F5: Self = Self { idx: 37, symbol: "f5" };
    pub const G5: Self = Self { idx: 38, symbol: "g5" };
    pub const H5: Self = Self { idx: 39, symbol: "h5" };
    pub const A6: Self = Self { idx: 40, symbol: "a6" };
    pub const B6: Self = Self { idx: 41, symbol: "b6" };
    pub const C6: Self = Self { idx: 42, symbol: "c6" };
    pub const D6: Self = Self { idx: 43, symbol: "d6" };
    pub const E6: Self = Self { idx: 44, symbol: "e6" };
    pub const F6: Self = Self { idx: 45, symbol: "f6" };
    pub const G6: Self = Self { idx: 46, symbol: "g6" };
    pub const H6: Self = Self { idx: 47, symbol: "h6" };
    pub const A7: Self = Self { idx: 48, symbol: "a7" };
    pub const B7: Self = Self { idx: 49, symbol: "b7" };
    pub const C7: Self = Self { idx: 50, symbol: "c7" };
    pub const D7: Self = Self { idx: 51, symbol: "d7" };
    pub const E7: Self = Self { idx: 52, symbol: "e7" };
    pub const F7: Self = Self { idx: 53, symbol: "f7" };
    pub const G7: Self = Self { idx: 54, symbol: "g7" };
    pub const H7: Self = Self { idx: 55, symbol: "h7" };
    pub const A8: Self = Self { idx: 56, symbol: "a8" };
    pub const B8: Self = Self { idx: 57, symbol: "b8" };
    pub const C8: Self = Self { idx: 58, symbol: "c8" };
    pub const D8: Self = Self { idx: 59, symbol: "d8" };
    pub const E8: Self = Self { idx: 60, symbol: "e8" };
    pub const F8: Self = Self { idx: 61, symbol: "f8" };
    pub const G8: Self = Self { idx: 62, symbol: "g8" };
    pub const H8: Self = Self { idx: 63, symbol: "h8" };

    pub const SQUARES: [&'static Self; 64] = [
        &Self::A1, &Self::B1, &Self::C1, &Self::D1, &Self::E1, &Self::F1, &Self::G1, &Self::H1,
        &Self::A2, &Self::B2, &Self::C2, &Self::D2, &Self::E2, &Self::F2, &Self::G2, &Self::H2,
        &Self::A3, &Self::B3, &Self::C3, &Self::D3, &Self::E3, &Self::F3, &Self::G3, &Self::H3,
        &Self::A4, &Self::B4, &Self::C4, &Self::D4, &Self::E4, &Self::F4, &Self::G4, &Self::H4,
        &Self::A5, &Self::B5, &Self::C5, &Self::D5, &Self::E5, &Self::F5, &Self::G5, &Self::H5,
        &Self::A6, &Self::B6, &Self::C6, &Self::D6, &Self::E6, &Self::F6, &Self::G6, &Self::H6,
        &Self::A7, &Self::B7, &Self::C7, &Self::D7, &Self::E7, &Self::F7, &Self::G7, &Self::H7,
        &Self::A8, &Self::B8, &Self::C8, &Self::D8, &Self::E8, &Self::F8, &Self::G8, &Self::H8,
    ];

    pub fn parse(symbol: &str) -> Option<&'static Self> {
        match SYMBOL_MAP.get(symbol) {
            Some(square) => Option::from(*square),
            None => None,
        }
    }

    pub fn at(rank: usize, file: usize) -> &'static Square {
        Self::SQUARES[rank * 8 + file]
    }
}
