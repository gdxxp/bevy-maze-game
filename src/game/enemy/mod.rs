pub mod enemy;
pub mod snake;

use bevy::prelude::*;

use self::{enemy::gen_enemy, snake::{snake_movement, snake_attack, Snake, cleanup_snake}};

use super::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), gen_enemy)
            .add_systems(Update, (snake_movement, snake_attack).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::LoadObstacle), cleanup_snake)
            .register_type::<Snake>();
    }
}