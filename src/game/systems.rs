use crate::GameState;
use bevy::prelude::*;

pub fn global_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<GameState>>,
    current_app_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if current_app_state.get().eq(&(GameState::Running).into()) {
            next_app_state.set(GameState::GameOver);
        } else {
            next_app_state.set(GameState::Running);
        }
    }
}
