use bevy::prelude::*;
use bevy::audio::PlaybackMode;

#[derive(Bundle)]
pub struct MusicBundle {
    audio: AudioBundle,
    music: Music,
}

#[derive(Component)]
pub struct Music;

#[derive(Event)]
pub struct StartMusicEvent;

#[derive(Event)]
pub struct StopMusicEvent;

#[derive(Event)]
pub struct PlaySoundEvent(pub SoundID);

#[derive(Debug)]
pub enum SoundID {
    AlloyMachine,
    ParticleAccelerator,
    // TODO: Continue this list
}

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

pub fn start_music(
    mut ev_startmusic: EventReader<StartMusicEvent>,
    mut q_music: Query<&mut PlaybackSettings, With<Music>>,
) {
    for _ in ev_startmusic.read() {
        let mut music = q_music.single_mut();

        music.paused = false;
    }
}

pub fn play_sound(
    mut ev_sound: EventReader<PlaySoundEvent>,
) {
    for ev in ev_sound.read() {
        info!("Playing sound: {:?}", ev.0);
    }
}
