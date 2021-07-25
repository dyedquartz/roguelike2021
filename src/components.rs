use bevy::prelude::*;
use bevy_tilemap::point::Point2;

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
    pub viewshed: Viewshed,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub position: Position,
    pub render: Render,
    pub viewshed: Viewshed,
}

pub struct Viewshed {
    pub visible_tiles: Vec<Point2>,
    pub range: i32,
    pub dirty: bool,
}

pub struct BlocksTile;

pub struct GameLog;