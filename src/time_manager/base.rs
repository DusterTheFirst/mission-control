use std::fmt::{self, Display, Formatter};

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
