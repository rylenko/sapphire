/*!
Double ratchet algorithm implementation for Sapphire.
*/

#![feature(error_in_core)]
#![no_std]

extern crate alloc;

pub mod crypto;
#[cfg(any(test, feature = "default-crypto"))]
pub mod default_crypto;
mod state;

pub use state::State;
