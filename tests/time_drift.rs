use music_timer::{music_time::MusicTime, music_timer_engine::MusicTimerState};
use std::time::{Duration, SystemTime};

struct PerformanceState {
    start_time: SystemTime,
    previous_time: Duration,
    total_time: Duration,
    target_sleep_duration: Duration,
    // event_trigger_time: Duration,
}

impl PerformanceState {
    fn new(target_sleep_duration: Duration) -> Self {
        PerformanceState {
            start_time: SystemTime::now(),
            previous_time: Duration::default(),
            total_time: Duration::default(),
            target_sleep_duration,
        }
    }
}

impl MusicTimerState for PerformanceState {
    fn on_beat_interval(&mut self, now_time: &MusicTime) {
        self.previous_time = self.total_time;
        self.total_time = SystemTime::now()
            .duration_since(self.start_time)
            .expect("Time flow reversed");

        let time_delta = self.total_time - self.previous_time;

        println!(
            "|{}.{}.{}| {:?} -> {:?} <> {:?} ",
            now_time.get_bar(),
            now_time.get_beat(),
            now_time.get_beat_interval(),
            self.total_time,
            time_delta,
            self.target_sleep_duration
        );
    }
    fn on_beat(&mut self, _now_time: &MusicTime) {}
    fn on_bar(&mut self, _now_time: &MusicTime) {}
}

#[test]
fn test_drift_60bpm() {
    use std::thread;

    let mut performer = music_timer::create_performance_engine(4, 4, 60.0);
    let mut performer_state = PerformanceState::new(performer.get_beat_interval_duration());
    let end_time = MusicTime::new(3, 1, 1);

    while performer.get_current_time() < &end_time {
        performer.pulse(&mut performer_state);
        thread::sleep(Duration::from_millis(1000 / 60));
    }

    let calculated_play_back_duration = performer.get_beat_interval_duration() * 8 * 4 * 2;
    println!("calculated_play_back_duration: {:?}", calculated_play_back_duration);
    let time_error_bound = Duration::from_millis(50);
    let lower_bound = calculated_play_back_duration - time_error_bound;
    let upper_bound = calculated_play_back_duration + time_error_bound;

    assert!(
        performer_state.total_time > lower_bound,
        "Time is too slow"
    );
    assert!(performer_state.total_time < upper_bound, "Time was too fast");
}


#[test]
fn test_drift_140bpm() {
    use std::thread;

    let mut performer = music_timer::create_performance_engine(3, 4, 140.0);
    let mut performer_state = PerformanceState::new(performer.get_beat_interval_duration());
    let end_time = MusicTime::new(3, 1, 1);

    while performer.get_current_time() < &end_time {
        performer.pulse(&mut performer_state);
        thread::sleep(Duration::from_millis(1000 / 60));
    }

    let calculated_play_back_duration = performer.get_beat_interval_duration() * 8 * 3 * 2;
    println!("calculated_play_back_duration: {:?}", calculated_play_back_duration);
    let time_error_bound = Duration::from_millis(50);
    let lower_bound = calculated_play_back_duration - time_error_bound;
    let upper_bound = calculated_play_back_duration + time_error_bound;

    assert!(
        performer_state.total_time > lower_bound,
        "Time is too slow"
    );
    assert!(performer_state.total_time < upper_bound, "Time was too fast");
}
