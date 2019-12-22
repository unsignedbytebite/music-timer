#![allow(dead_code)]

//!
//! Easy interface for changes in music time.
//!

use super::{
    music_time::MusicTime, music_time_counter::MusicTimeCounter, time_signature::TimeSignature,
};
use std::time::{Duration, SystemTime};

const STRING_PANIC_TIME_FLOW: &str = "Hello John Titor, you reversed time!";

/// This trait is used by `MusicTimerEngine` for callbacks in changes of music time.
/// Invoke it to make the most of the performance engine.
pub trait MusicTimerState {
    /// Called when the beat interval changes.
    ///
    /// # Arguments
    /// - `current_time` - The current time at which this callback has been triggered.
    fn on_beat_interval(&mut self, current_time: &MusicTime);

    /// Called when the beat changes.
    ///
    /// # Arguments
    /// - `current_time` - The current time at which this callback has been triggered.
    fn on_beat(&mut self, current_time: &MusicTime);

    /// Called when the bar changes
    ///
    /// # Arguments
    /// - `current_time` - The current time at which this callback has been triggered.
    fn on_bar(&mut self, current_time: &MusicTime);
}

/// The engine uses all of this crate's utilities to allow to use of a music
/// performance state system that triggers callbacks. Its aims are to allow
/// for an easy interface for changes in music time.
pub struct MusicTimerEngine {
    total_time: Duration,
    previous_time: Duration,
    start_time: SystemTime,
    event_trigger_time: Duration,
    music_counter: MusicTimeCounter,
    event_trigger_target: Duration,
    previous_music_time: MusicTime,
}

impl MusicTimerEngine {
    /// Create a new `MusicTimerEngine` with a `TimeSignature` and bpm.
    ///
    /// # Arguments
    /// * `time_signature` - The time signature for the performance.
    /// * `bpm` - The beats per minute used for the performance.
    ///
    /// # Example
    /// ```
    /// use music_timer::{music_timer_engine::MusicTimerEngine, time_signature::TimeSignature};
    /// let mut performer = MusicTimerEngine::new(TimeSignature::new(3, 4), 155.0);
    /// ```
    pub fn new(time_signature: TimeSignature, bpm: f32) -> Self {
        let music_counter = MusicTimeCounter::new(time_signature);
        let event_trigger_target = music_counter.beat_interval_target_frames(bpm);
        MusicTimerEngine {
            total_time: Duration::default(),
            previous_time: Duration::default(),
            start_time: SystemTime::now(),
            event_trigger_time: event_trigger_target,
            music_counter,
            event_trigger_target,
            previous_music_time: MusicTime::new(0, 0, 0),
        }
    }

    /// Pulse the engine. The time since the last pulse is used to evaluate if there is
    /// a change in music time. It is suggested to call this from a loop.
    ///
    /// # Arguments
    /// * `state` - The _trait_ `MusicTimerState` used for changes in music time callbacks.TimeSignature
    ///
    /// # Example
    /// ```
    /// use music_timer::{music_timer_engine::{MusicTimerEngine, MusicTimerState}, music_time::MusicTime};
    /// struct PerformanceState;
    /// impl MusicTimerState for PerformanceState {
    ///     fn on_beat_interval(&mut self, current_time: &MusicTime) {
    ///       // Do something on the beat interval
    ///     }
    ///     fn on_beat(&mut self, current_time: &MusicTime) {
    ///         // Do something on the beat
    ///     }
    ///     fn on_bar(&mut self, current_time: &MusicTime) {
    ///         // Do something on the bar
    ///     }
    /// }
    /// let mut performer_state = PerformanceState{};
    /// let mut performer = music_timer::create_performance_engine(3, 4, 155.0);
    /// performer.pulse(&mut performer_state);
    /// ```
    pub fn pulse<TimerState: MusicTimerState>(&mut self, state: &mut TimerState) {
        // Progress total time
        self.previous_time = self.total_time;
        // Time should never reverse else you're in trouble
        self.total_time = SystemTime::now()
            .duration_since(self.start_time)
            .expect(STRING_PANIC_TIME_FLOW);

        // Advance by delta
        let time_delta = self.total_time - self.previous_time;
        self.event_trigger_time += time_delta;

        // Check for an advance in the beat interval
        let is_beat_interval_advanced = self.event_trigger_time >= self.event_trigger_target;
        if is_beat_interval_advanced {
            let cached_current_time = self.music_counter.current_time().clone();
            state.on_beat_interval(&cached_current_time);

            let now_time = {
                self.music_counter.advance_beat_interval();
                self.music_counter.current_time()
            };

            let is_beat_changed = self.previous_music_time.get_beat() != now_time.get_beat();
            if is_beat_changed {
                state.on_beat(&now_time);
            }

            let is_bar_changed = self.previous_music_time.get_bar() == now_time.get_bar();
            if is_bar_changed {
                state.on_bar(&now_time);
            }

            self.previous_music_time = self.music_counter.current_time().clone();

            // Reset and calibrate drift - https://www.youtube.com/watch?v=Gm7lcZiLOus&t=30s
            let initial_d = self.event_trigger_time - self.event_trigger_target;
            self.event_trigger_time = initial_d;
        }
    }

    /// Gets the duration of time between beat intervals. Handy for sleeping threads.
    ///
    /// # Example
    /// ```
    /// let mut performer = music_timer::create_performance_engine(3, 4, 155.0);
    ///
    /// // We can set the delay to be half the trigger target. This will give
    /// // us a reasonable cycle speed with enough buffer to keep an accurate time.
    /// // This of course is not needed if the application is managing thread sleeping.
    /// // The shorter the sleep duration of the thread, the more accurate the
    /// // time triggering will be. In most cases setting the sleep to 60fps is recommended for
    /// // < 180bpm @ 4/4.
    /// let sleep_duration = performer.get_beat_interval_duration() / 2;
    /// println!("SLEEP_DURATION: {:?}", sleep_duration);
    /// std::thread::sleep(sleep_duration);
    /// ```
    ///
    pub fn get_beat_interval_duration(&self) -> Duration {
        self.event_trigger_target
    }

    /// Gets the current music time of the performance.
    pub fn get_current_time(&self) -> &MusicTime {
        self.music_counter.current_time()
    }

    /// Sets the current music time.
    ///
    /// # Arguments
    /// * `time` - The new music time to set.
    pub fn set_music_timer(&mut self, time: MusicTime) -> &mut Self {
        self.music_counter.set_current_time(time);
        self
    }
}
