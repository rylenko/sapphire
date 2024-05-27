/*!
Auxiliary things for encrypting and decrypting data.
*/

#![feature(error_in_core)]
#![no_std]

mod auth;
pub mod decrypt;
mod encrypt;
mod key;

pub use {auth::Tag, decrypt::decrypt, encrypt::encrypt};
