use iced::{tooltip::Position, Element, Row, Text, Tooltip};

use crate::style::{self, colors::Color};

pub mod ground_station_status;
pub mod instrument;
pub mod telemetry_status;

pub(self) fn mono_label_text_tooltip<'m, Message: 'm>(
    label: &str,
    text: impl Into<String>,
    tooltip: &str,
    color: Option<Color>,
) -> Element<'m, Message> {
    Tooltip::new(mono_label_text(label, text, color), tooltip, Position::Top)
        .style(style::Tooltip)
        .into()
}

pub(self) fn mono_label_text<'m, Message: 'm>(
    label: &str,
    text: impl Into<String>,
    color: Option<Color>,
) -> Element<'m, Message> {
    let text = Text::new(text).font(style::fonts::MONOSPACE);

    Row::new()
        .push(Text::new(label).font(style::fonts::MONOSPACE))
        .push(Text::new(": ").font(style::fonts::MONOSPACE))
        .push(if let Some(color) = color {
            text.color(color)
        } else {
            text
        })
        .into()
}
