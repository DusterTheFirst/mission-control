use std::sync::Once;

use time::{macros::format_description, Duration, OffsetDateTime, Time};
use tracing::error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalTime {
    date_time: OffsetDateTime,
}

impl LocalTime {
    /// Get the current date time in the local timezone or in UTC
    /// if the local timezone could not be determined
    pub fn now() -> Self {
        static ONCE: Once = Once::new();

        Self {
            date_time: OffsetDateTime::now_local().unwrap_or_else(|error| {
                ONCE.call_once(|| {
                    error!(%error, "Unable to determine time-zone, using UTC for local time");
                });

                OffsetDateTime::now_utc()
            }),
        }
    }

    /// Quantize an `LocalTime` to the nearest second
    ///
    /// This is useful to make sure times on the display will update together
    pub fn quantize(&self) -> Self {
        Self {
            date_time: self.date_time.replace_time(
                Time::from_hms(
                    self.date_time.hour(),
                    self.date_time.minute(),
                    self.date_time.second(),
                )
                .expect("LocalTime contained an invalid time"),
            ),
        }
    }

    pub fn duration_since(&self, local_time: &Self) -> Duration {
        self.date_time - local_time.date_time
    }

    pub const fn is_utc(&self) -> bool {
        self.date_time.offset().is_utc()
    }

    pub fn format(&self) -> String {
        self.date_time
            .format(format_description!(
                "[hour repr:24]:[minute]:[second].[subsecond digits:1]"
            ))
            .expect("unable to format date time")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VehicleTime {
    vehicle_time: interlink::vehicle_time::VehicleTime,
    received_time: LocalTime,
}

impl VehicleTime {
    pub fn from_packet(vehicle_time: interlink::vehicle_time::VehicleTime) -> Self {
        Self {
            received_time: LocalTime::now(),
            vehicle_time,
        }
    }

    pub const fn as_duration(&self) -> Duration {
        Duration::new(
            self.vehicle_time.as_secs() as i64,
            (self.vehicle_time.subsec_micros() * 1_000) as i32,
        )
    }

    pub const fn received(&self) -> LocalTime {
        self.received_time
    }
}
