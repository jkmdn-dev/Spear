use spear::{ChessBoard, Move, MoveFlag, Square, FEN};

fn main() {
    let mut board = ChessBoard::from_fen(&FEN::kiwipete_position());
    board.draw_board();

    let mv = Move::from_squares( Square::from_string("g2"), Square::from_string("g4"), MoveFlag::DOUBLE_PUSH );
    board.make_move(mv);
    board.draw_board();

    let mv = Move::from_squares( Square::from_string("h8"), Square::from_string("g8"), MoveFlag::QUIET_MOVE );
    board.make_move(mv);
    board.draw_board();

    let mv = Move::from_squares( Square::from_string("e1"), Square::from_string("g1"), MoveFlag::KING_SIDE_CASTLE );
    board.make_move(mv);
    board.draw_board();
}