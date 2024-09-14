use spear::{Perft, FEN};

fn main() {
    let fens = [
        (FEN::start_position(), 7, 6, 3195901860),
        (FEN::kiwipete_position(), 6, 5, 8031647685),
        (
            FEN::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"),
            8,
            7,
            3009794393,
        ),
        (
            FEN::from_str("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"),
            6,
            5,
            706045033,
        ),
        (
            FEN::from_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
            6,
            5,
            3048196529,
        ),
        (
            FEN::from_str(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            ),
            6,
            5,
            6923051137,
        ),
    ];

    println!("Bulk:");
    for _ in 0..2 {
        for (index, fen) in fens.clone().into_iter().enumerate() {
            let (result_nodes, result_duration) = Perft::perft::<true, false, false>(&fen.0, fen.1);
            if result_nodes != fen.3 {
                println!(
                    "Position {index} has failed in {result_duration}ms ({}nps)",
                    result_nodes * 1000 / result_duration
                );
            } else {
                println!(
                    "Position {index} has passed in {result_duration}ms ({}nps)",
                    result_nodes * 1000 / result_duration
                );
            }
        }
    }

    println!("\nNon-bulk:");
    for fen in &fens {
        let mut nodes = 0u128;
        let mut duration = 0u128;

        for _ in 0..4 {
            let (result_nodes, result_duration) =
                Perft::perft::<false, false, false>(&fen.0, fen.2);
            nodes += result_nodes;
            duration += result_duration;
        }

        println!("{}nps", nodes * 1000 / duration);
    }
}
