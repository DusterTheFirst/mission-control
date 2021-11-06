use chart::MyChart;
use chart_container::create_chart;
use iced::{
    button, executor,
    keyboard::{self, KeyCode, Modifiers},
    widget,
    window::{self, Mode},
    Application, Button, Clipboard, Color, Command, Element, HorizontalAlignment, Length, Settings,
    Subscription, Text,
};
use iced_native::{event, subscription, Event};

use crate::style::ControlCluster;

mod chart;
mod chart_container;
mod style;

pub fn main() -> iced::Result {
    InstrumentCluster::run(Settings {
        antialiasing: true,
        exit_on_close_request: true,
        window: window::Settings {
            min_size: Some((800, 760)),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Default)]
struct Charts {
    c00: MyChart<Message>,
    c01: MyChart<Message>,
    c02: MyChart<Message>,
    c03: MyChart<Message>,
    c04: MyChart<Message>,
    c05: MyChart<Message>,
    c06: MyChart<Message>,
    c07: MyChart<Message>,
    c08: MyChart<Message>,
    c09: MyChart<Message>,
    c10: MyChart<Message>,
    c11: MyChart<Message>,
    c12: MyChart<Message>,
    c13: MyChart<Message>,
    c14: MyChart<Message>,
    c15: MyChart<Message>,
    c16: MyChart<Message>,
    c17: MyChart<Message>,
    c18: MyChart<Message>,
    c19: MyChart<Message>,
    c20: MyChart<Message>,
    c21: MyChart<Message>,
    c22: MyChart<Message>,
    c23: MyChart<Message>,
    c24: MyChart<Message>,
    c25: MyChart<Message>,
    c26: MyChart<Message>,
    c27: MyChart<Message>,
    c28: MyChart<Message>,
    c29: MyChart<Message>,
    c30: MyChart<Message>,
    c31: MyChart<Message>,
    c32: MyChart<Message>,
    c33: MyChart<Message>,
    c34: MyChart<Message>,
    c35: MyChart<Message>,
    c36: MyChart<Message>,
    c37: MyChart<Message>,
    c38: MyChart<Message>,
    c39: MyChart<Message>,
    c40: MyChart<Message>,
    c41: MyChart<Message>,
    c42: MyChart<Message>,
    c43: MyChart<Message>,
    c44: MyChart<Message>,
    c45: MyChart<Message>,
    c46: MyChart<Message>,
    c47: MyChart<Message>,
    c48: MyChart<Message>,
    c49: MyChart<Message>,
}

#[derive(Debug)]
struct InstrumentCluster {
    quit: bool,
    window_focused: bool,
    window_mode: Mode,
    window_size: (u32, u32),

    charts: Charts,

    quit_button: button::State,
    fullscreen_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Quit,
    ToggleFullscreen,
    WindowFocusChange(bool),
    WindowSizeChange((u32, u32)),
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
                window_mode: Mode::Fullscreen,
                window_size: (0, 0),

                charts: Charts::default(),

                quit_button: button::State::default(),
                fullscreen_button: button::State::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Instrument Cluster")
    }

    fn background_color(&self) -> Color {
        style::colors::BACKGROUND
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
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
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
        })
    }

    fn view(&mut self) -> Element<Self::Message> {
        widget::Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                widget::Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(widget::Space::new(Length::Fill, Length::Fill))
                    .push(create_chart(&mut self.charts.c01))
                    .push(create_chart(&mut self.charts.c02))
                    .push(create_chart(&mut self.charts.c03))
                    .push(create_chart(&mut self.charts.c04)),
            )
            .push(
                widget::Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(create_chart(&mut self.charts.c10))
                    .push(widget::Space::new(Length::Fill, Length::FillPortion(4))),
            )
            .push(
                widget::Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(create_chart(&mut self.charts.c20))
                    .push(widget::Space::new(Length::Fill, Length::Fill))
                    .push(
                        widget::Container::new(
                            widget::Column::new()
                                .push(
                                    widget::Text::new("Control Cluster")
                                        .size(32)
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .width(Length::Fill),
                                )
                                .push(widget::Space::new(Length::Fill, Length::Fill))
                                .push(
                                    widget::Text::new(format!(
                                        "Window Size: {:?}",
                                        self.window_size
                                    ))
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .width(Length::Fill),
                                )
                                .push(
                                    widget::Text::new(format!("Focused: {}", self.window_focused))
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .width(Length::Fill),
                                )
                                .push(
                                    widget::Row::new()
                                        .push(
                                            Button::new(
                                                &mut self.fullscreen_button,
                                                Text::new(match self.window_mode {
                                                    Mode::Windowed => "Fullscreen",
                                                    Mode::Fullscreen => "Windowed",
                                                }),
                                            )
                                            .on_press(Message::ToggleFullscreen)
                                            .style(ControlCluster),
                                        )
                                        .push(widget::Space::new(Length::Fill, Length::Shrink))
                                        .push(
                                            Button::new(&mut self.quit_button, Text::new("Quit"))
                                                .on_press(Message::Quit)
                                                .style(ControlCluster),
                                        ),
                                ),
                        )
                        .width(Length::Fill)
                        .height(Length::FillPortion(2))
                        .padding(10)
                        .style(ControlCluster),
                    )
                    .push(widget::Space::new(Length::Fill, Length::Fill)),
            )
            .push(
                widget::Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(create_chart(&mut self.charts.c30))
                    .push(widget::Space::new(Length::Fill, Length::FillPortion(4))),
            )
            .push(
                widget::Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(widget::Space::new(Length::Fill, Length::Fill))
                    .push(create_chart(&mut self.charts.c41))
                    .push(create_chart(&mut self.charts.c42))
                    .push(create_chart(&mut self.charts.c43))
                    .push(create_chart(&mut self.charts.c44)),
            )
            .into()
    }

    fn mode(&self) -> window::Mode {
        self.window_mode
    }

    fn should_exit(&self) -> bool {
        self.quit
    }
}
