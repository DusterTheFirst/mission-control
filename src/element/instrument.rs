use std::fmt::Debug;

use heapless::HistoryBuffer;
use iced::{Container, Element, Length};
use plotters::prelude::*;

use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::{
    station_time::{StationTime, TimeBase},
    style,
};

#[derive(Debug)]
pub struct Instrument<D, const WIDTH: isize> {
    datum: HistoryBuffer<D, WIDTH>,
    title: String,
}

impl<D, const WIDTH: i32> Instrument<D, WIDTH> {
    pub fn new<S: Into<String>>(title: S) -> Self {
        let datum = HistoryBuffer::with_capacity();

        Self {
            datum,
            title: title.into(),
        }
    }

    pub fn view<'s, Message: 's>(
        &'s mut self,
        time: &'s StationTime,
        time_base: TimeBase,
    ) -> InstrumentChart<D, WIDTH> {
        InstrumentChart(self, time, time_base)
    }

    pub fn add_datum(&mut self, datum: D) {
        self.datum.push_overwrite(datum);
    }
}

impl<'a, Message: 'a, D, const WIDTH: i32> From<InstrumentChart<'a, D, WIDTH>>
    for Element<'a, Message>
{
    fn from(element: InstrumentChart<'a, D, WIDTH>) -> Self {
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
pub struct InstrumentChart<'i, D, const WIDTH: i32>(
    &'i mut Instrument<D, WIDTH>,
    &'i StationTime,
    TimeBase,
);

impl<'i, Message, D, const WIDTH: i32> Chart<Message> for InstrumentChart<'i, D, WIDTH> {
    #[inline]
    fn build_chart<DB: DrawingBackend>(&self, mut builder: ChartBuilder<DB>) {
        let InstrumentChart(instrument, time, time_base) = self;

        let x_range = {
            let current_time = time.get_elapsed(*time_base);

            let current_seconds = current_time.as_seconds_f64();

            let x_min = (current_seconds - instrument.width).max(0.0);
            let x_max = current_seconds.max(instrument.width);

            x_min..x_max
        };

        // After this point, we should be able to draw construct a chart context
        let mut chart = builder
            .margin(5)
            .margin_right(20)
            // Set the caption of the chart
            .caption(
                &instrument.title,
                FontDesc::new(FontFamily::SansSerif, 20.0, FontStyle::Normal)
                    .color(&style::colors::TEXT),
            )
            // Set the size of the label region
            .x_label_area_size(25)
            .y_label_area_size(40)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(x_range, 0f64..10f64)
            .unwrap();

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
            .x_labels(10)
            .y_labels(5)
            .draw()
            .unwrap();

        // And we can draw something in the drawing area
        chart
            .draw_series(LineSeries::new(
                vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
                &RED,
            ))
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..=100)
                    .map(|x| x as f64 / 10.0)
                    .map(|x| (x, x.sin() * 5.0 + 5.0)),
                &GREEN,
            ))
            .unwrap();
    }
}
