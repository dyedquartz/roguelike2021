use bevy::prelude::*;

use crate::GameState;

pub fn state_manager(
    mut state: ResMut<State<GameState>>
) {
    match state.current() {
        GameState::AwaitingInput => {
            return;
        },
        GameState::PlayerTurn => {
            state.set(GameState::AwaitingInput).expect("Unable to change state");
        }
        GameState::PreRun => {},
        GameState::Running => {},
    }
}