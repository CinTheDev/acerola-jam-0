use bevy::prelude::*;

#[derive(Resource)]
pub struct LoseTimer {
    timer: Timer,
}

#[derive(Event)]
pub struct TimerRunout();

#[derive(Event)]
pub struct TimerStop();

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

pub fn setup_losetimer(mut commands: Commands) {
    commands.insert_resource(LoseTimer {
        timer: Timer::from_seconds(100.0, TimerMode::Once),
    });
}

pub fn check_losetimer(
    mut lose_timer: ResMut<LoseTimer>,
    time: Res<Time>,
    mut ev_timer_runout: EventWriter<TimerRunout>,
) {
    lose_timer.timer.tick(time.delta());
    info!("Remaining time: {}", lose_timer.timer.remaining_secs());

    if ! lose_timer.timer.finished() { return }

    // Executes every frame when timer done

    if ! lose_timer.timer.just_finished() { return }

    // Executes only once
    ev_timer_runout.send(TimerRunout());
}
