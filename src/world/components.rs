use bevy::prelude::*;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub enum GroundType {
    Left,
    Right,
}
