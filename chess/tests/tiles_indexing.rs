use chess::board::Board;

#[test]
pub fn index_tiles() {
    let board1 = Board::new(20, 25);

    assert!(board1.tile_at(20, 25).is_none());
    assert!(board1.tile_at(19, 24).is_some());
    assert!(board1.tile_at(0, 0).is_some());
    assert!(board1.tile_at(board1.width()-1, board1.height()-1).is_some());

    let default = Board::new(8, 8);

    assert!(default.tile_at(0, 0).is_some());
    assert!(default.tile_at(7, 7).is_some());
    assert!(default.tile_at(8, 7).is_none());
}