use std::{
    any::{type_name, TypeId},
    fmt::Debug,
};

use interlink::proto::Vector3;

use super::reading::Reading;

pub trait View: 'static + Debug {
    type Reading: Reading;
    type Raw;

    const TITLE: &'static str;

    fn ingest_reading(raw: Self::Raw) -> Self::Reading;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataView {
    Accelerometer,
    Magnetometer,
}

impl DataView {
    pub fn from_view<C: View>() -> Self {
        match TypeId::of::<C>() {
            id if id == TypeId::of::<Accelerometer>() => Self::Accelerometer,
            id if id == TypeId::of::<Magnetometer>() => Self::Magnetometer,
            _ => panic!("{} is not a known DataView", type_name::<C>()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Accelerometer;

impl View for Accelerometer {
    type Reading = Vector3<f64>;
    type Raw = Vector3<i32>;

    const TITLE: &'static str = "Acceleration";

    fn ingest_reading(raw: Self::Raw) -> Self::Reading {
        Vector3 {
            x: (raw.x as f64 * 9.81) / 1000.0,
            y: (raw.y as f64 * 9.81) / 1000.0,
            z: (raw.z as f64 * 9.81) / 1000.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Magnetometer;

impl View for Magnetometer {
    type Reading = Vector3<f64>;
    type Raw = Vector3<i32>;

    const TITLE: &'static str = "Magnetic Field";

    fn ingest_reading(raw: Self::Raw) -> Self::Reading {
        Vector3 {
            x: raw.x as f64 / 1000.0,
            y: raw.y as f64 / 1000.0,
            z: raw.z as f64 / 1000.0,
        }
    }
}
