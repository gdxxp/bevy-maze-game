use bevy::prelude::*;
use rand::prelude::*;

use crate::game::ground::ground::GroundSize;
use crate::game::resource::{CurrentGrid, Distance};
use crate::ROAD_SPACE;
use super::snake::gen_snake;

pub fn gen_enemy(
    commands: Commands,
    asset_server: Res<AssetServer>,
    current_grid: Res<CurrentGrid>,
    distance: Res<Distance>,
    size: Res<GroundSize>,
) {
    match distance.0 {
        1600..=2000 => {
            gen_simple_enemy(commands, asset_server, current_grid, size);
        },
        _ => return,
    }
}

fn gen_simple_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_grid: Res<CurrentGrid>,
    size: Res<GroundSize>,
) {
    let mut rng = thread_rng();
    let size = size.0 + 1;

    for (row_index, row) in current_grid.0.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            if !value && rng.gen_bool(0.2) {
                gen_snake(&mut commands, &asset_server, (col_index as f32 + 0.5) * ROAD_SPACE as f32 - (size*100/2) as f32, (size*100/2) as f32 - (row_index as f32 + 0.5) * ROAD_SPACE as f32);
            }
        }
    }
}