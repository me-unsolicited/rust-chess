use crate::engine::board::{Board, Color};
use crate::engine::mov::Move;
use crate::engine::piece::PieceType;

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
    eval += KNIGHT_VALUE * white_knights.count_ones() as i32;
    eval += BISHOP_VALUE * white_bishops.count_ones() as i32;
    eval += ROOK_VALUE * white_rooks.count_ones() as i32;
    eval += QUEEN_VALUE * white_queens.count_ones() as i32;

    eval -= PAWN_VALUE * black_pawns.count_ones() as i32;
    eval -= KNIGHT_VALUE * black_knights.count_ones() as i32;
    eval -= BISHOP_VALUE * black_bishops.count_ones() as i32;
    eval -= ROOK_VALUE * black_rooks.count_ones() as i32;
    eval -= QUEEN_VALUE * black_queens.count_ones() as i32;

    eval
}

pub fn evaluate_exchange(board: &Board, mov: &Move) -> i32 {

    let mut capture_sq = mov.to.idx as i32;
    if let Some(ep_target) = board.en_passant_target {
        let moving_pawn = 0 != board.placement.pawns & (1 << mov.from.idx);
        if moving_pawn && ep_target.idx == mov.to.idx {
            match board.turn {
                Color::WHITE => capture_sq -= 8,
                Color::BLACK => capture_sq += 8,
            }
        }
    }

    let mut promotion_eval = 0;
    if let Some(piece_type) = mov.promotion {
        promotion_eval += evaluate_piece(piece_type, mov.to.idx as i32);
    }

    evaluate_sq(board, capture_sq) - evaluate_sq(board, mov.from.idx as i32) + promotion_eval
}

fn evaluate_sq(board: &Board, sq: i32) -> i32 {
    let bit = 1 << sq;

    let mut eval = 0;
    eval += PAWN_VALUE * ((bit & board.placement.pawns) >> sq) as i32;
    eval += KNIGHT_VALUE * ((bit & board.placement.knights) >> sq) as i32;
    eval += BISHOP_VALUE * ((bit & board.placement.bishops) >> sq) as i32;
    eval += ROOK_VALUE * ((bit & board.placement.rooks) >> sq) as i32;
    eval += QUEEN_VALUE * ((bit & board.placement.queens) >> sq) as i32;

    eval
}

fn evaluate_piece(piece_type: &PieceType, _sq: i32) -> i32 {
    match piece_type {
        &PieceType::PAWN => PAWN_VALUE,
        &PieceType::KNIGHT => KNIGHT_VALUE,
        &PieceType::BISHOP => BISHOP_VALUE,
        &PieceType::ROOK => ROOK_VALUE,
        &PieceType::QUEEN => QUEEN_VALUE,
        _ => 0,
    }
}
