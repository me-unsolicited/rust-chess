use rand::RngCore;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use crate::engine::bb::BitIterator;
use crate::engine::board::{Board, Color};

const SEED: [u8; 16] = [
    '*' as u8,
    '*' as u8,
    '*' as u8,
    'r' as u8,
    'u' as u8,
    's' as u8,
    't' as u8,
    '|' as u8,
    'c' as u8,
    'h' as u8,
    'e' as u8,
    's' as u8,
    's' as u8,
    '*' as u8,
    '*' as u8,
    '*' as u8,
];

struct Hash {
    pawns: [u64; 64],
    knights: [u64; 64],
    bishops: [u64; 64],
    rooks: [u64; 64],
    queens: [u64; 64],
    kings: [u64; 64],
    white: [u64; 64],
    black: [u64; 64],
    turn: [u64; 2],
    castle_wq: [u64; 2],
    castle_wk: [u64; 2],
    castle_bq: [u64; 2],
    castle_bk: [u64; 2],
    en_passant: [u64; 8],
}

impl Hash {
    pub fn new() -> Self {
        let mut rng: XorShiftRng = SeedableRng::from_seed(SEED);

        Self {
            pawns: hashes_64(&mut rng),
            knights: hashes_64(&mut rng),
            bishops: hashes_64(&mut rng),
            rooks: hashes_64(&mut rng),
            queens: hashes_64(&mut rng),
            kings: hashes_64(&mut rng),
            white: hashes_64(&mut rng),
            black: hashes_64(&mut rng),
            turn: hashes_2(&mut rng),
            castle_wq: hashes_2(&mut rng),
            castle_wk: hashes_2(&mut rng),
            castle_bq: hashes_2(&mut rng),
            castle_bk: hashes_2(&mut rng),
            en_passant: hashes_8(&mut rng),
        }
    }
}

fn hashes_2(rng: &mut impl RngCore) -> [u64; 2] {
    let mut hashes = [0; 2];
    for hash in hashes.iter_mut() {
        *hash = rng.next_u64();
    }

    hashes
}

fn hashes_8(rng: &mut impl RngCore) -> [u64; 8] {
    let mut hashes = [0; 8];
    for hash in hashes.iter_mut() {
        *hash = rng.next_u64();
    }

    hashes
}

fn hashes_64(rng: &mut impl RngCore) -> [u64; 64] {
    let mut hashes = [0; 64];
    for hash in hashes.iter_mut() {
        *hash = rng.next_u64();
    }

    hashes
}


lazy_static! {
    static ref HASH: Hash = Hash::new();
}

pub fn of(position: &Board) -> u64 {
    let mut hash = 0;
    hash ^= hash_from(position.placement.pawns, HASH.pawns);
    hash ^= hash_from(position.placement.knights, HASH.knights);
    hash ^= hash_from(position.placement.bishops, HASH.bishops);
    hash ^= hash_from(position.placement.rooks, HASH.rooks);
    hash ^= hash_from(position.placement.queens, HASH.queens);
    hash ^= hash_from(position.placement.kings, HASH.kings);
    hash ^= hash_from(position.placement.white, HASH.white);
    hash ^= hash_from(position.placement.black, HASH.black);
    hash ^= HASH.turn[(position.turn == Color::WHITE) as usize];
    hash ^= HASH.castle_wq[(position.castle_rights.queenside_w) as usize];
    hash ^= HASH.castle_wk[(position.castle_rights.kingside_w) as usize];
    hash ^= HASH.castle_bq[(position.castle_rights.queenside_b) as usize];
    hash ^= HASH.castle_bk[(position.castle_rights.kingside_b) as usize];
    if let Some(square) = position.en_passant_target {
        hash ^= HASH.en_passant[(square.idx % 8) as usize];
    }

    hash
}

fn hash_from(bits: u64, hashes: [u64; 64]) -> u64 {
    let mut hash = 0;
    for i in BitIterator::from(bits) {
        hash ^= hashes[i as usize];
    }

    hash
}
