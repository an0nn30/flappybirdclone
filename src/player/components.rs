use bevy::prelude::*;

pub const GRAVITY: f32 = 15.;
pub const FLAP_STRENGTH: Vec2 = Vec2::new(0.0, 450.0);

#[derive(Component)]
pub struct Player;
