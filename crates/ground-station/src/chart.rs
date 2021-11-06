use std::fmt::Debug;

use iced::{
    canvas::{Cache, Frame},
    Size,
};
use plotters::prelude::*;

use plotters_iced::{Chart, ChartBuilder, DrawingBackend};

use crate::style;

#[derive(Debug)]
pub struct Instrument {
    cache: Cache,
}

impl Default for Instrument {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
        }
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
                "This is our first plot",
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
    }
}
