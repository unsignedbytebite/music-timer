#![allow(dead_code)]

//!
//! Data structure of numerator(upper) and denominator(lower) values of music time signature
//!

#[derive(Clone, Copy, Debug)]
/// Data structure of numerator(upper) and denominator(lower) values of music time signature
pub struct TimeSignature {
    numerator: u8,
    denominator: u8,
}

impl TimeSignature {
    /// Create new signature with the defined numerator(upper) and denominator(lower) values.
    ///
    /// # Arguments
    /// - `numerator` - The upper value of a time signature
    /// - `denominator` - The lower value of a time signature
    ///
    /// # Example
    /// ```
    /// let time_signature = music_timer::time_signature::TimeSignature::new(4, 4);
    /// ```
    pub fn new(numerator: u8, denominator: u8) -> TimeSignature {
        TimeSignature {
            numerator,
            denominator,
        }
    }

    /// Returns `true` if the time signature is valid. Current limitations of this crate
    /// recommend that denominator values should only be 2, 4, 8, 16 or 32. The
    /// numerator cannot be 0. It is your responsibility to create a valid `TimeSignature`.
    pub fn is_valid(&self) -> bool {
        let denominator = self.denominator;

        self.numerator > 0
            && (denominator == 2
                || denominator == 4
                || denominator == 8
                || denominator == 16
                || denominator == 32)
    }

    /// Get the top value of the time signature.
    pub fn get_numerator(&self) -> u8 {
        self.numerator
    }

    /// Get the bottom value of the time signature.
    pub fn get_denominator(&self) -> u8 {
        self.denominator
    }

    // Return the numerator and denominator as a tuple.
    pub fn as_tuple(&self) -> (u8, u8) {
        (self.numerator, self.denominator)
    }
}

impl PartialEq for TimeSignature {
    fn eq(&self, other: &TimeSignature) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

impl Default for TimeSignature {
    // Default is `TimeSignature::new(4,4)`.
    fn default() -> TimeSignature {
        TimeSignature {
            numerator: 4,
            denominator: 4,
        }
    }
}

#[test]
fn test_valid() {
    assert_eq!(TimeSignature::default().is_valid(), true);
    assert_eq!(TimeSignature::new(4, 5).is_valid(), false);
    assert_eq!(TimeSignature::new(0, 2).is_valid(), false);
}

#[test]
fn test_equality() {
    let a = TimeSignature::default();
    let b = TimeSignature::new(4, 4);
    assert_eq!(a, b);

    let a = TimeSignature::new(3, 4);
    let b = TimeSignature::new(4, 4);
    assert_ne!(a, b);
}
