use crate::player::systems::{
    animation_setup, bird_flap_animation, check_if_scored, player_movement, reset_player,
    reset_score, spawn_player,
};

use crate::player::resources::{BirdTextures, PlayerFlightState};
use crate::GameState;
use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AnimationSystemSet;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BirdTextures>()
            .add_state::<PlayerFlightState>()
            .add_systems(
                Startup,
                (animation_setup, spawn_player.after(animation_setup)),
            )
            .add_systems(
                Update,
                (
                    player_movement,
                    check_if_scored,
                    bird_flap_animation.run_if(in_state(PlayerFlightState::Falling)),
                    (reset_player, reset_score).run_if(in_state(GameState::Reset)),
                ),
            );
    }
}
