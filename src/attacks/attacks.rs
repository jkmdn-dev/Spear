use crate::{base_structures::Side, Bitboard, ChessBoard, Piece, Square};

use super::{bishop_attacks::BishopAttacks, king_attacks::KingAttacks, knight_attacks::KnightAttacks, pawn_attacks::PawnsAttacks, rook_attacks::RookAttacks};

pub struct Attacks;
impl Attacks {
    #[inline]
    pub fn get_king_attacks_for_square(square: Square) -> Bitboard {
        KingAttacks::ATTACK_TABLE[square.get_value() as usize]
    }

    #[inline]
    pub fn get_knight_attacks_for_square(square: Square) -> Bitboard {
        KnightAttacks::ATTACK_TABLE[square.get_value() as usize]
    }

    #[inline]
    pub fn get_pawn_attacks_for_square(square: Square, side: Side) -> Bitboard {
        PawnsAttacks::ATTACK_TABLE[side.get_value() as usize][square.get_value() as usize]
    }

    #[inline]
    pub fn get_bishop_attacks_for_square(square: Square, occupancy: Bitboard) -> Bitboard {
        BishopAttacks::get_bishop_attacks(square, occupancy)
    }

    #[inline]
    pub fn get_rook_attacks_for_square(square: Square, occupancy: Bitboard) -> Bitboard {
        RookAttacks::get_rook_attacks(square, occupancy)
    }
}

impl ChessBoard {
    pub fn all_attackers_to_square(&self, occupancy: Bitboard, square: Square, attacker_side: Side) -> Bitboard {
        let queens = self.get_piece_mask(Piece::QUEEN);
        ((Attacks::get_knight_attacks_for_square(square) & self.get_piece_mask(Piece::KNIGHT))
        | (Attacks::get_king_attacks_for_square(square) & self.get_piece_mask(Piece::KING))
        | (Attacks::get_pawn_attacks_for_square(square, attacker_side.flipped()) & self.get_piece_mask(Piece::PAWN))
        | (Attacks::get_rook_attacks_for_square(square, occupancy) & (self.get_piece_mask(Piece::ROOK) | queens))
        | (Attacks::get_bishop_attacks_for_square(square, occupancy) & (self.get_piece_mask(Piece::BISHOP) | queens)))
        & self.get_occupancy_for_side(attacker_side)
    }

    pub fn is_square_attacked_with_occupancy(&self, square: Square, attacker_side: Side, occupancy: Bitboard) -> bool {
        self.all_attackers_to_square(occupancy, square, attacker_side).is_not_empty()
    }

    #[inline]
    pub fn is_square_attacked(&self, square: Square, attacker_side: Side) -> bool {
        self.is_square_attacked_with_occupancy(square, attacker_side, self.get_occupancy())
    }
}