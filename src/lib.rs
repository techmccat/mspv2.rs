//! Multiwii Serial Protocol (MSP) traffic decoder and structures
//!
//! Incomplete. Includes some structures from Cleanflight and Betaflight.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

mod commands;
mod packet;
pub mod structs;

pub use commands::*;
pub use packet::*;
