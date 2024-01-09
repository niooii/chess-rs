use chess::game::Game;

#[test]
pub fn calc_default() {
    let game = Game::original().unwrap();

    assert!(game.calculate_moves_for("White".to_string()).unwrap().len() + game.calculate_moves_for("Black".to_string()).unwrap().len() == 20);
}