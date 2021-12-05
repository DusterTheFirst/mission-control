//! Structures and types used to represent the communications protocol between the station and a vehicle.

use derive_more::Deref;
use serde::{Deserialize, Serialize};

use crate::vehicle_time::VehicleTime;

/// Packet sent from the station to the vehicle.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PacketUp {
    /// Packet sent to the vehicle upon connecting.
    ///
    /// The vehicle should respond with a [`PacketDown::Hello`].
    Welcome,
}

/// Packet sent from the vehicle to the station.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PacketDown {
    /// When the packet was sent.
    pub time: VehicleTime,
    /// Data contained within the packet.
    pub data: PacketDownData,
}

/// Data portion of a packet sent from the vehicle to the station.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PacketDownData {
    /// Sent in response to [`PacketUp::Welcome`] to identify the vehicle.
    Hello(VehicleIdentification),
    /// Reading from the magnetometer
    Magnetometer(MagnetometerReading),
    /// Reading from the accelerometer
    Accelerometer(AccelerometerReading),
}

/// Identification information about a vehicle
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VehicleIdentification {
    /// Vehicle name.
    pub name: heapless::String<32>,
    /// Vehicle firmware version.
    pub version: heapless::String<32>,
}

/// Raw magnetometer data in nT (nanotesla).
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct MagnetometerReading(Vector3<i32>);

/// Raw accelerometer data in mg (milli-g) where 1g is 9.8m/s².
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct AccelerometerReading(Vector3<i32>);

/// Generic 3 component vector
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3<D> {
    /// X component.
    pub x: D,
    /// Y component.
    pub y: D,
    /// Z component.
    pub z: D,
}
