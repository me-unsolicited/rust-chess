const NO_MOVE: u64 = 0;

lazy_static! {
    pub static ref PAWN_MOVES: [u64; 64] = init_pawn_moves();
    pub static ref PAWN_ATTACKS: [u64; 64] = init_pawn_attacks();
    pub static ref KNIGHT_MOVES: [u64; 64] = init_knight_moves();
    pub static ref BISHOP_MOVES: [u64; 64] = init_bishop_moves();
    pub static ref ROOK_MOVES: [u64; 64] = init_rook_moves();
    pub static ref QUEEN_MOVES: [u64; 64] = init_queen_moves();
    pub static ref KING_MOVES: [u64; 64] = init_king_moves();
}

fn init_pawn_moves() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for sq in 0..64 {
        moves[sq] = init_pawn_move(sq as i32);
    }

    moves
}

fn init_pawn_move(sq: i32) -> u64 {
    let (rank, file) = to_rank_file(sq);

    if rank == 0 || rank == 7 {
        return NO_MOVE;
    }

    if rank == 1 {
        return to_bit(2, file) | to_bit(3, file);
    }

    to_bit(rank + 1, file)
}

fn init_pawn_attacks() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for sq in 0..64 {
        moves[sq] = init_pawn_attack(sq as i32);
    }

    moves
}

fn init_pawn_attack(sq: i32) -> u64 {
    let (rank, file) = to_rank_file(sq);

    if rank == 0 || rank == 7 {
        return NO_MOVE;
    }

    let left = to_bit(rank + 1, file - 1);
    let right = to_bit(rank + 1, file + 1);

    left | right
}

fn init_knight_moves() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for sq in 0..64 {
        moves[sq] = init_knight_move(sq as i32);
    }

    moves
}

fn init_knight_move(sq: i32) -> u64 {
    let (rank, file) = to_rank_file(sq);

    // like clock hands; get it?
    //
    // |  |11|  | 1|  |
    // |10|  |  |  | 2|
    // |  |  | N|  |  |
    // | 8|  |  |  | 4|
    // |  | 7|  | 5|  |

    let one = to_bit(rank + 2, file + 1);
    let two = to_bit(rank + 1, file + 2);
    let four = to_bit(rank - 1, file + 2);
    let five = to_bit(rank - 2, file + 1);
    let seven = to_bit(rank - 2, file - 1);
    let eight = to_bit(rank - 1, file - 2);
    let ten = to_bit(rank + 1, file - 2);
    let eleven = to_bit(rank + 2, file - 1);

    one | two | four | five | seven | eight | ten | eleven
}

fn init_bishop_moves() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for sq in 0..64 {
        moves[sq] = init_bishop_move(sq as i32);
    }

    moves
}

fn init_bishop_move(sq: i32) -> u64 {
    let (rank, file) = to_rank_file(sq);

    // like a compass
    //
    // |NW|  |NE|
    // |  | B|  |
    // |SW|  |SE|

    let ne = walk_to_edge(rank, file, -1, 1);
    let se = walk_to_edge(rank, file, 1, 1);
    let sw = walk_to_edge(rank, file, 1, -1);
    let nw = walk_to_edge(rank, file, -1, -1);

    ne | se | sw | nw
}

fn init_rook_moves() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for sq in 0..64 {
        moves[sq] = init_rook_move(sq as i32);
    }

    moves
}

fn init_rook_move(sq: i32) -> u64 {
    let (rank, file) = to_rank_file(sq);

    // like a compass
    //
    // | |N| |
    // |W|R|E|
    // | |S| |

    let n = walk_to_edge(rank, file, 1, 0);
    let e = walk_to_edge(rank, file, 0, 1);
    let s = walk_to_edge(rank, file, -1, 0);
    let w = walk_to_edge(rank, file, 0, -1);

    n | e | s | w
}

fn init_queen_moves() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for sq in 0..64 {
        moves[sq] = ROOK_MOVES[sq] | BISHOP_MOVES[sq];
    }

    moves
}

fn init_king_moves() -> [u64; 64] {
    let mut moves: [u64; 64] = [0; 64];
    for sq in 0..64 {
        moves[sq] = init_king_move(sq as i32);
    }

    moves
}

fn init_king_move(sq: i32) -> u64 {
    let (rank, file) = to_rank_file(sq);

    // like a compass
    //
    // |NW|N |NE|
    // | W| K| E|
    // |SW|S |SE|

    let n = to_bit(rank + 1, file);
    let ne = to_bit(rank + 1, file + 1);
    let e = to_bit(rank, file + 1);
    let se = to_bit(rank - 1, file + 1);
    let s = to_bit(rank - 1, file);
    let sw = to_bit(rank - 1, file - 1);
    let w = to_bit(rank, file - 1);
    let nw = to_bit(rank + 1, file - 1);

    n | ne | e | se | s | sw | w | nw
}

fn walk_to_edge(rank: i32, file: i32, rank_walk: i32, file_walk: i32) -> u64 {
    let (mut r, mut f) = (rank, file);
    let mut walk = NO_MOVE;
    loop {
        r += rank_walk;
        f += file_walk;
        let bit = to_bit(r, f);
        walk |= bit;
        if bit == NO_MOVE { break; }
    }

    walk
}

pub fn to_rank_file(sq: i32) -> (i32, i32) {
    (sq / 8, sq % 8)
}

fn to_bit(rank: i32, file: i32) -> u64 {
    if rank < 0 || rank > 7 || file < 0 || file > 7 {
        return NO_MOVE;
    }

    (1 as u64) << (rank * 8 + file)
}

pub fn is_blocked(from: i32, to: i32, blockers: u64) -> bool {
    let (from_rank, from_file) = to_rank_file(from);
    let (to_rank, to_file) = to_rank_file(to);

    let rank_dir = (to_rank - from_rank).signum();
    let file_dir = (to_file - from_file).signum();

    let mut blocked = false;
    let mut reached_rank = rank_dir == 0;
    let mut reached_file = file_dir == 0;

    let mut rank = from_rank;
    let mut file = from_file;

    while !blocked && (!reached_rank || !reached_file) {
        rank += rank_dir;
        file += file_dir;
        blocked = NO_MOVE != blockers & to_bit(rank, file);
        reached_rank = reached_rank || rank == to_rank;
        reached_file = reached_file || file == to_file;
    }

    blocked
}

pub fn is_capture_blocked(from: i32, to: i32, blockers: u64, captures: u64) -> bool {
    let (from_rank, from_file) = to_rank_file(from);
    let (to_rank, to_file) = to_rank_file(to);

    let rank_dir = (to_rank - from_rank).signum();
    let file_dir = (to_file - from_file).signum();

    let mut blocked = false;
    let mut capture = false;
    let mut reached_rank = rank_dir == 0;
    let mut reached_file = file_dir == 0;

    let mut rank = from_rank;
    let mut file = from_file;

    while !blocked && !capture && (!reached_rank || !reached_file) {
        rank += rank_dir;
        file += file_dir;
        let bit = to_bit(rank, file);
        blocked = NO_MOVE != blockers & bit;
        capture = NO_MOVE != captures & bit;
        reached_rank = reached_rank || rank == to_rank;
        reached_file = reached_file || file == to_file;
    }

    blocked || (capture && (!reached_rank || !reached_file))
}

pub struct BitIterator {
    bits: u64
}

impl From<u64> for BitIterator {
    fn from(bits: u64) -> Self {
        BitIterator { bits }
    }
}

impl Iterator for BitIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {

        if self.bits == 0 {
            return None;
        }

        let value = self.bits.trailing_zeros();
        self.bits ^= 1 << value;

        Some(value as i32)
    }
}
