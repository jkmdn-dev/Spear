use std::time::Instant;

use crate::{base_structures::Side, ChessBoard, MoveHistory, FEN};

pub struct Perft;
impl Perft {
    pub fn perft<const BULK: bool, const SPLIT: bool, const PRINT: bool>(
        fen: &FEN,
        depth: u8,
    ) -> (u128, u128) {
        let board = ChessBoard::from_fen(fen, &mut MoveHistory::new());

        if PRINT {
            board.draw_board();
            println!("-----------------------------------------------------------");
            println!("  Starting PERFT");
            println!("  Depth: {depth}");
            println!("  Split: {SPLIT}");
            println!("  Bulk: {BULK}");
            println!("-----------------------------------------------------------");
        }

        let timer = Instant::now();
        let result = if board.side_to_move() == Side::WHITE { perft_internal::<BULK, SPLIT, PRINT, true, false>(&board, depth, true) } else { perft_internal::<BULK, SPLIT, PRINT, false, true>(&board, depth, true) };
        let duration = timer.elapsed().as_millis();

        if PRINT {
            println!("-----------------------------------------------------------");
            println!(
                "  Perft ended! {} nodes, {:.2}s, {:.2} Mnps",
                result,
                duration as f64 / 1000.0,
                ((result * 1000) as f64 / duration as f64) / 1_000_000f64
            );
            println!("-----------------------------------------------------------");
        }

        (result, duration)
    }
}

fn perft_internal<const BULK: bool, const SPLIT: bool, const PRINT: bool, const STM_WHITE: bool, const NSTM_WHITE: bool>(
    board: &ChessBoard,
    depth: u8,
    first: bool,
) -> u128 {
    let mut node_count = 0u128;

    if BULK && depth == 1 {
        board.map_moves(|_| {
            node_count += 1;
        });
        return node_count;
    }

    if !BULK && depth == 0 {
        return 1;
    }

    board.map_moves(|mv| {
        let mut board_copy = *board;
        board_copy.make_move::<STM_WHITE, NSTM_WHITE>(mv, &mut MoveHistory::new());
        let result = perft_internal::<BULK, SPLIT, PRINT, NSTM_WHITE, STM_WHITE>(&board_copy, depth - 1, false);
        node_count += result;

        if SPLIT && PRINT && first {
            println!("{mv} - {result}")
        }
    });

    node_count
}
