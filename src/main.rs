use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

mod player;
mod generate_colliders;
mod timer;
mod ui;
mod sound;

#[derive(Component)]
pub struct RaycastCursor;

#[derive(Component)]
pub struct Respawn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup,
            cursor_grab,
            generate_colliders::generate_colliders,
            player::tasks::setup,
            timer::setup_losetimer,
            ui::setup,
            sound::instance_music,
        ))
        .add_systems(Update, (
            (
                player::move_player,
                player::items::hold_item,
            ).chain(),
            player::raycast_items,
            player::items::update_item_pos,
            player::items::enable_itemdrops,
            player::items::pickup_item,
            player::items::drop_item,
            player::items::cancel_itemdrop,
        ))
        .add_systems(Update, (
            player::tasks::q_t_de::check_all_tasks_finished,
            player::tasks::q_t_de::check_final_button_input,
            player::tasks::q_t_de::check_dark_matter_finished,
            player::tasks::alloy_machine::check_if_finished,
            player::tasks::alloy_machine::check_alloy_finished,
            player::tasks::computer::check_activation,
            player::tasks::computer::input_from_keyboard,
            player::tasks::computer::task_success,
            player::tasks::particle_accelerator::check_coppertask,
            player::tasks::particle_accelerator::rotate_button::check_button_interaction,
            player::tasks::particle_accelerator::rotate_button::rotate_buttons,
            player::tasks::particle_accelerator::rotate_button::disable_buttons,
            player::tasks::particle_accelerator::check_buttons_solution,
        ))
        .add_systems(Update, (
            timer::check_losetimer,
            timer::timer_runout,
            timer::timer_stop,
            timer::timer_reset,
            ui::ui_timer::update_timer_ui,
            ui::ui_cursor::check_cursor,
            ui::ui_tasks::check_task_darkmatter,
            ui::ui_tasks::check_task_exoticalloy,
            ui::ui_tasks::check_task_alloyplacement,
            ui::ui_tasks::check_task_particleaccelerator,
            ui::ui_tasks::check_task_computer,
            ui::ui_tasks::check_task_finalbutton,
            ui::ui_ending::check_good_ending,
            ui::ui_ending::check_bad_ending,
            ui::ui_ending::fade_background,
            ui::ui_ending::swipe_text,
            ui::ui_ending::buttons::check_button_restart,
            ui::ui_ending::buttons::check_button_quit,
            ui::ui_ending::buttons::pressed_button_restart,
        ))
        .add_systems(Update, (
            sound::start_music,
        ))
        .add_event::<player::items::PickupEvent>()
        .add_event::<player::items::DropCancelEvent>()
        .add_event::<player::items::DropEvent>()
        .add_event::<player::tasks::alloy_machine::AlloyCreationFinshed>()
        .add_event::<player::tasks::alloy_machine::AlloyPlacementFinished>()
        .add_event::<player::tasks::particle_accelerator::ParticleAcceleratorFinished>()
        .add_event::<player::tasks::computer::SuccessEvent>()
        .add_event::<player::tasks::q_t_de::DarkMatterFinished>()
        .add_event::<player::tasks::q_t_de::FinalButtonActivated>()
        .add_event::<timer::TimerRunout>()
        .add_event::<timer::TimerStop>()
        .add_event::<timer::ResetTimer>()
        .add_event::<ui::ui_ending::buttons::RestartEvent>()
        .add_event::<sound::StartMusicEvent>()
        .add_event::<sound::StopMusicEvent>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut ev_startmusic: EventWriter<sound::StartMusicEvent>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("lab.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    player::items::spawn_items::spawn_all_items(&mut commands, &asset_server);
    player::instance_player(&mut commands);

    ev_startmusic.send(sound::StartMusicEvent);
}

fn cursor_grab(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = query.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}
