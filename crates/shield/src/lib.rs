/*!
Double ratchet algorithm implementation for Sapphire.

TODO: Should I use simple `as_bytes()` to get key bytes instead of `zerocopy::AsBytes` derivation?
*/

#![feature(error_in_core)]
#![no_std]

mod chain;
mod header;
pub mod key;
