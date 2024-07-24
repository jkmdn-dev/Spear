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
        let queens = self.get_piece_mask_for_side(Piece::QUEEN, attacker_side);
        (Attacks::get_bishop_attacks_for_square(square, occupancy)
            & (self.get_piece_mask_for_side(Piece::BISHOP, attacker_side) | queens))
            | (Attacks::get_knight_attacks_for_square(square) & self.get_piece_mask_for_side(Piece::KNIGHT, attacker_side))
            | (Attacks::get_rook_attacks_for_square(square, occupancy)
                & (self.get_piece_mask_for_side(Piece::ROOK, attacker_side) | queens))
            | (Attacks::get_pawn_attacks_for_square(square, self.side_to_move())
                & self.get_piece_mask_for_side(Piece::PAWN, attacker_side))
            | (Attacks::get_king_attacks_for_square(square) & self.get_piece_mask_for_side(Piece::KING, attacker_side))
    }

    pub fn is_square_attacked_with_occupancy(&self, square: Square, attacker_side: Side, occupancy_mask: Bitboard) -> bool {
        let bishop_queen_mask =
            self.get_piece_mask_for_side(Piece::BISHOP, attacker_side) | self.get_piece_mask_for_side(Piece::QUEEN, attacker_side);
        let rook_queen_mask =
            self.get_piece_mask_for_side(Piece::ROOK, attacker_side) | self.get_piece_mask_for_side(Piece::QUEEN, attacker_side);

        (Attacks::get_bishop_attacks_for_square(square, occupancy_mask) & bishop_queen_mask).is_not_empty()
        || (Attacks::get_knight_attacks_for_square(square) & self.get_piece_mask_for_side(Piece::KNIGHT, attacker_side))
            .is_not_empty()
        || (Attacks::get_rook_attacks_for_square(square, occupancy_mask) & rook_queen_mask).is_not_empty()
        || (Attacks::get_pawn_attacks_for_square(square, attacker_side.flipped())
            & self.get_piece_mask_for_side(Piece::PAWN, attacker_side)).is_not_empty()
        || (Attacks::get_king_attacks_for_square(square) & self.get_piece_mask_for_side(Piece::KING, attacker_side))
            .is_not_empty()
    }

    #[inline]
    pub fn is_square_attacked(&self, square: Square, attacker_side: Side) -> bool {
        self.is_square_attacked_with_occupancy(square, attacker_side, self.get_occupancy())
    }
}