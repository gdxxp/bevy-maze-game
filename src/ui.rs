use crate::game::{resource::Distance, player::player::Player};

use bevy::prelude::*;

pub struct GameUI;

#[derive(Component)]
pub struct EnergyText;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_energy_ui);
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Distance",
                        TextStyle {
                            font_size: 16.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                EnergyText,
            ));
        });
}

fn update_energy_ui(mut texts: Query<&mut Text, With<EnergyText>>, player: Query<&Player, With<Player>>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Energy: {:?} ", player.single().energy as i16);
    }
}