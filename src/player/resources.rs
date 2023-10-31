use bevy::prelude::*;

#[derive(Resource)]
pub struct BirdTextures {
    pub textures: Vec<Handle<Image>>,
}

impl Default for BirdTextures {
    fn default() -> Self {
        Self { textures: vec![] }
    }
}
