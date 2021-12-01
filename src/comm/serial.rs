use std::{
    hash::Hash,
    io::{BufRead, BufReader, ErrorKind},
    thread,
    time::Duration,
};

use flume::{Receiver, Sender};
use iced::{futures::stream::BoxStream, Subscription};
use iced_native::subscription::Recipe;
use interlink::{phy, proto::{PacketDown, PacketUp}};
use serialport::SerialPortType;
use tracing::{debug, error, trace, warn};

#[derive(Debug, Clone)]
pub struct SerialSubscription {
    receiver: Receiver<SerialEvent>,
}

#[derive(Debug, Clone)]
pub enum SerialEvent {
    PacketReceived(PacketDown),
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
            let mut port = match serialport::new(port, 0)
                .flow_control(serialport::FlowControl::Hardware)
                .timeout(Duration::from_millis(10))
                .open()
            {
                Ok(port) => port,
                Err(error) => {
                    error!(%error, "Unable to open serial port");
                    continue;
                }
            };

            debug!("Connected to board");

            sender.send(SerialEvent::Connected).unwrap();

            port.write_data_terminal_ready(true).unwrap();

            // TODO: better
            match port.write_all(
                postcard::to_allocvec_cobs(&PacketUp::Welcome)
                    .unwrap()
                    .as_slice(),
            ) {
                Ok(()) => {}
                Err(e) => match e.kind() {
                    ErrorKind::TimedOut => {
                        warn!("Serial port not connected");
                        return;
                    }
                    _ => panic!("{}", e),
                },
            }
            
            let mut data_storage = Vec::with_capacity(phy::serial::BUFFER_SIZE);
            let mut buffered_port = BufReader::with_capacity(9, port.as_mut());

            loop {
                let amount = match buffered_port.read_until(phy::serial::COBS_SENTINEL, &mut data_storage) {
                    Ok(amount) => amount,
                    Err(error) if error.kind() == ErrorKind::TimedOut => {
                        /* Suppress time outs */
                        continue;
                    }
                    Err(error) => {
                        error!(%error, "Failed to communicate to serial port");
                        break;
                    }
                };

                if amount > phy::serial::BUFFER_SIZE {
                    trace!(
                        "Received {} bytes more than expected over serial",
                        amount - phy::serial::BUFFER_SIZE
                    );
                }

                match postcard::from_bytes_cobs::<PacketDown>(&mut data_storage[..amount]) {
                    Ok(packet) => {
                        // TODO: error handle
                        sender.send(SerialEvent::PacketReceived(packet)).unwrap();
                    }
                    Err(error) => {
                        error!(%error, "Failed to deserialize data");
                        break;
                    }
                }

                data_storage.clear();
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
