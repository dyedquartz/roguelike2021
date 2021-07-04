use crate::components::{Player, Position, Viewshed};
use crate::map::{Map, TileType};
use crate::shadowcasting::RPAShadowcasting;
use bevy::prelude::*;
use bevy_tilemap::point::Point2;

pub fn visibility(
    mut map_data: ResMut<Map>,
    mut viewshed_query: Query<(&mut Viewshed, &Position, Option<&Player>)>,
) {
    for (mut viewshed, position, player) in viewshed_query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles.clear();

            for (rel_x, rel_y, visible) in RPAShadowcasting::new(viewshed.range, |x, y| {
                map_data.tiles[map_data.xy_idx(x + position.x, y + position.y)] == TileType::Wall
            }) {
                if visible {
                    viewshed
                        .visible_tiles
                        .push(Point2::new(position.x + rel_x, position.y + rel_y));
                }
            }

            viewshed.visible_tiles.retain(|p| {
                p.x >= -map_data.width / 2
                    && p.x < map_data.width / 2 - 1
                    && p.y >= -map_data.height / 2
                    && p.y < map_data.height / 2 - 1
            });

            if let Some(_p) = player {
                for t in map_data.visible_tiles.iter_mut() {
                    *t = false;
                }

                for vis in viewshed.visible_tiles.iter() {
                    let idx = map_data.xy_idx(vis.x, vis.y);
                    map_data.revealed_tiles[idx] = true;
                    map_data.visible_tiles[idx] = true;
                }
            }
        }
    }
}
