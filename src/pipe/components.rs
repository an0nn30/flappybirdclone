use bevy::prelude::*;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct PipePair {
    pub spacing: f32,
}

#[derive(Component)]
pub struct Scorable {
    pub scored: bool,
}

// #[derive(Bundle)]
// pub struct PipePairBundle {
//     pipe_pair: PipePair,
//     #[bundle]
//     sprite_bundle: SpriteBundle,
// }
