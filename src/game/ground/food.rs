use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_rapier2d::prelude::{Collider, CollisionEvent, ActiveEvents};
use rand::prelude::*;

use crate::game::player::player::Player;

use super::obstacle::Obstacle;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Berry {
    energy: f32,
}

pub fn gen_berry(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    obstacles: Query<(&Transform, &Obstacle)>,
) {
    for (obstacle_transform, _) in obstacles.iter() {
        let mut rng = thread_rng();
        let x = obstacle_transform.translation.x + ((rng.gen_bool(0.5) as i16 * 2 - 1) * rng.gen_range(10..=50)) as f32;
        let y = obstacle_transform.translation.y + ((rng.gen_bool(0.5) as i16 * 2 - 1) * rng.gen_range(10..=50)) as f32;
    
        if rng.gen_bool(0.2) {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    texture: asset_server.load("ground/berry.png"),
                    transform: Transform::from_xyz( x, y, 1.0),
                    ..default()
                },
                Berry {
                    energy: rng.gen_range(5.0..=10.0),
                },
                Name::new("Berry"),
                Collider::cuboid(2.5, 2.5),
                ActiveEvents::COLLISION_EVENTS,
            ));
        }
    }
}

pub fn eat_berry(
    mut commands: Commands<'_, '_>,
    mut ce: EventReader<CollisionEvent>,
    mut player: Query<&mut Player>,
    berry: Query<&Berry>,
) {
    for event in ce.iter() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (player_entity, berry_entity) = (*e1, *e2);
            if let (Some(mut player), Some(berry)) = (player.get_mut(player_entity).ok(), berry.get(berry_entity).ok()) {
                player.energy_gain(berry.energy);
                commands.entity(berry_entity).despawn_recursive();
            }
        }
    }
}

pub fn cleanup_berry(
    mut commands: Commands<'_, '_>,
    mut berries: Query<(Entity, &Berry)>,
) {
    for (berry_entity, _) in &mut berries {
        commands.entity(berry_entity).despawn();
    }
}