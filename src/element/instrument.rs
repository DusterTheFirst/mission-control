use iced::{button, Button, Element, Length};
use plotters_iced::{Chart, ChartWidget};

use crate::style;

use self::data_view::{DataView, View};

pub mod data_view;
pub mod reading;
pub mod time_series;
pub mod vector;

#[derive(Debug, Clone, Copy)]
pub enum InstrumentMessage {
    Selected(DataView),
}

fn instrument_view<'s, V: View, C: Chart<InstrumentMessage> + 's>(
    button_state: &'s mut button::State,
    chart: C,
) -> Element<'s, InstrumentMessage> {
    Button::new(button_state, ChartWidget::new(chart))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Instrument)
        .on_press(InstrumentMessage::Selected(DataView::from_view::<V>()))
        .into()
}
