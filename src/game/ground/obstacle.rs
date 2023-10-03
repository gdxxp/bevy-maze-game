use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use rand::prelude::*;use bevy::utils::HashSet;
use std::collections::VecDeque;

use crate::game::player::player::Player;
use crate::game::resource::CurrentGrid;
use super::exit::Exit;
use crate::ground::GroundSize;
use crate::GameState;
use crate::config::*;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Obstacle;

pub fn gen_obstacle(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    exit: Query<(&Transform, &Exit)>,
    player: Query<&Transform, With<Player>>,
    size: Res<GroundSize>,
    current_grid: ResMut<CurrentGrid>,
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

    create_path(&mut grid_h, &mut grid_v, (start_x, start_y), (exit_x, exit_y), current_grid);
    
    for i in 0..grid_size {
        for j in 0..grid_size {
            let texture: Handle<Image> = asset_server.load("ground/horizontal_jungle_wall.png");
            if grid_h[i as usize][j as usize] {
                commands.spawn((
                    SpriteBundle {
                        texture,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(ROAD_SPACE as f32, 10.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz( (i as f32 + 0.5) * ROAD_SPACE as f32 - (size*100/2) as f32, (size*100/2) as f32 - (j as f32 + 0.5) * ROAD_SPACE as f32, 1.0),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid((ROAD_SPACE/2) as f32, 5.0),
                    Obstacle,
                ));
            }
        }
    }

    for i in 0..grid_size {
        for j in 0..grid_size {
            let texture: Handle<Image> = asset_server.load("ground/vertical_jungle_wall.png");
            if grid_v[i as usize][j as usize] {
                commands.spawn((
                    SpriteBundle {
                        texture,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(10.0, ROAD_SPACE as f32)),
                            ..default()
                        },
                        transform: Transform::from_xyz( (i as f32 + 0.5) * ROAD_SPACE as f32 - (size*100/2) as f32, (size*100/2) as f32 - (j as f32 + 0.5) * ROAD_SPACE as f32, 1.0),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(5.0, (ROAD_SPACE/2) as f32),
                    Obstacle,
                ));
            }
        }
    }

    next_state.set(GameState::InGame);
}

fn create_path(grid_h: &mut Vec<Vec<bool>>, grid_v: &mut Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize), mut current_grid: ResMut<CurrentGrid>) {

    let directions = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut rng = thread_rng();

    let mut current = start;
    let mut chosen_grid;
    let mut path_set = HashSet::new();
    let mut path_stack = VecDeque::new();
    if rng.gen_bool(0.5) {
        chosen_grid = grid_h.clone();
    } else {
        chosen_grid = grid_v.clone();
    }

    while current != end {
        grid_h[current.0][current.1] = false;
        grid_v[current.0][current.1] = false;
        chosen_grid[current.0][current.1] = false;
        let mut valid_neighbors = vec![];
        let mut unvisited_neighbors = vec![];

        for direction in &directions {
            let neighbor = (current.0 as isize + direction.0, 
                current.1 as isize + direction.1);

            if neighbor.0 < 0 || neighbor.1 < 0 {
                continue;
            }

            let neighbor = (neighbor.0 as usize, neighbor.1 as usize);

            if is_valid_index(&chosen_grid, neighbor, &path_set,) {
                if chosen_grid[neighbor.0][neighbor.1] == false {
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

    grid_h[current.0][current.1] = false;
    grid_v[current.0][current.1] = false;
    current_grid.0 = chosen_grid;
}

fn is_valid_index(grid: &Vec<Vec<bool>>,  index: (usize, usize), path_set: &HashSet<(usize, usize)>) -> bool {
    index.0 < grid.len() && index.1 < grid[0].len() && !path_set.contains(&index)
}

fn manhattan_distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    ((pos1.0 as isize - pos2.0 as isize).abs() +
    (pos1.1 as isize - pos2.1 as isize).abs()) as usize
}

pub fn cleanup_obstacle(
    mut commands: Commands,
    mut obstacles: Query<(Entity, &mut Obstacle)>,
) {
    for (obstacle_entity, mut _wall) in &mut obstacles {
        commands.entity(obstacle_entity).despawn();
    }
}