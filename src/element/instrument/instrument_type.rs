use std::{convert::Infallible, fmt::Debug};

use interlink::proto::Vector3;

use super::reading::{EmptyReading, Reading};

pub trait InstrumentType: 'static + Debug {
    type Reading: Reading;
    type Raw;

    const TITLE: &'static str;

    fn reading_from_raw(raw: Self::Raw) -> Self::Reading;
}

#[derive(Debug, Clone, Copy)]
pub struct Placeholder;

impl InstrumentType for Placeholder {
    type Reading = EmptyReading;
    type Raw = Infallible;

    const TITLE: &'static str = "Placeholder";

    fn reading_from_raw(_raw: Self::Raw) -> Self::Reading {
        panic!("Attempted to create a reading in a placeholder instrument")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Accelerometer;

impl InstrumentType for Accelerometer {
    type Reading = Vector3<f64>;
    type Raw = Vector3<i32>;

    const TITLE: &'static str = "Acceleration";

    fn reading_from_raw(raw: Self::Raw) -> Self::Reading {
        Vector3 {
            x: (raw.x as f64 * 9.81) / 1000.0,
            y: (raw.y as f64 * 9.81) / 1000.0,
            z: (raw.z as f64 * 9.81) / 1000.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Magnetometer;

impl InstrumentType for Magnetometer {
    type Reading = Vector3<f64>;
    type Raw = Vector3<i32>;

    const TITLE: &'static str = "Magnetic Field";

    fn reading_from_raw(raw: Self::Raw) -> Self::Reading {
        Vector3 {
            x: raw.x as f64 / 1000.0,
            y: raw.y as f64 / 1000.0,
            z: raw.z as f64 / 1000.0,
        }
    }
}
