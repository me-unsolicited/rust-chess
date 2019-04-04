const NO_MOVE: u64 = 0;

lazy_static! {
    pub static ref PAWN_MOVES: [u64; 64] = init_pawn_moves();
    pub static ref PAWN_ATTACKS: [u64; 64] = init_pawn_attacks();
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

fn to_rank_file(sq: usize) -> (usize, usize) {
    (sq / 8, sq % 8)
}

fn to_bit(rank: usize, file: usize) -> u64 {
    1 << (u64)(rank * 8 + file)
}
