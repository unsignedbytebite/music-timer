#![allow(dead_code)]

use super::time_signature::TimeSignature;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
/// Data structure that holds music time and logic when advancing beats and beat intervals.
pub struct MusicTime {
    bar: u16,
    beat: u8,
    beat_interval: u8,
}

impl MusicTime {
    /// Create a new `MusicTime`.
    ///
    /// # Arguments
    /// * `bar` - The musical bar.
    /// * `beat` - The musical beat.
    /// * `beat` - The musical beat interval, the subdivisions of a beat.
    ///
    /// # Example
    /// ```
    /// let time = music_timer::music_time::MusicTime::new(1, 1, 1);
    /// ```
    pub fn new(bar: u16, beat: u8, beat_interval: u8) -> MusicTime {
        MusicTime {
            bar,
            beat,
            beat_interval,
        }
    }

    /// Get the bar number.
    pub fn get_bar(&self) -> u16 {
        self.bar
    }

    /// Get the beat number.
    pub fn get_beat(&self) -> u8 {
        self.beat
    }

    /// Get the interval between the beat.
    pub fn get_beat_interval(&self) -> u8 {
        self.beat_interval
    }

    /// Advance the beat by 1. The bar number will increase if the beat
    /// exceeds the `TimeSignature` numerator.
    ///
    /// # Arguments
    /// * `time_signature` - The time signature to constrain the music time by.
    ///
    /// # Example
    /// ```
    /// use music_timer::{time_signature::TimeSignature, music_time::MusicTime};
    /// let time_signature = TimeSignature::new(4, 4);
    /// let mut a = MusicTime::default();
    /// assert_eq!(a.get_bar() == 1 && a.get_beat() == 1, true);
    /// a.advance_beat(&time_signature);
    /// assert_eq!(a.get_bar() == 1 && a.get_beat() == 2, true);
    /// a.advance_beat(&time_signature);
    /// assert_eq!(a.get_bar() == 1 && a.get_beat() == 3, true);
    /// a.advance_beat(&time_signature);
    /// assert_eq!(a.get_bar() == 1 && a.get_beat() == 4, true);
    /// a.advance_beat(&time_signature);
    /// assert_eq!(a.get_bar() == 2 && a.get_beat() == 1, true);
    /// ```
    pub fn advance_beat(&mut self, time_signature: &TimeSignature) {
        if self.beat >= time_signature.get_numerator() {
            self.beat = 1;
            self.bar += 1;
        } else {
            self.beat += 1;
        }
    }

    /// Advance the beat interval by 1. The beat number will increase if the beat interval
    /// exceeds the the interval resolution of `8`. Then The bar number will increase if the beat
    /// exceeds the `TimeSignature` numerator.
    ///
    /// # Arguments
    /// * `time_signature` - The time signature to constrain the music time by.
    ///
    /// # Example
    /// ```
    /// use music_timer::{time_signature::TimeSignature, music_time::MusicTime};
    /// let time_signature = TimeSignature::new(4, 4);
    /// let mut a = MusicTime::default();
    /// assert_eq!(a, MusicTime::new(1, 1, 1));
    /// a.advance_beat_interval(&time_signature);
    /// assert_eq!(a, MusicTime::new(1, 1, 2));
    /// a.advance_beat_interval(&time_signature);
    /// a.advance_beat_interval(&time_signature);
    /// a.advance_beat_interval(&time_signature);
    /// a.advance_beat_interval(&time_signature);
    /// a.advance_beat_interval(&time_signature);
    /// a.advance_beat_interval(&time_signature);
    /// assert_eq!(a, MusicTime::new(1, 1, 8));
    /// a.advance_beat_interval(&time_signature);
    /// assert_eq!(a, MusicTime::new(1, 2, 1));
    /// ```
    pub fn advance_beat_interval(&mut self, time_signature: &TimeSignature) {
        const INTERVAL_RESOLUTION: u8 = 16;
        if self.beat_interval >= INTERVAL_RESOLUTION / 2 {
            self.beat_interval = 1;
            self.advance_beat(time_signature);
        } else {
            self.beat_interval += 1;
        }
    }
}

impl PartialEq for MusicTime {
    fn eq(&self, other: &Self) -> bool {
        self.bar == other.bar
            && self.beat == other.beat
            && self.beat_interval == other.beat_interval
    }
}

impl PartialOrd for MusicTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let other_time_sum = other.bar * 100 + other.beat as u16 * 10 + other.beat_interval as u16;
        let self_time_sum = self.bar * 100 + self.beat as u16 * 10 + self.beat_interval as u16;
        self_time_sum.partial_cmp(&other_time_sum)
    }
}

impl Default for MusicTime {
    /// Default is `MusicTime::new(1,1,1)`
    fn default() -> MusicTime {
        MusicTime {
            bar: 1,
            beat: 1,
            beat_interval: 1,
        }
    }
}

#[test]
fn test_order() {
    assert_eq!(MusicTime::new(1, 1, 1) < MusicTime::new(2, 1, 1), true);
    assert_eq!(MusicTime::new(2, 1, 1) > MusicTime::new(1, 1, 1), true);
    assert_eq!(MusicTime::new(1, 1, 1) <= MusicTime::new(1, 1, 1), true);
    assert_eq!(MusicTime::new(1, 1, 1) >= MusicTime::new(1, 1, 1), true);

    assert_eq!(MusicTime::new(1, 1, 1) < MusicTime::new(1, 2, 1), true);
    assert_eq!(MusicTime::new(1, 2, 1) > MusicTime::new(1, 1, 1), true);
    assert_eq!(MusicTime::new(1, 1, 1) < MusicTime::new(1, 2, 1), true);
    assert_eq!(MusicTime::new(1, 2, 1) > MusicTime::new(1, 1, 1), true);

    assert_eq!(MusicTime::new(1, 1, 1) < MusicTime::new(1, 1, 2), true);
    assert_eq!(MusicTime::new(1, 1, 2) > MusicTime::new(1, 1, 1), true);
    assert_eq!(MusicTime::new(1, 1, 1) < MusicTime::new(1, 1, 2), true);
    assert_eq!(MusicTime::new(1, 1, 2) > MusicTime::new(1, 1, 1), true);
}

#[test]
fn test_equality() {
    let a = MusicTime::new(1, 2, 3);
    let b = MusicTime::new(1, 2, 3);
    assert_eq!(a == b, true);

    let a = MusicTime::default();
    let b: MusicTime = Default::default();
    assert_eq!(a == b, true);
    assert_eq!(a.get_bar() == 1 && b.get_bar() == 1, true);

    let a = MusicTime::new(2, 1, 1);
    let b = MusicTime::new(2, 3, 2);
    assert_eq!(a == b, false);
}

#[test]
fn test_advance() {
    let time_signature = TimeSignature::new(4, 4);
    let mut a = MusicTime::default();
    assert_eq!(a.get_bar() == 1 && a.get_beat() == 1, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 1 && a.get_beat() == 2, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 1 && a.get_beat() == 3, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 1 && a.get_beat() == 4, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 2 && a.get_beat() == 1, true);

    let time_signature = TimeSignature::new(3, 4);
    let mut a = MusicTime::default();
    assert_eq!(a.get_bar() == 1 && a.get_beat() == 1, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 1 && a.get_beat() == 2, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 1 && a.get_beat() == 3, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 2 && a.get_beat() == 1, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 2 && a.get_beat() == 2, true);

    let time_signature = TimeSignature::new(1, 4);
    let mut a = MusicTime::default();
    assert_eq!(a.get_bar() == 1 && a.get_beat() == 1, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 2 && a.get_beat() == 1, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 3 && a.get_beat() == 1, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 4 && a.get_beat() == 1, true);
    a.advance_beat(&time_signature);
    assert_eq!(a.get_bar() == 5 && a.get_beat() == 1, true);
}

#[test]
fn test_advance_beat_interval() {
    let time_signature = TimeSignature::new(4, 4);
    let mut a = MusicTime::default();
    assert_eq!(a, MusicTime::new(1, 1, 1));
    a.advance_beat_interval(&time_signature);
    assert_eq!(a, MusicTime::new(1, 1, 2));
    a.advance_beat_interval(&time_signature);
    a.advance_beat_interval(&time_signature);
    a.advance_beat_interval(&time_signature);
    a.advance_beat_interval(&time_signature);
    a.advance_beat_interval(&time_signature);
    a.advance_beat_interval(&time_signature);
    assert_eq!(a, MusicTime::new(1, 1, 8));
    a.advance_beat_interval(&time_signature);
    assert_eq!(a, MusicTime::new(1, 2, 1));
}
