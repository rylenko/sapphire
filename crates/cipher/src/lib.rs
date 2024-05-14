/*!
Auxiliary things for encrypting and decrypting data.
*/

mod encrypt;
// pub mod error;
mod kdf;
mod mac;

pub use encrypt::encrypt;
