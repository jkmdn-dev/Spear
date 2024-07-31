use std::u16;

use crate::{base_structures::Side, Bitboard, ChessBoard};

pub struct ChessBoardPacked {
    pub(crate) board: [Bitboard; 4],
    side_to_move: u8,
    score: u16,
    result: i8
}

impl ChessBoardPacked {
    pub fn from_board(board: &ChessBoard, score: f32) -> Self {
        Self {
            board: board_to_compressed(board),
            side_to_move: board.side_to_move().get_raw(),
            score: (score * u16::MAX as f32) as u16,
            result: 0
        }
    } 

    pub fn apply_result(&mut self, winner: Side) {
        self.result = if winner == Side::WHITE { 1 } else { -1 }
    }

    pub fn get_white_perspective_score(&self) -> f32 {
        let stm_score = self.score as f32 / u16::MAX as f32;
        if self.side_to_move == Side::WHITE.get_raw() { stm_score } else { 1.0 - stm_score }
    }
}

fn board_to_compressed(board: &ChessBoard) -> [Bitboard; 4] {
    let mut result = [Bitboard::EMPTY; 4];

    board.get_occupancy().map(|square|{
        let piece = board.get_piece_on_square(square);
        let color = board.get_piece_color_on_square(square);
        for bit_index in 0..3usize {
            if (piece.get_raw() & 1 << bit_index) > 0 {
                result[bit_index].set_bit(square);
            }
        }
        if color == Side::BLACK {
            result[3].set_bit(square);
        }
    });

    result
}