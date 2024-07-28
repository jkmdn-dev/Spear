use crate::{base_structures::{Move, Side}, CastleRight, ChessBoard, MoveFlag, MoveHistory, Piece, Square};

impl ChessBoard {
    pub fn make_move(&mut self, mv: Move, move_history: &mut MoveHistory) {
        self.make_move_stm(mv, mv.get_from_square(), mv.get_to_square(), move_history)
    }

    #[inline]
    fn make_move_stm(&mut self, mv: Move, from_square: Square, to_square: Square, move_history: &mut MoveHistory) {
        if self.side_to_move() == Side::WHITE {
            self.make_move_move_flag::<true, false>(mv, from_square, to_square, move_history)
        } else {
            self.make_move_move_flag::<false, true>(mv, from_square, to_square, move_history)
        }
    }

    #[inline]
    fn make_move_move_flag<const STM_WHITE: bool, const NSTM_WHITE: bool>(&mut self, mv: Move, from_square: Square, to_square: Square, move_history: &mut MoveHistory) {
        match mv.get_flag() {
            MoveFlag::QUIET_MOVE => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::QUIET_MOVE }>(mv, from_square, to_square, move_history),
            MoveFlag::DOUBLE_PUSH => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::DOUBLE_PUSH }>(mv, from_square, to_square, move_history),
            MoveFlag::KING_SIDE_CASTLE => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::KING_SIDE_CASTLE }>(mv, from_square, to_square, move_history),
            MoveFlag::QUEEN_SIDE_CASTLE => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::QUEEN_SIDE_CASTLE }>(mv, from_square, to_square, move_history),
            MoveFlag::CAPTURE => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::CAPTURE }>(mv, from_square, to_square, move_history),
            MoveFlag::EN_PASSANT => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::EN_PASSANT }>(mv, from_square, to_square, move_history),
            MoveFlag::KNIGHT_PROMOTION => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::KNIGHT_PROMOTION }>(mv, from_square, to_square, move_history),
            MoveFlag::BISHOP_PROMOTION => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::BISHOP_PROMOTION }>(mv, from_square, to_square, move_history),
            MoveFlag::ROOK_PROMOTION => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::ROOK_PROMOTION }>(mv, from_square, to_square, move_history),
            MoveFlag::QUEEN_PROMOTION => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::QUEEN_PROMOTION }>(mv, from_square, to_square, move_history),
            MoveFlag::KNIGHT_PROMOTION_CAPTURE => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::KNIGHT_PROMOTION_CAPTURE }>(mv, from_square, to_square, move_history),
            MoveFlag::BISHOP_PROMOTION_CAPTURE => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::BISHOP_PROMOTION_CAPTURE }>(mv, from_square, to_square, move_history),
            MoveFlag::ROOK_PROMOTION_CAPTURE => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::ROOK_PROMOTION_CAPTURE }>(mv, from_square, to_square, move_history),
            MoveFlag::QUEEN_PROMOTION_CAPTURE => self.make_move_moved_piece::<STM_WHITE, NSTM_WHITE, { MoveFlag::QUEEN_PROMOTION_CAPTURE }>(mv, from_square, to_square, move_history),
            _ => unreachable!()
        } 
    }

    #[inline]
    fn make_move_moved_piece<const STM_WHITE: bool, const NSTM_WHITE: bool, const MOVE_FLAG: u16>(&mut self, mv: Move, from_square: Square, to_square: Square, move_history: &mut MoveHistory) {
        let moved_piece = self.get_piece_on_square(from_square);
        match moved_piece {
            Piece::PAWN => self.make_move_captrued_piece::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, { PAWN }>(mv, from_square, to_square, move_history),
            Piece::KNIGHT => self.make_move_captrued_piece::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, { KNIGHT }>(mv, from_square, to_square, move_history),
            Piece::BISHOP => self.make_move_captrued_piece::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, { BISHOP }>(mv, from_square, to_square, move_history),
            Piece::ROOK => self.make_move_captrued_piece::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, { ROOK }>(mv, from_square, to_square, move_history),
            Piece::QUEEN => self.make_move_captrued_piece::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, { QUEEN }>(mv, from_square, to_square, move_history),
            Piece::KING => self.make_move_captrued_piece::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, { KING }>(mv, from_square, to_square, move_history),
            Piece::NONE => self.make_move_captrued_piece::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, { NONE }>(mv, from_square, to_square, move_history),
            _ => unreachable!()
        }
    }

    #[inline]
    fn make_move_captrued_piece<const STM_WHITE: bool, const NSTM_WHITE: bool, const MOVE_FLAG: u16, const MOVED_PIECE: u8>(&mut self, mv: Move, from_square: Square, to_square: Square, move_history: &mut MoveHistory) {
        if MOVE_FLAG & MoveFlag::CAPTURE == 0 {
            self.make_move_internal::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, MOVED_PIECE, { NONE }>(mv, from_square, to_square, move_history);
            return;
        } 

        let captured_piece = self.get_piece_on_square(to_square);
        match captured_piece {
            Piece::PAWN => self.make_move_internal::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, MOVED_PIECE, { PAWN }>(mv, from_square, to_square, move_history),
            Piece::KNIGHT => self.make_move_internal::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, MOVED_PIECE, { KNIGHT }>(mv, from_square, to_square, move_history),
            Piece::BISHOP => self.make_move_internal::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, MOVED_PIECE, { BISHOP }>(mv, from_square, to_square, move_history),
            Piece::ROOK => self.make_move_internal::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, MOVED_PIECE, { ROOK }>(mv, from_square, to_square, move_history),
            Piece::QUEEN => self.make_move_internal::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, MOVED_PIECE, { QUEEN }>(mv, from_square, to_square, move_history),
            Piece::NONE => self.make_move_internal::<STM_WHITE, NSTM_WHITE, MOVE_FLAG, MOVED_PIECE, { NONE }>(mv, from_square, to_square, move_history),
            _ => unreachable!()
        }
    }
    
    fn make_move_internal<const STM_WHITE: bool, const NSTM_WHITE: bool, const MOVE_FLAG: u16, const MOVED_PIECE: u8, const CAPTURED_PIECE: u8>(&mut self, mv: Move, from_square: Square, to_square: Square, move_history: &mut MoveHistory) {

        if CAPTURED_PIECE != NONE {
            self.remove_piece_on_square::<NSTM_WHITE>(to_square, Piece::from_raw(CAPTURED_PIECE));
        }
    
        //add promotion exception later
        self.set_piece_on_square::<STM_WHITE>(to_square, Piece::from_raw(MOVED_PIECE));
        self.remove_piece_on_square::<STM_WHITE>(from_square, Piece::from_raw(MOVED_PIECE));
    
        if MOVED_PIECE == PAWN || MOVE_FLAG & MoveFlag::CAPTURE > 0 {
            *self.state.get_half_move_counter_mut() = 0;
            move_history.reset()
        } else {
            *self.state.get_half_move_counter_mut() += 1;
        }
    
        let mut castle_rights = self.castle_rights().get_raw();
        castle_rights &= !(CastleRight::ROOK_MASKS[from_square.get_raw() as usize]
            | CastleRight::ROOK_MASKS[to_square.get_raw() as usize]);
        let castle_rights_difference = self.castle_rights().get_raw() ^ castle_rights;
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
    
        match MOVE_FLAG {
            MoveFlag::DOUBLE_PUSH => {
                *self.state.get_en_passant_mut() = to_square ^ 8;
                self.state
                    .get_key_mut()
                    .update_en_passant_hash(to_square ^ 8);
            }
            MoveFlag::KING_SIDE_CASTLE | MoveFlag::QUEEN_SIDE_CASTLE => {
                let king_side = usize::from(MOVE_FLAG == MoveFlag::KING_SIDE_CASTLE);
                let side_flip = 56 * usize::from(STM_WHITE) as u8;
                let rook_from_square = side_flip + CastleRight::ROOK_POSITIONS[king_side];
                let rook_to_square = side_flip + [3, 5][king_side];
                self.remove_piece_on_square::<STM_WHITE>(
                    Square::from_raw(rook_from_square),
                    Piece::ROOK,
                );
                self.set_piece_on_square::<STM_WHITE>(
                    Square::from_raw(rook_to_square),
                    Piece::ROOK,
                );
            }
            MoveFlag::EN_PASSANT => self.remove_piece_on_square::<NSTM_WHITE>(
                to_square ^ 8,
                Piece::PAWN,
            ),
            MoveFlag::KNIGHT_PROMOTION.. => {
                let promotion_piece = mv.get_promotion_piece();
                self.remove_piece_on_square::<STM_WHITE>(to_square, Piece::PAWN);
                self.set_piece_on_square::<STM_WHITE>(to_square, promotion_piece);
            }
            _ => {}
        }
    
        self.state.get_side_to_move_mut().mut_flip();
        self.state.get_key_mut().update_side_to_move_hash();

        move_history.push(self.get_key())
    }
}

const PAWN: u8 = 0;
const KNIGHT: u8 = 1;
const BISHOP: u8 = 2;
const ROOK: u8 = 3;
const QUEEN: u8 = 4;
const KING: u8 = 5;
const NONE: u8 = 6;