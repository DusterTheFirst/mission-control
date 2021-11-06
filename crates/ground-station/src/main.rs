use chart::Instrument;
use iced::{
    button, executor,
    keyboard::{self, KeyCode, Modifiers},
    widget,
    window::{self, Mode},
    Align, Application, Button, Clipboard, Color, Command, Container, Element, HorizontalAlignment,
    Length, Settings, Subscription, Text,
};
use iced_native::{event, subscription, Event};
use plotters_iced::{Chart, ChartWidget};

mod chart;
mod style;

pub fn main() -> iced::Result {
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
        widget::Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            .padding(10)
            .push(
                widget::Row::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .spacing(10)
                    .push(widget::Space::new(Length::Fill, Length::Fill))
                    .push(create_chart(&mut self.charts.c10))
                    .push(create_chart(&mut self.charts.c20))
                    .push(create_chart(&mut self.charts.c30))
                    .push(widget::Space::new(Length::Fill, Length::Fill)),
            )
            .push(
                widget::Row::new()
                    .width(Length::Fill)
                    .height(Length::FillPortion(4))
                    .spacing(10)
                    .push(
                        widget::Column::new()
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .spacing(10)
                            .push(create_chart(&mut self.charts.c01))
                            .push(create_chart(&mut self.charts.c02))
                            .push(create_chart(&mut self.charts.c03))
                            .push(create_chart(&mut self.charts.c04)),
                    )
                    .push(
                        widget::Container::new(
                            widget::Container::new(
                                widget::Column::new()
                                    .push(
                                        widget::Text::new("Control Cluster")
                                            .size(32)
                                            .horizontal_alignment(HorizontalAlignment::Center),
                                    )
                                    .push(
                                        widget::Text::new(format!(
                                            "Window Size: {:?}",
                                            self.window_size
                                        ))
                                        .horizontal_alignment(HorizontalAlignment::Center),
                                    )
                                    .push(
                                        widget::Text::new(format!(
                                            "Focused: {}",
                                            self.window_focused
                                        ))
                                        .horizontal_alignment(HorizontalAlignment::Center),
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
                        widget::Column::new()
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .spacing(10)
                            .push(create_chart(&mut self.charts.c41))
                            .push(create_chart(&mut self.charts.c42))
                            .push(create_chart(&mut self.charts.c43))
                            .push(create_chart(&mut self.charts.c44)),
                    ),
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
