use spear::{ChessBoard, Perft, FEN};

fn main() {
    Perft::perft::<false, true, true>(&FEN::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"),6);
    return;

    let board = &ChessBoard::from_fen(&FEN::from_str("r3k2r/p1ppqpb1/1n2pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R3Kb1R w KQkq - 0 1"));
    board.draw_board();

    board.map_moves(|mv| {
        println!("{mv} - {}", mv.get_flag());
        //let mut board_copy = *board;
        //board_copy.make_move(mv, board_copy.get_fen().to_string());
        //board_copy.draw_board();
    })
}
