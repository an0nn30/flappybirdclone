use crate::pipe::components::{Pipe, PipePair, Scorable};
use crate::pipe::resources::PipeSpawnTimer;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use log::debug;
use rand::Rng;

pub fn spawn_pipes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    pipe_spawn_timer: Res<PipeSpawnTimer>,
) {
    if pipe_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let pipe_pair_entity = build_pipe_pair(&mut commands, &asset_server, &window_query);
        let y = random_range(180.0, window.height() - 100.0);
        debug!("Spawning pipe on y: {}", y);
        commands
            .entity(pipe_pair_entity)
            .insert(Transform::from_xyz(window.width() + 50., y, 0.0))
            .insert(Scorable { scored: false });
    }
}

pub fn pipe_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut pipe_pair_query: Query<(Entity, &mut Transform), With<PipePair>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (entity, mut transform) in pipe_pair_query.iter_mut() {
        let speed = 150.;
        transform.translation.x -= speed * time.delta_seconds();
        if transform.translation.x < -window.width() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn build_pipe_pair(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    _window_query: &Query<&Window, With<PrimaryWindow>>,
) -> Entity {
    let spacing = 300.;

    let pipe_size = Vec2::new(50.0, 300.0);
    let collider_size = Vec2::new(52., 480.);
    // Common components for both ground sprites
    let pipe1_components = (
        Pipe,
        RigidBody::Fixed,
        Collider::cuboid(collider_size.x / 2.0, collider_size.y / 2.0),
        ActiveEvents::COLLISION_EVENTS,
    );

    let pipe2_components = (
        Pipe,
        RigidBody::Fixed,
        Collider::cuboid(collider_size.x / 2.0, collider_size.y / 2.0),
        ActiveEvents::COLLISION_EVENTS,
    );

    // This function will spawn the pipes as a single entity, which will make it easier to manipulate the pair together.
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::default(),
                ..default()
            },
            PipePair { spacing },
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("textures/bottom-pipe.png"),
                    transform: Transform {
                        translation: Vec3::new(0.0, -spacing / 2.0 - pipe_size.y / 2.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                pipe1_components,
            ));
            // Add top pipe
            parent.spawn((
                SpriteBundle {
                    texture: asset_server.load("textures/top-pipe.png"),
                    transform: Transform {
                        translation: Vec3::new(0.0, spacing / 2.0 + pipe_size.y / 2.0, 0.0),
                        // scale: Vec3::new(1.0, -1.0, 1.0), // Inverted scale for the bottom pipe
                        ..Default::default()
                    },
                    ..Default::default()
                },
                pipe2_components,
            ));

            // Add bottom pipe
        })
        .id()
}

pub fn tick_pipe_spawn_timer(mut pipe_spawn_timer: ResMut<PipeSpawnTimer>, time: Res<Time>) {
    pipe_spawn_timer.timer.tick(time.delta());
}

fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng(); // Get a copy of the random number generator
    rng.gen_range(min..max) // Generate a random number in the range
}

pub fn reset_pipes(mut commands: Commands, mut pipe_query: Query<Entity, With<PipePair>>) {
    for entity in pipe_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
