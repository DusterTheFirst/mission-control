use std::{f32::consts::FRAC_PI_2, ops::Range};

use iced::{
    button,
    canvas::{path::Arc, Cursor, Frame, Geometry, LineCap, Path, Program, Stroke, Text},
    Canvas, Element, Length, Point, Rectangle, Vector,
};
use plotters::{
    prelude::LineSeries,
    style::{Color, FontDesc, ShapeStyle, RED},
};
use plotters_backend::{DrawingBackend, FontFamily, FontStyle};
use plotters_iced::{Chart, ChartBuilder, ChartWidget};

use crate::{style, time_manager::unit::VehicleTime};

use super::{data_view::View, instrument_view, reading::VectorReading, InstrumentMessage};

#[derive(Debug)]
pub struct VectorInstrument<V: View>
where
    V::Reading: VectorReading,
{
    reading: Option<(VehicleTime, V::Reading)>,

    button_state: button::State,
}

impl<V: View> VectorInstrument<V>
where
    V::Reading: VectorReading,
{
    pub fn new() -> Self {
        Self {
            reading: None,

            button_state: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<InstrumentMessage> {
        instrument_view::<V, _>(
            &mut self.button_state,
            ChartWidget::new(VectorInstrumentView::<V> {
                reading: self.reading.as_ref(),
            }),
        )
    }

    pub fn view_alt(&mut self) -> Element<InstrumentMessage> {
        instrument_view::<V, _>(
            &mut self.button_state,
            Canvas::new(VectorInstrumentView::<V> {
                reading: self.reading.as_ref(),
            })
            .width(Length::Fill)
            .height(Length::Fill),
        )
    }

    pub fn set_reading(&mut self, vehicle_time: VehicleTime, raw: V::Raw) {
        let reading = V::ingest_reading(raw);

        self.reading.replace((vehicle_time, reading));
    }
}

#[derive(Debug)]
pub struct VectorInstrumentView<'i, V: View>
where
    V::Reading: VectorReading,
{
    reading: Option<&'i (VehicleTime, V::Reading)>,
}

impl<'i, V: View> Program<InstrumentMessage> for VectorInstrumentView<'i, V>
where
    V::Reading: VectorReading,
{
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let margin = 10.0;

        let mut frame = Frame::new(bounds.size());
        let center = frame.center();
        let radius = frame.width().min(frame.height()) / 2.0 - margin;

        frame.stroke(
            &Path::new(|path| {
                path.move_to(center);
                path.circle(center, radius);
            }),
            Stroke {
                color: style::colors::TEXT.into(),
                width: 1.0,
                ..Default::default()
            },
        );

        if let Some((_time, reading)) = self.reading {
            let angle = reading.y().atan2(reading.x()) as f32;

            let display_angle = -angle;

            frame.stroke(
                &Path::new(|path| {
                    path.arc(Arc {
                        center,
                        radius: radius / 2.0,
                        start_angle: 0.0,
                        end_angle: display_angle,
                    })
                }),
                Stroke {
                    color: style::colors::ACCENT.into(),
                    width: 1.0,
                    ..Default::default()
                },
            );

            frame.stroke(
                &Path::new(|path| {
                    path.move_to(frame.center());
                    path.line_to(Point::new(
                        center.x + radius * display_angle.cos(),
                        center.y + radius * display_angle.sin(),
                    ));
                }),
                Stroke {
                    color: style::colors::TEXT.into(),
                    width: 3.0,
                    line_cap: LineCap::Square,
                    ..Default::default()
                },
            );

            frame.fill_text(Text {
                color: style::colors::TEXT.into(),
                content: angle.to_degrees().to_string(),
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}

impl<'i, V: View> VectorInstrumentView<'i, V>
where
    V::Reading: VectorReading,
{
    fn x_range(&self) -> Range<f64> {
        let x = self
            .reading
            .map(|(_time, reading)| reading.x())
            .unwrap_or(1.0);

        x.min(0.0)..x.max(1.0)
    }

    fn y_range(&self) -> Range<f64> {
        let y = self
            .reading
            .map(|(_time, reading)| reading.y())
            .unwrap_or(1.0);

        y.min(0.0)..y.max(1.0)
    }

    fn z_range(&self) -> Range<f64> {
        let z = self
            .reading
            .map(|(_time, reading)| reading.z())
            .unwrap_or(1.0);

        z.min(0.0)..z.max(1.0)
    }
}

impl<'i, V: View> Chart<InstrumentMessage> for VectorInstrumentView<'i, V>
where
    V::Reading: VectorReading,
{
    fn build_chart<DB: DrawingBackend>(&self, mut builder: ChartBuilder<DB>) {
        let mut chart = builder
            .caption(
                format!("{} Vector", V::TITLE),
                FontDesc::new(FontFamily::SansSerif, 20.0, FontStyle::Normal)
                    .color(&style::colors::TEXT),
            )
            .build_cartesian_3d(
                {
                    let x_range = self.x_range();
                    debug_assert!(
                        x_range.start < x_range.end,
                        "{} >= {}",
                        x_range.start,
                        x_range.end
                    );
                    x_range
                },
                {
                    let y_range = self.y_range();
                    debug_assert!(
                        y_range.start < y_range.end,
                        "{} >= {}",
                        y_range.start,
                        y_range.end
                    );
                    y_range
                },
                {
                    let z_range = self.z_range();
                    debug_assert!(
                        z_range.start < z_range.end,
                        "{} >= {}",
                        z_range.start,
                        z_range.end
                    );
                    z_range
                },
            )
            .expect("failed to build vector chart");

        // chart.with_projection(|mut p| {
        //     // p.pitch = 1.0; // TODO:
        //     p.scale = 0.75;
        //     p.into_matrix() // build the projection matrix
        // });

        // TODO: styling
        chart
            .configure_axes()
            .label_style(
                FontDesc::new(FontFamily::SansSerif, 10.0, FontStyle::Normal)
                    .color(&style::colors::TEXT),
            )
            .axis_panel_style(ShapeStyle::from(&style::colors::AXIS.mix(0.45)).stroke_width(1))
            .bold_grid_style(&style::colors::GRID_LINES)
            // Disable minor grid lines
            .light_grid_style(&plotters::style::TRANSPARENT)
            .draw()
            .expect("failed to draw vector chart axis");

        if let Some((_time, reading)) = self.reading {
            chart
                .draw_series(LineSeries::new(
                    [(0.0, 0.0, 0.0), (reading.x(), reading.y(), reading.z())],
                    &RED,
                ))
                .expect("failed to draw vector");
        }
    }
}
