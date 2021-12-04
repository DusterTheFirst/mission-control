#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::unwrap_in_result, clippy::missing_const_for_fn)]

use std::{borrow::Cow, time::Duration};

use comm::serial::{SerialEvent, SerialSubscription};
use element::instrument::Instrument;
use iced::{
    button, executor,
    keyboard::{self, KeyCode, Modifiers},
    pick_list,
    window::{self, Mode},
    Align, Application, Button, Clipboard, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, PickList, Row, Settings, Subscription, Text,
};
use iced_native::{event, subscription, Event};
use insomnia::Lock;
use interlink::{
    phy::InterlinkMethod,
    proto::{PacketDown, PacketDownData, VehicleIdentification},
};
use time_manager::{base::TimeBase, unit::VehicleTime, TimeManager};
use tracing_subscriber::EnvFilter;
use util::inhibit_sleep;

use crate::element::{
    ground_station_status::ground_station_status, telemetry_status::telemetry_status,
};

mod comm;
mod element;
mod style;
mod time_manager;
mod util;

pub fn main() -> iced::Result {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    InstrumentCluster::run(Settings {
        antialiasing: true,
        exit_on_close_request: true,
        ..Default::default()
    })
}

#[derive(Debug)]
struct Charts {
    c01: Instrument,
    c02: Instrument,
    c03: Instrument,
    c04: Instrument,
    c10: Instrument,
    c20: Instrument,
    c30: Instrument,
    c41: Instrument,
    c42: Instrument,
    c43: Instrument,
    c44: Instrument,
}

struct InstrumentCluster {
    quit: bool,
    window_focused: bool,
    window_mode: Mode,
    window_size: (u32, u32),

    time: TimeManager,
    time_base: TimeBase,

    charts: Charts,

    serial: SerialSubscription,
    interlink: Option<InterlinkMethod>,
    vehicle: Option<VehicleIdentification>,

    time_base_picker: pick_list::State<TimeBase>,
    quit_button: button::State,
    fullscreen_button: button::State,

    #[allow(dead_code)]
    sleep_lock: Option<Box<dyn Lock>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Quit,
    Refresh,
    ToggleFullscreen,
    WindowFocusChange { focused: bool },
    WindowSizeChange { width: u32, height: u32 },
    SerialEvent(SerialEvent),
    ChangeTimeBase(TimeBase),
}

impl Application for InstrumentCluster {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                quit: false,
                window_focused: true,
                window_mode: Mode::Windowed,
                window_size: (0, 0),

                charts: Charts {
                    c01: Instrument::new("01", 5.0),
                    c02: Instrument::new("02", 5.0),
                    c03: Instrument::new("03", 5.0),
                    c04: Instrument::new("04", 5.0),
                    c10: Instrument::new("10", 5.0),
                    c20: Instrument::new("20", 5.0),
                    c30: Instrument::new("30", 5.0),
                    c41: Instrument::new("41", 5.0),
                    c42: Instrument::new("42", 5.0),
                    c43: Instrument::new("43", 5.0),
                    c44: Instrument::new("44", 5.0),
                },

                time: TimeManager::setup(),
                time_base: TimeBase::GroundControl,

                serial: SerialSubscription::start(Duration::from_secs(1)),
                interlink: None,
                vehicle: None,

                time_base_picker: pick_list::State::default(),
                quit_button: button::State::default(),
                fullscreen_button: button::State::default(),

                sleep_lock: inhibit_sleep(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Ground Station")
    }

    fn background_color(&self) -> Color {
        style::colors::BACKGROUND.into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        // Update current time
        self.time.update_now();

        match message {
            Message::Quit => self.quit = true,
            Message::ToggleFullscreen => {
                self.window_mode = match self.window_mode {
                    Mode::Fullscreen => Mode::Windowed,
                    Mode::Windowed => Mode::Fullscreen,
                }
            }
            Message::WindowFocusChange { focused } => self.window_focused = focused,
            Message::WindowSizeChange { width, height } => self.window_size = (width, height),
            Message::Refresh => { /* TODO: replace with something better? */ }
            Message::SerialEvent(SerialEvent::PacketReceived(PacketDown { time, data })) => {
                let time = VehicleTime::from_packet(time);

                self.time.packet_received(time);

                match data {
                    PacketDownData::Magnetometer { x, y, z } => {
                        self.charts.c01.add_datum(x as f64 / 1000.0, time);
                        self.charts.c02.add_datum(y as f64 / 1000.0, time);
                        self.charts.c03.add_datum(z as f64 / 1000.0, time);
                    }
                    PacketDownData::Accelerometer { x, y, z } => {
                        self.charts.c41.add_datum((x as f64 * 9.81) / 1000.0, time);
                        self.charts.c42.add_datum((y as f64 * 9.81) / 1000.0, time);
                        self.charts.c43.add_datum((z as f64 * 9.81) / 1000.0, time);
                    }
                    PacketDownData::Hello(vehicle_identification) => {
                        self.vehicle.replace(vehicle_identification);
                    }
                }
            }
            Message::SerialEvent(SerialEvent::Connected) => {
                self.interlink = Some(InterlinkMethod::Serial);
            }
            Message::SerialEvent(SerialEvent::Disconnected) => {
                if self.interlink == Some(InterlinkMethod::Serial) {
                    self.interlink.take();
                }
                self.vehicle.take();
            }
            Message::ChangeTimeBase(time_base) => self.time_base = time_base,
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            self.serial.subscription().map(Message::SerialEvent),
            // TODO: update differently
            iced::time::every(Duration::from_millis(50)).map(|_| Message::Refresh),
            subscription::events_with(|event, status| match (event, status) {
                (
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key_code: KeyCode::Enter,
                        modifiers:
                            Modifiers {
                                alt: true,
                                control: false,
                                logo: false,
                                shift: false,
                            },
                    }),
                    event::Status::Ignored,
                ) => Some(Message::ToggleFullscreen),
                (Event::Window(iced_native::window::Event::Focused), _) => {
                    Some(Message::WindowFocusChange { focused: true })
                }
                (Event::Window(iced_native::window::Event::Unfocused), _) => {
                    Some(Message::WindowFocusChange { focused: false })
                }
                (Event::Window(iced_native::window::Event::Resized { width, height }), _) => {
                    Some(Message::WindowSizeChange { width, height })
                }
                _ => None,
            }),
        ])
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(10)
                .padding(10)
                .push(
                    Row::new()
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .spacing(10)
                        .push(telemetry_status(self.time, self.interlink, &self.vehicle))
                        .push(
                            self.charts
                                .c10
                                .view::<Self::Message>(&self.time, self.time_base),
                        )
                        .push(
                            self.charts
                                .c20
                                .view::<Self::Message>(&self.time, self.time_base),
                        )
                        .push(
                            self.charts
                                .c30
                                .view::<Self::Message>(&self.time, self.time_base),
                        )
                        .push(ground_station_status(self.time)),
                )
                .push(
                    Row::new()
                        .width(Length::Fill)
                        .height(Length::FillPortion(4))
                        .spacing(10)
                        .push(
                            Column::new()
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .spacing(10)
                                .push(
                                    self.charts
                                        .c01
                                        .view::<Self::Message>(&self.time, self.time_base),
                                )
                                .push(
                                    self.charts
                                        .c02
                                        .view::<Self::Message>(&self.time, self.time_base),
                                )
                                .push(
                                    self.charts
                                        .c03
                                        .view::<Self::Message>(&self.time, self.time_base),
                                )
                                .push(
                                    self.charts
                                        .c04
                                        .view::<Self::Message>(&self.time, self.time_base),
                                ),
                        )
                        .push(
                            Container::new(
                                Container::new(
                                    Column::new()
                                        .push(
                                            Text::new("Control Cluster")
                                                .size(32)
                                                .horizontal_alignment(HorizontalAlignment::Center),
                                        )
                                        .push(PickList::new(
                                            &mut self.time_base_picker,
                                            Cow::Borrowed(TimeBase::ALL),
                                            Some(self.time_base),
                                            Message::ChangeTimeBase,
                                        ))
                                        .push(
                                            Text::new(format!(
                                                "Window Size: {:?}",
                                                self.window_size
                                            ))
                                            .horizontal_alignment(HorizontalAlignment::Center),
                                        )
                                        .push(
                                            Text::new(format!("Focused: {}", self.window_focused))
                                                .horizontal_alignment(HorizontalAlignment::Center),
                                        )
                                        .push(
                                            Row::new()
                                                .push(
                                                    Button::new(
                                                        &mut self.fullscreen_button,
                                                        Text::new(match self.window_mode {
                                                            Mode::Windowed => "Fullscreen",
                                                            Mode::Fullscreen => "Windowed",
                                                        }),
                                                    )
                                                    .on_press(Message::ToggleFullscreen)
                                                    .style(style::ControlCluster),
                                                )
                                                .push(
                                                    Button::new(
                                                        &mut self.quit_button,
                                                        Text::new("Quit"),
                                                    )
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
                                .height(Length::Shrink),
                            )
                            .width(Length::FillPortion(3))
                            .height(Length::Fill)
                            .center_x()
                            .center_y(),
                        )
                        .push(
                            Column::new()
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .spacing(10)
                                .push(
                                    self.charts
                                        .c41
                                        .view::<Self::Message>(&self.time, self.time_base),
                                )
                                .push(
                                    self.charts
                                        .c42
                                        .view::<Self::Message>(&self.time, self.time_base),
                                )
                                .push(
                                    self.charts
                                        .c43
                                        .view::<Self::Message>(&self.time, self.time_base),
                                )
                                .push(
                                    self.charts
                                        .c44
                                        .view::<Self::Message>(&self.time, self.time_base),
                                ),
                        ),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Window)
        .into()
    }

    fn mode(&self) -> window::Mode {
        self.window_mode
    }

    fn should_exit(&self) -> bool {
        self.quit
    }
}
