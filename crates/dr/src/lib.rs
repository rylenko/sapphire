/*!
Double ratchet algorithm implementation for Sapphire.
*/

#![feature(error_in_core)]
#![no_std]

#[macro_use]
mod macros;
mod cipher;
mod clue;
mod draft;
mod error;
mod hdr;
pub mod key;
mod msg_chain;
mod recv;
mod root;
mod send;
mod skipped_msg_keys;
mod state;

pub use state::State;
