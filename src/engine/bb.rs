const NO_MOVE: u64 = 0;

lazy_static! {
    pub static ref PAWN_MOVES: [u64; 64] = init_pawn_moves();
    pub static ref PAWN_ATTACKS: [u64; 64] = init_pawn_attacks();
    pub static ref KNIGHT_MOVES: [u64; 64] = init_knight_moves();
}

fn init_pawn_moves() -> [u64; 64] {
    let mut moves = [u64; 64];
    for sq in 0..64 {
        moves[sq] = init_pawn_move(sq);
    }

    moves
}

fn init_pawn_move(sq: usize) -> u64 {
    let (rank, file) = to_rank_file(sq);

    if rank == 0 || rank == 8 {
        return NO_MOVE;
    }

    if rank == 1 {
        return to_bit(2, file) | to_bit(3, file);
    }

    to_bit(rank + 1, file)
}

fn init_pawn_attacks() -> [u64; 64] {
    let mut moves = [u64; 64];
    for sq in 0..64 {
        moves[sq] = init_pawn_attack(sq);
    }

    moves
}

fn init_pawn_attack(sq: usize) -> u64 {
    let (rank, file) = to_rank_file(sq);

    if rank == 0 || rank == 8 {
        return NO_MOVE;
    }

    let mut left = NO_MOVE;
    if file > 0 {
        left = to_bit(rank + 1, file - 1);
    }

    let mut right = NO_MOVE;
    if right < 7 {
        right = to_bit(rank + 1, file + 1);
    }

    left | right
}

fn init_knight_moves() -> [u64; 64] {
    let mut moves = [u64; 64];
    for sq in 0..64 {
        moves[sq] = init_knight_move(sq);
    }

    moves
}


fn init_knight_move(sq: usize) -> u64 {
    let (rank, file) = to_rank_file(sq);

    // like clock hands; get it?
    //
    // |  |11|  | 1|  |
    // |10|  |  |  | 2|
    // |  |  | N|  |  |
    // | 8|  |  |  | 4|
    // |  | 7|  | 5|  |

    let mut one = NO_MOVE;
    if rank < 6 && file < 7 {
        one = to_bit(rank + 2, file + 1)
    }

    let mut two = NO_MOVE;
    if rank < 7 && file < 6 {
        two = to_bit(rank + 1, file + 2)
    }

    let mut four = NO_MOVE;
    if rank > 0 && file < 6 {
        four = to_bit(rank - 1, file + 2)
    }

    let mut five = NO_MOVE;
    if rank > 1 && file < 7 {
        five = to_bit(rank - 2, file + 1)
    }

    let mut seven = NO_MOVE;
    if rank > 1 && file > 0 {
        seven = to_bit(rank - 2, file - 1)
    }

    let mut eight = NO_MOVE;
    if rank > 0 && file > 1 {
        eight = to_bit(rank - 1, file - 2)
    }

    let mut ten = NO_MOVE;
    if rank < 7 && file > 1 {
        ten = to_bit(rank + 1, file - 2)
    }

    let mut eleven = NO_MOVE;
    if rank < 6 && file > 0 {
        eleven = to_bit(rank + 2, file - 1)
    }

    one | two | four | five | seven | eight | ten | eleven
}

fn to_rank_file(sq: usize) -> (usize, usize) {
    (sq / 8, sq % 8)
}

fn to_bit(rank: usize, file: usize) -> u64 {
    1 << (u64)(rank * 8 + file)
}
