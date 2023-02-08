use rusty_engine::prelude::*;

struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_label: Vec<String>,
    spawn_timer: Timer,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_label: vec![],
            spawn_timer: Timer::from_seconds(1.0, false),
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // your actual game logic goes;
    game_state.current_score += 1;
    println!("Current score is :{}", game_state.current_score);
}
