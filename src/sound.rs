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

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SoundID {
    TaskComplete,
    AlloyMachine,
    ParticleAccelerator,
    ComputerDenied,
    // TODO: Continue this list
}

#[derive(Resource)]
pub struct SoundHandles {
    task_complete: Handle<AudioSource>,
    alloy_machine: Handle<AudioSource>,
    particle_accelerator: Handle<AudioSource>,
    computer_denied: Handle<AudioSource>,
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
    commands.insert_resource(SoundHandles {
        task_complete: asset_server.load("sound/Task_Complete.ogg"),
        alloy_machine: asset_server.load("sound/AlloyMachine.ogg"),
        particle_accelerator: asset_server.load("sound/Particle_Accelerator.ogg"),
        computer_denied: asset_server.load("sound/Computer_Denied.ogg"),
    });
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
    mut commands: Commands,
    mut ev_sound: EventReader<PlaySoundEvent>,
    sound_handles: Res<SoundHandles>,
) {
    for ev in ev_sound.read() {
        let handle = get_handle_from_id(ev.0, &sound_handles);

        commands.spawn(AudioBundle {
            source: handle,
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            }
        });

        info!("Playing sound: {:?}", ev.0);
    }
}

fn get_handle_from_id(id: SoundID, handles: &Res<SoundHandles>) -> Handle<AudioSource> {
    match id {
        SoundID::TaskComplete => handles.task_complete.clone(),
        SoundID::AlloyMachine => handles.alloy_machine.clone(),
        SoundID::ParticleAccelerator => handles.particle_accelerator.clone(),
        SoundID::ComputerDenied => handles.computer_denied.clone(),
    }
}
