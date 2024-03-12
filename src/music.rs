use bevy::prelude::*;
use bevy::audio::PlaybackMode

#[derive(Bundle)]
pub struct MusicBundle {
    audio: AudioBundle,
    music: Music,
}

#[derive(Component)]
pub struct Music;

pub fn instance_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(MusicBundle {
        audio: AudioBundle {
            source: asset_server.load("sound/Vacuum_Decay.ogg"),
            settings: PlaybackSettings {
                paused: true,
                mode: PlaybackMode::Once,
                ..default()
            }
        },
        music: Music,
    });
}
