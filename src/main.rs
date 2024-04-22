use crate::game_state::GameState;

mod trail;
mod ball;
mod planet;
mod game_state;
mod drag_drop_fling;

fn main() -> tetra::Result {
    GameState::start()
}