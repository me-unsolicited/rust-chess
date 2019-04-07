use std::u64;

use crate::engine::bb;
use crate::engine::bb::BitIterator;
use crate::engine::mov::Move;
use crate::engine::piece::PieceType;
use crate::engine::square::Square;

const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Copy, Clone)]
pub struct Board {
    placement: Placement,
    turn: Color,
    castle_rights: CastleRights,
    en_passant_target: Option<&'static Square>,
    halfmove_clock: u16,
    fullmove_number: u16,
}

#[derive(Copy, Clone)]
struct Placement {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,

    white: u64,
    black: u64,
}

#[derive(Copy, Clone)]
struct CastleRights {
    kingside_w: bool,
    queenside_w: bool,
    kingside_b: bool,
    queenside_b: bool,
}

#[derive(PartialEq, Copy, Clone)]
enum Color {
    WHITE = 0xffffff,
    BLACK = 0x000000,
}

impl Color {
    fn other(&self) -> Self {
        match self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        }
    }
}

impl Board {
    pub fn new(fen: &str) -> Board {
        let mut parts = fen.split_whitespace();

        let fen_placement = parts.next().expect("expected FEN piece placement");
        let fen_turn = parts.next().expect("expected FEN active color");
        let fen_castle_rights = parts.next().expect("expected FEN castling rights");
        let fen_en_passant_target = parts.next().expect("expected FEN en passant target");
        let fen_halfmove_clock = parts.next().expect("expected FEN halfmove clock");
        let fen_fullmove_number = parts.next().expect("expected FEN fullmove number");

        Board {
            placement: parse_placement(fen_placement).expect("failed to parse FEN piece placement"),
            turn: parse_turn(fen_turn).expect("failed to parse FEN en passant target"),
            castle_rights: parse_castle_rights(fen_castle_rights).expect("failed to parse FEN castling rights"),
            en_passant_target: Square::parse(fen_en_passant_target),
            halfmove_clock: fen_halfmove_clock.parse().expect("failed to parse FEN halfmove clock"),
            fullmove_number: fen_fullmove_number.parse().expect("failed to parse FEN fullmove number"),
        }
    }

    pub fn start_pos() -> Board { Board::new(START_FEN) }

    pub fn gen_moves(&self) -> Vec<Move> {

        // always generate moves from the perspective of the white pieces
        let mirror;
        let position = if self.turn == Color::BLACK {
            mirror = self.mirror();
            &mirror
        } else {
            self
        };

        let check_restriction = position.get_check_restriction();

        let mut moves = Vec::new();
        moves.append(&mut position.gen_pawn_moves(check_restriction));
        moves.append(&mut position.gen_knight_moves(check_restriction));
        moves.append(&mut position.gen_bishop_moves(check_restriction));
        moves.append(&mut position.gen_rook_moves(check_restriction));
        moves.append(&mut position.gen_queen_moves(check_restriction));
        moves.append(&mut position.gen_king_moves());

        // mirror the moves back to black perspective if necessary
        if self.turn == Color::BLACK {
            for mov in moves.iter_mut() {
                *mov = mov.mirror();
            }
        }

        moves
    }

    fn get_check_restriction(&self) -> u64 {
        let king = self.placement.white & self.placement.kings;
        let king_sq = bb::to_sq(king);

        // is a pawn checking the king?
        let pawn_bits = bb::PAWN_ATTACKS[bb::mirror_sq(king_sq) as usize].swap_bytes();
        let pawn_attackers = pawn_bits & self.placement.black & self.placement.pawns;
        if pawn_attackers != 0 {
            return pawn_attackers;
        }

        // is a knight checking the king?
        let jump_bits = bb::KNIGHT_MOVES[king_sq as usize];
        let jump_attackers = jump_bits & self.placement.black & self.placement.knights;
        if jump_attackers != 0 {
            return jump_attackers;
        }

        // is the king in check along a diagonal?
        let blockers = self.placement.white | self.placement.black;
        let diag_bits = bb::BISHOP_MOVES[king_sq as usize];
        let diag_attackers = diag_bits & self.placement.black & (self.placement.bishops | self.placement.queens);

        for sq in BitIterator::from(diag_attackers) {
            let (is_check, walk) = bb::walk_towards(king_sq, sq, blockers);
            if is_check {
                return walk;
            }
        }

        // is the king in check along a rank/file?
        let line_bits = bb::ROOK_MOVES[king_sq as usize];
        let line_attackers = line_bits & self.placement.black & (self.placement.rooks | self.placement.queens);

        for sq in BitIterator::from(line_attackers) {
            let (is_check, walk) = bb::walk_towards(king_sq, sq, blockers);
            if is_check {
                return walk;
            }
        }

        // king is not in check; there is no restriction
        !0
    }

    pub fn gen_pawn_moves(&self, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();

        let pawns = self.placement.white & self.placement.pawns;
        for sq in BitIterator::from(pawns) {
            moves.append(&mut self.gen_pawn_moves_from(sq, check_restriction));
        }

        moves
    }

    pub fn gen_pawn_moves_from(&self, sq: i32, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();
        let from = Square::SQUARES[sq as usize];

        // non-attacking moves
        let targets = bb::PAWN_MOVES[sq as usize];
        for to_sq in BitIterator::from(targets) {
            if !bb::has_bit(check_restriction, to_sq) {
                continue;
            }

            let blockers = self.placement.white | self.placement.black;
            if bb::is_blocked(sq, to_sq, blockers) {
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
        let targets = bb::PAWN_ATTACKS[sq as usize];
        for to_sq in BitIterator::from(targets) {
            if !bb::has_bit(check_restriction, to_sq) {
                continue;
            }

            let ep_capture = if self.en_passant_target.is_some() {
                1 << self.en_passant_target.unwrap().idx
            } else {
                0
            };
            let captures = self.placement.black | ep_capture;
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

    pub fn gen_knight_moves(&self, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();

        let knights = self.placement.white & self.placement.knights;
        for sq in BitIterator::from(knights) {
            moves.append(&mut self.gen_knight_moves_from(sq, check_restriction));
        }

        moves
    }

    pub fn gen_knight_moves_from(&self, sq: i32, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();
        let from = Square::SQUARES[sq as usize];

        let targets = bb::KNIGHT_MOVES[sq as usize];
        for to_sq in BitIterator::from(targets) {
            if !bb::has_bit(check_restriction, to_sq) {
                continue;
            }

            let blocked = 0 != self.placement.white & (1 << to_sq);
            if blocked {
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

    pub fn gen_bishop_moves(&self, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();

        let bishops = self.placement.white & self.placement.bishops;
        for sq in BitIterator::from(bishops) {
            moves.append(&mut self.gen_bishop_moves_from(sq, check_restriction));
        }

        moves
    }

    pub fn gen_bishop_moves_from(&self, sq: i32, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();
        let from = Square::SQUARES[sq as usize];

        let targets = bb::BISHOP_MOVES[sq as usize];
        for to_sq in BitIterator::from(targets) {
            if !bb::has_bit(check_restriction, to_sq) {
                continue;
            }

            let blockers = self.placement.white;
            let captures = self.placement.black;
            if bb::is_capture_blocked(sq, to_sq, blockers, captures) {
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

    pub fn gen_rook_moves(&self, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();

        let rooks = self.placement.white & self.placement.rooks;
        for sq in BitIterator::from(rooks) {
            moves.append(&mut self.gen_rook_moves_from(sq, check_restriction));
        }

        moves
    }

    pub fn gen_rook_moves_from(&self, sq: i32, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();
        let from = Square::SQUARES[sq as usize];

        let targets = bb::ROOK_MOVES[sq as usize];
        for to_sq in BitIterator::from(targets) {
            if !bb::has_bit(check_restriction, to_sq) {
                continue;
            }

            let blockers = self.placement.white;
            let captures = self.placement.black;
            if bb::is_capture_blocked(sq, to_sq, blockers, captures) {
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

    pub fn gen_queen_moves(&self, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();

        let queens = self.placement.white & self.placement.queens;
        for sq in BitIterator::from(queens) {
            moves.append(&mut self.gen_queen_moves_from(sq, check_restriction));
        }

        moves
    }

    pub fn gen_queen_moves_from(&self, sq: i32, check_restriction: u64) -> Vec<Move> {
        let mut moves = Vec::new();
        let from = Square::SQUARES[sq as usize];

        let targets = bb::QUEEN_MOVES[sq as usize];
        for to_sq in BitIterator::from(targets) {
            if !bb::has_bit(check_restriction, to_sq) {
                continue;
            }

            let blockers = self.placement.white;
            let captures = self.placement.black;
            if bb::is_capture_blocked(sq, to_sq, blockers, captures) {
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

    pub fn gen_king_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        let kings = self.placement.white & self.placement.kings;
        for sq in BitIterator::from(kings) {
            moves.append(&mut self.gen_king_moves_from(sq));
        }

        moves
    }

    pub fn gen_king_moves_from(&self, sq: i32) -> Vec<Move> {
        let mut moves = Vec::new();
        let from = Square::SQUARES[sq as usize];

        let targets = bb::KING_MOVES[sq as usize];
        for to_sq in BitIterator::from(targets) {

            let blockers = self.placement.white;
            let captures = self.placement.black;
            if bb::is_capture_blocked(sq, to_sq, blockers, captures) {
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

    pub fn update(&self, mov: Move) -> Self {
        let from_sq = mov.from.idx as i32;
        let to_sq = mov.to.idx as i32;

        let mut pawns = self.placement.pawns;
        let mut knights = self.placement.knights;
        let mut bishops = self.placement.bishops;
        let mut rooks = self.placement.rooks;
        let mut queens = self.placement.queens;
        let mut kings = self.placement.kings;

        let mut moving = None;
        if bb::has_bit(pawns, from_sq) {
            pawns = bb::clear_bit(pawns, from_sq);
            moving = Some(&PieceType::PAWN);
        } else if bb::has_bit(knights, from_sq) {
            knights = bb::clear_bit(knights, from_sq);
            moving = Some(&PieceType::KNIGHT);
        } else if bb::has_bit(bishops, from_sq) {
            bishops = bb::clear_bit(bishops, from_sq);
            moving = Some(&PieceType::BISHOP);
        } else if bb::has_bit(rooks, from_sq) {
            rooks = bb::clear_bit(rooks, from_sq);
            moving = Some(&PieceType::ROOK);
        } else if bb::has_bit(queens, from_sq) {
            queens = bb::clear_bit(queens, from_sq);
            moving = Some(&PieceType::QUEEN);
        } else if bb::has_bit(kings, from_sq) {
            kings = bb::clear_bit(kings, from_sq);
            moving = Some(&PieceType::KING);
        }

        let moving = moving.expect("tried to move a missing piece");
        let setting = match mov.promotion {
            Some(pt) => pt,
            _ => moving,
        };

        match *setting {
            PieceType::PAWN => { pawns = bb::set_bit(pawns, to_sq); }
            PieceType::KNIGHT => { knights = bb::set_bit(knights, to_sq); }
            PieceType::BISHOP => { bishops = bb::set_bit(bishops, to_sq); }
            PieceType::ROOK => { rooks = bb::set_bit(rooks, to_sq); }
            PieceType::QUEEN => { queens = bb::set_bit(queens, to_sq); }
            PieceType::KING => { kings = bb::set_bit(kings, to_sq); }
            _ => panic!("somehow setting down an unknown piece type"),
        }

        let mut capture_sq = to_sq;
        if *moving == PieceType::PAWN && self.en_passant_target.is_some() &&
            to_sq == self.en_passant_target.unwrap().idx as i32 {
            if self.turn == Color::WHITE {
                capture_sq -= 8;
            } else {
                capture_sq += 8;
            }
        }

        if *moving != PieceType::PAWN {
            pawns = bb::clear_bit(pawns, capture_sq);
        }
        if *moving != PieceType::KNIGHT {
            knights = bb::clear_bit(knights, capture_sq);
        }
        if *moving != PieceType::BISHOP {
            bishops = bb::clear_bit(bishops, capture_sq);
        }
        if *moving != PieceType::ROOK {
            rooks = bb::clear_bit(rooks, capture_sq);
        }
        if *moving != PieceType::QUEEN {
            queens = bb::clear_bit(queens, capture_sq);
        }
        if *moving != PieceType::KING {
            kings = bb::clear_bit(kings, capture_sq);
        }

        let castling_rook = mov.get_castling_rook();
        if let Some(rook_move) = castling_rook {
            rooks = bb::clear_bit(rooks, rook_move.from.idx as i32);
            rooks = bb::set_bit(rooks, rook_move.to.idx as i32);
        }

        let mut kingside_w = self.castle_rights.kingside_w;
        let mut queenside_w = self.castle_rights.queenside_w;
        let mut kingside_b = self.castle_rights.kingside_b;
        let mut queenside_b = self.castle_rights.queenside_b;

        let mut white = self.placement.white;
        let mut black = self.placement.black;

        match self.turn {
            Color::WHITE => {
                white = bb::clear_bit(white, from_sq);
                white = bb::set_bit(white, to_sq);
                black = bb::clear_bit(black, capture_sq);
                let white_ref = &mut white;
                if let Some(rook_move) = castling_rook {
                    *white_ref = bb::clear_bit(*white_ref, rook_move.from.idx as i32);
                    *white_ref = bb::set_bit(*white_ref, rook_move.to.idx as i32);
                    if rook_move.from.idx == Square::A1.idx {
                        queenside_w = false;
                    } else {
                        kingside_w = false;
                    }
                }
            }
            Color::BLACK => {
                black = bb::clear_bit(black, from_sq);
                black = bb::set_bit(black, to_sq);
                white = bb::clear_bit(white, capture_sq);
                let black_ref = &mut black;
                if let Some(rook_move) = castling_rook {
                    *black_ref = bb::clear_bit(*black_ref, rook_move.from.idx as i32);
                    *black_ref = bb::set_bit(*black_ref, rook_move.to.idx as i32);
                    if rook_move.from.idx == Square::A8.idx {
                        queenside_b = false;
                    } else {
                        kingside_b = false;
                    }
                }
            }
        }

        let (from_rank, _) = bb::to_rank_file(from_sq);
        let (to_rank, _) = bb::to_rank_file(to_sq);
        let mut en_passant_target = None;
        if *moving == PieceType::PAWN {
            if self.turn == Color::WHITE && from_rank == 1 && to_rank == 3 {
                en_passant_target = Some(Square::SQUARES[(to_sq - 8) as usize]);
            } else if self.turn == Color::BLACK && from_rank == 6 && to_rank == 4 {
                en_passant_target = Some(Square::SQUARES[(to_sq + 8) as usize]);
            }
        }

        Board {
            placement: Placement {
                pawns,
                knights,
                bishops,
                rooks,
                queens,
                kings,
                white,
                black,
            },
            turn: self.turn.other(),
            castle_rights: CastleRights {
                kingside_w,
                queenside_w,
                kingside_b,
                queenside_b,
            },
            en_passant_target,
            halfmove_clock: 0,
            fullmove_number: 0,
        }
    }

    pub fn mirror(&self) -> Board {

        // symmetrically swap white/black positions
        Board {
            placement: Placement {
                pawns: self.placement.pawns.swap_bytes(),
                knights: self.placement.knights.swap_bytes(),
                bishops: self.placement.bishops.swap_bytes(),
                rooks: self.placement.rooks.swap_bytes(),
                queens: self.placement.queens.swap_bytes(),
                kings: self.placement.kings.swap_bytes(),

                // white -> black, black -> white
                white: self.placement.black.swap_bytes(),
                black: self.placement.white.swap_bytes(),
            },
            turn: self.turn.other(),
            castle_rights: CastleRights {
                kingside_w: self.castle_rights.kingside_b,
                queenside_w: self.castle_rights.queenside_b,
                kingside_b: self.castle_rights.kingside_w,
                queenside_b: self.castle_rights.queenside_w,
            },
            en_passant_target: match self.en_passant_target {
                Some(square) => Some(square.mirror()),
                None => None,
            },
            halfmove_clock: self.halfmove_clock,
            fullmove_number: self.halfmove_clock,
        }
    }
}

fn parse_placement(fen: &str) -> Result<Placement, &str> {
    let mut pawns: u64 = 0;
    let mut knights: u64 = 0;
    let mut bishops: u64 = 0;
    let mut rooks: u64 = 0;
    let mut queens: u64 = 0;
    let mut kings: u64 = 0;

    let mut white: u64 = 0;
    let mut black: u64 = 0;

    let mut fen_ranks: Vec<&str> = fen.split_terminator("/").collect();
    fen_ranks.reverse();
    if fen_ranks.len() != 8 {
        return Err(fen);
    }

    for rank in 0..8 {
        let fen_rank = fen_ranks[rank];

        let mut file = 0;
        for c in fen_rank.chars() {
            let symbol = c.to_string();

            if file >= 8 {
                return Err(fen);
            }

            if let Ok(n) = symbol.parse::<usize>() {
                file += n;
                continue;
            };

            let square = Square::at(rank, file);

            match PieceType::parse(&symbol[..]) {
                Some(piece_type) => {
                    match *piece_type {
                        PieceType::PAWN => place(&mut pawns, square),
                        PieceType::KNIGHT => place(&mut knights, square),
                        PieceType::BISHOP => place(&mut bishops, square),
                        PieceType::ROOK => place(&mut rooks, square),
                        PieceType::QUEEN => place(&mut queens, square),
                        PieceType::KING => place(&mut kings, square),
                        _ => return Err(fen)
                    };

                    if c.is_uppercase() {
                        place(&mut white, square);
                    } else {
                        place(&mut black, square);
                    }
                }
                None => return Err(fen),
            }

            file += 1;
        }
    }

    Result::Ok(Placement {
        pawns,
        knights,
        bishops,
        rooks,
        queens,
        kings,

        white,
        black,
    })
}

fn parse_turn(fen: &str) -> Result<Color, &str> {
    match fen {
        "w" => Result::Ok(Color::WHITE),
        "b" => Result::Ok(Color::BLACK),
        _ => Err(fen),
    }
}

fn parse_castle_rights(fen: &str) -> Result<CastleRights, &str> {
    let mut kingside_w = false;
    let mut queenside_w = false;
    let mut kingside_b = false;
    let mut queenside_b = false;

    for c in fen.chars() {
        if c == '-' {
            break;
        }

        match c {
            'K' => kingside_w = true,
            'Q' => queenside_w = true,
            'k' => kingside_b = true,
            'q' => queenside_b = true,
            _ => return Result::Err(fen),
        }
    }

    Result::Ok(CastleRights {
        kingside_w,
        queenside_w,
        kingside_b,
        queenside_b,
    })
}

fn place(target: &mut u64, square: &Square) {
    *target |= 1 << square.idx;
}
