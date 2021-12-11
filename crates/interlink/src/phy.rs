//! Information about the physical link layer between the vehicle and ground station.

/// Enum containing all physical interlink methods.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterlinkMethod {
    /// Physical commination takes place over a [`serial`] port.
    Serial,
}

/// Universal Serial Bus.
///
/// Structures and constants that help the vehicle and ground station communicate
/// and identify each other.
pub mod serial {
    /// USB Vendor ID: 0x1209
    ///
    /// <http://voti.nl/pids/>
    pub const VID: u16 = 0x16C0;
    /// USB Product ID: 0x0001
    ///
    /// <http://voti.nl/pids/>
    pub const PID: u16 = 0x03E8;

    /// The size of the Serial buffer to use.
    pub const BUFFER_SIZE: usize = 2048;

    /// The sentinel byte to delimit packets.
    pub const COBS_SENTINEL: u8 = 0x00;
}

// TODO: :D
// pub mod ble {}
// pub mod rf {}
// pub mod laser {}
