use std::sync::Arc;

use chess::{self, board::Board, game::Game};

fn main() {
    let game = Game::original().unwrap();

    game.board().print_state();
}
