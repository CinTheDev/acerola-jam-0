use bevy::prelude::*;

#[derive(Component)]
struct Cursor;

pub fn spawn_ui(parent: &mut ChildBuilder, asset_server: Res<AssetServer>) {
    parent.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            image: UiImage::new(asset_server.load("img/cursor_default.png")),
            ..default()
        },
        Cursor,
    ));
}
