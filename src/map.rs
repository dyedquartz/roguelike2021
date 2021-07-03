use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use crate::components::{Player, PlayerBundle, Position, Render};
use crate::{ARENA_HEIGHT, ARENA_WIDTH, Collisions, GameState};

pub fn build_map(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut collisions: ResMut<Collisions>,
    mut query: Query<&mut Tilemap>,
) {
    for mut map in query.iter_mut() {
        info!("Loading Map");

        map.insert_chunk((0, 0)).unwrap();

        let mut tiles = Vec::new();

        // Fill with floor
        for y in 0..ARENA_HEIGHT {
            for x in 0..ARENA_WIDTH {
                let y = y - ARENA_HEIGHT / 2;
                let x = x - ARENA_WIDTH / 2;

                let tile = Tile {
                    point: (x, y),
                    sprite_index: '.' as usize,
                    sprite_order: 0,
                    tint: Color::GRAY,
                };

                tiles.push(tile);
            }
        }

        let room_width = 16;
        let room_height = 16;

        // X-axis Boundries
        for x in 0..room_width {
            let x = x - room_width / 2;
            let tile_a = (x, -room_height / 2);
            let tile_b = (x, room_height / 2 - 1);
            tiles.push(Tile {
                point: tile_a,
                sprite_index: '#' as usize,
                sprite_order: 0,
                tint: Color::GRAY,
            });
            tiles.push(Tile {
                point: tile_b,
                sprite_index: '#' as usize,
                sprite_order: 0,
                tint: Color::GRAY,
            });
            
            collisions.0.insert(tile_a);
            collisions.0.insert(tile_b);
        }

        // Y-axis boundries
        for y in 0..room_height {
            let y = y - room_height / 2;
            let tile_a = (-room_width / 2, y);
            let tile_b = (room_width / 2 - 1, y);
            tiles.push(Tile {
                point: tile_a,
                sprite_index: '#' as usize,
                sprite_order: 0,
                tint: Color::GRAY,
            });
            tiles.push(Tile {
                point: tile_b,
                sprite_index: '#' as usize,
                sprite_order: 0,
                tint: Color::GRAY,
            });

            collisions.0.insert(tile_a);
            collisions.0.insert(tile_b);
        }


        // Spawn Player
        let player_index = '@' as usize;

        let player_tile = Tile {
            point: (0, 0),
            sprite_order: 2,
            sprite_index: player_index,
            tint: Color::GREEN,
        };
        tiles.push(player_tile);

        commands.spawn().insert_bundle(PlayerBundle {
            player: Player,
            position: Position { x: 0, y: 0 },
            render: Render {
                sprite_index: player_index,
                sprite_order: 2,
                tint: Color::GREEN,
            },
        });

        map.insert_tiles(tiles).unwrap();

        map.spawn_chunk((0, 0)).unwrap();

        game_state.set(GameState::Running).unwrap();
    }
}
