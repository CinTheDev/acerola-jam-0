use bevy::prelude::*;

#[derive(Component)]
struct Cursor;

pub fn spawn_ui(parent: &mut ChildBuilder, asset_server: Res<AssetServer>) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Grid,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|root| {
        root.spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(8.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                image: UiImage::new(asset_server.load("img/cursor_default.png")),
                ..default()
            },
            Cursor,
        ));
    });
}
