/*!
Auxiliary things for encrypting and decrypting data.
*/

#![feature(error_in_core)]
#![no_std]

mod auth;
mod decrypt;
mod encrypt;
pub mod error;
mod key;

pub use {auth::Tag, decrypt::decrypt, encrypt::encrypt};
