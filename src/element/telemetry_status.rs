use iced::{Align, Column, Element, Length, Space, Text};
use interlink::phy::InterlinkMethod;

use crate::{
    element::mono_label_text_tooltip,
    station_time::{format_duration, StationTime},
    style, Message,
};

pub fn telemetry_status(
    station_time: StationTime,
    interlink: Option<InterlinkMethod>,
) -> Element<'static, Message> {
    Column::new()
        .push(Space::new(Length::Shrink, Length::Fill))
        .push(Text::new("Telemetry").size(32))
        .push(Space::new(Length::Shrink, Length::Units(16)))
        .push(time_since_last_packet(station_time))
        .push(mono_label_text_tooltip(
            "RSSI",
            "TODO:",
            "Received Signal Strength Indicator",
            None,
        ))
        .push(interlink_method(interlink))
        .push(Space::new(Length::Shrink, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Align::Center)
        .spacing(2)
        .into()
}

fn time_since_last_packet(station_time: StationTime) -> Element<'static, Message> {
    let (time_since_last_packet, color) =
        if let Some(time_since_last_packet) = station_time.time_since_last_packet() {
            let color = match time_since_last_packet.whole_milliseconds() {
                0..=100 => style::colors::TEXT,
                101..=1000 => style::colors::WARNING,
                _ => style::colors::ERROR,
            };

            (format_duration(time_since_last_packet), color)
        } else {
            ("--:--:--.-".to_string(), style::colors::SECONDARY_TEXT) // TODO: specific color for this
        };

    mono_label_text_tooltip(
        "TSLP",
        time_since_last_packet,
        "Time Since Last Packet",
        Some(color),
    )
}

fn interlink_method(interlink: Option<InterlinkMethod>) -> Element<'static, Message> {
    let (interlink, color) = match interlink {
        Some(InterlinkMethod::Serial) => ("Serial", style::colors::ACTIVE),
        None => ("None", style::colors::SECONDARY_TEXT),
    };

    mono_label_text_tooltip(
        "Interlink",
        interlink,
        "Method of Communication to the Device",
        Some(color),
    )
}
