use std::{fmt::Debug, marker::PhantomData};

use plotters::prelude::*;

use plotters_iced::{Chart, ChartBuilder, DrawingBackend};

pub mod style {
    pub struct ChartContainer;
    impl iced::container::StyleSheet for ChartContainer {
        fn style(&self) -> iced::container::Style {
            iced::container::Style {
                background: Some(iced::Color::BLACK.into()),
                text_color: Some(iced::Color::WHITE),
                ..Default::default()
            }
        }
    }
}

#[derive(Debug)]
pub struct MyChart<T>(PhantomData<T>);

impl<T> Default for MyChart<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Chart<T> for MyChart<T> {
    fn build_chart<DB: DrawingBackend>(&self, mut builder: ChartBuilder<DB>) {
        // After this point, we should be able to draw construct a chart context
        let mut chart = builder
            .margin(10)
            // Set the caption of the chart
            .caption("This is our first plot", ("sans-serif", 40).into_font())
            // Set the size of the label region
            .x_label_area_size(20)
            .y_label_area_size(40)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(0f32..10f32, 0f32..10f32)
            .unwrap();

        // Then we can draw a mesh
        chart
            .configure_mesh()
            .bold_line_style(&plotters::style::colors::WHITE.mix(0.1))
            .light_line_style(&plotters::style::colors::WHITE.mix(0.05))
            .axis_style(ShapeStyle::from(&plotters::style::colors::WHITE.mix(0.45)).stroke_width(1))
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(5)
            .y_labels(5)
            // We can also change the format of the label text
            .y_label_formatter(&|x| format!("{:.3}", x))
            .y_label_style(
                ("sans-serif", 15)
                    .into_font()
                    .color(&plotters::style::colors::WHITE.mix(0.65))
                    .transform(FontTransform::Rotate90),
            )
            .x_label_style(
                ("sans-serif", 15)
                    .into_font()
                    .color(&plotters::style::colors::WHITE.mix(0.65))
                    .transform(FontTransform::Rotate90),
            )
            .draw()
            .unwrap();

        // And we can draw something in the drawing area
        chart
            .draw_series(LineSeries::new(
                vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
                &RED,
            ))
            .unwrap();
        // Similarly, we can draw point series
        chart
            .draw_series(PointSeries::of_element(
                vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
                5,
                &RED,
                &|c, s, st| {
                    EmptyElement::at(c)
                        + Circle::new((0, 0), s, st.filled())
                        + Text::new(
                            format!("{:?}", c),
                            (10, 0),
                            ("sans-serif", 10)
                                .into_font()
                                .color(&plotters::style::colors::WHITE.mix(0.65)),
                        )
                },
            ))
            .unwrap();
    }
}
