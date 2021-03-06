use crate::engine::bb;
use crate::engine::bb::BitIterator;
use crate::engine::board::{Board, Color, Placement};
use crate::engine::mov::{KINGSIDE_CASTLE_W, Move, QUEENSIDE_CASTLE_W};
use crate::engine::square::Square;

pub fn gen_moves(board: &Board) -> Vec<Move> {

    // always generate moves from the perspective of the white pieces
    let mirror;
    let position = if board.turn == Color::BLACK {
        mirror = board.mirror();
        &mirror
    } else {
        board
    };

    let check_restriction = get_check_restriction(position);

    let mut moves = Vec::new();
    moves.append(&mut gen_pawn_moves(position, check_restriction));
    moves.append(&mut gen_knight_moves(position, check_restriction));
    moves.append(&mut gen_bishop_moves(position, check_restriction));
    moves.append(&mut gen_rook_moves(position, check_restriction));
    moves.append(&mut gen_queen_moves(position, check_restriction));
    moves.append(&mut gen_king_moves(position));
    moves.append(&mut gen_castling_moves(position, check_restriction));

    // mirror the moves back to black perspective if necessary
    if board.turn == Color::BLACK {
        for mov in moves.iter_mut() {
            *mov = mov.mirror();
        }
    }

    moves
}

pub fn get_check_restriction(board: &Board) -> u64 {
    let king = board.placement.white & board.placement.kings;
    let king_sq = bb::to_sq(king);

    get_check_restriction_at(&board.placement, king_sq)
}

fn get_check_restriction_at(placement: &Placement, king_sq: i32) -> u64 {

    // starting assumption is no restriction, i.e. king is not in check
    // double check may result in 0, full restriction, i.e. the king must move
    let mut check_restriction = !0;

    // is a pawn checking the king?
    let pawn_bits = bb::PAWN_ATTACKS[king_sq as usize];
    let pawn_attackers = pawn_bits & placement.black & placement.pawns;
    if pawn_attackers != 0 {
        check_restriction &= pawn_attackers;
    }

    // is a knight checking the king?
    let jump_bits = bb::KNIGHT_MOVES[king_sq as usize];
    let jump_attackers = jump_bits & placement.black & placement.knights;
    if jump_attackers != 0 {
        check_restriction &= jump_attackers;
    }

    // is the king in check along a diagonal?
    let blockers = placement.white | placement.black;
    let diag_bits = bb::BISHOP_MOVES[king_sq as usize];
    let diag_attackers = diag_bits & placement.black & (placement.bishops | placement.queens);

    for sq in BitIterator::from(diag_attackers) {
        let (is_check, walk) = bb::walk_towards(king_sq, sq, blockers);
        if is_check {
            check_restriction &= walk;
        }
    }

    // is the king in check along a rank/file?
    let line_bits = bb::ROOK_MOVES[king_sq as usize];
    let line_attackers = line_bits & placement.black & (placement.rooks | placement.queens);

    for sq in BitIterator::from(line_attackers) {
        let (is_check, walk) = bb::walk_towards(king_sq, sq, blockers);
        if is_check {
            check_restriction &= walk;
        }
    }

    check_restriction
}

fn get_pin_restriction(board: &Board, sq: i32) -> u64 {

    // piece placements after the square is cleared
    let mut into_placement = board.placement;

    // clear square in new position, sans king
    into_placement.pawns = bb::clear_bit(into_placement.pawns, sq);
    into_placement.knights = bb::clear_bit(into_placement.knights, sq);
    into_placement.bishops = bb::clear_bit(into_placement.bishops, sq);
    into_placement.rooks = bb::clear_bit(into_placement.rooks, sq);
    into_placement.queens = bb::clear_bit(into_placement.queens, sq);
    into_placement.white = bb::clear_bit(into_placement.white, sq);
    into_placement.black = bb::clear_bit(into_placement.black, sq);

    // see if the king is now in check
    let king = into_placement.white & into_placement.kings;
    get_check_restriction_at(&into_placement, bb::to_sq(king))
}

fn gen_pawn_moves(board: &Board, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();

    let pawns = board.placement.white & board.placement.pawns;
    for sq in BitIterator::from(pawns) {
        moves.append(&mut gen_pawn_moves_from(board, sq, check_restriction));
    }

    moves
}

fn gen_pawn_moves_from(board: &Board, sq: i32, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();
    let from = Square::SQUARES[sq as usize];

    let pin_restriction = get_pin_restriction(board, sq);
    let restriction = check_restriction & pin_restriction;

    // non-attacking moves
    let blockers = board.placement.white | board.placement.black;
    let targets = bb::PAWN_MOVES[sq as usize] & restriction;
    for to_sq in BitIterator::from(targets) {
        if bb::is_blocked(sq, to_sq, blockers, 0) {
            continue;
        }

        let mov = Move {
            from,
            to: Square::SQUARES[to_sq as usize],
            promotion: None,
        };

        let (rank, _) = bb::to_rank_file(to_sq);
        if rank < 7 {
            moves.push(mov);
        } else {
            moves.append(&mut mov.enumerate_promotions());
        }
    }

    // attacks
    let targets = bb::PAWN_ATTACKS[sq as usize] & restriction;
    for to_sq in BitIterator::from(targets) {
        let ep_capture = if board.en_passant_target.is_some() {
            1 << board.en_passant_target.unwrap().idx
        } else {
            0
        };

        let captures = board.placement.black | ep_capture;
        if 0 == captures & (1 << to_sq) {
            continue;
        }

        let mov = Move {
            from,
            to: Square::SQUARES[to_sq as usize],
            promotion: None,
        };

        let (rank, _) = bb::to_rank_file(to_sq);
        if rank < 7 {
            moves.push(mov);
        } else {
            moves.append(&mut mov.enumerate_promotions());
        }
    }

    moves
}

fn gen_knight_moves(board: &Board, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();

    let knights = board.placement.white & board.placement.knights;
    for sq in BitIterator::from(knights) {
        moves.append(&mut gen_knight_moves_from(board, sq, check_restriction));
    }

    moves
}

fn gen_knight_moves_from(board: &Board, sq: i32, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();
    let from = Square::SQUARES[sq as usize];

    let pin_restriction = get_pin_restriction(board, sq);
    let restriction = check_restriction & pin_restriction;

    let targets = bb::KNIGHT_MOVES[sq as usize] & restriction & !board.placement.white;
    for to_sq in BitIterator::from(targets) {
        moves.push(Move {
            from,
            to: Square::SQUARES[to_sq as usize],
            promotion: None,
        });
    }

    moves
}

fn gen_bishop_moves(board: &Board, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();

    let bishops = board.placement.white & board.placement.bishops;
    for sq in BitIterator::from(bishops) {
        moves.append(&mut gen_bishop_moves_from(board, sq, check_restriction));
    }

    moves
}

fn gen_bishop_moves_from(board: &Board, sq: i32, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();
    let from = Square::SQUARES[sq as usize];

    let pin_restriction = get_pin_restriction(board, sq);
    let restriction = check_restriction & pin_restriction;

    let targets = bb::BISHOP_MOVES[sq as usize] & restriction;
    for to_sq in BitIterator::from(targets) {
        let blockers = board.placement.white;
        let captures = board.placement.black;
        if bb::is_blocked(sq, to_sq, blockers, captures) {
            continue;
        }

        moves.push(Move {
            from,
            to: Square::SQUARES[to_sq as usize],
            promotion: None,
        });
    }

    moves
}

fn gen_rook_moves(board: &Board, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();

    let rooks = board.placement.white & board.placement.rooks;
    for sq in BitIterator::from(rooks) {
        moves.append(&mut gen_rook_moves_from(board, sq, check_restriction));
    }

    moves
}

fn gen_rook_moves_from(board: &Board, sq: i32, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();
    let from = Square::SQUARES[sq as usize];

    let pin_restriction = get_pin_restriction(board, sq);
    let restriction = check_restriction & pin_restriction;

    let targets = bb::ROOK_MOVES[sq as usize] & restriction;
    for to_sq in BitIterator::from(targets) {
        let blockers = board.placement.white;
        let captures = board.placement.black;
        if bb::is_blocked(sq, to_sq, blockers, captures) {
            continue;
        }

        moves.push(Move {
            from,
            to: Square::SQUARES[to_sq as usize],
            promotion: None,
        });
    }

    moves
}

fn gen_queen_moves(board: &Board, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();

    let queens = board.placement.white & board.placement.queens;
    for sq in BitIterator::from(queens) {
        moves.append(&mut gen_queen_moves_from(board, sq, check_restriction));
    }

    moves
}

fn gen_queen_moves_from(board: &Board, sq: i32, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();
    let from = Square::SQUARES[sq as usize];

    let pin_restriction = get_pin_restriction(board, sq);
    let restriction = check_restriction & pin_restriction;

    let targets = bb::QUEEN_MOVES[sq as usize] & restriction;
    for to_sq in BitIterator::from(targets) {
        let blockers = board.placement.white;
        let captures = board.placement.black;
        if bb::is_blocked(sq, to_sq, blockers, captures) {
            continue;
        }

        moves.push(Move {
            from,
            to: Square::SQUARES[to_sq as usize],
            promotion: None,
        });
    }

    moves
}

fn gen_king_moves(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    let kings = board.placement.white & board.placement.kings;
    for sq in BitIterator::from(kings) {
        moves.append(&mut gen_king_moves_from(board, sq));
    }

    moves
}

fn gen_king_moves_from(board: &Board, sq: i32) -> Vec<Move> {
    let mut moves = Vec::new();
    let from = Square::SQUARES[sq as usize];

    let targets = bb::KING_MOVES[sq as usize];
    for to_sq in BitIterator::from(targets) {
        let blockers = board.placement.white;
        let captures = board.placement.black;
        if bb::is_blocked(sq, to_sq, blockers, captures) {
            continue;
        }

        // don't move into check
        if is_into_check(board, sq, to_sq) {
            continue;
        }

        moves.push(Move {
            from,
            to: Square::SQUARES[to_sq as usize],
            promotion: None,
        });
    }

    moves
}

fn gen_castling_moves(board: &Board, check_restriction: u64) -> Vec<Move> {
    let mut moves = Vec::new();

    // cannot castle while in check
    if !0 != check_restriction {
        return moves;
    }

    let rights = board.castle_rights;
    let blockers = board.placement.white | board.placement.black;

    if rights.queenside_w {
        // verify that king is not in check along castling path
        let (mut can_castle, walk) = bb::walk_towards((QUEENSIDE_CASTLE_W.0).0.idx as i32, (QUEENSIDE_CASTLE_W.0).1.idx as i32, blockers);
        let (rook_free, _) = bb::walk_towards((QUEENSIDE_CASTLE_W.1).0.idx as i32, (QUEENSIDE_CASTLE_W.1).1.idx as i32, blockers);

        can_castle &= rook_free;

        if can_castle {
            for sq in BitIterator::from(walk) {
                let opposition = bb::KING_MOVES[sq as usize] & board.placement.black & board.placement.kings;
                if 0 != opposition || !0 != get_check_restriction_at(&board.placement, sq) {
                    can_castle = false;
                    break;
                }
            }
        }

        if can_castle {
            moves.push(Move {
                from: (QUEENSIDE_CASTLE_W.0).0,
                to: (QUEENSIDE_CASTLE_W.0).1,
                promotion: None,
            })
        }
    }

    if rights.kingside_w {
        // verify that king is not in check along castling path
        let (mut can_castle, walk) = bb::walk_towards((KINGSIDE_CASTLE_W.0).0.idx as i32, (KINGSIDE_CASTLE_W.0).1.idx as i32, blockers);
        let (rook_free, _) = bb::walk_towards((KINGSIDE_CASTLE_W.1).0.idx as i32, (KINGSIDE_CASTLE_W.1).1.idx as i32, blockers);

        can_castle &= rook_free;

        if can_castle {
            for sq in BitIterator::from(walk) {
                let opposition = bb::KING_MOVES[sq as usize] & board.placement.black & board.placement.kings;
                if 0 != opposition || !0 != get_check_restriction_at(&board.placement, sq) {
                    can_castle = false;
                    break;
                }
            }
        }

        if can_castle {
            moves.push(Move {
                from: (KINGSIDE_CASTLE_W.0).0,
                to: (KINGSIDE_CASTLE_W.0).1,
                promotion: None,
            })
        }
    }

    moves
}

fn is_into_check(board: &Board, king_sq: i32, to_sq: i32) -> bool {

    // can't approach opposing king
    let opposition = bb::KING_MOVES[to_sq as usize] & board.placement.black & board.placement.kings;
    if 0 != opposition {
        return true;
    }

    // piece placements after the king is moved
    let mut into_placement = board.placement;

    // clear current king position
    into_placement.kings = bb::clear_bit(into_placement.kings, king_sq);
    into_placement.white = bb::clear_bit(into_placement.white, king_sq);

    // set new king position
    into_placement.kings = bb::set_bit(into_placement.kings, to_sq);
    into_placement.white = bb::set_bit(into_placement.white, to_sq);

    // clear pieces in case of capture
    into_placement.pawns = bb::clear_bit(into_placement.pawns, to_sq);
    into_placement.knights = bb::clear_bit(into_placement.knights, to_sq);
    into_placement.bishops = bb::clear_bit(into_placement.bishops, to_sq);
    into_placement.rooks = bb::clear_bit(into_placement.rooks, to_sq);
    into_placement.queens = bb::clear_bit(into_placement.queens, to_sq);
    into_placement.black = bb::clear_bit(into_placement.black, to_sq);

    0 != !get_check_restriction_at(&into_placement, to_sq)
}
