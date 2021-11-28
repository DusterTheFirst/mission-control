use iced::{Align, Column, Element, Length, Space, Text};
use interlink::phy::InterlinkMethod;

use crate::{
    element::mono_label_text_tooltip,
    station_time::{format_duration, StationTime},
    style::{self, colors::Color},
    Message,
};

pub fn telemetry_status(
    station_time: StationTime,
    interlink: Option<InterlinkMethod>,
) -> Element<'static, Message> {
    Column::new()
        .push(Space::new(Length::Shrink, Length::Fill))
        .push(Text::new("Telemetry").size(32))
        .push(Space::new(Length::Shrink, Length::Units(16)))
        .push(interlink_method(interlink))
        .push(time_since_last_packet(station_time))
        .push(Space::new(Length::Shrink, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Align::Center)
        .spacing(2)
        .into()
}

fn interpolate_error_color(progress: f32) -> Color {
    let error_progress = progress.clamp(0.0, 1.0);
    let warn_progress = 1.0 - progress;

    let error = style::colors::ERROR;
    let warn = style::colors::WARNING;

    Color::from_rgb(
        (warn.r as f32 * warn_progress + error.r as f32 * error_progress).clamp(0.0, 255.0) as u8,
        (warn.g as f32 * warn_progress + error.g as f32 * error_progress).clamp(0.0, 255.0) as u8,
        (warn.b as f32 * warn_progress + error.b as f32 * error_progress).clamp(0.0, 255.0) as u8,
    )
}

fn time_since_last_packet(station_time: StationTime) -> Element<'static, Message> {
    let (time_since_last_packet, color) =
        if let Some(time_since_last_packet) = station_time.time_since_last_packet() {
            let color = match time_since_last_packet.whole_milliseconds() {
                0..=500 => style::colors::GOOD,
                i @ 501..=5000 => interpolate_error_color((i - 500) as f32 / 5000.0),
                _ => style::colors::ERROR,
            };

            (format_duration(time_since_last_packet), color)
        } else {
            ("--:--:--.-".to_string(), style::colors::SECONDARY_TEXT)
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

    // TODO: show only when using communication method where matters
    // mono_label_text_tooltip(
    //         "RSSI",
    //         "TODO:",
    //         "Received Signal Strength Indicator",
    //         None,
    // )

    // TODO:
    // mono_label_text_tooltip(
    //     "Data Rate",
    //     "???",
    //     "Packets Received per Second",
    //     Some(style::colors::WARNING),
    // )

    // TODO: other information?
    mono_label_text_tooltip(
        "Interlink",
        interlink,
        "Physical Method of Communication",
        Some(color),
    )
}
