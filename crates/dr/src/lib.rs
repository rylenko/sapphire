/*!
Double ratchet algorithm implementation for Sapphire.
*/

#![feature(error_in_core)]
#![no_std]

mod cipher;
mod error;
mod hdr;
pub mod key;
mod msg_chain;
mod recv;
mod root;
mod send;
mod skipped_msg_keys;
mod state;
mod utils;

pub use {state::State, utils::create_encrypted_hdr_buf};
