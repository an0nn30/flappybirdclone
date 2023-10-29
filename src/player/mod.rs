use crate::player::systems::{player_movement, spawn_player};
use crate::GameState;
use bevy::prelude::*;
use std::sync::OnceState;

pub mod components;
mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}
