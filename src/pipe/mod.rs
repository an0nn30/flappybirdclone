use crate::pipe::resources::PipeSpawnTimer;
use crate::pipe::systems::{pipe_movement, spawn_pipes, tick_pipe_spawn_timer};
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PipeSpawnTimer>()
            .add_systems(Update, (tick_pipe_spawn_timer, spawn_pipes, pipe_movement));
    }
}
