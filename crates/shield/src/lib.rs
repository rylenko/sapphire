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
		skip_message_keys_limit: u32,
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
				skip_message_keys_limit,
			),
			sending_chain: chain::Sending::new(
				Some(sending_chain_key),
				Some(sending_chain_header_key),
				sending_chain_next_header_key,
			),
		}
	}

	/// Creates new Bob's state.
	#[must_use]
	pub fn new_bob(
		private_key: x25519_dalek::StaticSecret,
		root_key: key::Root,
		sending_chain_next_header_key: key::Header,
		receiving_chain_next_header_key: key::Header,
		skip_message_keys_limit: u32,
	) -> Self {
		Self {
			local_private_key: private_key,
			remote_public_key: None,
			root_chain: chain::Root::new(root_key),
			receiving_chain: chain::Receiving::new(
				receiving_chain_next_header_key,
				skip_message_keys_limit,
			),
			sending_chain: chain::Sending::new(
				None,
				None,
				sending_chain_next_header_key,
			),
		}
	}

	/// Upgrades chain master keys using local [private key] and passed new
	/// remote [public key].
	///
	/// With the leakage of the [master chain key]s, subsequent [message key]s
	/// leak as well. This algorithm prevents this by upgrading the chains with
	/// new keys derived from the Diffie-Hellman output.
	///
	/// [private key]: x25519_dalek::StaticSecret
	/// [public key]: x25519_dalek::PublicKey
	/// [master chain key]: crate::key::Master
	/// [message key]: crate::key::Message
	fn diffie_hellman_ratchet(
		&mut self,
		remote_public_key: x25519_dalek::PublicKey,
	) {
		// Derive new master and next header keys for the receiving chain.
		let (master_key, next_header_key) = self.root_chain.forward(
			&self.local_private_key.diffie_hellman(&remote_public_key),
		);
		// Upgrade receiving chain with derived new master and next header
		// keys.
		self.receiving_chain.upgrade(master_key, next_header_key);

		// Generate new private key to upgrade sending chain using it.
		self.local_private_key = x25519_dalek::StaticSecret::random();

		// Derive new master and next header keys for the sending chain.
		let (master_key, next_header_key) = self.root_chain.forward(
			&self.local_private_key.diffie_hellman(&remote_public_key),
		);
		// Upgrade sending chain with derived new master and next header keys.
		self.sending_chain.upgrade(master_key, next_header_key);

		// Update remote public key.
		self.remote_public_key = Some(remote_public_key);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_diffie_hellman_ratchet() {
		// Create Alice and Bob.
		let mut bob = super::Shield::new_bob(
			x25519_dalek::StaticSecret::random(),
			crate::key::Root::new([1; 32]),
			crate::key::Header::new([2; 32]),
			crate::key::Header::new([3; 32]),
			123,
		);
		let mut alice = super::Shield::new_alice(
			x25519_dalek::PublicKey::from(&bob.local_private_key),
			crate::key::Root::new([1; 32]),
			crate::key::Header::new([3; 32]),
			crate::key::Header::new([2; 32]),
			321,
		);

		// Run Diffie-Hellman ratchet on the Bob's side using Alice's public
		// key.
		bob.diffie_hellman_ratchet(x25519_dalek::PublicKey::from(
			&alice.local_private_key,
		));

		// Test that root chains are not equal because of Bob's root chain
		// double forward moving.
		assert_ne!(bob.root_chain, alice.root_chain);

		// Compare Bob's receiving and Alice's sending chains.
		assert_eq!(
			bob.receiving_chain.master_key(),
			alice.sending_chain.master_key()
		);
		assert_eq!(
			bob.receiving_chain.header_key(),
			alice.sending_chain.header_key()
		);
		assert_eq!(
			bob.receiving_chain.next_header_key(),
			alice.sending_chain.next_header_key()
		);

		// But test, that Bob's sending chain are not equal to Alice's
		// receiving chain because of Bob's root chain double forward moving.
		assert_ne!(
			bob.sending_chain.master_key(),
			alice.receiving_chain.master_key()
		);
		assert_ne!(
			bob.sending_chain.header_key(),
			alice.receiving_chain.header_key()
		);
		assert_ne!(
			bob.sending_chain.next_header_key(),
			alice.receiving_chain.next_header_key()
		);
		// But Bob's header key equals to Alice's next header key.
		assert_eq!(
			bob.sending_chain.header_key(),
			Some(alice.receiving_chain.next_header_key())
		);

		// Run Diffie-Hellman ratchet on the ALice's side using Bob's public
		// key.
		alice.diffie_hellman_ratchet(x25519_dalek::PublicKey::from(
			&bob.local_private_key,
		));

		// Test that root chains are not equal because of Alice's root chain
		// double forward moving.
		assert_ne!(alice.root_chain, bob.root_chain);

		// Compare Alice's receiving and Bob's sending chains after Alice's
		// ratchet.
		assert_eq!(
			alice.receiving_chain.master_key(),
			bob.sending_chain.master_key()
		);
		assert_eq!(
			alice.receiving_chain.header_key(),
			bob.sending_chain.header_key()
		);
		assert_eq!(
			alice.receiving_chain.next_header_key(),
			bob.sending_chain.next_header_key()
		);

		// But test, that Alice's sending chain are not equal to Bob's
		// receiving chain because of ALice's root chain double forward moving.
		assert_ne!(
			alice.sending_chain.master_key(),
			bob.receiving_chain.master_key()
		);
		assert_ne!(
			alice.sending_chain.header_key(),
			bob.receiving_chain.header_key()
		);
		assert_ne!(
			alice.sending_chain.next_header_key(),
			bob.receiving_chain.next_header_key()
		);
		// But Bob's header key equals to Alice's next header key.
		assert_eq!(
			alice.sending_chain.header_key(),
			Some(bob.receiving_chain.next_header_key())
		);
	}
}
