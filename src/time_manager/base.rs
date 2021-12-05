use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum TimeBase {
    #[display(fmt = "Ground Control Time")]
    GroundControl,
    #[display(fmt = "Vehicle On Time")]
    VehicleTime,
    #[display(fmt = "Mission Time")]
    Mission,
}

impl TimeBase {
    pub const ALL: &'static [TimeBase] = &[
        TimeBase::GroundControl,
        TimeBase::VehicleTime,
        TimeBase::Mission,
    ];
}
