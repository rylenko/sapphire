/// Common trait for receiving and sending chains.
pub(super) trait MsgChain {
	type KdfError: core::error::Error;
	type KdfOk<'a>
	where
		Self: 'a;

	/// Wrapper for [`kdf_inner`].
	///
	/// [`kdf_inner`]: Self::kdf_inner
	fn kdf(&mut self) -> Result<Self::KdfOk<'_>, Self::KdfError>;

	/// Moves chain forward. Common implementation for all. Should be wrapped
	/// with [kdf].
	///
	/// [kdf]: Self::kdf
	fn kdf_inner(
		key: &super::key::MsgChain,
	) -> (super::key::MsgChain, super::key::Msg) {
		use hkdf::hmac::Mac as _;

		const CHAIN_KEY_MAC_BYTE: u8 = 0x2;
		const MSG_KEY_MAC_BYTE: u8 = 0x1;

		// Create mac with key bytes as key
		let mut chain_mac =
			hkdf::hmac::Hmac::<sha2::Sha256>::new_from_slice(key.as_bytes())
				.expect("Any size is good.");
		let mut msg_mac = chain_mac.clone();

		// Update key HMACs with their bytes
		chain_mac.update(&[CHAIN_KEY_MAC_BYTE]);
		msg_mac.update(&[MSG_KEY_MAC_BYTE]);

		// Finalize HMACs
		(
			From::<[u8; 32]>::from(chain_mac.finalize().into_bytes().into()),
			From::<[u8; 32]>::from(msg_mac.finalize().into_bytes().into()),
		)
	}

	/// Upgrades the chain after Diffie-Hellman ratchet.
	fn upgrade(
		&mut self,
		new_key: super::key::MsgChain,
		new_next_header_key: super::key::Header,
	);
}
