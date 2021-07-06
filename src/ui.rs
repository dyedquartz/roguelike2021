use bevy::prelude::*;

use crate::{config::InputConfig, UI_WIDTH};

pub fn setup_ui(
    mut commands: Commands,
    input_config: Res<InputConfig>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    // Root Node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        // Bottom Border
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(UI_WIDTH), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            right: Val::Px(0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    material: materials.add(Color::WHITE.into()),
                    ..Default::default()
                })
                // Bottom Fill
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            material: materials.add(Color::DARK_GRAY.into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Bottom hints
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        bottom: Val::Px(5.0),
                                        left: Val::Px(15.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    format!(
                                        "{:?}{:?}{:?}{:?}: Movement\n: Inventory\n: Pickup",
                                        input_config.up,
                                        input_config.left,
                                        input_config.down,
                                        input_config.right
                                    ),
                                    TextStyle {
                                        font: asset_server.load("fonts/CascadiaCode.ttf"),
                                        font_size: 12.0,
                                        color: Color::WHITE,
                                    },
                                    TextAlignment {
                                        horizontal: HorizontalAlign::Left,
                                        ..Default::default()
                                    },
                                ),
                                ..Default::default()
                            });
                        });
                });
        });
}
