use bevy::{prelude::*, utils::HashSet};
use bevy_tilemap::prelude::*;

mod character;
mod components;
mod map;
mod player;
mod rect;

const ARENA_WIDTH: i32 = 80;
const ARENA_HEIGHT: i32 = 50;
const FONT_WIDTH: f32 = 8.0;
const FONT_HEIGHT: f32 = 8.0;
const ATLAS_WIDTH: usize = 16;
const ATLAS_HEIGHT: usize = 16;

pub struct Collisions(HashSet<(i32, i32)>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    PreRun,
    Running,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/terminalwhite8x8_aa_ro.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(FONT_WIDTH, FONT_HEIGHT),
        ATLAS_WIDTH,
        ATLAS_HEIGHT,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let tilemap = Tilemap::builder()
        .dimensions(1, 1)
        .chunk_dimensions(ARENA_WIDTH as u32, ARENA_HEIGHT as u32, 1)
        .texture_dimensions(FONT_WIDTH as u32, FONT_HEIGHT as u32)
        .texture_atlas(texture_atlas_handle.clone())
        // Map layer
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
            },
            0,
        )
        // Item layer
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
            },
            1,
        )
        // Player layer
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Sparse,
            },
            2,
        )
        .finish()
        .unwrap();

    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    };
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert_bundle(tilemap_components)
        .insert(Timer::from_seconds(0.075, true));
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Tutorial 2021".to_string(),
            width: ARENA_WIDTH as f32 * FONT_WIDTH,
            height: ARENA_HEIGHT as f32 * FONT_WIDTH,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_state(GameState::PreRun)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Collisions(HashSet::default()))
        .add_startup_system(setup.system())
        .add_system_set(SystemSet::on_enter(GameState::PreRun).with_system(map::build_map.system()))
        .add_system_set(SystemSet::on_update(GameState::Running).with_system(player::character_movement.system()))
        .run();
}
