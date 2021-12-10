use iced::{button, Button, Container, Element, Length, Space};

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

pub struct PlaceholderInstrument {}

impl PlaceholderInstrument {
    pub fn view<'s>() -> Element<'s, InstrumentMessage> {
        Container::new(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Instrument)
            .into()
    }
}

fn instrument_view<'s, V: View, E: Into<Element<'s, InstrumentMessage>> + 's>(
    button_state: &'s mut button::State,
    content: E,
) -> Element<'s, InstrumentMessage> {
    Button::new(button_state, content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Instrument)
        .on_press(InstrumentMessage::Selected(DataView::from_view::<V>()))
        .into()
}
