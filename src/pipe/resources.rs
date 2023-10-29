use bevy::prelude::*;

pub const PIPE_SPAWN_TIME: f32 = 2.0;

#[derive(Resource)]
pub struct PipeSpawnTimer {
    pub timer: Timer,
}

impl Default for PipeSpawnTimer {
    fn default() -> PipeSpawnTimer {
        PipeSpawnTimer {
            timer: Timer::from_seconds(PIPE_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
