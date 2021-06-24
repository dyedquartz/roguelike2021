use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/arial10x10.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(10.0, 10.0), 32, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite::new(32),
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
        ..Default::default()
    });
    for i in 0..10 {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(i + 16),
            transform: Transform::from_translation(Vec3::new(i as f32 * 10.0, 0.0, 0.0)),
            ..Default::default()
        });
    }
    for i in 0..26 {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(i + 96),
            transform: Transform::from_translation(Vec3::new(i as f32 * 10.0, -10.0, 10.0)),
            ..Default::default()
        });
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
