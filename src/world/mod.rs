use crate::world::systems::{move_ground, spawn_background, spawn_bricks};
use bevy::prelude::*;

mod components;
mod systems;

pub struct World;

impl Plugin for World {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_background, spawn_bricks))
            .add_systems(Update, move_ground);
    }
}
