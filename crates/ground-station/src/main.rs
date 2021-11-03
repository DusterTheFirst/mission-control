use chart::MyChart;
use iced::{
    button, executor, widget,
    window::{self, Mode},
    Application, Button, Clipboard, Command, Container, Element, Length, Settings, Text,
};
use plotters_iced::ChartWidget;

mod chart;

pub fn main() -> iced::Result {
    GroundStation::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Debug, Default)]
struct GroundStation {
    quit: bool,

    chart: MyChart<Message>,

    quit_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Quit,
}

impl Application for GroundStation {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::Quit => self.quit = true,
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        widget::Column::new()
            .push(Button::new(&mut self.quit_button, Text::new("Quit")).on_press(Message::Quit))
            .push(
                Container::new(
                    ChartWidget::new(&mut self.chart)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(chart::style::ChartContainer),
            )
            .into()
    }

    fn mode(&self) -> window::Mode {
        Mode::Fullscreen
    }

    fn should_exit(&self) -> bool {
        self.quit
    }
}
