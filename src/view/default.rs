use std::borrow::Cow;

use iced::{
    button, pick_list, window::Mode, Align, Button, Column, Container, Element,
    HorizontalAlignment, Length, PickList, Row, Text,
};
use interlink::{phy::InterlinkMethod, proto::VehicleIdentification};

use crate::{
    element::{
        ground_station_status::ground_station_status,
        instrument::{
            data_view::{Accelerometer, Magnetometer},
            time_series::TimeSeriesInstrument,
            vector::VectorInstrument,
            PlaceholderInstrument,
        },
        telemetry_status::telemetry_status,
    },
    style,
    time_manager::{base::TimeBase, TimeManager},
    InstrumentCluster, Message,
};

pub fn view(app: &mut InstrumentCluster) -> Element<Message> {
    Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .push(top_row(app.time, app.interlink, app.vehicle.as_ref()))
        .push(
            Row::new()
                .width(Length::Fill)
                .height(Length::FillPortion(4))
                .spacing(10)
                .push(left_column(
                    &mut app.instruments.magnetic_field_time,
                    &mut app.instruments.magnetic_field_vector,
                    &app.time,
                    app.time_base,
                ))
                .push(
                    Container::new(control_cluster(
                        &mut app.time_base_picker,
                        app.time_base,
                        app.window_size,
                        app.window_focused,
                        &mut app.fullscreen_button,
                        app.window_mode,
                        &mut app.quit_button,
                    ))
                    .width(Length::FillPortion(3))
                    .height(Length::Fill)
                    .center_x()
                    .center_y(),
                )
                .push(right_column(
                    &mut app.instruments.acceleration_time,
                    &mut app.instruments.acceleration_vector,
                    &app.time,
                    app.time_base,
                )),
        )
        .into()
}

fn top_row(
    time: TimeManager,
    interlink: Option<InterlinkMethod>,
    vehicle: Option<&VehicleIdentification>,
) -> Element<Message> {
    Row::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .push(telemetry_status(time, interlink, vehicle))
        .push(PlaceholderInstrument::view().map(Message::Instrument))
        .push(PlaceholderInstrument::view().map(Message::Instrument))
        .push(PlaceholderInstrument::view().map(Message::Instrument))
        .push(ground_station_status(time))
        .into()
}

fn left_column<'app>(
    magnetic_field_time: &'app mut TimeSeriesInstrument<Magnetometer>,
    magnetic_field_vector: &'app mut VectorInstrument<Magnetometer>,
    time: &'app TimeManager,
    time_base: TimeBase,
) -> Element<'app, Message> {
    Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .push(
            magnetic_field_time
                .view(time, time_base)
                .map(Message::Instrument),
        )
        .push(magnetic_field_vector.view_alt().map(Message::Instrument))
        .push(PlaceholderInstrument::view().map(Message::Instrument))
        .push(PlaceholderInstrument::view().map(Message::Instrument))
        .into()
}

fn right_column<'app>(
    acceleration_time: &'app mut TimeSeriesInstrument<Accelerometer>,
    acceleration_vector: &'app mut VectorInstrument<Accelerometer>,
    time: &'app TimeManager,
    time_base: TimeBase,
) -> Element<'app, Message> {
    Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .push(
            acceleration_time
                .view(time, time_base)
                .map(Message::Instrument),
        )
        .push(acceleration_vector.view().map(Message::Instrument))
        .push(PlaceholderInstrument::view().map(Message::Instrument))
        .push(PlaceholderInstrument::view().map(Message::Instrument))
        .into()
}

fn control_cluster<'app>(
    time_base_picker: &'app mut pick_list::State<TimeBase>,
    time_base: TimeBase,
    window_size: (u32, u32),
    window_focused: bool,
    fullscreen_button: &'app mut button::State,
    window_mode: Mode,
    quit_button: &'app mut button::State,
) -> Element<'app, Message> {
    Container::new(
        Column::new()
            .push(
                Text::new("Control Cluster")
                    .size(32)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(PickList::new(
                time_base_picker,
                Cow::Borrowed(TimeBase::ALL),
                time_base.into(),
                Message::ChangeTimeBase,
            ))
            .push(
                Text::new(format!("Window Size: {:?}", window_size))
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(
                Text::new(format!("Focused: {}", window_focused))
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(
                Row::new()
                    .push(
                        Button::new(
                            fullscreen_button,
                            Text::new(match window_mode {
                                Mode::Windowed => "Fullscreen",
                                Mode::Fullscreen => "Windowed",
                            }),
                        )
                        .on_press(Message::ToggleFullscreen)
                        .style(style::ControlCluster),
                    )
                    .push(
                        Button::new(quit_button, Text::new("Quit"))
                            .on_press(Message::Quit)
                            .style(style::ControlCluster),
                    )
                    .spacing(50),
            )
            .spacing(10)
            .align_items(Align::Center)
            .width(Length::Shrink)
            .height(Length::Shrink),
    )
    .padding(10)
    .style(style::ControlCluster)
    .width(Length::Shrink)
    .height(Length::Shrink)
    .into()
}
