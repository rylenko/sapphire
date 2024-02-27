/// Root chain of [`State`].
///
/// [`State`]: super::state::State.
#[derive(Clone, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub(super) struct Root {
	/// Is initially a shared secret. Later is the output root key of KDF.
	key: super::key::Root,
}

impl Root {
	#[inline]
	#[must_use]
	pub(super) const fn new(key: super::key::Root) -> Self {
		Self { key }
	}

	/// Key derivation function.
	///
	/// # Return
	///
	/// New chain and header keys.
	pub(super) fn kdf(
		&mut self,
		input: &super::key::SharedSecret,
	) -> (super::key::MsgChain, super::key::Hdr) {
		const HKDF_OUT_SIZE: usize = 96;
		const HKDF_INFO: &[u8] = b"root_kdf_info";

		// Get output key material with new keys via HKDF
		let mut hkdf_out = zeroize::Zeroizing::new([0; HKDF_OUT_SIZE]);
		hkdf::Hkdf::<sha2::Sha256>::new(
			Some(self.key.as_bytes()),
			input.as_bytes(),
		)
		.expand(HKDF_INFO, hkdf_out.as_mut())
		.expect("`HKDF_OUT_SIZE` must be a good length.");

		// Split output into keys
		self.key.copy_from_slice(&hkdf_out[..32]);
		let mut msg_chain_key = super::key::MsgChain::from([0; 32]);
		msg_chain_key.copy_from_slice(&hkdf_out[32..64]);
		let mut hdr_key = super::key::Hdr::from([0; 32]);
		hdr_key.copy_from_slice(&hkdf_out[64..]);

		(msg_chain_key, hdr_key)
	}

	#[cfg(test)]
	#[inline]
	#[must_use]
	pub(super) const fn key(&self) -> &super::key::Root {
		&self.key
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_kdf() {
		// Create chain
		let key = super::super::key::Root::from([1; 32]);
		let mut chain = super::Root::new(key.clone());

		// Create local and remote keys
		let remote_public_key = super::super::key::Public::from([222; 32]);
		let local_private_key = super::super::key::Private::random();

		// Calculate DH input and use KDF
		chain.kdf(&local_private_key.dh(remote_public_key));
		assert_ne!(chain.key, key);
	}
}
