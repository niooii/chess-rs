use std::sync::Arc;

use chess::{board::Board, game::Game};

#[test]
pub fn board_creation() {
    let board = Board::new(8, 8);
    assert!(!Arc::ptr_eq(&board.tile_at(0, 1).unwrap(), &board.tile_at(1, 1).unwrap()));
    // assert!(game.board().pieces().len() == 32);
}

#[test]
pub fn piece_count() {
    let game = Game::original().unwrap();
    assert!(game.board().pieces().len() == 32);
}