#![allow(dead_code)]

//!
//! Data structures that handles advancing music time within a time signature.
//!

use super::{music_time::MusicTime, time_signature::TimeSignature};
use std::time::Duration;

#[derive(Debug)]
/// Data structure that handles advancing music time within a time signature.
pub struct MusicTimeCounter {
    current_time: MusicTime,
    time_signature: TimeSignature,
}

impl MusicTimeCounter {
    /// Create a new `MusicTimeCounter` with a `TimeSignature`.
    /// # Arguments
    ///
    /// * `time_signature` - The `TimeSignature` the `MusicTimeCounter` is constrained by.
    ///
    /// # Example
    /// ```
    /// use music_timer::{time_signature::TimeSignature, music_time_counter::MusicTimeCounter};
    /// let timer = MusicTimeCounter::new(TimeSignature::new(3, 4));
    /// ```
    pub fn new(time_signature: TimeSignature) -> Self {
        MusicTimeCounter {
            current_time: MusicTime::new(0, time_signature.get_numerator(), 8),
            time_signature,
        }
    }

    /// Gets the current time of the counter.
    pub fn current_time(&self) -> &MusicTime {
        &self.current_time
    }

    /// Advance this counter by 1 beat.
    pub fn advance_beat(&mut self) {
        self.current_time.advance_beat(&self.time_signature);
    }

    /// Advance this counter by 1 beat interval.
    pub fn advance_beat_interval(&mut self) {
        self.current_time
            .advance_beat_interval(&self.time_signature);
    }

    /// Gets the time duration between beats.
    /// # Arguments
    ///
    /// * `bpm` - Beats per minute
    pub fn beat_target_frames(&self, bpm: f32) -> Duration {
        let seconds_per_beat = 60.0 / bpm;
        let beat_pulse_speed = seconds_per_beat * 1000000000.0;
        Duration::from_nanos(beat_pulse_speed as u64)
    }

    /// Gets the time duration between beat intervals.
    /// # Arguments
    ///
    /// * `bpm` - Beats per minute.
    pub fn beat_interval_target_frames(&self, bpm: f32) -> Duration {
        const INTERVAL_RESOLUTION: f32 = 16.0 / 2.0;
        let seconds_per_beat_interval = (60.0 / bpm) / INTERVAL_RESOLUTION;
        let beat_interval_pulse_speed = seconds_per_beat_interval * 1000000000.0;
        Duration::from_nanos(beat_interval_pulse_speed as u64)
    }
}

impl Default for MusicTimeCounter {
    /// Default `MusicTimeCounter` is created with a default `TimeSignature`.
    fn default() -> Self {
        MusicTimeCounter::new(TimeSignature::default())
    }
}

#[test]
fn test_beat_target_frames() {
    let timer = MusicTimeCounter::new(TimeSignature::new(4, 4));
    let duration = timer.beat_target_frames(80.0);
    let expected = Duration::from_millis(750);
    assert_eq!(duration, expected);

    let timer = MusicTimeCounter::new(TimeSignature::new(4, 4));
    let duration = timer.beat_target_frames(60.0);
    let expected = Duration::from_secs(1);
    assert_eq!(duration, expected);
}

#[test]
fn test_beat_interval_target_frames() {
    let timer = MusicTimeCounter::new(TimeSignature::new(4, 4));
    let duration = timer.beat_interval_target_frames(60.0);
    let expected = Duration::from_millis(125);
    assert_eq!(duration, expected);
}
