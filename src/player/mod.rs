use crate::player::systems::{check_if_scored, player_movement, spawn_player};

use bevy::prelude::*;


pub mod components;
mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, check_if_scored);
    }
}
