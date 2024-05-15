/*!
Auxiliary things for encrypting and decrypting data.
*/

#![feature(error_in_core)]
#![no_std]

mod decrypt;
mod encrypt;
pub mod error;
mod kdf;
mod mac;

pub use {decrypt::decrypt, encrypt::encrypt};
