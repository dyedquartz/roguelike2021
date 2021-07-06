use bevy::prelude::*;

use crate::UI_WIDTH;

pub fn setup_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    // Root Node
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    })
    // Bottom Border
    .with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
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
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                material: materials.add(Color::DARK_GRAY.into()),
                ..Default::default()
            });
        });
    });
}