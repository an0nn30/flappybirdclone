use bevy::prelude::*;

pub enum Sounds {
    HIT,
    FLAP,
    POINT,
}

pub fn play_sound(mut commands: &mut Commands, asset_server: &AssetServer, sound: Sounds) {
    let sound_file = match sound {
        Sounds::HIT => "audio/hit.ogg".to_string(),
        Sounds::FLAP => "audio/wing.ogg".to_string(),
        Sounds::POINT => "audio/point.ogg".to_string(),
    };

    commands.spawn(AudioBundle {
        source: asset_server.load(sound_file),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}
