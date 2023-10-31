use crate::pipe::resources::PipeSpawnTimer;
use crate::pipe::systems::{pipe_movement, reset_pipes, spawn_pipes, tick_pipe_spawn_timer};
use crate::GameState;
use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PipeSpawnTimer>()
            .add_systems(
                Update,
                (tick_pipe_spawn_timer, spawn_pipes, pipe_movement)
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(Update, reset_pipes.run_if(in_state(GameState::Reset)));
    }
}
