use std::fmt::Debug;

use iced::{
    canvas::{Cache, Frame},
    Container, Length, Size,
};
use plotters::prelude::*;

use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use time::{Duration, Instant};
use tracing::info;

use crate::style;

#[derive(Debug)]
pub struct Instrument {
    datum: Vec<()>,
    title: String,

    // TODO: change timebase between ground control, vehicle on, and mission start
    ground_control_time: Duration,
    vehicle_on_time: Option<Instant>,
    mission_start: Option<Instant>,

    cache: Cache,
}

impl Instrument {
    pub fn new<S: Into<String>>(title: S) -> Self {
        let datum = Vec::with_capacity(1 << 10); // TODO: calculate capacity better;

        Self {
            datum,
            ground_control_time: Duration::ZERO,
            title: title.into(),
            cache: Cache::new(),
        }
    }
}

impl Instrument {
    // TODO: pass times here rather than storing them in state
    pub fn view<'s, Message: 's>(&'s mut self) -> Container<'_, Message> {
        Container::new(
            ChartWidget::new(self)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Instrument)
    }

    pub fn update_time(&mut self, ground_control_time: Duration) {
        self.ground_control_time = ground_control_time;

        self.cache.clear();
    }

    pub fn add_datum(&mut self) {
        // TODO:

        self.cache.clear();
    }
}

impl<Message> Chart<Message> for Instrument {
    #[inline]
    fn draw<F: Fn(&mut Frame)>(&self, size: Size, f: F) -> iced::canvas::Geometry {
        self.cache.draw(size, f)
    }

    #[inline]
    fn build_chart<DB: DrawingBackend>(&self, mut builder: ChartBuilder<DB>) {
        // After this point, we should be able to draw construct a chart context
        let mut chart = builder
            .margin(5)
            .margin_right(20)
            // Set the caption of the chart
            .caption(
                &self.title,
                FontDesc::new(FontFamily::SansSerif, 20.0, FontStyle::Normal)
                    .color(&style::colors::TEXT),
            )
            // Set the size of the label region
            .x_label_area_size(25)
            .y_label_area_size(40)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(0f32..10f32, 0f32..10f32)
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
                    .map(|x| x as f32 / 10.0)
                    .map(|x| (x, x.sin() * 5.0 + 5.0)),
                &GREEN,
            ))
            .unwrap();
    }
}
