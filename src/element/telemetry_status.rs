use iced::{Align, Column, Element, Length, Space, Text};
use interlink::{phy::InterlinkMethod, proto::VehicleIdentification};

use crate::{
    element::mono_label_text_tooltip,
    style::{
        self,
        colors::{self, Color},
    },
    time_manager::{format_duration, TimeManager},
};

pub fn telemetry_status<'m, Message: 'm>(
    time_manager: &TimeManager,
    interlink: Option<InterlinkMethod>,
    vehicle: Option<&VehicleIdentification>,
) -> Element<'m, Message> {
    Column::new()
        .push(Space::new(Length::Shrink, Length::Fill))
        .push(Text::new("Telemetry").size(32))
        .push(Space::new(Length::Shrink, Length::Units(16)))
        .push(vehicle_id(vehicle))
        .push(interlink_method(interlink))
        .push(time_since_last_packet(time_manager))
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

fn time_since_last_packet<'m, Message: 'm>(time_manager: &TimeManager) -> Element<'m, Message> {
    let (time_since_last_packet, color) =
        if let Some(time_since_last_packet) = time_manager.duration_since_last_packet() {
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

fn interlink_method<'m, Message: 'm>(interlink: Option<InterlinkMethod>) -> Element<'m, Message> {
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

fn vehicle_id<'m, Message: 'm>(vehicle: Option<&VehicleIdentification>) -> Element<'m, Message> {
    match vehicle {
        Some(vehicle_id) => mono_label_text_tooltip(
            vehicle_id.name.as_str(),
            vehicle_id.version.as_str(),
            "Vehicle Information",
            Some(colors::ACCENT),
        ),
        None => Space::new(Length::Shrink, Length::Shrink).into(),
    }
}
