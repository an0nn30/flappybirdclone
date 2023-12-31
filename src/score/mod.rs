use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;


pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
    }
}
