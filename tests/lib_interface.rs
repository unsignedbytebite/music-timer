use music_timer::{music_time::MusicTime, music_timer_engine::MusicTimerState};

struct PerformanceState {
    current_time: MusicTime,
    count_bars: u8,
    count_beats: u8,
    count_beat_intervals: u8,
    music_events: Vec<String>,
}

impl PerformanceState {
    fn new() -> Self {
        PerformanceState {
            current_time: MusicTime::default(),
            count_bars: 0,
            count_beats: 0,
            count_beat_intervals: 0,
            music_events: Vec::new(),
        }
    }
}

impl MusicTimerState for PerformanceState {
    fn on_beat_interval(&mut self, now_time: &MusicTime) {
        self.current_time = now_time.clone();
        self.count_beat_intervals += 1;
        let event_string = format!("on_beat_interval: {:?}", now_time);
        println!("{}", event_string);
        self.music_events.push(event_string);
    }
    fn on_beat(&mut self, now_time: &MusicTime) {
        self.current_time = now_time.clone();
        self.count_beats += 1;
        let event_string = format!("on_beat: {:?}", now_time);
        println!("{}", event_string);
        self.music_events.push(event_string);
    }
    fn on_bar(&mut self, now_time: &MusicTime) {
        self.current_time = now_time.clone();
        self.count_bars += 1;
        let event_string = format!("on_bar: {:?}", now_time);
        println!("{}", event_string);
        self.music_events.push(event_string);
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
    assert_eq!(
        performer_state.music_events,
        vec![
            "on_beat_interval: MusicTime { bar: 1, beat: 1, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 1, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 1, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 1, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 1, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 1, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 1, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 1, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 1, beat: 2, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 2, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 2, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 2, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 2, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 2, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 2, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 2, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 2, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 1, beat: 3, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 3, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 3, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 3, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 3, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 3, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 3, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 3, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 1, beat: 3, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 2, beat: 1, beat_interval: 1 }",
            "on_bar: MusicTime { bar: 2, beat: 1, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 1, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 1, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 1, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 1, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 1, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 1, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 1, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 1, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 2, beat: 2, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 2, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 2, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 2, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 2, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 2, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 2, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 2, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 2, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 2, beat: 3, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 3, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 3, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 3, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 3, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 3, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 3, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 3, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 2, beat: 3, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 3, beat: 1, beat_interval: 1 }",
            "on_bar: MusicTime { bar: 3, beat: 1, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 1, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 1, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 1, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 1, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 1, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 1, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 1, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 1, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 3, beat: 2, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 2, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 2, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 2, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 2, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 2, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 2, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 2, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 2, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 3, beat: 3, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 3, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 3, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 3, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 3, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 3, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 3, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 3, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 3, beat: 3, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 4, beat: 1, beat_interval: 1 }",
            "on_bar: MusicTime { bar: 1, beat: 1, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 1, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 1, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 1, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 1, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 1, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 1, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 1, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 1, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 4, beat: 2, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 2, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 2, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 2, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 2, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 2, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 2, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 2, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 2, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 4, beat: 3, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 3, beat_interval: 1 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 3, beat_interval: 2 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 3, beat_interval: 3 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 3, beat_interval: 4 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 3, beat_interval: 5 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 3, beat_interval: 6 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 3, beat_interval: 7 }",
            "on_beat_interval: MusicTime { bar: 4, beat: 3, beat_interval: 8 }",
            "on_beat: MusicTime { bar: 4, beat: 4, beat_interval: 1 }",
            "on_bar: MusicTime { bar: 4, beat: 4, beat_interval: 1 }",
        ]
    );
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
