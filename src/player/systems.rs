use crate::player::components::Player;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodyForces;

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
        SpriteBundle {
            texture: asset_server.load("textures/yellowbird-midflap.png"),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        Player,
        RigidBody::Dynamic,
        Collider::cuboid(collider_size.x / 2.0, collider_size.y / 2.0),
        Velocity::zero(),
        GravityScale(9.8),
    )); // Normal gravity scale, adjust if needed
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    // Define the flap strength
    let flap_strength = Vec2::new(0.0, 350.0); // Adjust as needed

    let rotation_speed = 1.0; // Adjust as needed
    let max_rotation = 90.0f32.to_radians(); // 90 degrees in radians
    let min_rotation = -30.0f32.to_radians(); // -30 degrees in radians

    if let Ok((mut velocity, mut transform)) = player_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            // Flap on spacebar press
            // Apply the flap force
            velocity.linvel = flap_strength;
            println!("Velocity: {:?}", velocity);
        }

        // Determine rotation direction based on vertical velocity
        let rotation_change = rotation_speed * time.delta_seconds().to_radians();
        if velocity.linvel.y > 0.0 && transform.rotation.to_euler(EulerRot::XYZ).2 < max_rotation {
            transform.rotate(Quat::from_rotation_z(rotation_change));
        } else if velocity.linvel.y < 0.0
            && transform.rotation.to_euler(EulerRot::XYZ).2 > min_rotation
        {
            transform.rotate(Quat::from_rotation_z(-rotation_change));
        }
    } else {
        eprintln!("unable to load player");
    }
}
