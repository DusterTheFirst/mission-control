use std::{sync::Once, time::Duration};

use chart::Instrument;
use iced::{
    button, executor,
    keyboard::{self, KeyCode, Modifiers},
    tooltip::Position,
    window::{self, Mode},
    Align, Application, Button, Clipboard, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Settings, Space, Subscription, Text, Tooltip,
};
use iced_native::{event, subscription, Event};
use plotters_iced::{Chart, ChartWidget};
use time::{macros::format_description, OffsetDateTime, Time};
use tracing::{error, info, warn};

mod chart;
mod style;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt().pretty().init();

    InstrumentCluster::run(Settings {
        antialiasing: true,
        exit_on_close_request: true,
        ..Default::default()
    })
}

#[derive(Debug, Default)]
struct Charts {
    c00: Instrument,
    c01: Instrument,
    c02: Instrument,
    c03: Instrument,
    c04: Instrument,
    c05: Instrument,
    c06: Instrument,
    c07: Instrument,
    c08: Instrument,
    c09: Instrument,
    c10: Instrument,
    c11: Instrument,
    c12: Instrument,
    c13: Instrument,
    c14: Instrument,
    c15: Instrument,
    c16: Instrument,
    c17: Instrument,
    c18: Instrument,
    c19: Instrument,
    c20: Instrument,
    c21: Instrument,
    c22: Instrument,
    c23: Instrument,
    c24: Instrument,
    c25: Instrument,
    c26: Instrument,
    c27: Instrument,
    c28: Instrument,
    c29: Instrument,
    c30: Instrument,
    c31: Instrument,
    c32: Instrument,
    c33: Instrument,
    c34: Instrument,
    c35: Instrument,
    c36: Instrument,
    c37: Instrument,
    c38: Instrument,
    c39: Instrument,
    c40: Instrument,
    c41: Instrument,
    c42: Instrument,
    c43: Instrument,
    c44: Instrument,
    c45: Instrument,
    c46: Instrument,
    c47: Instrument,
    c48: Instrument,
    c49: Instrument,
}

#[derive(Debug)]
struct InstrumentCluster {
    quit: bool,
    window_focused: bool,
    window_mode: Mode,
    window_size: (u32, u32),

    local_time: OffsetDateTime,
    station_start_time: OffsetDateTime,
    mission_start_time: Option<OffsetDateTime>,

    charts: Charts,

    quit_button: button::State,
    fullscreen_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Quit,
    LocalTime(OffsetDateTime),
    ToggleFullscreen,
    WindowFocusChange(bool),
    WindowSizeChange((u32, u32)),
}

fn get_local_time() -> OffsetDateTime {
    static ONCE: Once = Once::new();

    OffsetDateTime::now_local().unwrap_or_else(|e| {
        ONCE.call_once(|| {
            error!("{}", e);
            warn!("Using UTC for local time");
        });

        OffsetDateTime::now_utc()
    })
}

fn format_duration(duration: time::Duration) -> String {
    format!(
        "{:02}:{:02}:{:02}.{:01}",
        duration.whole_hours(),
        duration.whole_minutes(),
        duration.whole_seconds(),
        duration.subsec_milliseconds() / 100
    )
}

impl Application for InstrumentCluster {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let local_time = get_local_time();

        (
            Self {
                quit: false,
                window_focused: true,
                window_mode: Mode::Fullscreen,
                window_size: (0, 0),

                charts: Charts::default(),

                local_time,
                // Artificially sync the local time with the ground control time
                station_start_time: local_time.replace_time(
                    Time::from_hms(local_time.hour(), local_time.minute(), local_time.second())
                        .unwrap(),
                ),
                mission_start_time: None,

                quit_button: button::State::default(),
                fullscreen_button: button::State::default(),
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
        match message {
            Message::Quit => self.quit = true,
            Message::ToggleFullscreen => {
                self.window_mode = match self.window_mode {
                    Mode::Fullscreen => Mode::Windowed,
                    Mode::Windowed => Mode::Fullscreen,
                }
            }
            Message::WindowFocusChange(focus) => self.window_focused = focus,
            Message::WindowSizeChange(size) => self.window_size = size,
            Message::LocalTime(local_time) => {
                /* TODO: replace with something better? */
                self.local_time = local_time
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            // TODO: update differently
            iced::time::every(Duration::from_millis(50))
                .map(|_| Message::LocalTime(get_local_time())),
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
                    Some(Message::WindowFocusChange(true))
                }
                (Event::Window(iced_native::window::Event::Unfocused), _) => {
                    Some(Message::WindowFocusChange(false))
                }
                (Event::Window(iced_native::window::Event::Resized { width, height }), _) => {
                    Some(Message::WindowSizeChange((width, height)))
                }
                _ => None,
            }),
        ])
    }

    fn view(&mut self) -> Element<Self::Message> {
        // TODO: mono space font
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
                        .push(
                            Column::new()
                                .push(Space::new(Length::Shrink, Length::Fill))
                                .push(Text::new("Telemetry").size(32))
                                .push(Space::new(Length::Shrink, Length::Units(16)))
                                .push(
                                    Tooltip::new(
                                        Text::new("TSLP: TODO:"),
                                        "Time Since Last Packet",
                                        Position::FollowCursor,
                                    )
                                    .style(style::Tooltip),
                                )
                                .push(
                                    Tooltip::new(
                                        Text::new("RSSI: TODO:"),
                                        "Received Signal Strength Indicator",
                                        Position::FollowCursor,
                                    )
                                    .style(style::Tooltip),
                                )
                                .push(Text::new("Uplink: TODO:"))
                                .push(Space::new(Length::Shrink, Length::Fill))
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_items(Align::Center)
                                .spacing(2),
                        )
                        .push(create_chart(&mut self.charts.c10))
                        .push(create_chart(&mut self.charts.c20))
                        .push(create_chart(&mut self.charts.c30))
                        .push(
                            Column::new()
                                .push(Space::new(Length::Shrink, Length::Fill))
                                .push(Text::new("Ground Station").size(32))
                                .push(Space::new(Length::Shrink, Length::Units(16)))
                                .push(
                                    Tooltip::new(
                                        Text::new(format!(
                                            "{}: {}",
                                            if self.local_time.offset().is_utc() {
                                                "UTC"
                                            } else {
                                                "SLT"
                                            },
                                            self.local_time
                                                .format(format_description!(
                                                    "[hour repr:24]:[minute]:[second].[subsecond digits:1]"
                                                ))
                                                .unwrap(),
                                        )).font(style::fonts::roboto_mono::REGULAR),
                                        if self.local_time.offset().is_utc() {
                                            "Universal Coordinated Time"
                                        } else {
                                            "Station Local Time"
                                        },
                                        Position::FollowCursor,
                                    )
                                    .style(style::Tooltip),
                                )
                                .push(
                                    Tooltip::new(
                                        Text::new(format!(
                                            "GCT: {}",
                                            format_duration(
                                                self.local_time - self.station_start_time
                                            )
                                        )).font(style::fonts::roboto_mono::REGULAR),
                                        "Ground Control Time",
                                        Position::FollowCursor,
                                    )
                                    .style(style::Tooltip),
                                )
                                .push(
                                    Tooltip::new(
                                        Text::new(format!("VOT: {}", format_duration(time::Duration::ZERO /* TODO: */))).font(style::fonts::roboto_mono::REGULAR),
                                        "Vehicle On Time",
                                        Position::FollowCursor,
                                    )
                                    .style(style::Tooltip),
                                )
                                .push(
                                    Tooltip::new(
                                        Text::new(format!("MIT: {}", format_duration(time::Duration::ZERO /* TODO: */))).font(style::fonts::roboto_mono::REGULAR),
                                        "MIssion Time",
                                        Position::FollowCursor,
                                    )
                                    .style(style::Tooltip),
                                )
                                .push(Space::new(Length::Shrink, Length::Fill))
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_items(Align::Center)
                                .spacing(2),
                        ),
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
                                .push(create_chart(&mut self.charts.c01))
                                .push(create_chart(&mut self.charts.c02))
                                .push(create_chart(&mut self.charts.c03))
                                .push(create_chart(&mut self.charts.c04)),
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
                                .push(create_chart(&mut self.charts.c41))
                                .push(create_chart(&mut self.charts.c42))
                                .push(create_chart(&mut self.charts.c43))
                                .push(create_chart(&mut self.charts.c44)),
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

pub fn create_chart<'chart, Message: 'chart>(
    chart: &'chart mut impl Chart<Message>,
) -> Container<'chart, Message> {
    Container::new(
        ChartWidget::new(chart)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(style::Instrument)
}
