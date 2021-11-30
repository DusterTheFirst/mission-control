//! Communication protocol used between the ground station and the vehicle
//!
//! FIXME: better docs
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![no_std]

/// Information about the physical link layer between the vehicle and ground station
pub mod phy {
    /// Enum containing all physical interlink methods
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum InterlinkMethod {
        /// Physical commination takes place over [`serial`]
        Serial,
    }

    /// Universal Serial Bus
    ///
    /// Structures and constants that help the vehicle and ground station communicate
    /// and identify each other
    pub mod serial {
        /// USB Vendor ID 0x1209
        ///
        /// http://voti.nl/pids/
        pub const VID: u16 = 0x16C0;
        /// USB Product ID 0x0001
        ///
        /// http://voti.nl/pids/
        pub const PID: u16 = 0x03E8;

        /// The size of the Serial buffer to use
        pub const BUFFER_SIZE: usize = 2048;

        /// The sentinel byte to delimit packets
        pub const COBS_SENTINEL: u8 = 0x00;
    }

    // TODO: :D
    // pub mod ble {}
    // pub mod rf {}
    // pub mod laser {}
}

/// Structures and types used to represent the communications protocol between the station and a vehicle
pub mod proto {
    use serde::{Deserialize, Serialize};

    /// Packet sent from the station to the vehicle
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum PacketUp {
        /// Packet sent to the vehicle upon connecting
        ///
        /// The vehicle should respond with a [`PacketDown::Hello`]
        Welcome
    }

    /// Packet sent from the vehicle to the station
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub enum PacketDown {
        /// Sent in response to [`PacketUp::Welcome`] to identify the vehicle
        Hello {
            /// Vehicle name
            name: heapless::String<32>,
            /// Vehicle firmware version
            version: heapless::String<32>,
        },
        /// Raw magnetometer data in nT (nanotesla)
        Magnetometer {
            /// X component
            x: i32,
            /// Y component
            y: i32,
            /// Z component
            z: i32,
        },
        /// Raw accelerometer data in mg (milli-g) where 1g is 9.8m/s²
        Accelerometer {
            /// X component
            x: i32,
            /// Y component
            y: i32,
            /// Z component
            z: i32,
        },
    }
}
