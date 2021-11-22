//! Communication protocol used between the ground station and the device
//!
//! FIXME: better docs
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![no_std]

/// Information about the physical link layer between the device and ground station
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
    /// Structures and constants that help the device and ground station communicate
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
    }

    // TODO: :D
    // pub mod ble {}
    // pub mod rf {}
    // pub mod laser {}
}

/// Structures and types used to represent the communications protocol between the station and a device
pub mod proto {
    use serde::{Deserialize, Serialize};

    /// Packet sent between station and device
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Packet {
        /// TODO: Replace me with something meaningful
        TEST,
    }
}
