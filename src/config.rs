use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InputConfig {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub pick_up: KeyCode,
    pub inventory: KeyCode,
}

pub fn open_config(mut commands: Commands) {
    let input_config: InputConfig = ron::from_str(&std::fs::read_to_string("assets/config/input.ron").unwrap()).unwrap();
    commands.insert_resource(input_config);
}