use iced::{Align, Column, Element, Length, Space, Text};
use time::{macros::format_description, OffsetDateTime};

use crate::{
    station_time::{format_duration, StationTime, TimeBase},
    style::colors::Color,
    Message,
};

use super::mono_label_text_tooltip;

pub fn ground_station_status(station_time: StationTime) -> Element<'static, Message> {
    let now = station_time.now();
    let utc = now.offset().is_utc();

    Column::new()
        .push(Space::new(Length::Shrink, Length::Fill))
        .push(Text::new("Ground Station").size(32))
        .push(Space::new(Length::Shrink, Length::Units(16)))
        .push(station_local_time(now, utc))
        .push(station_time_with_tooltip(
            station_time,
            TimeBase::GroundControl,
        ))
        .push(station_time_with_tooltip(station_time, TimeBase::VehicleTime))
        .push(station_time_with_tooltip(station_time, TimeBase::Mission))
        .push(Space::new(Length::Shrink, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Align::Center)
        .spacing(2)
        .into()
}

fn station_local_time(now: OffsetDateTime, utc: bool) -> Element<'static, Message> {
    let (label, tooltip) = if utc {
        ("UTC", "Universal Coordinated Time")
    } else {
        ("SLT", "Station Local Time")
    };

    let time = now
        .format(format_description!(
            "[hour repr:24]:[minute]:[second].[subsecond digits:1]"
        ))
        .unwrap();

    mono_label_text_tooltip(
        label,
        time,
        tooltip,
        Some(Color::from_rgb(0xFF, 0xFF, 0xFF)),
    )
}

fn station_time_with_tooltip(
    station_time: StationTime,
    time_base: TimeBase,
) -> Element<'static, Message> {
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
        format_duration(station_time.get_elapsed(time_base)),
        tooltip,
        Some(color),
    )
}
