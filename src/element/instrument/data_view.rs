use std::fmt::Debug;

use interlink::proto::Vector3;

use super::reading::Reading;

pub trait View: 'static + Debug {
    type Reading: Reading;
    type Raw;

    const DATA_VIEW: DataView;

    const TITLE: &'static str;

    fn ingest_reading(raw: Self::Raw) -> Self::Reading;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataView {
    Accelerometer,
    Magnetometer,
    Temperature,
}

impl DataView {
    pub fn from_view<V: View>() -> Self {
        V::DATA_VIEW
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Accelerometer;

impl View for Accelerometer {
    type Reading = Vector3<f64>;
    type Raw = Vector3<i32>;

    const DATA_VIEW: DataView = DataView::Accelerometer;

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

    const DATA_VIEW: DataView = DataView::Magnetometer;

    const TITLE: &'static str = "Magnetic Field";

    fn ingest_reading(raw: Self::Raw) -> Self::Reading {
        Vector3 {
            x: raw.x as f64 / 1000.0,
            y: raw.y as f64 / 1000.0,
            z: raw.z as f64 / 1000.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Temperature;

impl View for Temperature {
    type Reading = f64;
    type Raw = f32;

    const DATA_VIEW: DataView = DataView::Temperature;

    const TITLE: &'static str = "Temperature";

    fn ingest_reading(raw: Self::Raw) -> Self::Reading {
        raw as f64
    }
}
