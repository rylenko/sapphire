/*!
Double ratchet algorithm implementation for Sapphire.
*/

#![feature(error_in_core)]
#![no_std]

extern crate alloc;

mod cipher;
mod error;
mod header;
pub mod key;
mod msg_chain;
mod recv;
mod root;
mod send;
mod skipped_msg_keys;
mod state;

pub use state::State;
