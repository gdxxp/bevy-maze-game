pub mod player;
pub mod ground;
pub mod resource;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use player::PlayerPlugin;
use ground::GroundPlugin;
use resource::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .insert_resource(Money(100.0))
            .register_type::<Money>()
            .insert_resource(Distance(2000))
            .register_type::<Distance>()
            .add_plugins(GroundPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
                RapierDebugRenderPlugin::default(),));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    LoadGame,
    LoadObstacle,
    InGame,
}