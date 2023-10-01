pub mod player;

use bevy::prelude::*;

use player::*;

use super::{GameState, ground::ground::ground_setup};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, (player_movement, camera_movement, energy_check).run_if(in_state(GameState::InGame)))
            .add_systems(OnEnter(GameState::LoadGame), player_transmission.after(ground_setup))
            .register_type::<Camera>()
            .register_type::<Player>();
    }
}