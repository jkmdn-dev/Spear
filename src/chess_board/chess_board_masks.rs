use crate::{
    attacks::{Attacks, Rays},
    base_structures::Side,
    Bitboard, ChessBoard, Piece,
};

#[derive(Clone, Copy, Default)]
pub struct ChessBoardMasks {
    checkers: Bitboard,
}

impl ChessBoard {
    #[inline]
    pub fn is_in_check(&self) -> bool {
        self.masks.checkers.is_not_empty()
    }

    #[inline]
    pub fn checkers(&self) -> Bitboard {
        self.masks.checkers
    }

    #[inline]
    pub fn generate_checkers_mask(&mut self) {
        self.masks.checkers = self.all_attackers_to_square(
            self.get_occupancy(),
            self.get_king_square(self.side_to_move()),
            self.side_to_move().flipped(),
        );
    }

    pub fn generate_ortographic_pins_mask(&self) -> Bitboard {
        let attacker_color = self.side_to_move().flipped();
        let king_square = self.get_king_square(self.side_to_move());
        let relevant_pieces = self.get_piece_mask_for_side(Piece::ROOK, attacker_color)
            | self.get_piece_mask_for_side(Piece::QUEEN, attacker_color);
        let potential_pinners = Attacks::get_rook_attacks_for_square(
            king_square,
            self.get_occupancy_for_side(attacker_color),
        ) & relevant_pieces;
        let mut result = Bitboard::EMPTY;
        potential_pinners.map(|potential_pinner| {
            let ray = Rays::get_ray(king_square, potential_pinner);
            if (ray & self.get_occupancy_for_side(self.side_to_move())).only_one_bit() {
                result |= ray;
            }
        });
        result
    }

    pub fn generate_diagonal_pins_mask(&self) -> Bitboard {
        let attacker_color = self.side_to_move().flipped();
        let king_square = self.get_king_square(self.side_to_move());
        let relevant_pieces = self.get_piece_mask_for_side(Piece::BISHOP, attacker_color)
            | self.get_piece_mask_for_side(Piece::QUEEN, attacker_color);
        let potential_pinners = Attacks::get_bishop_attacks_for_square(
            king_square,
            self.get_occupancy_for_side(attacker_color),
        ) & relevant_pieces;
        let mut result = Bitboard::EMPTY;
        potential_pinners.map(|potential_pinner| {
            let ray = Rays::get_ray(king_square, potential_pinner);
            if (ray & self.get_occupancy_for_side(self.side_to_move())).only_one_bit() {
                result |= ray;
            }
        });
        result
    }

    pub fn generate_attack_map(&self, attacker_side: Side) -> Bitboard {
        let mut threats = Bitboard::EMPTY;

        let king_square = self.get_king_square(attacker_side.flipped());
        let occupancy = self.get_occupancy() ^ king_square.get_bit();

        let attacker_pieces = self.get_occupancy_for_side(attacker_side);
        let queens = self.get_piece_mask(Piece::QUEEN);

        (attacker_pieces & (self.get_piece_mask(Piece::ROOK) | queens)).map(|rook_square| {
            threats |= Attacks::get_rook_attacks_for_square(rook_square, occupancy)
        });

        (attacker_pieces & (self.get_piece_mask(Piece::BISHOP) | queens)).map(|bishop_square| {
            threats |= Attacks::get_bishop_attacks_for_square(bishop_square, occupancy)
        });

        (attacker_pieces & self.get_piece_mask(Piece::KING))
            .map(|king_square| threats |= Attacks::get_king_attacks_for_square(king_square));

        (attacker_pieces & self.get_piece_mask(Piece::KNIGHT))
            .map(|knight_square| threats |= Attacks::get_knight_attacks_for_square(knight_square));

        (attacker_pieces & self.get_piece_mask(Piece::PAWN)).map(|pawn_square| {
            threats |= Attacks::get_pawn_attacks_for_square(pawn_square, attacker_side)
        });

        threats
    }
}
