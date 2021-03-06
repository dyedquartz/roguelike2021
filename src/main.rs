use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_tilemap::prelude::*;
use map::Map;

mod character;
mod components;
mod config;
mod map;
mod map_system;
mod player;
mod rect;
mod shadowcasting;
mod state_manager_system;
mod ui;
mod visibility_system;

const ARENA_WIDTH: i32 = 80;
const ARENA_HEIGHT: i32 = 50;
const UI_WIDTH: f32 = 200.0;
const FONT_WIDTH: f32 = 8.0;
const FONT_HEIGHT: f32 = 8.0;
const ATLAS_WIDTH: usize = 16;
const ATLAS_HEIGHT: usize = 16;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
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

    // Spawn Camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation.x = UI_WIDTH / 2.0;
    commands.spawn_bundle(camera);
    commands
        .spawn()
        .insert_bundle(tilemap_components)
        .insert(Timer::from_seconds(0.075, true));
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Roguelike Tutorial 2021".to_string(),
            width: ARENA_WIDTH as f32 * FONT_WIDTH + UI_WIDTH,
            height: ARENA_HEIGHT as f32 * FONT_HEIGHT,
            resizable: false,
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(GameState::PreRun)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Map::default())
        .add_startup_system(setup.system())
        .add_startup_system(config::open_config.system())
        .add_system(state_manager_system::state_manager.system())
        .add_system_set(SystemSet::on_enter(GameState::PreRun).with_system(map::build_map.system()).with_system(ui::setup_ui.system()))
        .add_system_set(
            SystemSet::on_update(GameState::AwaitingInput)
                .with_system(player::character_movement.system()),
        )
        // .add_system_set(
        //     SystemSet::on_update(GameState::Running)
        //         .with_system(player::character_movement.system())
        //         .with_system(map_system::map_indexing.system())
        //         .with_system(map_system::draw_map.system())
        //         .with_system(visibility_system::visibility.system()),
        // )
        .add_system_set(
            SystemSet::on_enter(GameState::PlayerTurn)
                .with_system(visibility_system::visibility.system().label("visibility"))
                .with_system(map_system::map_indexing.system().after("visibility"))
                .with_system(map_system::draw_map.system().after("visibility")),
        )
        .run();
}
