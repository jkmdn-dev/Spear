use crate::{attacks::Rays, base_structures::Side, ChessBoard, Move};

pub struct MoveGen;
impl ChessBoard {
    #[inline]
    pub fn map_moves<F: FnMut(Move)>(&self, mut method: F){
        Self::map_moves_internal::<F, false>(&self, &mut method)
    }

    #[inline]
    pub fn map_captures<F: FnMut(Move)>(&self, mut method: F){
        Self::map_moves_internal::<F, true>(&self, &mut method)
    }

    fn map_moves_internal<F: FnMut(Move), const CAPTURE_ONLY: bool>(&self, method: &mut F) {
        let diagonal_pins = self.generate_diagonal_pins_mask();
        let ortographic_pins = self.generate_ortographic_pins_mask();
        let king_square = self.get_king_square(self.side_to_move());
        let checkers = self.generate_checkers_mask();
        let attack_map = self.generate_attack_map(self.side_to_move().flipped());

        MoveGen::generate_king_moves::<F, CAPTURE_ONLY>(self, attack_map, king_square, method);

        if checkers.is_empty() {
            if !CAPTURE_ONLY {
                MoveGen::generate_castle_moves(self, attack_map, king_square, method)
            }

            let push_map = !self.get_occupancy();
            let capture_map = self.get_occupancy_for_side(self.side_to_move().flipped());

            if self.side_to_move() == Side::WHITE {
                MoveGen::generate_pawn_moves::<F, true, CAPTURE_ONLY>(self, push_map, capture_map, diagonal_pins, ortographic_pins, method)
            } else {
                MoveGen::generate_pawn_moves::<F, false, CAPTURE_ONLY>(self, push_map, capture_map, diagonal_pins, ortographic_pins, method)
            }

            MoveGen::generate_piece_moves::<F, CAPTURE_ONLY, { MoveGen::KNIGHT }>(self, push_map, capture_map, diagonal_pins, ortographic_pins, method);
            MoveGen::generate_piece_moves::<F, CAPTURE_ONLY, { MoveGen::BISHOP }>(self, push_map, capture_map, diagonal_pins, ortographic_pins, method);
            MoveGen::generate_piece_moves::<F, CAPTURE_ONLY, { MoveGen::ROOK }>(self, push_map, capture_map, diagonal_pins, ortographic_pins, method);
        } else if (checkers & (checkers - 1)).is_empty() {
            let checker = checkers.ls1b_square();
            let push_map = Rays::get_ray(king_square, checker).exclude(checker);
            if self.side_to_move() == Side::WHITE {
                MoveGen::generate_pawn_moves::<F, true, CAPTURE_ONLY>(self, push_map, checkers, diagonal_pins, ortographic_pins, method)
            } else {
                MoveGen::generate_pawn_moves::<F, false, CAPTURE_ONLY>(self, push_map, checkers, diagonal_pins, ortographic_pins, method)
            }

            MoveGen::generate_piece_moves::<F, CAPTURE_ONLY, { MoveGen::KNIGHT }>(self, push_map, checkers, diagonal_pins, ortographic_pins, method);
            MoveGen::generate_piece_moves::<F, CAPTURE_ONLY, { MoveGen::BISHOP }>(self, push_map, checkers, diagonal_pins, ortographic_pins, method);
            MoveGen::generate_piece_moves::<F, CAPTURE_ONLY, { MoveGen::ROOK }>(self, push_map, checkers, diagonal_pins, ortographic_pins, method);
        }
    }
}