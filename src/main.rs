use spear::{ChessBoard, FEN};

fn main() {
    let board = ChessBoard::from_fen(&FEN::kiwipete_position());
    board.draw_board();
}
