use crate::{
    base_structures::{Side, ZobristKey},
    CastleRight, ChessBoard, Piece, Square,
};

#[derive(Clone, Copy, Default)]
pub struct ChessBoardState {
    zobrist: ZobristKey,
    half_moves: u8,
    en_passant: Square,
    side_to_move: Side,
    castle_rights: CastleRight,
    //free 4 bytes
}

impl ChessBoardState {
    #[inline]
    pub(super) fn get_key_mut(&mut self) -> &mut ZobristKey {
        &mut self.zobrist
    }

    #[inline]
    pub(super) fn get_side_to_move_mut(&mut self) -> &mut Side {
        &mut self.side_to_move
    }

    #[inline]
    pub(super) fn get_en_passant_mut(&mut self) -> &mut Square {
        &mut self.en_passant
    }

    #[inline]
    pub(super) fn get_castle_rights_mut(&mut self) -> &mut CastleRight {
        &mut self.castle_rights
    }

    #[inline]
    pub(super) fn get_half_move_counter_mut(&mut self) -> &mut u8 {
        &mut self.half_moves
    }
}

impl ChessBoard {
    #[inline]
    pub fn get_key(&self) -> ZobristKey {
        self.state.zobrist
    }

    #[inline]
    pub fn en_passant_square(&self) -> Square {
        self.state.en_passant
    }

    #[inline]
    pub fn side_to_move(&self) -> Side {
        self.state.side_to_move
    }

    #[inline]
    pub fn castle_rights(&self) -> CastleRight {
        self.state.castle_rights
    }

    #[inline]
    pub fn half_move_counter(&self) -> u8 {
        self.state.half_moves
    }

    #[inline]
    pub fn is_insufficient_material(&self) -> bool {
        let pawns = self.get_piece_mask(Piece::PAWN).is_empty();
        let major_pieces =
            (self.get_piece_mask(Piece::ROOK) | self.get_piece_mask(Piece::QUEEN)).is_empty();
        let white_minor_pieces = (self.get_piece_mask_for_side::<true>(Piece::KNIGHT)
            | self.get_piece_mask_for_side::<true>(Piece::BISHOP))
        .pop_count()
            < 2;
        let black_minor_pieces = (self.get_piece_mask_for_side::<false>(Piece::KNIGHT)
            | self.get_piece_mask_for_side::<false>(Piece::BISHOP))
        .pop_count()
            < 2;
        pawns && major_pieces && white_minor_pieces && black_minor_pieces
    }

    #[inline]
    pub fn three_fold(&self) -> bool {
        let mut appearance_count = 0;
        for mv_key in 0..self.state.half_moves as usize {
            if self.move_history[mv_key] == self.state.zobrist {
                appearance_count += 1
            }
        }
        appearance_count >= 2
    }
}
