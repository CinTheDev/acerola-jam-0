use bevy::prelude::*;

#[derive(Resource)]
pub struct LoseTimer {
    timer: Timer,
}

pub fn setup_losetimer(mut commands: Commands) {
    commands.insert_resource(LoseTimer {
        timer: Timer::from_seconds(10.0, TimerMode::Once),
    });
}

pub fn check_losetimer(
    mut lose_timer: ResMut<LoseTimer>,
    time: Res<Time>,
) {
    lose_timer.timer.tick(time.delta());
    info!("Remaining time: {}", lose_timer.timer.remaining_secs());

    if ! lose_timer.timer.finished() { return }

    // Executes every frame when timer done

    if ! lose_timer.timer.just_finished() { return }

    // Executes only once
    info!("Timer has finished");
}
