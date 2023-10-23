use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use rand::{thread_rng, Rng};

use crate::game::player::player::Player;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Snake{
    pub speed: f32,
    pub timer: Timer,
    pub direction: Vec2,
}

pub fn gen_snake(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position_x: f32,
    position_y: f32,
) {
    let random_direction = random_direction();
    let texture: Handle<Image> = asset_server.load("enemy/snake.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            transform: Transform::from_xyz(position_x, position_y, 1.0),
            texture,
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(6.0, 1.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        Snake {
            speed: 60.0,
            timer: Timer::from_seconds(2.6, TimerMode::Repeating),
            direction: random_direction,
        },
        Velocity::zero(),
        ActiveEvents::COLLISION_EVENTS,
        Name::new("Snake")
    ));
}

pub fn snake_movement(
    time: Res<Time>,
    mut snake_query: Query<(&mut Snake, &mut Velocity)>,
) {
    for (mut snake, mut rb_vels) in snake_query.iter_mut() {
        if snake.timer.tick(time.delta()).just_finished() {
            snake.direction = random_direction();
        }
        let mut move_delta = snake.direction;
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }
        rb_vels.linvel = move_delta * snake.speed;
    }
}

pub fn snake_attack(
    mut ce: EventReader<CollisionEvent>,
    mut player: Query<&mut Player>,
    snake: Query<&Snake>,
) {
    for event in ce.iter() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (entity_1, entity_2) = (*e1, *e2);
            if let (Some(mut player), Some(_snake)) = (player.get_mut(entity_1).ok(), snake.get(entity_2).ok()) {
                player.energy_lose(20.0);
            }
        }
    }
}

fn random_direction() -> Vec2 {
    let mut rng = rand::thread_rng();
    let dx = rng.gen_range(-1.0..1.0);
    let dy = rng.gen_range(-1.0..1.0);
    Vec2::new(dx as f32, dy as f32)
}

pub fn cleanup_snake(
    mut commands: Commands<'_, '_>,
    mut snakes: Query<(Entity, &Snake)>,
) {
    for (snake_entity, _) in &mut snakes {
        commands.entity(snake_entity).despawn();
    }
}