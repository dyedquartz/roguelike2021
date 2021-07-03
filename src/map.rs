use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use crate::components::{Player, PlayerBundle, Position, Render};
use crate::{ARENA_HEIGHT, ARENA_WIDTH, Collisions, GameState, rect};

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

        // Fill with walls
        for y in 0..ARENA_HEIGHT {
            for x in 0..ARENA_WIDTH {
                let y = y - ARENA_HEIGHT / 2;
                let x = x - ARENA_WIDTH / 2;

                let tile = Tile {
                    point: (x, y),
                    sprite_index: '#' as usize,
                    sprite_order: 0,
                    tint: Color::GRAY,
                };

                tiles.push(tile);
            }
        }

        // Spawn room
        add_room(&mut tiles, &mut collisions, rect::Rect::new(-5, -5, 10, 10));


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

pub fn add_room(tiles: &mut Vec<Tile<(i32, i32)>>, collisions: &mut Collisions, size: rect::Rect) {
    for x in size.x1..=size.x2 {
        for y in size.y1..=size.y2 {
            tiles.push(Tile {
                point: (x, y),
                sprite_order: 0,
                sprite_index: '.' as usize,
                tint: Color::GRAY,
            })
        }
    }
}