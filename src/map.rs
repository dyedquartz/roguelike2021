use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use rand::prelude::*;

use crate::components::{EnemyBundle, Player, PlayerBundle, Position, Render, Viewshed};
use crate::{rect, GameState, ARENA_HEIGHT, ARENA_WIDTH};
use std::cmp::{max, min};

pub fn build_map(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut map_data: ResMut<Map>,
    mut query: Query<&mut Tilemap>,
) {
    for mut map in query.iter_mut() {
        info!("Loading Map");

        map.insert_chunk((0, 0)).unwrap();

        let mut tiles = Vec::new();
        let mut rooms = Vec::new();

        // Fill with blanks
        for y in 0..ARENA_HEIGHT {
            for x in 0..ARENA_WIDTH {
                let y = y - ARENA_HEIGHT / 2;
                let x = x - ARENA_WIDTH / 2;

                let tile = Tile {
                    point: (x, y),
                    sprite_index: ' ' as usize,
                    sprite_order: 0,
                    tint: Color::GRAY,
                };

                tiles.push(tile);
            }
        }

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = thread_rng();

        // Spawn Rooms
        for _ in 0..MAX_ROOMS {
            let w = rng.gen_range(MIN_SIZE..=MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE..=MAX_SIZE);
            let x = rng.gen_range((-map_data.width / 2)..(map_data.width / 2 - w - 1));
            let y = rng.gen_range((-map_data.height / 2)..(map_data.height / 2 - h - 1));
            let new_room = rect::Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map_data.apply_room(&new_room);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                    if rng.gen() {
                        map_data.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map_data.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map_data.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map_data.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }
        }

        // Spawn Enemies
        for room in rooms.iter().skip(1) {
            let (x,y) = room.center();
            let enemy_tile = Tile {
                point: (x, y),
                sprite_order: 2,
                sprite_index: 'g' as usize,
                tint: Color::RED,
            };
            tiles.push(enemy_tile);
            commands.spawn_bundle(EnemyBundle {
                position: Position {
                    x,
                    y,
                },
                render: Render {
                    sprite_index: 'g' as usize,
                    sprite_order: 2,
                    tint: Color::RED,
                },
                viewshed: Viewshed {
                    visible_tiles: Vec::new(),
                    range: 8,
                    dirty: true
                }
            });
        }

        // Spawn Player
        let player_index = '@' as usize;
        let (player_x, player_y) = rooms[0].center();

        let player_tile = Tile {
            point: (player_x, player_y),
            sprite_order: 2,
            sprite_index: player_index,
            tint: Color::GREEN,
        };
        tiles.push(player_tile);

        commands.spawn().insert_bundle(PlayerBundle {
            player: Player,
            position: Position {
                x: player_x,
                y: player_y,
            },
            render: Render {
                sprite_index: player_index,
                sprite_order: 2,
                tint: Color::GREEN,
            },
            viewshed: Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            },
        });

        map_data.rooms = rooms;

        map.insert_tiles(tiles).unwrap();

        map.spawn_chunk((0, 0)).unwrap();

        game_state.set(GameState::PlayerTurn).unwrap();
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<rect::Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tiles: vec![TileType::Wall; (ARENA_WIDTH * ARENA_HEIGHT) as usize],
            rooms: Vec::new(),
            width: ARENA_WIDTH,
            height: ARENA_HEIGHT,
            revealed_tiles: vec![false; (ARENA_WIDTH * ARENA_HEIGHT) as usize],
            visible_tiles: vec![false; (ARENA_WIDTH * ARENA_HEIGHT) as usize],
            blocked: vec![false; (ARENA_WIDTH * ARENA_HEIGHT) as usize],
            tile_content: vec![Vec::new(); (ARENA_WIDTH * ARENA_HEIGHT) as usize],
        }
    }
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y + self.height / 2) * self.width + x + self.width / 2) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width - self.width / 2;
        let y = idx as i32 / self.width - self.height / 2;
        (x, y)
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    fn apply_room(&mut self, room: &rect::Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
}
