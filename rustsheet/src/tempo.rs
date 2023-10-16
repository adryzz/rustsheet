use std::{fmt, num::NonZeroU8};

use thiserror::Error;

use crate::math;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSignature {
    pub num: NonZeroU8,
    pub den: NonZeroU8,
}

impl TimeSignature {
    /// Attempts to create a new TimeSignature, or fails if either the numerator or denominator are zero.
    /// ```
    /// use rustsheet::tempo::TimeSignature;
    ///
    /// fn main() {
    ///     // Create a 1/4 time signature
    ///     let ts = TimeSignature::new(1, 4);
    ///     assert_eq!(ts.is_err(), false);
    /// }
    /// ```
    /// This will return an error instead.
    /// ```
    /// use rustsheet::tempo::TimeSignature;
    ///
    /// fn main() {
    ///     // Attempt to create a 0/4 time signature
    ///     let ts = TimeSignature::new(0, 4);
    ///     assert_eq!(ts.is_err(), true);
    /// }
    /// ```
    pub fn new(num: u8, den: u8) -> Result<Self, TimeSignatureError> {
        if num == 0 || den == 0 {
            return Err(TimeSignatureError::InvalidRatio(num, den));
        }

        Ok(Self {
            num: NonZeroU8::new(num).unwrap(),
            den: NonZeroU8::new(den).unwrap(),
        })
    }

    /// Simplifies the ratio, if possible
    ///
    /// ```
    /// use rustsheet::tempo::TimeSignature;
    ///
    /// fn main() {
    ///     // Create a 2/4 and a 1/2 time signature
    ///     let ts1 = TimeSignature::new(2, 4).unwrap();
    ///     let ts2 = TimeSignature::new(1, 2).unwrap();
    ///
    ///     // Simplifying the first time signature results in the second time signature
    ///     assert_eq!(ts1.simplify(), ts2);
    /// }
    /// ```
    /// This can't be simplified any further.
    /// ```
    /// use rustsheet::tempo::TimeSignature;
    ///
    /// fn main() {
    ///     // Create a 5/12 time signatuee
    ///     let ts = TimeSignature::new(5, 12).unwrap();
    ///
    ///     // You can't simplify the time signature any further
    ///     assert_eq!(ts.simplify(), ts);
    /// }
    /// ```
    pub fn simplify(&self) -> Self {
        let gcd = math::gcd(self.num.get(), self.den.get());

        let simplified_num = NonZeroU8::new(self.num.get() / gcd).unwrap();
        let simplified_den = NonZeroU8::new(self.den.get() / gcd).unwrap();

        TimeSignature {
            num: simplified_num,
            den: simplified_den,
        }
    }
}

impl fmt::Display for TimeSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl Default for TimeSignature {
    fn default() -> Self {
        // SAFETY: four is always not equal to zero
        unsafe {
            Self {
                num: NonZeroU8::new_unchecked(4),
                den: NonZeroU8::new_unchecked(4),
            }
        }
    }
}

pub trait TimeSignatureralElement {
    fn get_tempo(&self) -> TimeSignature;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoteSize {
    pub unit: NoteSizeUnit,
    pub modifiers: Option<NoteSizeModifiers>,
}

impl From<NoteSizeUnit> for NoteSize {
    fn from(value: NoteSizeUnit) -> Self {
        NoteSize {
            unit: value,
            modifiers: None,
        }
    }
}

/// Expressed as
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteSizeUnit {
    /// 8/1
    Large = 11,
    /// 4/1
    Long = 10,
    /// 2/1
    Breve = 9,
    /// 1/1
    #[default]
    SemiBreve = 8,
    /// 1/2
    Minim = 7,
    /// 1/4
    Crotchet = 6,
    /// 1/8
    Quaver = 5,
    /// 1/16
    SemiQuaver = 4,
    /// 1/32
    DemiSemiQuaver = 3,
    /// 1/64
    HemiDemiSemiQuaver = 2,
    /// 1/128
    SemiHemiDemiSemiQuaver = 1,
    /// 1/256
    DemiSemiHemiDemiSemiQuaver = 0,
}

#[derive(Debug, Clone, Copy)]
pub enum NoteSizeModifiers {
    Dot,
    DoubleDot,
    TripleDot,
    Crown,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum TimeSignatureError {
    #[error("Invalid ratio: {0}/{1}")]
    InvalidRatio(u8, u8),
}
