use bevy::prelude::*;

pub const GRAVITY: f32 = 15.;
pub const FLAP_STRENGTH: Vec2 = Vec2::new(0.0, 450.0);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct BirdFlap {
    pub(crate) flap_state: usize,
    pub(crate) timer: Timer,
}

impl Default for BirdFlap {
    fn default() -> Self {
        BirdFlap {
            flap_state: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating), // Adjust time for flap speed
        }
    }
}
