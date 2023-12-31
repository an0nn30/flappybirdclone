use crate::pipe::components::{PipePair, Scorable};
use crate::player::components::{BirdFlap, Player, FLAP_STRENGTH, GRAVITY};
use crate::player::resources::{BirdTextures, PlayerFlightState};
use crate::score::resources::Score;
use crate::sounds::{play_sound, Sounds};
use crate::ui::components::ScoreText;
use crate::GameState;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use std::default::Default;

// TODO: Figure out how to get the correct window size
pub fn spawn_player(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    bird_textures: Res<BirdTextures>,
) {
    let window = window_query.get_single().unwrap();

    // Set the size of the collider
    let collider_size = Vec2::new(16.0, 16.0); // Adjust the size as needed

    commands.spawn((
        setup_sprite(&window, &bird_textures),
        Player,
        RigidBody::Dynamic,
        Collider::cuboid(collider_size.x / 2.0, collider_size.y / 2.0),
        Velocity::zero(),
        GravityScale(GRAVITY),
        BirdFlap::default(),
    )); // Normal gravity scale, adjust if needed
}

pub fn player_movement(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    touch_input: Res<Events<TouchInput>>,
    game_state: Res<State<GameState>>,
    mut player_query: Query<(&mut Velocity, &mut Transform, &mut GravityScale), With<Player>>,
    time: Res<Time>,
    mut player_state: ResMut<NextState<PlayerFlightState>>,
    bird_textures: Res<BirdTextures>,
    mut query: Query<(&mut BirdFlap, &mut Handle<Image>)>,
) {
    let rotation_speed = 90.5; // Adjust as needed
    let max_rotation = 30.0f32.to_radians(); // 90 degrees in radians
    let min_rotation = -30.0f32.to_radians(); // -30 degrees in radians

    if let Ok((mut velocity, mut transform, mut gravity)) = player_query.get_single_mut() {
        if game_state.get().ne(&(GameState::Running).into()) {
            velocity.linvel = Vec2::new(0., 0.);
            gravity.0 = 0.0;
            return;
        } else {
            gravity.0 = GRAVITY;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            // Flap on spacebar press
            // Apply the flap force
            velocity.linvel = FLAP_STRENGTH;
            player_state.set(PlayerFlightState::Flying);
            debug!("Velocity: {:?}", velocity);
            transform.rotation = Quat::from_rotation_z(max_rotation);
            play_sound(&mut commands, &asset_server, Sounds::FLAP);
        }

        if velocity.linvel.y < 0. {
            player_state.set(PlayerFlightState::Falling);
        } else {
            if let Ok((mut _flap, mut texture)) = query.get_single_mut() {
                debug!("Changing to flying");
                *texture = bird_textures.textures[0].clone();
            }
        }

        // Determine rotation direction based on vertical velocity
        let rotation_change = rotation_speed * time.delta_seconds().to_radians();

        if velocity.linvel.y < 0.0 && transform.rotation.to_euler(EulerRot::XYZ).2 > min_rotation {
            transform.rotate(Quat::from_rotation_z(-rotation_change));
        }
    } else {
        error!("unable to load player");
    }

    for event in touch_input.get_reader().iter(&touch_input) {
        if event.phase == TouchPhase::Started {
            // If there's a touch, consider it as a 'flap' action
            if let Ok((mut velocity, mut transform, _gravity)) = player_query.get_single_mut() {
                play_sound(&mut commands, &asset_server, Sounds::FLAP);
                debug!("Velocity: {:?}", velocity);
                velocity.linvel = FLAP_STRENGTH;
                transform.rotation = Quat::from_rotation_z(max_rotation);
            }
        }
    }
}

fn setup_sprite(window: &Window, texture: &BirdTextures) -> SpriteBundle {
    // #[cfg(target_os = "ios")]
    // let sprite = Sprite {
    //     custom_size: Some(Vec2::new(40., 30.)),
    //     ..default()
    // };
    //
    // #[cfg(not(target_os = "ios"))]
    let sprite = Sprite::default();

    SpriteBundle {
        texture: texture.textures[0].clone(),
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        sprite,
        ..default()
    }
}

pub fn check_if_scored(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bird_query: Query<&Transform, With<Player>>,
    mut pipe_query: Query<(&Transform, &mut Scorable), With<PipePair>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(bird_transform) = bird_query.get_single() {
        let bird_x = bird_transform.translation.x;

        for (pipe_transform, mut scorable) in pipe_query.iter_mut() {
            if scorable.scored {
                continue;
            }
            let pipe_x = pipe_transform.translation.x;

            // Define a tolerance for when the score should be incremented
            let tolerance = 10.0; // Adjust this value as needed

            if (bird_x - pipe_x).abs() < tolerance {
                // Increment score
                score.value += 1;
                scorable.scored = true;
                update_score(&mut commands, &asset_server, score, window);
                play_sound(&mut commands, &asset_server, Sounds::POINT);

                break; // Prevents multiple increments for the same pipe pair
            }
        }
    }
}

fn update_score(
    commands: &mut Commands,
    asset_server: &AssetServer,
    mut score: ResMut<Score>,
    window: &Window,
) {
    // Check if there is an existing score display entity and despawn it along with its children
    if let Some(entity) = score.display_entity {
        commands.entity(entity).despawn_recursive();
        score.display_entity = None;
    }

    // Logic for spawning the new score display based on `score.value`...
    let score_string = score.value.to_string();
    let number_width = 20.0; // Adjust this value as needed for your textures

    // Create a new parent entity for the score display
    let parent_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    window.width() / 2.,
                    window.height() / 2. + 200.0,
                    1.,
                ),
                ..default()
            },
            ScoreText,
        ))
        .id();
    score.display_entity = Some(parent_entity);

    // Spawn children entities for each digit in the score
    for (i, digit) in score_string.chars().enumerate() {
        let texture_path = format!("textures/{}.png", digit);
        let texture_handle = asset_server.load(&texture_path);

        commands.entity(parent_entity).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: texture_handle,
                transform: Transform::from_xyz(i as f32 * number_width, 0., 1.0),
                ..Default::default()
            });
        });
    }
}

pub fn reset_player(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut bird_query: Query<(&mut Velocity, &mut Transform, &mut GravityScale), With<Player>>,
) {
    let window = window_query.get_single().unwrap();
    let (mut velocity, mut transform, mut gravity) = bird_query.get_single_mut().unwrap();
    velocity.linvel = Vec2::new(0., 0.);
    transform.translation = Vec3::new(window.width() / 2., window.height() / 2., 0.);
    transform.rotation = Quat::from_rotation_z(0.);
    gravity.0 = 0.;
}

// TODO: This does not despawn the pipes if we've scored...for some reason...
pub fn reset_score(
    mut commands: Commands,
    score_query: Query<Entity, With<ScoreText>>,
    mut score: ResMut<Score>,
) {
    if let Ok(score) = score_query.get_single() {
        debug!("Despawning score");
        commands.entity(score).despawn_recursive();
    }
    score.value = 0;
    score.display_entity = None;
}

pub fn bird_flap_animation(
    time: Res<Time>,
    bird_textures: Res<BirdTextures>,
    mut query: Query<(&mut BirdFlap, &mut Handle<Image>)>,
    game_state: Res<State<GameState>>,
) {
    if game_state.get().ne(&(GameState::Running)) {
        return;
    }
    for (mut flap, mut texture) in query.iter_mut() {
        flap.timer.tick(time.delta());
        if flap.timer.just_finished() {
            flap.flap_state = (flap.flap_state + 1) % bird_textures.textures.len();
            *texture = bird_textures.textures[flap.flap_state].clone();
        }
    }
}

pub fn animation_setup(asset_server: Res<AssetServer>, mut bird_textures: ResMut<BirdTextures>) {
    let textures = vec![
        asset_server.load("textures/yellowbird-downflap.png"),
        asset_server.load("textures/yellowbird-midflap.png"),
        asset_server.load("textures/yellowbird-upflap.png"),
    ];

    bird_textures.textures = textures;
}
