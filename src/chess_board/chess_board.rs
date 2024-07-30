use colored::Colorize;

use crate::{base_structures::Side, CastleRights, Piece, Square, FEN};

use super::{chess_board_pieces::ChessBoardPieces, chess_board_state::ChessBoardState};

#[derive(Clone, Copy, Default)]
pub struct ChessBoard {
    pub(super) pieces: ChessBoardPieces,
    //pub(super) masks: ChessBoardMasks,
    pub(super) state: ChessBoardState,
}

impl ChessBoard {
    pub fn from_fen(fen: &FEN) -> Self {
        let mut board = Self {
            pieces: ChessBoardPieces::default(),
            //masks: ChessBoardMasks::default(),
            state: ChessBoardState::default(),
        };

        for (rank_index, rank) in fen.board.clone().into_iter().enumerate() {
            let mut index = 0;
            let mut file = 0;
            while file < 8 {
                let square = Square::from_coords((7 - rank_index) as u8, file);
                let piece_char = rank.as_bytes()[index] as char;
                if piece_char.is_numeric() {
                    file += piece_char.to_string().parse::<usize>().unwrap() as u8;
                    index += 1;
                    continue;
                }

                if piece_char == 'P' {
                    board.set_piece_on_square::<true>(square, Piece::PAWN)
                } else if piece_char == 'N' {
                    board.set_piece_on_square::<true>(square, Piece::KNIGHT)
                } else if piece_char == 'B' {
                    board.set_piece_on_square::<true>(square, Piece::BISHOP)
                } else if piece_char == 'R' {
                    board.set_piece_on_square::<true>(square, Piece::ROOK)
                } else if piece_char == 'Q' {
                    board.set_piece_on_square::<true>(square, Piece::QUEEN)
                } else if piece_char == 'K' {
                    board.set_piece_on_square::<true>(square, Piece::KING)
                } else if piece_char == 'p' {
                    board.set_piece_on_square::<false>(square, Piece::PAWN)
                } else if piece_char == 'n' {
                    board.set_piece_on_square::<false>(square, Piece::KNIGHT)
                } else if piece_char == 'b' {
                    board.set_piece_on_square::<false>(square, Piece::BISHOP)
                } else if piece_char == 'r' {
                    board.set_piece_on_square::<false>(square, Piece::ROOK)
                } else if piece_char == 'q' {
                    board.set_piece_on_square::<false>(square, Piece::QUEEN)
                } else if piece_char == 'k' {
                    board.set_piece_on_square::<false>(square, Piece::KING)
                }

                index += 1;
                file += 1;
            }
        }

        if fen.side_to_move == "w" {
            *board.state.get_side_to_move_mut() = Side::WHITE;
        } else {
            *board.state.get_side_to_move_mut() = Side::BLACK;
        }

        let king_square = if board.side_to_move() == Side::WHITE {
            board.get_king_square::<false>()
        } else {
            board.get_king_square::<true>()
        };

        if if board.side_to_move() == Side::WHITE {
            board.is_square_attacked::<false, true>(king_square)
        } else {
            board.is_square_attacked::<true, false>(king_square)
        } {
            print!("Illegal position!\n");
            return Self {
                pieces: ChessBoardPieces::default(),
                //masks: ChessBoardMasks::default(),
                state: ChessBoardState::default(),
            };
        }

        if fen.castle_rights.contains('K') {
            board
                .state
                .get_castle_rights_mut()
                .set_right(CastleRights::WHITE_KING);
        }
        if fen.castle_rights.contains('Q') {
            board
                .state
                .get_castle_rights_mut()
                .set_right(CastleRights::WHITE_QUEEN);
        }
        if fen.castle_rights.contains('k') {
            board
                .state
                .get_castle_rights_mut()
                .set_right(CastleRights::BLACK_KING);
        }
        if fen.castle_rights.contains('q') {
            board
                .state
                .get_castle_rights_mut()
                .set_right(CastleRights::BLACK_QUEEN);
        }

        *board.state.get_en_passant_mut() = Square::NULL;
        if fen.en_passant_square != "-" {
            let new_square = Square::from_string(&fen.en_passant_square);
            *board.state.get_en_passant_mut() = new_square;
        }

        *board.state.get_half_move_counter_mut() = fen.half_move_counter.parse().unwrap();

        board
    }

    //from datapack

    pub fn get_fen(&self) -> FEN {
        let mut fen = String::new();

        // Piece placement
        for rank in (0..8).rev() {
            let mut empty_count = 0;
            for file in 0..8 {
                let square = Square::from_coords(rank, file);
                let piece = self.get_piece_on_square(square);
                let side = self.get_piece_color_on_square(square);
                if piece != Piece::NONE {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    let piece_char = piece.to_char();
                    if side == Side::WHITE {
                        fen.push(piece_char.to_uppercase().next().unwrap());
                    } else {
                        fen.push(piece_char);
                    }
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        // Side to move
        fen.push(' ');
        if self.side_to_move() == Side::WHITE {
            fen.push('w');
        } else {
            fen.push('b');
        }

        // Castling rights
        fen.push(' ');
        let mut castling_rights = String::new();
        if self.castle_rights().has_right(CastleRights::WHITE_KING) {
            castling_rights.push('K');
        }
        if self.castle_rights().has_right(CastleRights::WHITE_QUEEN) {
            castling_rights.push('Q');
        }
        if self.castle_rights().has_right(CastleRights::BLACK_KING) {
            castling_rights.push('k');
        }
        if self.castle_rights().has_right(CastleRights::BLACK_QUEEN) {
            castling_rights.push('q');
        }
        if castling_rights.is_empty() {
            castling_rights.push('-');
        }
        fen.push_str(&castling_rights);

        // En passant target square
        fen.push(' ');
        if self.en_passant_square() == Square::NULL {
            fen.push('-');
        } else {
            fen.push_str(&self.en_passant_square().to_string());
        }

        // Halfmove clock and fullmove number
        fen.push(' ');
        fen.push_str(&self.half_move_counter().to_string());
        fen.push(' ');
        fen.push_str(&(1).to_string());

        FEN::from_string(fen)
    }

    pub fn draw_board(&self) {
        let piece_icons: [[&str; 6]; 2] = [
            [" P ", " N ", " B ", " R ", " Q ", " K "],
            [" p ", " n ", " b ", " r ", " q ", " k "],
        ];

        let mut info = Vec::new();
        let fen = format!("FEN: {}", self.get_fen());
        info.push(fen.as_str());
        let zobrist = format!("Zobrist Key: {}", self.get_key());
        info.push(zobrist.as_str());

        let castle_rights = format!("Castle Rights: {}", self.castle_rights());
        info.push(castle_rights.as_str());
        let side_sign = format!("Side To Move: {}", self.side_to_move());
        info.push(side_sign.as_str());
        let en_passant = format!("En Passant: {}", self.en_passant_square());
        info.push(en_passant.as_str());
        let half_moves = format!("Half Moves: {}", self.half_move_counter());
        info.push(half_moves.as_str());
        let in_check = format!(
            "In Check: {}",
            if self.side_to_move() == Side::WHITE {
                self.is_in_check::<true, false>()
            } else {
                self.is_in_check::<false, true>()
            }
        );
        info.push(in_check.as_str());
        let xxx = format!("");
        info.push(xxx.as_str());

        let mut result = " ------------------------\n".to_string();
        for rank in (0..8).rev() {
            result += "|".to_string().as_str();
            for file in 0..8 {
                let square = Square::from_coords(rank, file);
                if square == self.en_passant_square() {
                    result += " x ";
                    continue;
                }

                let piece_type = self.get_piece_on_square(square);
                let piece_side = self.get_piece_color_on_square(square);
                if piece_type == Piece::NONE {
                    result += " . ";
                } else if piece_side == Side::BLACK {
                    result += piece_icons[usize::from(Side::BLACK)][usize::from(piece_type)]
                        .blue()
                        .to_string()
                        .as_str();
                } else {
                    result += piece_icons[usize::from(Side::WHITE)][usize::from(piece_type)]
                        .yellow()
                        .to_string()
                        .as_str();
                }
            }
            result += format!("| {}", info[(7 - rank) as usize]).as_str();
            result += "\n".to_string().as_str();
        }
        result += " ------------------------\n".to_string().as_str();
        print!("{}\n", result);
    }
}
