use music_timer::{music_time::MusicTime, music_timer_engine::MusicTimerState};

struct PerformanceState {
    is_playing: bool,
    performance_end: MusicTime,
    events: Vec<MusicTime>,
    event_head: usize,
}

impl MusicTimerState for PerformanceState {
    fn on_beat_interval(&mut self, current_time: &MusicTime) {
        let event_triggered =
            self.event_head < self.events.len() && *current_time == self.events[self.event_head];

        // Advance the event head
        if event_triggered {
            self.event_head += 1;
        }

        // Print out esoteric data
        println!(
            "{:02}.{}.{} = {}",
            current_time.get_bar(),
            current_time.get_beat(),
            current_time.get_beat_interval(),
            event_triggered
        );

        // Check to end the performance
        self.is_playing = *current_time < self.performance_end;
    }
    fn on_beat(&mut self, _current_time: &MusicTime) {
        // Do something on the beat
    }
    fn on_bar(&mut self, _current_time: &MusicTime) {
        //Do something on the bar
    }
}
fn main() {
    use std::thread;

    // Create the performer_state with bunch of events
    let mut performer_state = PerformanceState {
        is_playing: true,
        performance_end: MusicTime::new(4, 3, 8),
        events: vec![
            MusicTime::new(1, 1, 1),
            MusicTime::new(2, 2, 5),
            MusicTime::new(4, 3, 8),
        ],
        event_head: 0,
    };

    // Run our main loop
    let mut performer = music_timer::create_performance_engine(3, 4, 155.0);

    // We can set the delay to be half the trigger target. This will give
    // us a resonable cycle speed with enough buffer to keep an accurate time.
    // This of course is not needed if the application is managing thread sleeping.
    // The shorter the sleep duration of the thread, the more accurate the
    // time triggering will be. In most cases setting the sleep to 60fps is recommended for
    // < 180bpm @ 4/4.
    let sleep_duration = performer.get_beat_interval_duration() / 2;
    println!("SLEEP_DURATION: {:?}", sleep_duration);

    while performer_state.is_playing {
        // Pass in our performance state to trigger our on event callback functions
        performer.pulse(&mut performer_state);
        thread::sleep(sleep_duration);
    }
}
