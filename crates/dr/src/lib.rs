/*!
Double ratchet algorithm implementation for Sapphire.
*/

#![forbid(missing_docs)]
#![feature(error_in_core)]
#![no_std]

extern crate alloc;

mod code;
pub mod crypto;
#[cfg(any(test, feature = "default-crypto"))]
pub mod default_crypto;
mod state;

pub use state::{Num, State};
