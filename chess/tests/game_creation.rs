use chess::game::Game;

#[test]
pub fn default_game() {
    let game = Game::original().unwrap();
    game.validate_game().unwrap();
}