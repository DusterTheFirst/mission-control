use time::Duration;

use self::{
    base::TimeBase,
    unit::{LocalTime, VehicleTime},
};

#[derive(Debug, Clone)]
pub struct TimeManager {
    now: LocalTime,
    ground_control_on: LocalTime,

    last_packet: Option<LocalTime>,
    vehicle_time: Option<VehicleTime>,

    mission_start: Option<LocalTime>,
}

pub mod base;
pub mod unit;

impl TimeManager {
    pub fn setup() -> Self {
        let now = LocalTime::now();

        Self {
            now,
            // Artificially sync the local time with the ground control time
            ground_control_on: now.quantize(),
            vehicle_time: None,
            mission_start: None,
            last_packet: None,
        }
    }

    pub fn elapsed(&self, time_base: TimeBase) -> Duration {
        match time_base {
            TimeBase::GroundControl => self.now.duration_since(&self.ground_control_on),
            TimeBase::VehicleTime => self
                .vehicle_time
                .as_ref()
                .map(VehicleTime::as_duration)
                .unwrap_or(Duration::ZERO),
            TimeBase::Mission => self
                .mission_start
                .as_ref()
                .map(|mission_start| self.now.duration_since(mission_start))
                .unwrap_or(Duration::ZERO),
        }
    }

    pub fn rebase_vehicle_time(&self, vehicle_time: VehicleTime, time_base: TimeBase) -> Duration {
        match time_base {
            TimeBase::GroundControl => vehicle_time
                .received()
                .duration_since(&self.ground_control_on),
            TimeBase::VehicleTime => vehicle_time.as_duration(),
            TimeBase::Mission => self
                .mission_start
                .as_ref()
                .map(|mission_start| vehicle_time.received().duration_since(mission_start))
                .unwrap_or(Duration::ZERO),
        }
    }

    pub fn update_now(&mut self) {
        self.now = LocalTime::now();
    }

    pub const fn now(&self) -> LocalTime {
        self.now
    }

    pub fn packet_received(&mut self, vehicle_time: VehicleTime) {
        self.last_packet = Some(LocalTime::now());
        self.vehicle_time = Some(vehicle_time);
    }

    pub fn duration_since_last_packet(&self) -> Option<Duration> {
        self.last_packet
            .as_ref()
            .map(|last_packet| self.now.duration_since(last_packet))
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
