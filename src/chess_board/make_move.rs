use crate::{base_structures::Move, CastleRight, ChessBoard, MoveFlag, Piece, Square};

impl ChessBoard {
    pub fn make_move(&mut self, mv: Move) {
        let side_to_move = self.side_to_move().get_value();
        let from_square = mv.get_from_square();
        let to_square = mv.get_to_square();
        let moved_piece = self.get_piece_on_square(from_square);
        let captured_piece = if !mv.is_capture() {
            Piece::NONE
        } else {
            self.get_piece_on_square(to_square)
        };

        //add promotion exception later
        self.set_piece_on_square(to_square, self.side_to_move(), moved_piece);
        self.remove_piece_on_square(from_square, self.side_to_move(), moved_piece);

        if captured_piece != Piece::NONE {
            self.remove_piece_on_square(to_square, self.side_to_move().flipped(), captured_piece);
        }

        if moved_piece == Piece::PAWN || mv.is_capture() {
            *self.state.get_half_move_counter_mut() = 0;
        } else {
            *self.state.get_half_move_counter_mut() += 1;
        }

        let mut castle_rights = self.castle_rights().get_value();
        castle_rights &= !(CastleRight::ROOK_MASKS[from_square.get_value() as usize]
            | CastleRight::ROOK_MASKS[to_square.get_value() as usize]);
        let castle_rights_difference = self.castle_rights().get_value() ^ castle_rights;
        self.state
            .get_key_mut()
            .update_castle_rights_diff_hash(castle_rights_difference);
        *self.state.get_castle_rights_mut() = CastleRight::from_raw(castle_rights);

        let en_passant_square = self.en_passant_square();
        if en_passant_square != Square::NULL {
            self.state
                .get_key_mut()
                .update_en_passant_hash(en_passant_square);
        }

        *self.state.get_en_passant_mut() = Square::NULL;

        let flag = mv.get_flag();
        match flag {
            MoveFlag::DOUBLE_PUSH => {
                *self.state.get_en_passant_mut() = to_square ^ 8;
                self.state
                    .get_key_mut()
                    .update_en_passant_hash(to_square ^ 8);
            }
            MoveFlag::KING_SIDE_CASTLE | MoveFlag::QUEEN_SIDE_CASTLE => {
                let king_side = usize::from(flag == MoveFlag::KING_SIDE_CASTLE);
                let side_flip = 56 * side_to_move;
                let rook_from_square = side_flip + CastleRight::ROOK_POSITIONS[king_side];
                let rook_to_square = side_flip + [3, 5][king_side];
                self.remove_piece_on_square(
                    Square::from_raw(rook_from_square),
                    self.side_to_move(),
                    Piece::ROOK,
                );
                self.set_piece_on_square(
                    Square::from_raw(rook_to_square),
                    self.side_to_move(),
                    Piece::ROOK,
                );
            }
            MoveFlag::EN_PASSANT => self.remove_piece_on_square(
                to_square ^ 8,
                self.side_to_move().flipped(),
                Piece::PAWN,
            ),
            MoveFlag::KNIGHT_PROMOTION.. => {
                let promotion_piece = mv.get_promotion_piece();
                self.remove_piece_on_square(to_square, self.side_to_move(), Piece::PAWN);
                self.set_piece_on_square(to_square, self.side_to_move(), promotion_piece);
            }
            _ => {}
        }

        self.state.get_side_to_move_mut().mut_flip();
        self.state.get_key_mut().update_side_to_move_hash();

        self.generate_checkers_mask();

        self.move_history[self.half_move_counter() as usize] = self.get_key();
    }
}
