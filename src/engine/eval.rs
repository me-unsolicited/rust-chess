use crate::engine::board::Board;

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 300;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;

pub fn evaluate(board: &Board) -> i32 {

    let white_pawns = board.placement.white & board.placement.pawns;
    let white_knights = board.placement.white & board.placement.knights;
    let white_bishops = board.placement.white & board.placement.bishops;
    let white_rooks = board.placement.white & board.placement.rooks;
    let white_queens = board.placement.white & board.placement.queens;

    let black_pawns = board.placement.black & board.placement.pawns;
    let black_knights = board.placement.black & board.placement.knights;
    let black_bishops = board.placement.black & board.placement.bishops;
    let black_rooks = board.placement.black & board.placement.rooks;
    let black_queens = board.placement.black & board.placement.queens;

    let mut eval = 0;
    eval += PAWN_VALUE * white_pawns.count_ones() as i32;
    eval += BISHOP_VALUE * white_bishops.count_ones() as i32;
    eval += KNIGHT_VALUE * white_knights.count_ones() as i32;
    eval += ROOK_VALUE * white_rooks.count_ones() as i32;
    eval += QUEEN_VALUE * white_queens.count_ones() as i32;

    eval -= PAWN_VALUE * black_pawns.count_ones() as i32;
    eval -= BISHOP_VALUE * black_bishops.count_ones() as i32;
    eval -= KNIGHT_VALUE * black_knights.count_ones() as i32;
    eval -= ROOK_VALUE * black_rooks.count_ones() as i32;
    eval -= QUEEN_VALUE * black_queens.count_ones() as i32;

    eval
}
