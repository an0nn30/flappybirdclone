use crate::world::components::{Ground, GroundType};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let texture_handle = asset_server.load("textures/background-day.png");

    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., -10.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(window.width(), window.height())), // Set sprite size to window dimensions
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn spawn_bricks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let texture_width = window.width();
    let texture_handle = asset_server.load("textures/base.png");

    // Spawn the first sprite
    commands.spawn((
        SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform::from_xyz(-texture_width / 2., 50., -9.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(window.width(), window.height() / 3.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ground,
        GroundType::Left,
    ));

    // Spawn the second sprite, immediately to the right of the first
    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_xyz(texture_width / 2., 50., -9.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(window.width(), window.height() / 3.)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ground,
        GroundType::Right,
    ));
}

pub fn move_ground(
    time: Res<Time>,
    mut ground_query: Query<(&mut Transform, &GroundType)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let ground_width = window.width();
    let speed = 150.0; // Ground movement speed

    let mut left_ground_transform = None;
    let mut right_ground_transform = None;

    for (transform, ground_type) in ground_query.iter_mut() {
        match ground_type {
            GroundType::Left => left_ground_transform = Some(transform),
            GroundType::Right => right_ground_transform = Some(transform),
        }
    }

    if let Some(mut left_transform) = left_ground_transform {
        // Move the left ground
        left_transform.translation.x -= speed * time.delta_seconds();
        // Check and reposition if needed
        if left_transform.translation.x <= -ground_width / 2.0 {
            left_transform.translation.x += ground_width * 2.0;
        }
    }

    if let Some(mut right_transform) = right_ground_transform {
        // Move the right ground
        right_transform.translation.x -= speed * time.delta_seconds();
        // Check and reposition if needed
        if right_transform.translation.x <= -ground_width / 2.0 {
            right_transform.translation.x += ground_width * 2.0;
        }
    }
}
