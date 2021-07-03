use bevy::prelude::*;
use bevy_tilemap::prelude::*;

use crate::{Collisions, GameState, character::move_sprite, components::{Player, Position, Render}};

pub fn character_movement(
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    collisions: Res<Collisions>,
    mut map_query: Query<(&mut Tilemap, &mut Timer)>,
    mut player_query: Query<(&mut Position, &Render, &Player)>,
) {
    for (mut map, mut timer) in map_query.iter_mut() {
        for (mut position, render, _player) in player_query.iter_mut() {
            for key in keyboard_input.get_just_pressed() {
                let previous_position = *position;

                use KeyCode::*;
                match key {
                    W => {
                        try_move_player(
                            &collisions,
                            &mut position,
                            (0, 1),
                        );
                    }
                    A => {
                        try_move_player(
                            &collisions,
                            &mut position,
                            (-1, 0),
                        );
                    }
                    S => {
                        try_move_player(
                            &collisions,
                            &mut position,
                            (0, -1),
                        );
                    }
                    D => {
                        try_move_player(
                            &collisions,
                            &mut position,
                            (1, 0),
                        );
                    }

                    _ => {}
                }

                if previous_position != *position {
                    move_sprite(&mut map, previous_position, *position, render);
                }

            }
        }
    }
}

pub fn try_move_player(
    collisions: &Collisions,
    position: &mut Position,
    delta_xy: (i32, i32)
) {
    let new_pos = (position.x + delta_xy.0, position.y + delta_xy.1);
    if !collisions.0.contains(&new_pos) {
        position.x = new_pos.0;
        position.y = new_pos.1;
    }
}
