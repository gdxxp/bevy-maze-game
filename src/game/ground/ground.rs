use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use rand::prelude::*;


use crate::game::GameState;
use crate::config::*;
use crate::resource::Distance;

#[derive(Reflect, Default)]
enum GroundType {
    #[default]
    Jungle,
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Ground {
    ground_type: GroundType,
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Wall;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct GroundSize(pub i16);

pub fn ground_setup(commands: Commands, asset_server: Res<AssetServer>, distance: ResMut<Distance>, mut ground_size: ResMut<GroundSize>, mut next_state: ResMut<NextState<GameState>>,) {
    let mut rng = thread_rng();

    match distance.0 {
        1600..=2000 => {
            let size = rng.gen_range(SMALL_GROUND_MIN / 2..=SMALL_GROUND_MAX / 2) * 2;
            ground_size.0 = size.clone();
            gen_jungle(commands, asset_server, size);
        },
        0..=1599 => {
            let size = rng.gen_range(MEDIUM_GROUND_MIN / 2..=MEDIUM_GROUND_MAX / 2) * 2;
            ground_size.0 = size.clone();
            gen_jungle(commands, asset_server, size);
        },
        _ => return,
    }

    next_state.set(GameState::LoadObstacle);
}

fn gen_jungle(mut commands: Commands, asset_server: Res<AssetServer>, size: i16) {

    for i in -size/2..=size/2 {
        for j in -size/2..=size/2 {
            let texture: Handle<Image> = asset_server.load("ground/jungle.png");

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..default()
                    },
                    texture,
                    transform: Transform::from_xyz(0.0 + i as f32 * 100.0, 0.0 + j as f32 * 100.0, 0.0),
                    ..default()
                },
                Ground {
                    ground_type: GroundType::Jungle,
                },
                Name::new("Ground"),
            ));
        }
    }

    println!("jungle finished!");

    gen_boundary(commands, asset_server, size);
}

fn gen_boundary(mut commands: Commands, _asset_server: Res<AssetServer>, size: i16) {
    // top
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            transform: Transform::from_xyz(0.0, (size as f32 / 2.0 + 0.5) * 100.0, 1.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid((size as f32 / 2.0 + 0.5) * 100.0, 5.0),
        Wall,
    ));
    // bottom
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            transform: Transform::from_xyz(0.0, -(size as f32 / 2.0 + 0.5) * 100.0, 1.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid((size as f32 / 2.0 + 0.5) * 100.0, 5.0),
        Wall,
    ));
    // left
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            transform: Transform::from_xyz(-(size as f32 / 2.0 + 0.5) * 100.0, 0.0, 1.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(5.0, (size as f32 / 2.0 + 0.5) * 100.0),
        Wall,
    ));
    // right
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            transform: Transform::from_xyz((size as f32 / 2.0 + 0.5) * 100.0, 0.0, 1.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(5.0, (size as f32 / 2.0 + 0.5) * 100.0),
        Wall,
    ));
}