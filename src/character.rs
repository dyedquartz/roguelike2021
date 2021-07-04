use bevy_tilemap::prelude::*;

use crate::components::{Position, Render};

pub fn move_sprite(
    map: &mut Tilemap,
    previous_position: Position,
    position: Position,
    render: &Render,
) {
    map.clear_tile(
        (previous_position.x, previous_position.y),
        render.sprite_order,
    )
    .unwrap();

    let tile = Tile {
        point: (position.x, position.y),
        sprite_index: render.sprite_index,
        sprite_order: render.sprite_order,
        tint: render.tint,
    };

    map.insert_tile(tile).unwrap()
}
