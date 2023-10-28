use bevy::prelude::*;
use crate::world::systems::spawn_world;


mod systems;
mod components;

pub struct World;

impl Plugin for World {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}