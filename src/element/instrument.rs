use std::{any::TypeId, collections::VecDeque, fmt::Debug, ops::Range};

use iced::{button, Button, Element, Length};
use plotters::prelude::*;

use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::{
    style,
    time_manager::{base::TimeBase, unit::VehicleTime, TimeManager},
};

use self::{
    instrument_type::{InstrumentType, Placeholder},
    reading::Reading,
};

pub mod instrument_type;
pub mod reading;

#[derive(Debug)]
pub struct Instrument<IT: InstrumentType> {
    readings: VecDeque<(VehicleTime, IT::Reading)>,
    width: f64,

    button_state: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum InstrumentMessage {
    Selected(TypeId),
}

impl<IT: InstrumentType> Instrument<IT> {
    pub fn new(width: f64) -> Self {
        Self {
            readings: VecDeque::new(),
            width,

            button_state: button::State::new(),
        }
    }

    pub fn view<'s>(
        &'s mut self,
        time_manager: &'s TimeManager,
        time_base: TimeBase,
    ) -> Element<'s, InstrumentMessage> {
        let enabled = TypeId::of::<IT>() != TypeId::of::<Placeholder>();

        let button = Button::new(
            &mut self.button_state,
            ChartWidget::new(InstrumentView::<IT> {
                time_manager,
                time_base,
                readings: &self.readings,
                width: self.width,
            }),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Instrument);

        if enabled {
            button.on_press(InstrumentMessage::Selected(TypeId::of::<IT>()))
        } else {
            button
        }
        .into()
    }

    pub fn add_reading(&mut self, vehicle_time: VehicleTime, raw: IT::Raw) {
        let reading = IT::reading_from_raw(raw);

        self.readings.retain({
            let max_duration = vehicle_time.as_duration();
            let width = self.width;

            move |&(time, _)| (max_duration - time.as_duration()).as_seconds_f64() < width
        });

        self.readings.push_back((vehicle_time, reading));
    }
}

#[derive(Debug)]
pub struct InstrumentView<'i, IT: InstrumentType> {
    readings: &'i VecDeque<(VehicleTime, IT::Reading)>,
    width: f64,

    time_manager: &'i TimeManager,
    time_base: TimeBase,
}

impl<'i, IT: InstrumentType> InstrumentView<'i, IT> {
    pub fn x_range(&self) -> Range<f64> {
        let current_time = self.time_manager.elapsed(self.time_base);

        let current_seconds = current_time.as_seconds_f64();

        let x_min = (current_seconds - self.width).max(0.0);
        let x_max = current_seconds.max(self.width);

        x_min..x_max
    }

    pub fn y_range(&self) -> Range<f64> {
        let (min, max) = self
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
impl<'i, Message, IT: InstrumentType> Chart<Message> for InstrumentView<'i, IT> {
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
                IT::TITLE,
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
        for i in 0..IT::Reading::VALUES {
            let series = self.readings.iter().filter_map(|&(vehicle_time, datum)| {
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
                .draw_series(LineSeries::new(series, IT::Reading::style(i)))
                .expect("failed to draw series");
        }
    }
}
