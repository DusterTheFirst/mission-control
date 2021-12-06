use std::{fmt::Debug, iter, ops::Range};

use interlink::proto::Vector3;
use plotters::style::{ShapeStyle, BLUE, GREEN, RED};

pub type ReadingValuesIter<S> =
    iter::Map<iter::Zip<Range<usize>, iter::Repeat<S>>, fn((usize, S)) -> f64>;

pub trait Reading: Debug + Copy + Sized {
    const VALUES: usize;

    fn value(&self, index: usize) -> f64;

    // TODO: use
    fn label(index: usize) -> &'static str;
    fn style(index: usize) -> ShapeStyle;

    fn values(&self) -> ReadingValuesIter<Self> {
        (0..Self::VALUES)
            .zip(iter::repeat(*self))
            .map(|(index, datum)| datum.value(index))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EmptyReading {}

impl Reading for EmptyReading {
    const VALUES: usize = 0;

    fn value(&self, _: usize) -> f64 {
        panic!("attempted to access value from EmptyReading")
    }

    fn style(_: usize) -> ShapeStyle {
        panic!("attempted to access style from EmptyReading")
    }

    fn label(_: usize) -> &'static str {
        panic!("attempted to access label from EmptyReading")
    }
}

impl Reading for Vector3<f64> {
    const VALUES: usize = 3;

    fn value(&self, index: usize) -> f64 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!(
                "attempted to access value out of bounds: {} > {}",
                index,
                Self::VALUES - 1
            ),
        }
    }

    fn label(index: usize) -> &'static str {
        match index {
            0 => "x",
            1 => "y",
            2 => "z",
            _ => panic!(
                "attempted to access label out of bounds: {} > {}",
                index,
                Self::VALUES - 1
            ),
        }
    }

    fn style(index: usize) -> plotters::style::ShapeStyle {
        match index {
            0 => RED,
            1 => GREEN,
            2 => BLUE,
            _ => panic!(
                "attempted to access style out of bounds: {} > {}",
                index,
                Self::VALUES - 1
            ),
        }
        .into()
    }
}
