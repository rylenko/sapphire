/*!
Default implementation of Double Ratchet cryptography with zeroization on drop.
*/

mod error;
mod header_key;
mod key_pair;
mod provider;
mod public_key;

pub use {key_pair::KeyPair, provider::Provider};
