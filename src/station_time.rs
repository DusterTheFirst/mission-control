use std::{
    fmt::{self, Display, Formatter},
    sync::Once,
};

use interlink::vehicle_time::VehicleTime;
use time::{Duration, OffsetDateTime, Time};
use tracing::{error, warn};

#[derive(Debug, Clone, Copy)]
pub struct StationTime {
    now: OffsetDateTime,
    ground_control_on: OffsetDateTime,
    vehicle_time: Option<VehicleTime>,
    mission_start: Option<OffsetDateTime>,
    last_packet: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeBase {
    GroundControl,
    VehicleTime,
    Mission,
}

impl TimeBase {
    pub const ALL: &'static [TimeBase] = &[
        TimeBase::GroundControl,
        TimeBase::VehicleTime,
        TimeBase::Mission,
    ];
}

impl Display for TimeBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TimeBase::GroundControl => write!(f, "Ground Control Time"),
            TimeBase::VehicleTime => write!(f, "Vehicle On Time"),
            TimeBase::Mission => write!(f, "Mission Time"),
        }
    }
}

impl StationTime {
    /// Get the current date time in the local timezone or in UTC
    /// if the local timezone could not be determined
    fn current_time() -> OffsetDateTime {
        static ONCE: Once = Once::new();

        OffsetDateTime::now_local().unwrap_or_else(|e| {
            ONCE.call_once(|| {
                error!("{}", e);
                warn!("Using UTC for local time");
            });

            OffsetDateTime::now_utc()
        })
    }

    /// Quantize an `OffsetDateTime` to the nearest second
    ///
    /// This is useful to make sure times on the display will update together
    fn quantize(date_time: OffsetDateTime) -> OffsetDateTime {
        date_time.replace_time(
            Time::from_hms(date_time.hour(), date_time.minute(), date_time.second()).unwrap(),
        )
    }

    pub fn setup() -> Self {
        let now = Self::current_time();

        Self {
            now,
            // Artificially sync the local time with the ground control time
            ground_control_on: Self::quantize(now),
            vehicle_time: None,
            mission_start: None,
            last_packet: None,
        }
    }

    pub fn get_elapsed(&self, time_base: TimeBase) -> Duration {
        match time_base {
            TimeBase::GroundControl => self.now - self.ground_control_on,
            TimeBase::VehicleTime => self
                .vehicle_time
                .map(|vehicle_time| {
                    Duration::new(
                        vehicle_time.as_secs() as i64,
                        (vehicle_time.subsec_micros() * 1_000) as i32,
                    )
                })
                .unwrap_or(Duration::ZERO),
            TimeBase::Mission => self
                .mission_start
                .map(|mission_start| self.now - mission_start)
                .unwrap_or(Duration::ZERO),
        }
    }

    pub fn update_now(&mut self) {
        self.now = Self::current_time();
    }

    pub fn now(&self) -> OffsetDateTime {
        self.now
    }

    pub fn packet_received(&mut self, vehicle_time: VehicleTime) {
        self.last_packet = Some(Self::current_time());
        self.vehicle_time = Some(vehicle_time);
    }

    pub fn time_since_last_packet(&self) -> Option<Duration> {
        self.last_packet.map(|last_packet| self.now - last_packet)
    }
}

pub fn format_duration(duration: time::Duration) -> String {
    format!(
        "{:02}:{:02}:{:02}.{:01}",
        duration.whole_hours(),
        duration.whole_minutes() % 60,
        duration.whole_seconds() % 60,
        duration.subsec_milliseconds() / 100
    )
}
