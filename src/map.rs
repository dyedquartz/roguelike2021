use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use crate::components::{Player, PlayerBundle, Position, Render};
use crate::GameState;

pub fn build_map(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<&mut Tilemap>,
) {
    for mut map in query.iter_mut() {
        info!("Loading Map");

        map.insert_chunk((0, 0)).unwrap();

        let mut tiles = Vec::new();

        let player_index = '@' as usize;

        let player_tile = Tile {
            point: (0, 0),
            sprite_order: 2,
            sprite_index: player_index,
            tint: Color::WHITE,
        };
        tiles.push(player_tile);

        commands.spawn().insert_bundle(PlayerBundle {
            player: Player,
            position: Position { x: 0, y: 0 },
            render: Render {
                sprite_index: player_index,
                sprite_order: 2,
                tint: Color::WHITE,
            },
        });

        map.insert_tiles(tiles).unwrap();

        map.spawn_chunk((0, 0)).unwrap();

        game_state.set(GameState::Running).unwrap();
    }
}
