use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use crate::map::{Map, TileType};
use crate::components::{Position, BlocksTile};

pub fn map_indexing(
    mut map_data: ResMut<Map>,
    mut position_query: Query<(Entity, &Position, Option<&BlocksTile>)>
) {
    map_data.populate_blocked();
    map_data.clear_content_index();
    for (entity, position, blocks_tile) in position_query.iter() {
        let idx = map_data.xy_idx(position.x, position.y);

        if let Some(_p) = blocks_tile {
            map_data.blocked[idx] = true;
        }

        map_data.tile_content[idx].push(entity);
    }
}

pub fn draw_map(
    mut map_data: ResMut<Map>,
    mut tilemap_query: Query<&mut Tilemap>,
) {
    let mut tilemap = tilemap_query.single_mut().expect("There should only be one map");

    for (idx, tile) in map_data.tiles.iter().enumerate() {
        if map_data.revealed_tiles[idx] {
            let sprite_idx;
            let mut color;
            match tile {
                TileType::Floor => {
                    sprite_idx = '.' as usize;
                    color = Color::rgb_linear(0.0, 0.5, 0.5);
                }
                TileType::Wall => {
                    sprite_idx = '#' as usize;
                    color = Color::rgb_linear(0.0, 1.0, 0.0);
                }
            }
            if !map_data.visible_tiles[idx] {
                let gray = color.r() * 0.2126 + color.g() * 0.7152 + color.b() * 0.0722;
                color = Color::rgb_linear(gray, gray, gray);
            }

            let mut tile = tilemap.get_tile_mut(map_data.idx_xy(idx), 0).expect("Nonexistent Tile");

            tile.index = sprite_idx;
            tile.color = color;
        }
    }
}