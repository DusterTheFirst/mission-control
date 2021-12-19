#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::unwrap_in_result, clippy::missing_const_for_fn)]

use std::time::Duration;

use comm::serial::{SerialEvent, SerialSubscription};
use element::instrument::{
    data_view::{Accelerometer, DataView, Magnetometer},
    time_series::TimeSeriesInstrument,
    vector::VectorInstrument,
    InstrumentMessage,
};
use iced::{
    button, executor,
    keyboard::{self, KeyCode, Modifiers},
    pick_list,
    window::{self, Mode},
    Application, Clipboard, Color, Command, Container, Element, Length, Settings, Subscription,
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
use view::{acceleration, default, magnetic_field};

mod comm;
mod element;
mod style;
mod time_manager;
mod util;
mod view;

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
struct Instruments {
    magnetic_field_time: TimeSeriesInstrument<Magnetometer>,
    magnetic_field_vector: VectorInstrument<Magnetometer>,

    acceleration_time: TimeSeriesInstrument<Accelerometer>,
    acceleration_vector: VectorInstrument<Accelerometer>,
}

pub struct InstrumentCluster {
    quit: bool,
    window_focused: bool,
    window_mode: Mode,
    window_size: (u32, u32),

    time: TimeManager,
    time_base: TimeBase,

    instruments: Instruments,
    data_view: Option<DataView>,

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
    Instrument(InstrumentMessage),
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

                instruments: Instruments {
                    magnetic_field_time: TimeSeriesInstrument::new(5.0),
                    magnetic_field_vector: VectorInstrument::new(),

                    acceleration_time: TimeSeriesInstrument::new(5.0),
                    acceleration_vector: VectorInstrument::new(),
                },
                data_view: None,

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
                    PacketDownData::Magnetometer(reading) => {
                        self.instruments
                            .magnetic_field_time
                            .add_reading(time, reading);
                        self.instruments.magnetic_field_vector.set_reading(reading);
                    }
                    PacketDownData::Accelerometer(reading) => {
                        self.instruments
                            .acceleration_time
                            .add_reading(time, reading);
                        self.instruments.acceleration_vector.set_reading(reading);
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
            Message::Instrument(InstrumentMessage::Selected(data_view)) => {
                if self.data_view == Some(data_view) {
                    self.data_view.take();
                } else {
                    self.data_view.replace(data_view);
                }
            }
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
        Container::new(match self.data_view {
            None => default::view(self),
            Some(DataView::Accelerometer) => acceleration::view(self),
            Some(DataView::Magnetometer) => magnetic_field::view(self),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Window)
        .padding(10)
        .into()
    }

    fn mode(&self) -> window::Mode {
        self.window_mode
    }

    fn should_exit(&self) -> bool {
        self.quit
    }
}
