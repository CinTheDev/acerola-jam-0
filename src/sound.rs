use bevy::prelude::*;
use bevy::audio::PlaybackMode;

#[derive(Bundle)]
pub struct MusicBundle {
    audio: AudioBundle,
    music: Music,
}

#[derive(Bundle)]
pub struct PlayableSoundBundle {
    source: AudioBundle,
    properties: PlayableSound,
}

#[derive(Component)]
pub struct Music;

#[derive(Component)]
pub struct PlayableSound {
    id: SoundID,
}

#[derive(Event)]
pub struct StartMusicEvent;

#[derive(Event)]
pub struct StopMusicEvent;

#[derive(Event)]
pub struct PlaySoundEvent(pub SoundID);

#[derive(PartialEq, Clone, Copy, Debug)]
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

pub fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(get_sound(&asset_server, "sound/AlloyMachine.ogg".to_owned(), SoundID::AlloyMachine));
}

pub fn start_music(
    mut ev_startmusic: EventReader<StartMusicEvent>,
    mut q_music: Query<&mut PlaybackSettings, With<Music>>,
) {
    for _ in ev_startmusic.read() {
        return; // Disable music because it keeps distracting me :(
        let mut music = q_music.single_mut();

        music.paused = false;
    }
}

pub fn play_sound(
    mut ev_sound: EventReader<PlaySoundEvent>,
    mut q_soundplayer: Query<(&mut PlaybackSettings, &PlayableSound)>,
) {
    for ev in ev_sound.read() {
        let id = ev.0;
        
        for (mut sound, properties) in q_soundplayer.iter_mut() {
            if properties.id != id { continue }
            
            // Play sound
            sound.paused = false;
            info!("Playing sound: {:?}", id);
        }
    }
}

fn get_sound(
    asset_server: &Res<AssetServer>,
    source: String,
    id: SoundID,
) -> PlayableSoundBundle {
    PlayableSoundBundle {
        source: AudioBundle {
            source: asset_server.load(source),
            settings: PlaybackSettings {
                paused: true,
                mode: PlaybackMode::Once,
                ..default()
            }
        },
        properties: PlayableSound {
            id,
        }
    }
}
