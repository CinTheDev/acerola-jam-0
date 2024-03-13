use bevy::prelude::*;
use bevy::audio::PlaybackMode;
use rand;

use crate::ui::ui_ending::buttons::RestartEvent;

#[derive(Event)]
pub struct StartMusicEvent;

#[derive(Event)]
pub struct PlaySoundEvent(pub SoundID);

#[derive(Event)]
pub struct PlaySpatialSoundEvent(pub SoundID, pub Vec3);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SoundID {
    Music,
    TaskComplete,
    AlloyMachine,
    ParticleAccelerator,
    ComputerDenied,
    Keyboard,
    ItemGrab,
}

#[derive(Resource)]
pub struct SoundHandles {
    music: Handle<AudioSource>,
    task_complete: Handle<AudioSource>,
    alloy_machine: Handle<AudioSource>,
    particle_accelerator: Handle<AudioSource>,
    computer_denied: Handle<AudioSource>,
    keyboard: [Handle<AudioSource>; 5],
    item_grab: [Handle<AudioSource>; 5],
}

#[derive(Resource)]
pub struct SoundFadeout {
    fade_timer: Timer,
}

pub fn load_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_sound: EventWriter<PlaySoundEvent>
) {
    commands.insert_resource(SoundHandles {
        music: asset_server.load("sound/Vacuum_Decay.ogg"),
        task_complete: asset_server.load("sound/Task_Complete.ogg"),
        alloy_machine: asset_server.load("sound/AlloyMachine.ogg"),
        particle_accelerator: asset_server.load("sound/Particle_Accelerator.ogg"),
        computer_denied: asset_server.load("sound/Computer_Denied.ogg"),

        keyboard: core::array::from_fn(|i| {
            asset_server.load(format!("sound/keyboard/Keyboard{}.ogg", i+1))
        }),

        item_grab: core::array::from_fn(|i| {
            asset_server.load(format!("sound/item/grab{}.ogg", i+1))
        }),
    });

    let mut fade_timer = Timer::from_seconds(3.0, TimerMode::Once);
    fade_timer.pause();

    commands.insert_resource(SoundFadeout {
        fade_timer,
    });

    ev_sound.send(PlaySoundEvent(SoundID::Music));
}

// For handling sound/music fadeout and restart on replay
pub fn handle_sound_restart(
    q_sound: Query<&AudioSink>,
    mut fade_timer: ResMut<SoundFadeout>,
    time: Res<Time>,
) {
    fade_timer.fade_timer.tick(time.delta());

    if fade_timer.fade_timer.paused() { return }

    let vol = fade_timer.fade_timer.percent_left();

    for sound in q_sound.iter() {
        sound.set_volume(vol);
    }

    if ! fade_timer.fade_timer.finished() { return }

    // Despawn all sounds immeadiatly if fadeout has finished
    for sound in q_sound.iter() {
        sound.stop();
    }

    fade_timer.fade_timer.reset();
    fade_timer.fade_timer.pause();
}

pub fn stop_music(
    mut ev_stop: EventReader<RestartEvent>,
    mut fade_timer: ResMut<SoundFadeout>,
) {
    for _ in ev_stop.read() {
        fade_timer.fade_timer.unpause();
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

pub fn play_spatial_sound(
    mut commands: Commands,
    mut ev_sound: EventReader<PlaySpatialSoundEvent>,
    sound_handles: Res<SoundHandles>,
) {
    for ev in ev_sound.read() {
        let handle = get_handle_from_id(ev.0, &sound_handles);
        let position = ev.1;

        commands.spawn((
            AudioBundle {
                source: handle,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    spatial: true,
                    ..default()
                }
            },
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..default()
            }
        ));
    }
}

fn get_handle_from_id(id: SoundID, handles: &Res<SoundHandles>) -> Handle<AudioSource> {
    match id {
        SoundID::Music => handles.music.clone(),
        SoundID::TaskComplete => handles.task_complete.clone(),
        SoundID::AlloyMachine => handles.alloy_machine.clone(),
        SoundID::ParticleAccelerator => handles.particle_accelerator.clone(),
        SoundID::ComputerDenied => handles.computer_denied.clone(),
        SoundID::Keyboard => handles.keyboard[get_random_index(5)].clone(),
        SoundID::ItemGrab => handles.item_grab[get_random_index(5)].clone(),
    }
}

fn get_random_index(upper_bound: usize) -> usize {
    rand::random::<usize>() % upper_bound
}
