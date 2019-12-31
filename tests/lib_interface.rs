use music_timer::{music_time::MusicTime, music_timer_engine::MusicTimerState};

struct PerformanceState {
    current_time: MusicTime,
    count_bars: u8,
    count_beats: u8,
    count_beat_intervals: u8,
}

impl PerformanceState {
    fn new() -> Self {
        PerformanceState {
            current_time: MusicTime::default(),
            count_bars: 0,
            count_beats: 0,
            count_beat_intervals: 0,
        }
    }
}

impl MusicTimerState for PerformanceState {
    fn on_beat_interval(&mut self, now_time: &MusicTime) {
        self.current_time = now_time.clone();
        self.count_beat_intervals += 1;
    }
    fn on_beat(&mut self, now_time: &MusicTime) {
        self.current_time = now_time.clone();
        self.count_beats += 1;
    }
    fn on_bar(&mut self, now_time: &MusicTime) {
        self.current_time = now_time.clone();
        self.count_bars += 1;
    }
}

fn performance_runner(
    time_bpm: (u8, u8, u8),
    performer_state: &mut PerformanceState,
    end_time: &MusicTime,
) {
    use std::thread;

    let mut performer =
        music_timer::create_performance_engine(time_bpm.0, time_bpm.1, time_bpm.2 as f32);
    let sleep_duration = performer.get_beat_interval_duration() / 2;
    while performer_state.current_time < *end_time {
        performer.pulse(performer_state);
        thread::sleep(sleep_duration);
    }
}

#[test]
fn test_performance_example() {
    let end_time = MusicTime::new(4, 3, 8);
    let mut performer_state = PerformanceState::new();
    performance_runner((3, 4, 155), &mut performer_state, &end_time);

    assert_eq!(performer_state.current_time, end_time);
    assert_eq!(performer_state.count_bars, 4);
    assert_eq!(performer_state.count_beats, 3 * 4);
    assert_eq!(performer_state.count_beat_intervals, 8 * 3 * 4);
}
#[test]
fn test_standard() {
    let end_time = MusicTime::new(4, 4, 8);
    let mut performer_state = PerformanceState::new();
    performance_runner((4, 4, 120), &mut performer_state, &end_time);

    assert_eq!(performer_state.current_time, end_time);
    assert_eq!(performer_state.count_bars, 4);
    assert_eq!(performer_state.count_beats, 4 * 4);
    assert_eq!(performer_state.count_beat_intervals, 8 * 4 * 4);
}
#[test]
fn test_odd() {
    let end_time = MusicTime::new(4, 7, 8);
    let mut performer_state = PerformanceState::new();
    performance_runner((7, 8, 120), &mut performer_state, &end_time);

    assert_eq!(performer_state.current_time, end_time);
    assert_eq!(performer_state.count_bars, 4);
    assert_eq!(performer_state.count_beats, 7 * 4);
    assert_eq!(performer_state.count_beat_intervals, 8 * 7 * 4);
}
