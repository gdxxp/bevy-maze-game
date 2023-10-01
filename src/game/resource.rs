use bevy::prelude::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Distance(pub i16);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct CurrentGrid(pub Vec<Vec<bool>>);