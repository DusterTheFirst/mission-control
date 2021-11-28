use iced::{tooltip::Position, Element, Row, Text, Tooltip};

use crate::{
    style::{self, colors::Color},
    Message,
};

pub mod ground_station_status;
pub mod instrument;
pub mod telemetry_status;

pub(self) fn mono_label_text_tooltip<T: Into<String>>(
    label: &str,
    text: T,
    tooltip: &str,
    color: Option<Color>,
) -> Element<'static, Message> {
    let text = Text::new(text).font(style::fonts::MONOSPACE);

    Tooltip::new(
        Row::new()
            .push(Text::new(label).font(style::fonts::MONOSPACE))
            .push(Text::new(": ").font(style::fonts::MONOSPACE))
            .push(if let Some(color) = color {
                text.color(color)
            } else {
                text
            }),
        tooltip,
        Position::Top,
    )
    .style(style::Tooltip)
    .into()
}
