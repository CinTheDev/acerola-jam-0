use bevy::prelude::*;

#[derive(Resource)]
pub struct LoseTimer {
    pub timer: Timer,
}

#[derive(Event)]
pub struct TimerRunout();

#[derive(Event)]
pub struct TimerStop();

#[derive(Event)]
pub struct ResetTimer;

pub fn timer_runout(
    mut ev_timer_runout: EventReader<TimerRunout>,
) {
    for _ in ev_timer_runout.read() {
        info!("The game has been lost");

        // TODO: Do things to indicate game over
    }
}

pub fn timer_stop(
    mut ev_timerstop: EventReader<TimerStop>,
    mut lose_timer: ResMut<LoseTimer>,
) {
    for _ in ev_timerstop.read() {
        lose_timer.timer.pause();

        // TODO: Do things to indicate game won
    }
}

pub fn timer_reset(
    mut ev_resettimer: EventReader<ResetTimer>,
    mut lose_timer: ResMut<LoseTimer>,
) {
    for _ in ev_resettimer.read() {
        lose_timer.timer.reset();
        lose_timer.timer.unpause();
    }
}

pub fn setup_losetimer(mut commands: Commands) {
    let mut timer = Timer::from_seconds(300.0, TimerMode::Once);
    timer.pause();

    commands.insert_resource(LoseTimer {
        timer,
    });
}

pub fn check_losetimer(
    mut lose_timer: ResMut<LoseTimer>,
    time: Res<Time>,
    mut ev_timer_runout: EventWriter<TimerRunout>,
) {
    lose_timer.timer.tick(time.delta());

    if ! lose_timer.timer.finished() { return }

    // Executes every frame when timer done

    if ! lose_timer.timer.just_finished() { return }

    // Executes only once
    ev_timer_runout.send(TimerRunout());
}
