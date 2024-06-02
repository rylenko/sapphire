/*!
Double ratchet algorithm implementation for Sapphire.

TODO: Should I use simple `as_bytes()` to get key bytes instead of `zerocopy::AsBytes` derivation?
*/

#![feature(error_in_core)]
#![no_std]

mod chain;
mod header;
pub mod key;

/// Double Ratchet state of Alice or Bob.
#[derive(Clone)]
pub struct Shield {
	local_private_key: x25519_dalek::StaticSecret,
	remote_public_key: Option<x25519_dalek::PublicKey>,
	root_chain: chain::Root,
	receiving_chain: chain::Receiving,
	sending_chain: chain::Sending,
}

impl Shield {
	/// Creates new Alice's state.
	#[must_use]
	pub fn new_alice(
		bob_public_key: x25519_dalek::PublicKey,
		root_key: key::Root,
		sending_chain_header_key: key::Header,
		receiving_chain_next_header_key: key::Header,
		skipped_message_keys_max_len: usize,
	) -> Self {
		// Generate new local private key.
		let local_private_key = x25519_dalek::StaticSecret::random();

		// Create root chain.
		let mut root_chain = chain::Root::new(root_key);
		// Move root chain forward to get sending chain master and next header
		// keys.
		let (sending_chain_key, sending_chain_next_header_key) = root_chain
			.forward(&local_private_key.diffie_hellman(&bob_public_key));

		Self {
			local_private_key,
			remote_public_key: Some(bob_public_key),
			root_chain,
			receiving_chain: chain::Receiving::new(
				receiving_chain_next_header_key,
				skipped_message_keys_max_len,
			),
			sending_chain: chain::Sending::new(
				Some(sending_chain_key),
				Some(sending_chain_header_key),
				sending_chain_next_header_key,
			),
		}
	}
}
