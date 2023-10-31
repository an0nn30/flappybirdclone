use crate::game::systems::{check_collision, global_input, restart_game};

use bevy::prelude::*;

pub mod components;
mod systems;

pub struct GamePlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UISet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GameSet;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (global_input, check_collision, restart_game));
    }
}
