use bevy::prelude::*;

const ARENA_WIDTH: u32 = 80;
const ARENA_HEIGHT: u32 = 50;
const FONT_WIDTH: f32 = 8.0;
const FONT_HEIGHT: f32 = 8.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("textures/terminalwhite8x8_aa_ro.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(FONT_WIDTH, FONT_HEIGHT), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite {
            color: Color::WHITE,
            index: '@' as u32,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, FONT_HEIGHT, 0.0)),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            sprite: Sprite::new(Vec2::new(FONT_WIDTH, FONT_HEIGHT)),
            ..Default::default()
        });
    });
    for i in 0..10 {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                color: Color::RED,
                index: '0' as u32 + i,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(i as f32 * FONT_WIDTH, 0.0, 0.0)),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb_linear(0.0, 0.0, 1.0 / i as f32).into()),
                sprite: Sprite::new(Vec2::new(FONT_WIDTH, FONT_HEIGHT)),
                ..Default::default()
            });
        });
    }
    for i in 0..26 {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                color: Color::WHITE,
                index: 'a' as u32 + i,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(i as f32 * FONT_WIDTH, -FONT_HEIGHT, 0.0)),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb_linear(0.0, 1.0 / i as f32, 0.0).into()),
                sprite: Sprite::new(Vec2::new(FONT_WIDTH, FONT_HEIGHT)),
                ..Default::default()
            });
        });
    }
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
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .run();
}
