use std::{collections::VecDeque, hash::Hash, io::ErrorKind, thread, time::Duration};

use flume::{Receiver, Sender};
use iced::{futures::stream::BoxStream, Subscription};
use iced_native::subscription::Recipe;
use interlink::{phy, proto::Packet};
use serialport::{SerialPort, SerialPortType};
use tracing::{debug, error, trace, warn};

#[derive(Debug, Clone)]
pub struct SerialSubscription {
    receiver: Receiver<SerialEvent>,
}

#[derive(Debug, Clone, Copy)]
pub enum SerialEvent {
    PacketReceived { packet: Packet },
    Connected,
    Disconnected,
}

impl SerialSubscription {
    pub fn start(refresh_interval: Duration) -> Self {
        let (sender, receiver) = flume::unbounded();

        thread::spawn(move || serial_listener(sender, refresh_interval));

        Self { receiver }
    }

    pub fn subscription(&self) -> Subscription<SerialEvent> {
        Subscription::from_recipe(self.clone())
    }
}

impl<Hasher, Event> Recipe<Hasher, Event> for SerialSubscription
where
    Hasher: std::hash::Hasher,
{
    type Output = SerialEvent;

    fn hash(&self, state: &mut Hasher) {
        // Only one of these should ever exist due to its reconnecting nature
        (phy::serial::VID, phy::serial::PID).hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<Event>) -> BoxStream<Self::Output> {
        Box::pin(self.receiver.into_stream())
    }
}

pub fn serial_listener(sender: Sender<SerialEvent>, refresh_interval: Duration) {
    trace!("Serial subscription spawned");

    let mut first_retry = true;

    loop {
        if first_retry {
            debug!("Searching for board");
        }

        let device = try_find_serial_port();

        if let Some(port) = device {
            trace!("Connecting to board");
            let mut port = serialport::new(port, 0)
                .open()
                .expect("unable to open serial port");

            debug!("Connected to board");

            sender.send(SerialEvent::Connected).unwrap();

            port.write_data_terminal_ready(true).unwrap();

            read_data_from_serial_port(port.as_mut(), &sender);

            // Set the DTR signal low if performing a graceful shutdown
            port.write_data_terminal_ready(false).ok();

            sender.send(SerialEvent::Disconnected).unwrap();

            trace!("Closing serial connection");

            first_retry = true;
        } else {
            if first_retry {
                warn!("No board connected");
            }
            first_retry = false;
        }

        thread::sleep(refresh_interval);
    }
}

fn read_data_from_serial_port(port: &mut dyn SerialPort, sender: &Sender<SerialEvent>) {
    let mut all_data = VecDeque::<u8>::with_capacity(2048);
    let mut buffer = [0u8; 1028];

    loop {
        let new_data = match port.read(&mut buffer[..]) {
            Ok(amount) => &buffer[..amount],
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {
                    warn!("Serial port disconnected");
                    break;
                }
                _ => {
                    panic!("{}", e);
                }
            },
        };

        // Push the new data into the queue
        all_data.extend(new_data);
        let len = all_data.len();

        match postcard::take_from_bytes_cobs::<Packet>(all_data.make_contiguous()) {
            Ok((packet, leftovers)) => {
                for _ in 0..len - leftovers.len() {
                    all_data.pop_back(); // TODO: what the fuck is this
                }

                sender.send(SerialEvent::PacketReceived { packet }).unwrap(); // TODO: error handle
            }
            Err(postcard::Error::DeserializeUnexpectedEnd) => continue,
            Err(error) => {
                error!(?error, "Failed to deserialize data");
                break;
            }
        }
    }
}

pub fn try_find_serial_port() -> Option<String> {
    serialport::available_ports()
        .unwrap()
        .into_iter()
        .find_map(|port| match port.port_type {
            SerialPortType::UsbPort(usb) => {
                if usb.vid == phy::serial::VID && usb.pid == phy::serial::PID {
                    Some(port.port_name)
                } else {
                    None
                }
            }
            _ => None,
        })
}
