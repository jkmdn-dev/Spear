use spear::{ChessBoard, Move, MoveFlag, Square, FEN};

fn main() {
    let mut board = ChessBoard::from_fen(&FEN::kiwipete_position());
    board.draw_board();
}