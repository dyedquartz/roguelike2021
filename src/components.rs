use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct Render {
    pub sprite_index: usize,
    pub sprite_order: usize,
    pub tint: Color,
}

#[derive(Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: Position,
    pub render: Render,
}

#[derive(Default, Clone)]
pub struct GameState {
    pub map_loaded: bool,
    pub spawned: bool,
    pub collisions: HashSet<(i32, i32)>,
}
