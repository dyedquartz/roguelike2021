use bevy::prelude::*;

use crate::{GameState, components::GameLog};

pub fn state_manager(
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Text, With<GameLog>>,
) {
    match state.current() {
        GameState::AwaitingInput => {
            return;
        },
        GameState::PlayerTurn => {
            state.set(GameState::AwaitingInput).expect("Unable to change state");
            let mut text = query.single_mut().expect("only one gamelog should exist");

            text.sections.push(TextSection {
                value: "Test test test, new section!\n".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/CascadiaCode.ttf"),
                    font_size: 12.0,
                    color: Color::WHITE,
                }
            });
        }
        GameState::PreRun => {},
        GameState::Running => {},
    }
}