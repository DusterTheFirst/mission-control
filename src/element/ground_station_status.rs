use iced::{Align, Column, Element, Length, Space, Text};

use crate::{
    style::colors::Color,
    time_manager::{base::TimeBase, format_duration, unit::LocalTime, TimeManager},
};

use super::mono_label_text_tooltip;

pub fn ground_station_status<'m, Message: 'm>(time_manager: &TimeManager) -> Element<'m, Message> {
    Column::new()
        .push(Space::new(Length::Shrink, Length::Fill))
        .push(Text::new("Ground Station").size(32))
        .push(Space::new(Length::Shrink, Length::Units(16)))
        .push(station_local_time(time_manager.now()))
        .push(time_with_tooltip(time_manager, TimeBase::GroundControl))
        .push(time_with_tooltip(time_manager, TimeBase::VehicleTime))
        .push(time_with_tooltip(time_manager, TimeBase::Mission))
        .push(Space::new(Length::Shrink, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Align::Center)
        .spacing(2)
        .into()
}

fn station_local_time<'m, Message: 'm>(local_time: LocalTime) -> Element<'m, Message> {
    let (label, tooltip) = if local_time.is_utc() {
        ("UTC", "Universal Coordinated Time")
    } else {
        ("SLT", "Station Local Time")
    };

    mono_label_text_tooltip(
        label,
        local_time.format(),
        tooltip,
        Some(Color::from_rgb(0xFF, 0xFF, 0xFF)),
    )
}

fn time_with_tooltip<'m, Message: 'm>(
    time_manager: &TimeManager,
    time_base: TimeBase,
) -> Element<'m, Message> {
    let (label, tooltip, color) = match time_base {
        TimeBase::GroundControl => (
            "GCT",
            "Ground Control Time",
            Color::from_rgb(0xFF, 0x00, 0x00),
        ),
        TimeBase::VehicleTime => ("VOT", "Vehicle On Time", Color::from_rgb(0x00, 0xFF, 0x00)),
        TimeBase::Mission => ("MIT", "Mission Time", Color::from_rgb(0x00, 0x00, 0xFF)),
    };

    mono_label_text_tooltip(
        label,
        format_duration(time_manager.elapsed(time_base)),
        tooltip,
        Some(color),
    )
}
