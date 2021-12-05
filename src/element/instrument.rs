use std::{collections::VecDeque, fmt::Debug, iter, ops::Range};

use iced::{Container, Element, Length};
use interlink::proto::{AccelerometerReading, MagnetometerReading};
use plotters::prelude::*;

use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::{
    style,
    time_manager::{base::TimeBase, unit::VehicleTime, TimeManager},
};

pub type DatumValuesIter<S> =
    iter::Map<iter::Zip<Range<usize>, iter::Repeat<S>>, fn((usize, S)) -> f64>;

pub trait Reading: Debug + Copy + Sized {
    const VALUES: usize;

    fn value(&self, index: usize) -> f64;
    fn style(index: usize) -> ShapeStyle;
    fn title() -> &'static str;

    fn values(&self) -> DatumValuesIter<Self> {
        (0..Self::VALUES)
            .zip(iter::repeat(*self))
            .map(|(index, datum)| datum.value(index))
    }
}

impl Reading for AccelerometerReading {
    const VALUES: usize = 3;

    fn title() -> &'static str {
        "Acceleration"
    }

    fn value(&self, index: usize) -> f64 {
        match index {
            0 => (self.x as f64 * 9.81) / 1000.0,
            1 => (self.y as f64 * 9.81) / 1000.0,
            2 => (self.z as f64 * 9.81) / 1000.0,
            _ => panic!(
                "attempted to access value out of bounds: {} > {}",
                index,
                Self::VALUES - 1
            ),
        }
    }

    // TODO: better
    // TODO: macro?
    fn style(index: usize) -> ShapeStyle {
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

impl Reading for MagnetometerReading {
    const VALUES: usize = 3;

    fn title() -> &'static str {
        "Magnetic Field"
    }

    fn value(&self, index: usize) -> f64 {
        match index {
            0 => self.x as f64 / 1000.0,
            1 => self.y as f64 / 1000.0,
            2 => self.z as f64 / 1000.0,
            _ => panic!(
                "attempted to access value out of bounds: {} > {}",
                index,
                Self::VALUES - 1
            ),
        }
    }

    // TODO: better
    // TODO: macro?
    fn style(index: usize) -> ShapeStyle {
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

#[derive(Debug, Clone, Copy)]
pub struct EmptyReading {}

impl Reading for EmptyReading {
    const VALUES: usize = 0;

    fn title() -> &'static str {
        "-----"
    }

    fn value(&self, _: usize) -> f64 {
        panic!("attempted to access value from EmptyDatum")
    }

    fn style(_: usize) -> ShapeStyle {
        panic!("attempted to access style from EmptyDatum")
    }
}

#[derive(Debug)]
pub struct Instrument<R: Reading> {
    readings: VecDeque<(VehicleTime, R)>,
    width: f64,
}

impl<D: Reading> Instrument<D> {
    pub fn new(width: f64) -> Self {
        Self {
            readings: VecDeque::with_capacity(128),
            width,
        }
    }

    pub fn view<'s, Message: 's>(
        &'s mut self,
        time_manager: &'s TimeManager,
        time_base: TimeBase,
    ) -> InstrumentChart<D> {
        InstrumentChart {
            instrument: self,
            time_manager,
            time_base,
        }
    }

    pub fn add_reading(&mut self, vehicle_time: VehicleTime, reading: D) {
        self.readings.push_back((vehicle_time, reading));
    }

    // pub fn prune_datum(
    //     &mut self,
    //     x_range: &Range<f64>,
    //     time_manager: &TimeManager,
    //     time_base: TimeBase,
    // ) {
    //     // TODO: ?
    //     self.datum.retain(|&(time, _value)| {
    //         let time = time_manager
    //             .rebase_vehicle_time(time, time_base)
    //             .as_seconds_f64();

    //         x_range.contains(&time)
    //     })
    // }
}

impl<'a, Message: 'a, R: Reading> From<InstrumentChart<'a, R>> for Element<'a, Message> {
    fn from(element: InstrumentChart<'a, R>) -> Self {
        Container::new(
            ChartWidget::new(element)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Instrument)
        .into()
    }
}

#[derive(Debug)]
pub struct InstrumentChart<'i, D: Reading> {
    instrument: &'i mut Instrument<D>,
    time_manager: &'i TimeManager,
    time_base: TimeBase,
}

impl<'i, R: Reading> InstrumentChart<'i, R> {
    pub fn x_range(&self) -> Range<f64> {
        let current_time = self.time_manager.elapsed(self.time_base);

        let current_seconds = current_time.as_seconds_f64();

        let x_min = (current_seconds - self.instrument.width).max(0.0);
        let x_max = current_seconds.max(self.instrument.width);

        x_min..x_max
    }

    pub fn y_range(&self) -> Range<f64> {
        let (min, max) = self
            .instrument
            .readings
            .iter()
            .map(|(_time, value)| *value)
            .flat_map(|datum| datum.values())
            .fold((f64::NAN, f64::NAN), |(pre_min, pre_max), value| {
                (value.min(pre_min), value.max(pre_max))
            });

        min.min(0.0)..max.max(0.1)
    }
}

// Custom impl for Empty Datum
impl<'i, Message, R: Reading> Chart<Message> for InstrumentChart<'i, R> {
    #[inline]
    fn build_chart<DB: DrawingBackend>(&self, mut builder: ChartBuilder<DB>) {
        let x_range = self.x_range();
        let y_range = self.y_range();

        // TODO: do this somewhere else?
        // self.instrument
        //     .prune_datum(&x_range, self.time_manager, self.time_base);

        // After this point, we should be able to draw construct a chart context
        let mut chart = builder
            .margin(5)
            .margin_right(20)
            // Set the caption of the chart
            .caption(
                R::title(),
                FontDesc::new(FontFamily::SansSerif, 20.0, FontStyle::Normal)
                    .color(&style::colors::TEXT),
            )
            // Set the size of the label region
            .x_label_area_size(25)
            .y_label_area_size(40)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(x_range.clone(), y_range)
            .expect("failed to build chart");

        let axis_label_style = FontDesc::new(FontFamily::SansSerif, 12.0, FontStyle::Normal)
            .color(&style::colors::TEXT);

        // Then we can draw a mesh
        chart
            .configure_mesh()
            .axis_style(ShapeStyle::from(&style::colors::AXIS.mix(0.45)).stroke_width(1))
            .bold_line_style(&style::colors::GRID_LINES)
            // Disable minor grid lines
            .light_line_style(&plotters::style::TRANSPARENT)
            .label_style(
                FontDesc::new(FontFamily::SansSerif, 10.0, FontStyle::Normal)
                    .color(&style::colors::TEXT),
            )
            .x_label_style(axis_label_style.clone())
            .y_label_style(axis_label_style.clone())
            .axis_desc_style(axis_label_style)
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(5)
            .y_labels(5)
            .draw()
            .expect("failed to draw chart");

        // TODO: implement correctly
        // TODO: make sure this actually scales with time base
        // TODO: make sure this is tracking correctly cause uh oh

        // TODO: Separate when zoom in?
        for i in 0..R::VALUES {
            let series = self
                .instrument
                .readings
                .iter()
                .filter_map(|&(vehicle_time, datum)| {
                    let time = self
                        .time_manager
                        .rebase_vehicle_time(vehicle_time, self.time_base)
                        .as_seconds_f64();

                    if time < x_range.start {
                        None
                    } else {
                        Some((time, datum.value(i)))
                    }
                });

            chart
                .draw_series(LineSeries::new(series, R::style(i)))
                .expect("failed to draw series");
        }
    }
}
