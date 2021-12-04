//! Communication protocol used between the ground station and the vehicle.
//!
//! FIXME: better docs
#![forbid(unsafe_code)]
#![deny(
    clippy::unwrap_used,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::trivially_copy_pass_by_ref
)]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::missing_errors_doc,
    clippy::missing_const_for_fn
)]
#![no_std]

pub mod phy;
pub mod proto;
pub mod vehicle_time;
