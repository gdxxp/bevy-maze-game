use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use rand::prelude::*;

use crate::player::player::Player;
use crate::ground::ground::*;
use crate::resource::*;
use crate::game::GameState;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Exit;

pub fn gen_exit(mut commands: Commands, asset_server: Res<AssetServer>, ground_size: Res<GroundSize>) {
    let texture: Handle<Image> = asset_server.load("ground/exit.png");
    let mut rng = thread_rng();

    let size = ground_size.0;
    let exit_x: f32;
    let exit_y: f32;
    if rng.gen_bool(0.5) {
        exit_x = rng.gen_range(-(size as f32 / 2.0 + 0.4) * 100.0..=0.0);
        exit_y = (size as f32 / 2.0 + 0.3) * 100.0 ;
    } else {
        exit_y = rng.gen_range(0.0..=(size as f32 / 2.0 + 0.4) * 100.0);
        exit_x = -(size as f32 / 2.0 + 0.3) * 100.0 ;
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            texture,
            transform: Transform::from_xyz(exit_x, exit_y, 1.0),
            ..default()
        },
        Name::new("Exit"),
        Exit,
    ));

}

pub fn exit_check(
    exit: Query<(&mut Transform, &Exit, Entity), Without<Player>>,
    mut player: Query<&mut Transform, With<Player>>,  
    mut distance: ResMut<Distance>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let exit_transform = exit.single().0;
    let mut player_transform = player.single_mut();

    if exit_transform.translation.x - 15.0 <= player_transform.translation.x && exit_transform.translation.x + 15.0 >= player_transform.translation.x 
        && exit_transform.translation.y - 15.0 <= player_transform.translation.y && exit_transform.translation.y + 15.0 >= player_transform.translation.y
    {
        println!("next map!");

        player_transform.translation = Vec3::new( -exit_transform.translation.x, -exit_transform.translation.y, 1.0);

        distance.0 -= 10;
        next_state.set(GameState::LoadGame);
    }
}

pub fn cleanup_ground(
    mut commands: Commands,
    mut exit: Query<(Entity, &mut Exit)>,
    mut grounds: Query<(Entity, &mut Ground)>,
    mut walls: Query<(Entity, &mut Wall)>,
) {
    for (ground_entity, mut _ground) in &mut grounds {
        commands.entity(ground_entity).despawn_recursive();
    }

    for (wall_entity, mut _wall) in &mut walls {
        commands.entity(wall_entity).despawn_recursive();
    }

    for (exit_entity, _) in &mut exit {
        commands.entity(exit_entity).despawn_recursive();
    }
}