use crate::pipe::components::{Pipe, PipePair};
use crate::player::components::Player;
use crate::sounds::{play_sound, Sounds};

use crate::score::resources::Score;
use crate::ui::components::GameOverMessage;
use crate::world::components::Ground;
use crate::GameState;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub fn global_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<GameState>>,
    current_app_state: Res<State<GameState>>,
    touch_input: Res<Events<TouchInput>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if current_app_state.get().eq(&(GameState::Running).into()) {
            next_app_state.set(GameState::Paused);
        } else {
            next_app_state.set(GameState::Running);
        }
    }

    for event in touch_input.get_reader().iter(&touch_input) {
        if event.phase == TouchPhase::Started {
            next_app_state.set(GameState::Running);
        }
    }
}

pub fn check_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
    bird_query: Query<Entity, With<Player>>,
    ground_query: Query<Entity, With<Ground>>,
    pipe_query: Query<Entity, With<PipePair>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let bird_entity = bird_query.single();

    // Collect the ground and pipe entities into vectors for easy searching
    let _ground_entities: Vec<Entity> = ground_query.iter().collect();
    let _pipe_entities: Vec<Entity> = pipe_query.iter().collect();

    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            // Check if the bird is involved in the collision
            if bird_entity == *entity1 || bird_entity == *entity2 {
                debug!("Collision happened!");
                play_sound(&mut commands, &asset_server, Sounds::HIT);
                game_state.set(GameState::GameOver);
                break;
            }
        }
    }
}

pub fn restart_game(input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::R) {
        game_state.set(GameState::Reset);
    }
}
