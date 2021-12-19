use iced::{Column, Container, Element, Length, Row};

use crate::{
    element::{ground_station_status::ground_station_status, telemetry_status::telemetry_status},
    InstrumentCluster, Message,
};

pub fn view(app: &mut InstrumentCluster) -> Element<Message> {
    Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .push(
            Row::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .push(telemetry_status(
                    &app.time,
                    app.interlink,
                    app.vehicle.as_ref(),
                ))
                .push(
                    Container::new(
                        app.instruments
                            .acceleration_vector
                            .view(true)
                            .map(Message::Instrument),
                    )
                    .width(Length::FillPortion(2))
                    .height(Length::Fill),
                )
                .push(ground_station_status(&app.time)),
        )
        .push(
            app.instruments
                .acceleration_time
                .view(&app.time, app.time_base, true)
                .map(Message::Instrument),
        )
        .into()
}
