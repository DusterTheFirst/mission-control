//! Types representing moments in time

use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Type representing a moment in time relative to the vehicle's epoch.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VehicleTime {
    seconds: u32,
    subsec_micros: u32,
}

impl VehicleTime {
    /// A `VehicleTime` representing the instant the vehicle was initialized.
    pub const ZERO: Self = Self {
        seconds: 0,
        subsec_micros: 0,
    };

    /// Create a `VehicleTime`.
    #[cfg(feature = "vehicle")]
    pub fn new(seconds: u32, subsec_micros: u32) -> Self {
        Self {
            seconds,
            subsec_micros,
        }
    }

    /// Returns the total number of whole seconds represented by this `VehicleTime`.
    pub fn as_secs(&self) -> u32 {
        self.seconds
    }

    /// Returns the total number of whole milliseconds represented by this `VehicleTime`.
    pub fn as_millis(&self) -> u64 {
        self.seconds as u64 * 1_000 + self.subsec_micros as u64 / 1_000
    }

    ///  Returns the total number of whole microseconds represented by this `VehicleTime`.
    pub fn as_micros(&self) -> u64 {
        self.seconds as u64 * 1_000_000 + self.subsec_micros as u64
    }

    /// Returns the fractional part of this `VehicleTime`, in whole milliseconds.
    ///
    /// This method does **not** return the length of the duration when represented
    /// by milliseconds. The returned number always represents a fractional portion
    /// of a second (i.e., it is less than one thousand).
    pub fn subsec_millis(&self) -> u32 {
        self.subsec_micros / 1_000
    }

    /// Returns the fractional part of this `VehicleTime`, in whole microseconds.
    ///
    /// This method does **not** return the length of the duration when represented
    /// by milliseconds. The returned number always represents a fractional portion
    /// of a second (i.e., it is less than one million).
    pub fn subsec_micros(&self) -> u32 {
        self.subsec_micros
    }

    /// Returns the number of seconds represented by this `VehicleTime` as [`f32`].
    ///
    /// The returned value does include the fractional (nanosecond) part of the duration.
    pub fn as_secs_f32(&self) -> f32 {
        self.seconds as f32 + (self.subsec_micros as f32 / 1_000_000.0)
    }

    /// Returns the number of seconds represented by this `VehicleTime` as [`f64`].
    ///
    /// The returned value does include the fractional (nanosecond) part of the duration.
    pub fn as_secs_f64(&self) -> f64 {
        self.seconds as f64 + (self.subsec_micros as f64 / 1_000_000.0)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VehicleTime {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u32}.{=u32:06}", self.seconds, self.subsec_micros)
    }
}

impl Display for VehicleTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:06}", self.seconds, self.subsec_micros)
    }
}
