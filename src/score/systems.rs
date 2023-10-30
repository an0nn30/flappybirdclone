use crate::score::components::Sound;
use bevy::prelude::*;
use bevy::ui::AlignItems::Default;

use super::resources::*;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}
