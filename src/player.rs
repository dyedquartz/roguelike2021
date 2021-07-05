use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use crate::GameState;
use crate::components::Viewshed;
use crate::map::Map;
use crate::{
    character::move_sprite,
    components::{Player, Position, Render},
};

pub fn character_movement(
    mut gamestate: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    map_data: Res<Map>,
    mut map_query: Query<&mut Tilemap>,
    mut player_query: Query<(&mut Position, &Render, &Player, &mut Viewshed)>,
) {
    for mut map in map_query.iter_mut() {
        for (mut position, render, _player, mut viewshed) in player_query.iter_mut() {
            for key in keyboard_input.get_just_pressed() {
                let previous_position = *position;

                use KeyCode::*;
                match key {
                    W => {
                        if try_move_player(&map_data, &mut position, (0, 1)) {
                            viewshed.dirty = true;
                        };
                    }
                    A => {
                        if try_move_player(&map_data, &mut position, (-1, 0)) {
                            viewshed.dirty = true;
                        };
                    }
                    S => {
                        if try_move_player(&map_data, &mut position, (0, -1)) {
                            viewshed.dirty = true;
                        };
                    }
                    D => {
                        if try_move_player(&map_data, &mut position, (1, 0)) {
                            viewshed.dirty = true;
                        };
                    }

                    _ => {}
                }

                if previous_position != *position {
                    move_sprite(&mut map, previous_position, *position, render);
                    gamestate.set(GameState::PlayerTurn).unwrap();
                }
            }

        }
    }

}

pub fn try_move_player(map_data: &Map, position: &mut Position, delta_xy: (i32, i32)) -> bool {
    let new_x = position.x + delta_xy.0;
    let new_y = position.y + delta_xy.1;
    if !map_data.blocked[map_data.xy_idx(new_x, new_y)] {
        position.x = new_x;
        position.y = new_y;
        true
    } else {
        false
    }
}
