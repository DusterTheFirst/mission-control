use std::{collections::VecDeque, hash::Hash, thread, time::Duration};

use flume::{Receiver, Sender};
use iced::{futures::stream::BoxStream, Subscription};
use iced_native::subscription::Recipe;
use interlink::{phy, proto::PacketDown};
use serialport::{SerialPort, SerialPortType};
use tracing::{debug, error, trace, warn};

#[derive(Debug, Clone)]
pub struct SerialSubscription {
    receiver: Receiver<SerialEvent>,
}

#[derive(Debug, Clone)]
pub enum SerialEvent {
    PacketReceived,
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

        let port = try_find_serial_port();

        if let Some(port) = port {
            trace!("Connecting to board");
            let mut port = match serialport::new(port, 0).open() {
                Ok(port) => port,
                Err(error) => {
                    error!(%error, "Unable to open serial port");
                    continue;
                }
            };

            debug!("Connected to board");

            sender.send(SerialEvent::Connected).unwrap();

            port.write_data_terminal_ready(true).unwrap();

            match read_data_from_serial_port(port.as_mut(), &sender) {
                Ok(()) => {}
                Err(error) => error!(%error, "Failed to communicate to serial port"),
            }

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

fn read_data_from_serial_port(
    port: &mut dyn SerialPort,
    sender: &Sender<SerialEvent>,
) -> std::io::Result<()> {
    let mut data_storage = VecDeque::with_capacity(2048);

    // match port.write_all(
    //     postcard::to_allocvec_cobs(&PacketUp::Welcome)
    //         .unwrap()
    //         .as_slice(),
    // ) {
    //     Ok(()) => {}
    //     Err(e) => match e.kind() {
    //         ErrorKind::TimedOut => {
    //             warn!("Serial port not connected");
    //             return;
    //         }
    //         _ => panic!("{}", e),
    //     },
    // }

    loop {
        let mut input = [0u8; 64];

        let bytes_to_read = (port.bytes_to_read()? as usize).min(input.len());

        if bytes_to_read > 0 {
            let amount = port.read(&mut input[..bytes_to_read])?;
            data_storage.extend(&input[..amount]);
        }

        while !data_storage.is_empty() {
            let contiguous = data_storage.make_contiguous();
            let len = contiguous.len();
            match postcard::take_from_bytes_cobs::<PacketDown>(contiguous) {
                Ok((packet, leftovers)) => {
                    dbg!(packet);

                    let length = len - leftovers.len();

                    for _ in 0..length {
                        data_storage.pop_front(); // TODO: what the fuck is this
                    }
                    // Get rid of trailing null
                    data_storage.pop_front();

                    sender.send(SerialEvent::PacketReceived).unwrap(); // TODO: error handle
                }
                Err(postcard::Error::DeserializeUnexpectedEnd) => break,
                Err(error) => {
                    error!(%error, "Failed to deserialize data");
                    return Ok(());
                }
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
