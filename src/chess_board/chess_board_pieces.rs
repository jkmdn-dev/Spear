use crate::{base_structures::Side, Bitboard, ChessBoard, Piece, Square};

#[derive(Clone, Copy, Default)]
pub struct ChessBoardPieces {
    occupancy: [Bitboard; 2],
    pieces: [Bitboard; 6],
}
impl ChessBoardPieces {
    #[inline]
    pub fn get_occupancy_for_side(&self, side: Side) -> Bitboard {
        self.occupancy[side.get_value() as usize]
    }

    #[inline]
    pub fn get_piece_mask(&self, piece: Piece) -> Bitboard {
        self.pieces[piece.get_value() as usize]
    }

    pub fn get_piece_on_square(&self, square: Square) -> Piece {
        for piece_index in Piece::PAWN.get_value()..=Piece::KING.get_value() {
            if self.pieces[usize::from(piece_index)].get_bit(square) {
                return Piece::from_raw(piece_index);
            }
        }

        return Piece::NONE;
    }

    #[inline]
    pub fn get_piece_color_on_square(&self, square: Square) -> Side {
        if self.get_occupancy_for_side(Side::WHITE).get_bit(square) {
            Side::WHITE
        } else {
            Side::BLACK
        }
    }

    #[inline]
    pub fn set_piece_on_square(&mut self, square: Square, side: Side, piece: Piece) {
        self.pieces[piece.get_value() as usize].set_bit(square);
        self.occupancy[side.get_value() as usize].set_bit(square);
    }

    #[inline]
    pub fn remove_piece_on_square(&mut self, square: Square, side: Side, piece: Piece) {
        self.pieces[piece.get_value() as usize].pop_bit(square);
        self.occupancy[side.get_value() as usize].pop_bit(square);
    }
}

impl ChessBoard {
    #[inline]
    pub fn get_occupancy(&self) -> Bitboard {
        self.get_occupancy_for_side(Side::WHITE) | self.get_occupancy_for_side(Side::BLACK)
    }

    #[inline]
    pub fn get_occupancy_for_side(&self, side: Side) -> Bitboard {
        self.pieces.get_occupancy_for_side(side)
    }

    #[inline]
    pub fn get_piece_mask_for_side(&self, piece: Piece, side: Side) -> Bitboard {
        self.get_piece_mask(piece) & self.get_occupancy_for_side(side)
    }

    #[inline]
    pub fn get_piece_mask(&self, piece: Piece) -> Bitboard {
        self.pieces.get_piece_mask(piece)
    }

    #[inline]
    pub fn get_king_square(&self, side: Side) -> Square {
        self.get_piece_mask_for_side(Piece::KING, side)
            .ls1b_square()
    }

    #[inline]
    pub fn get_piece_on_square(&self, square: Square) -> Piece {
        self.pieces.get_piece_on_square(square)
    }

    #[inline]
    pub fn get_piece_color_on_square(&self, square: Square) -> Side {
        self.pieces.get_piece_color_on_square(square)
    }

    #[inline]
    pub fn set_piece_on_square(&mut self, square: Square, side: Side, piece: Piece) {
        self.pieces.set_piece_on_square(square, side, piece);
        self.state
            .get_key_mut()
            .update_piece_hash(piece, side, square)
    }

    #[inline]
    pub fn remove_piece_on_square(&mut self, square: Square, side: Side, piece: Piece) {
        self.pieces.remove_piece_on_square(square, side, piece);
        self.state
            .get_key_mut()
            .update_piece_hash(piece, side, square)
    }
}
