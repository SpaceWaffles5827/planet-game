use crate::game_state::GameState;

mod trail;
mod ball;
mod planet;
mod game_state;

fn main() -> tetra::Result {
    GameState::start()
}