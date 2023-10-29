use crate::player::components::Player;
use crate::GameState;
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodyForces;
use std::default::Default;

// TODO: Figure out how to get the correct window size
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // Set the size of the collider
    let collider_size = Vec2::new(16.0, 16.0); // Adjust the size as needed

    commands.spawn((
        setup_sprite(&asset_server, &window),
        Player,
        RigidBody::Dynamic,
        Collider::cuboid(collider_size.x / 2.0, collider_size.y / 2.0),
        Velocity::zero(),
        GravityScale(9.8),
    )); // Normal gravity scale, adjust if needed
}

pub fn player_movement(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    touch_input: Res<Events<TouchInput>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Velocity, &mut Transform, &mut GravityScale), With<Player>>,
    time: Res<Time>,
) {
    // Define the flap strength
    let flap_strength = Vec2::new(0.0, 350.0); // Adjust as needed

    let rotation_speed = 90.5; // Adjust as needed
    let max_rotation = 30.0f32.to_radians(); // 90 degrees in radians
    let min_rotation = -30.0f32.to_radians(); // -30 degrees in radians

    if let Ok((mut velocity, mut transform, mut gravity)) = player_query.get_single_mut() {
        if game_state.get().ne(&(GameState::Running).into()) {
            velocity.linvel = Vec2::new(0., 0.);
            gravity.0 = 0.0;
            return;
        } else {
            gravity.0 = 9.8;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            // Flap on spacebar press
            // Apply the flap force
            velocity.linvel = flap_strength;
            debug!("Velocity: {:?}", velocity);
            transform.rotation = Quat::from_rotation_z(max_rotation);
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
            if let Ok((mut velocity, mut transform, mut gravity)) = player_query.get_single_mut() {
                debug!("Velocity: {:?}", velocity);
                velocity.linvel = flap_strength;
                transform.rotation = Quat::from_rotation_z(max_rotation);
            }
        }
    }
}

fn setup_sprite(asset_server: &AssetServer, window: &Window) -> SpriteBundle {
    // #[cfg(target_os = "ios")]
    // let sprite = Sprite {
    //     custom_size: Some(Vec2::new(40., 30.)),
    //     ..default()
    // };
    //
    // #[cfg(not(target_os = "ios"))]
    let sprite = Sprite::default();

    SpriteBundle {
        texture: asset_server.load("textures/yellowbird-midflap.png"),
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        sprite,
        ..default()
    }
}
