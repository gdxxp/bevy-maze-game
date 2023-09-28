use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use rand::prelude::*;


use crate::game::GameState;
use crate::config::*;
use crate::game::player::player::Player;
use crate::resource::Distance;

use super::exit::Exit;

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

pub fn gen_obstacle(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    exit: Query<(&Transform, &Exit)>,
    player: Query<&Transform, With<Player>>,
    size: Res<GroundSize>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let size = size.0 + 1;

    let mut grid_h: Vec<Vec<bool>> = Vec::new();
    let mut grid_v: Vec<Vec<bool>> = Vec::new();
 
    let mut rng = thread_rng();
    let grid_size = size*100/ROAD_SPACE; 
    for _ in 0..grid_size {
        let mut row = Vec::new();
        for _ in 0..grid_size {
            row.push(rng.gen_bool(O_PROBABILITY));
        }
        grid_h.push(row);
    }
    
    for _ in 0..grid_size {
        let mut row = Vec::new();
        for _ in 0..grid_size {
            row.push(rng.gen_bool(O_PROBABILITY));
        }
        grid_v.push(row);
    }
    // start and end points
    let player_translation = player.single().translation;
    let exit_translation = exit.single().0.translation;

    let start_x = ((size * 100 / 2 + player_translation.x as i16 ) / ROAD_SPACE) as usize; 
    let start_y = ((size * 100 / 2 - player_translation.y as i16 ) / ROAD_SPACE) as usize ;
    let exit_x = ((size * 100 / 2 + exit_translation.x as i16) / ROAD_SPACE) as usize;
    let exit_y = ((size * 100 / 2 - exit_translation.y as i16) / ROAD_SPACE) as usize;

    create_path(&mut grid_h, &mut grid_v, (start_x, start_y), (exit_x, exit_y));
    
    for i in 0..grid_size {
        for j in 0..grid_size {
            if grid_h[i as usize][j as usize] {
                let texture = asset_server.load("ground/horizontal_jungle_wall.png");
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(ROAD_SPACE as f32, 10.0)),
                            ..default()
                        },
                        texture,
                        transform: Transform::from_xyz( (i as f32 + 0.5) * ROAD_SPACE as f32 - (size*100/2) as f32, (size*100/2) as f32 - (j as f32 + 0.5) * ROAD_SPACE as f32, 1.0),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid((ROAD_SPACE/2) as f32, 5.0),
                    Wall,
                ));
            }
        }
    }

    for i in 0..grid_size {
        for j in 0..grid_size {
            if grid_v[i as usize][j as usize] {
                let texture = asset_server.load("ground/vertical_jungle_wall.png");
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(10.0, ROAD_SPACE as f32)),
                            ..default()
                        },
                        texture,
                        transform: Transform::from_xyz( (i as f32 + 0.5) * ROAD_SPACE as f32 - (size*100/2) as f32, (size*100/2) as f32 - (j as f32 + 0.5) * ROAD_SPACE as f32, 1.0),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(5.0, (ROAD_SPACE/2) as f32),
                    Wall,
                ));
            }
        }
    }
    next_state.set(GameState::InGame);
}

fn create_path(grid_h: &mut Vec<Vec<bool>>, grid_v: &mut Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) {

    let directions = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut rng = thread_rng();

    let mut current = start;
    let current_grid;
    let mut path_set = HashSet::new();
    let mut path_stack = VecDeque::new();
    if rng.gen_bool(0.5) {
        current_grid = grid_h.clone();
    } else {
        current_grid = grid_v.clone();
    }

    while current != end {
        grid_h[current.0][current.1] = false;
        grid_v[current.0][current.1] = false;
        let mut valid_neighbors = vec![];
        let mut unvisited_neighbors = vec![];

        for direction in &directions {
            let neighbor = (current.0 as isize + direction.0, 
                current.1 as isize + direction.1);

            if neighbor.0 < 0 || neighbor.1 < 0 {
                continue;
            }

            let neighbor = (neighbor.0 as usize, neighbor.1 as usize);

            if is_valid_index(&current_grid, neighbor, &path_set,) {
                if current_grid[neighbor.0][neighbor.1] == false {
                    valid_neighbors.push(neighbor)
                } else {
                    unvisited_neighbors.push(neighbor)
                }
            }
        }

        path_set.insert(current);
        if !valid_neighbors.is_empty() {
            path_stack.push_back(current);
            current = *valid_neighbors.choose(&mut rng).unwrap();
        } else if !unvisited_neighbors.is_empty() {
            let mut distances = unvisited_neighbors
                .iter()
                .map(|&pos| (pos, manhattan_distance(pos, end)))
                .collect::<Vec<_>>();
            distances.sort_by(|&a, &b| a.1.cmp(&b.1));

            if !distances.is_empty() {
                path_stack.push_back(current);
                current = distances[0].0;
            }
        } else {
            current = path_stack.pop_back().unwrap();
        }
    }

    if current == end {
        println!("bingo! loop reaches end!");
    }
    grid_h[current.0][current.1] = false;
    grid_v[current.0][current.1] = false;
}

fn is_valid_index(grid: &Vec<Vec<bool>>,  index: (usize, usize), path_set: &HashSet<(usize, usize)>) -> bool {
    index.0 < grid.len() && index.1 < grid[0].len() && !path_set.contains(&index)
}

fn manhattan_distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    ((pos1.0 as isize - pos2.0 as isize).abs() +
    (pos1.1 as isize - pos2.1 as isize).abs()) as usize
}