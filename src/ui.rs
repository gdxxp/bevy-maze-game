use crate::game::resource::Distance;

use bevy::prelude::*;

pub struct GameUI;

#[derive(Component)]
pub struct DistanceText;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_money_ui);
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
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                DistanceText,
            ));
        });
}

fn update_money_ui(mut texts: Query<&mut Text, With<DistanceText>>, distance: Res<Distance>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Distance: {:?} km", distance.0);
    }
}