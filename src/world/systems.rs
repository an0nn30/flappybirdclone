use bevy::prelude::*;
use bevy::window::PrimaryWindow;


pub fn spawn_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    let texture_handle = asset_server.load("textures/background-day.png");

    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., -10.0), // z set to -10 to render behind everything else
        // sprite: Sprite::new(Vec2::new(window.width(), window.height())), // This assumes your background image covers the whole window
        ..Default::default()
    });
}