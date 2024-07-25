use crate::{attacks::Attacks, Bitboard, ChessBoard, Move, MoveFlag, Piece, Square};

use super::MoveGen;

impl MoveGen {
    pub fn generate_pawn_moves<F: FnMut(Move), const WHITE: bool, const CAPTURE_ONLY: bool>(board: &ChessBoard, push_map: Bitboard, capture_map: Bitboard, diagonal_pins: Bitboard, ortographic_pins: Bitboard, method: &mut F) {
        let promotion_rank = Bitboard::RANK_7 >> (board.side_to_move().get_raw() * 40) as u32;
        let double_push_rank = Bitboard::RANK_2 << (board.side_to_move().get_raw() * 40) as u32;
        let pawns = board.get_piece_mask_for_side(Piece::PAWN, board.side_to_move());

        let pushable_pawns = pawns & !diagonal_pins;
        let attack_pawns = pawns & !ortographic_pins;

        handle_pawn_captures(board, attack_pawns, capture_map, diagonal_pins, promotion_rank, method);

        if board.en_passant_square() != Square::NULL {
            handle_en_passant(board, attack_pawns, method)
        }

        if CAPTURE_ONLY {
           return;
        }

        handle_pawn_pushes::<F, WHITE>(pushable_pawns, push_map, ortographic_pins, promotion_rank, double_push_rank, method);
    }
}

fn handle_pawn_pushes<F: FnMut(Move), const WHITE: bool>(pushable_pawns: Bitboard, push_map: Bitboard, ortographic_pins: Bitboard, promotion_rank: Bitboard, double_push_rank: Bitboard, method: &mut F) { 
    let pinned_pawns = pushable_pawns & ortographic_pins;
    let not_pinned_pawns = pushable_pawns & !pinned_pawns;

    not_pinned_pawns.map(|pawn_square| {
        let to_square = if WHITE { pawn_square.shift_left(8)  } else { pawn_square.shift_right(8) };
        if push_map.get_bit(to_square) {
            method(Move::from_squares(pawn_square, to_square, MoveFlag::QUIET_MOVE))
        }
    });

    pinned_pawns.map(|pawn_square| {
        let to_square = if WHITE { pawn_square.shift_left(8)  } else { pawn_square.shift_right(8) };
        if (push_map & ortographic_pins).get_bit(to_square) {
            method(Move::from_squares(pawn_square, to_square, MoveFlag::QUIET_MOVE))
        }
    });

    (pushable_pawns & promotion_rank).map(|pawn_square| {
        let to_square = if WHITE { pawn_square.shift_left(8)  } else { pawn_square.shift_right(8) };
        if push_map.get_bit(to_square) {
            method(Move::from_squares(pawn_square, to_square, MoveFlag::KNIGHT_PROMOTION));
            method(Move::from_squares(pawn_square, to_square, MoveFlag::BISHOP_PROMOTION));
            method(Move::from_squares(pawn_square, to_square, MoveFlag::ROOK_PROMOTION));
            method(Move::from_squares(pawn_square, to_square, MoveFlag::QUEEN_PROMOTION));
        }
    });

    (not_pinned_pawns & double_push_rank).map(|pawn_square| {
        let passing_square = if WHITE { pawn_square.shift_left(8)  } else { pawn_square.shift_right(8) };
        let to_square = if WHITE { pawn_square.shift_left(16)  } else { pawn_square.shift_right(16) };
        if push_map.get_bit(passing_square) && push_map.get_bit(to_square) {
            method(Move::from_squares(pawn_square, to_square, MoveFlag::DOUBLE_PUSH))
        }
    });

    (pinned_pawns & double_push_rank).map(|pawn_square| {
        let passing_square = if WHITE { pawn_square.shift_left(8)  } else { pawn_square.shift_right(8) };
        let to_square = if WHITE { pawn_square.shift_left(16)  } else { pawn_square.shift_right(16) };
        if (push_map & ortographic_pins).get_bit(passing_square) && (push_map & ortographic_pins).get_bit(to_square) {
            method(Move::from_squares(pawn_square, to_square, MoveFlag::DOUBLE_PUSH))
        }
    });
}

fn handle_pawn_captures<F: FnMut(Move)>(board: &ChessBoard, attack_pawns: Bitboard, capture_map: Bitboard, diagonal_pins: Bitboard, promotion_rank: Bitboard, method: &mut F) { 
    let pinned_pawns = attack_pawns & diagonal_pins;
    let not_pinned_pawns = attack_pawns & !pinned_pawns;

    (not_pinned_pawns & !promotion_rank).map(|pawn_square| {
        let attacks = Attacks::get_pawn_attacks_for_square(pawn_square, board.side_to_move()) & capture_map;
        attacks.map(|to_square| {
            method(Move::from_squares(pawn_square, to_square, MoveFlag::CAPTURE))
        })
    });

    (pinned_pawns & !promotion_rank).map(|pawn_square| {
        let attacks = Attacks::get_pawn_attacks_for_square(pawn_square, board.side_to_move()) & capture_map & diagonal_pins;
        attacks.map(|to_square| {
            method(Move::from_squares(pawn_square, to_square, MoveFlag::CAPTURE))
        })
    });

    (not_pinned_pawns & promotion_rank).map(|pawn_square| {
        let attacks = Attacks::get_pawn_attacks_for_square(pawn_square, board.side_to_move()) & capture_map;
        attacks.map(|to_square| {
            method(Move::from_squares(pawn_square, to_square, MoveFlag::KNIGHT_PROMOTION_CAPTURE));
            method(Move::from_squares(pawn_square, to_square, MoveFlag::BISHOP_PROMOTION_CAPTURE));
            method(Move::from_squares(pawn_square, to_square, MoveFlag::ROOK_PROMOTION_CAPTURE));
            method(Move::from_squares(pawn_square, to_square, MoveFlag::QUEEN_PROMOTION_CAPTURE));
        })
    });

    (pinned_pawns & promotion_rank).map(|pawn_square| {
        let attacks = Attacks::get_pawn_attacks_for_square(pawn_square, board.side_to_move()) & capture_map & diagonal_pins;
        attacks.map(|to_square| {
            method(Move::from_squares(pawn_square, to_square, MoveFlag::KNIGHT_PROMOTION_CAPTURE));
            method(Move::from_squares(pawn_square, to_square, MoveFlag::BISHOP_PROMOTION_CAPTURE));
            method(Move::from_squares(pawn_square, to_square, MoveFlag::ROOK_PROMOTION_CAPTURE));
            method(Move::from_squares(pawn_square, to_square, MoveFlag::QUEEN_PROMOTION_CAPTURE));
        })
    });
}

fn handle_en_passant<F: FnMut(Move)>(board: &ChessBoard, attack_pawns: Bitboard, method: &mut F) { 
    let pawns = Attacks::get_pawn_attacks_for_square(board.en_passant_square(), board.side_to_move().flipped()) & attack_pawns;

    pawns.map(|pawn_square| {
        let mut board_copy = *board;
        let new_mv = Move::from_squares(pawn_square, board.en_passant_square(), MoveFlag::EN_PASSANT);
        board_copy.make_move(new_mv);

        let king_square = board_copy.get_king_square(board.side_to_move());
        if !board_copy.is_square_attacked(king_square, board.side_to_move().flipped()) {
            method(new_mv);
        }
    });
}