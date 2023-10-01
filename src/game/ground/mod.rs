
pub mod ground;
pub mod exit;
pub mod obstacle;
pub mod food;

use bevy::prelude::*;

use crate::game::GameState;
use ground::*;
use exit::*;
use obstacle::*;
use food::*;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GroundSize(50))
            .add_systems(OnEnter(GameState::LoadGame), (ground_setup, gen_exit.after(ground_setup)))
            .add_systems(OnExit(GameState::LoadGame), cleanup_obstacle)
            .add_systems(OnEnter(GameState::LoadObstacle), gen_obstacle)
            .add_systems(OnExit(GameState::LoadObstacle), cleanup_berry)
            .add_systems(OnEnter(GameState::InGame), gen_berry)
            .add_systems(OnExit(GameState::InGame), cleanup_ground)
            .add_systems(Update, (exit_check, eat_berry).run_if(in_state(GameState::InGame)))
            .register_type::<Ground>()
            .register_type::<Wall>()
            .register_type::<Obstacle>()
            .register_type::<Exit>()
            .register_type::<Berry>();
    }
}