const CHAIN_MAC_BYTE: u8 = 0x2;
const MSG_MAC_BYTE: u8 = 0x1;

type MacImpl = hmac::Hmac<sha2::Sha256>;

/// Common trait for sending and receiving chains.
pub(super) trait Msg {
	type Error: core::error::Error;
	type Ok<'a> where Self: 'a;

	/// Moves chain forward by using [`kdf`] internally.
	///
	/// # Return
	///
	/// Useful keys and data to create or receive new messages.
	fn fwd(&mut self) -> Result<Self::Ok<'_>, Self::Error>;

	/// Builds new chain and message keys using `key`.
	fn kdf(
		key: &crate::key::MsgChain,
	) -> (crate::key::MsgChain, crate::key::Msg) {
		use hmac::Mac as _;

		// Create message authentication code builders using key.
		let mut chain_mac = MacImpl::new_from_slice(key).expect("Any size is good.");
		let mut msg_mac = chain_mac.clone();

		// Update builders with magic bytes mentioned in the specification
		chain_mac.update(&[CHAIN_MAC_BYTE]);
		msg_mac.update(&[MSG_MAC_BYTE]);

		// Finalize builders to key structs
		(
			From::<[u8; 32]>::from(chain_mac.finalize().into_bytes().into()),
			From::<[u8; 32]>::from(msg_mac.finalize().into_bytes().into()),
		)
	}
}
