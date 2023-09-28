use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::game::ground::ground::GroundSize;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    pub speed: f32,
    pub run_rate: f32,
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Camera;

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 255.0,
        min_height: 144.0,
    };

    commands.spawn((
                camera,
                Camera,
    ));

    let texture = asset_server.load("player.png");

    commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                texture,
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            Player {
                speed: 300.0,
                run_rate: 2.0,
            },
            Name::new("Player"),
            RigidBody::Dynamic,
            GravityScale(0.0),
            Collider::cuboid(5.6, 8.0),
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in &mut player_info {
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);
        let run = keyboard_input.any_pressed([KeyCode::ShiftLeft]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        if run {
            rb_vels.linvel = move_delta * player.speed * player.run_rate;
        }
        else {
            rb_vels.linvel = move_delta * player.speed;
        }
    }
}

pub fn camera_movement(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    characters: Query<(&mut Transform, &Player)>,
) {
    for (transform, _player) in characters.iter() {
        let mut camera_transform = camera_query.single_mut();
        let current_position = camera_transform.translation;
        let cam_x = transform.translation.x as f32;
        let cam_y = transform.translation.y as f32;
        let target_position = Vec3::new(cam_x, cam_y, 999.0);
        let interpolation_factor = 0.5;
        let new_position = current_position.lerp(target_position, interpolation_factor);
        camera_transform.translation = new_position;
    }
}

pub fn player_transmission(
    mut player: Query<&mut Transform, With<Player>>,
    ground_size: Res<GroundSize>,
) {
    let mut rng = thread_rng();
    
    let size = ground_size.0;
    let start_x: f32;
    let start_y: f32;
    if rng.gen_bool(0.5) {
        start_x = rng.gen_range(0.0..=(size as f32 / 2.0 + 0.4) * 100.0);
        start_y = -(size as f32 / 2.0 + 0.3) * 100.0 ;
    } else {
        start_x = (size as f32 / 2.0 + 0.3) * 100.0 ;
        start_y = rng.gen_range(-(size as f32 / 2.0 + 0.4) * 100.0..=0.0);
    }

    player.single_mut().translation = Vec3::new(start_x, start_y, 1.0);
}