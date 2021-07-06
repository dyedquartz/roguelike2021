use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use crate::GameState;
use crate::components::Viewshed;
use crate::config::InputConfig;
use crate::map::Map;
use crate::{
    character::move_sprite,
    components::{Player, Position, Render},
};

pub fn character_movement(
    mut gamestate: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    input_config: Res<InputConfig>,
    map_data: Res<Map>,
    mut map_query: Query<&mut Tilemap>,
    mut player_query: Query<(&mut Position, &Render, &Player, &mut Viewshed)>,
) {
    let mut moved = false;
    for mut map in map_query.iter_mut() {
        for (mut position, render, _player, mut viewshed) in player_query.iter_mut() {
            for key in keyboard_input.get_just_pressed() {
                let previous_position = *position;

                if key == &input_config.up {
                    if try_move_player(&map_data, &mut position, (0, 1)) {
                        viewshed.dirty = true;
                    }
                } else if key == &input_config.left {
                    if try_move_player(&map_data, &mut position, (-1, 0)) {
                        viewshed.dirty = true;
                    }
                } else if key == &input_config.down {
                    if try_move_player(&map_data, &mut position, (0, -1)) {
                        viewshed.dirty = true;
                    }
                } else if key == &input_config.right {
                    if try_move_player(&map_data, &mut position, (1, 0)) {
                        viewshed.dirty = true;
                    }
                }

                if previous_position != *position {
                    move_sprite(&mut map, previous_position, *position, render);
                    viewshed.dirty = true;
                    moved = true;
                }
            }

        }
    }
    if moved == true {
        gamestate.set(GameState::PlayerTurn).unwrap();
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
