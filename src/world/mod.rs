use crate::world::systems::{move_ground, spawn_background, spawn_bricks};
use crate::GameState;
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_background, spawn_bricks))
            .add_systems(Update, move_ground.run_if(in_state(GameState::Running)));
    }
}
