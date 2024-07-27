use spear::{Perft, FEN};

fn main() {
    Perft::perft::<true, false, true>(&FEN::start_position(),7);
    Perft::perft::<false, false, true>(&FEN::start_position(),6);
}
