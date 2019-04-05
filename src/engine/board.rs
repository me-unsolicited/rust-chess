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

#[derive(Copy, Clone)]
enum Color {
    WHITE = 0xffffff,
    BLACK = 0x000000,
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
        let mut moves = Vec::new();
        moves.append(&mut self.gen_pawn_moves());
        moves.append(&mut self.gen_knight_moves());
        moves.append(&mut self.gen_bishop_moves());
        moves.append(&mut self.gen_rook_moves());
        moves.append(&mut self.gen_queen_moves());
        moves.append(&mut self.gen_king_moves());

        moves
    }

    pub fn gen_pawn_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        let pawns = self.placement.white & self.placement.pawns;
        for sq in BitIterator::from(pawns) {
            moves.append(&mut self.gen_pawn_moves_from(sq));
        }

        moves
    }

    pub fn gen_pawn_moves_from(&self, sq: i32) -> Vec<Move> {
        let mut moves = Vec::new();
        let from = Square::SQUARES[sq as usize];

        // non-attacking moves
        let targets = bb::PAWN_MOVES[sq as usize];
        for to_sq in BitIterator::from(targets) {
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

    pub fn gen_knight_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        let knights = self.placement.white & self.placement.knights;
        for sq in BitIterator::from(knights) {
            moves.append(&mut self.gen_knight_moves_from(sq));
        }

        moves
    }
    pub fn gen_knight_moves_from(&self, sq: i32) -> Vec<Move> {
        let mut moves = Vec::new();
        let from = Square::SQUARES[sq as usize];

        let targets = bb::KNIGHT_MOVES[sq as usize];
        for to_sq in BitIterator::from(targets) {
            let blocked = 0 != self.placement.white & (1 << to_sq);
            if blocked {
                continue;
            }

            moves.push(Move {
                from,
                to: Square::SQUARES[to_sq as usize],
                promotion: None
            });
        }

        moves
    }

    pub fn gen_bishop_moves(&self) -> Vec<Move> {
        Vec::new()
    }

    pub fn gen_rook_moves(&self) -> Vec<Move> {
        Vec::new()
    }

    pub fn gen_queen_moves(&self) -> Vec<Move> {
        Vec::new()
    }

    pub fn gen_king_moves(&self) -> Vec<Move> {
        Vec::new()
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
