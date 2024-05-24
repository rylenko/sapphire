type EvolveMacImpl = hmac::Hmac<sha2::Sha256>;

/// Chain key on which [sending] and [receiving] chains are based.
///
/// [receiving]: super::recv::Recv
/// [sending]: super::send::Send
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	PartialEq,
	zeroize::ZeroizeOnDrop,
	zerocopy::AsBytes,
)]
#[repr(transparent)]
pub(crate) struct Master([u8; 32]);

impl Master {
	/// Builds the new chain key using passed `bytes`.
	#[inline]
	#[must_use]
	pub(super) const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}

	/// Creates new master and [message] keys based on the current master key.
	/// The old master key is replaced with the new one and the [message] key
	/// is returned.
	///
	/// [message]: super::msg::Msg
	pub(crate) fn evolve(&mut self) -> super::msg::Msg {
		// Create message authentication code builders using current key.
		let mut master_mac: EvolveMacImpl =
			hmac::Mac::new_from_slice(&self.0).expect("Any size is good.");
		let mut msg_mac = master_mac.clone();

		// Update builders with magic bytes mentioned in the specification.
		hmac::Mac::update(&mut master_mac, &[0x2]);
		hmac::Mac::update(&mut msg_mac, &[0x1]);

		// Finalize builders to get new key bytes.
		let master_bytes: [u8; 32] =
			Into::into(hmac::Mac::finalize(master_mac).into_bytes());
		let msg_bytes: [u8; 32] =
			Into::into(hmac::Mac::finalize(msg_mac).into_bytes());

		// Replace old master key bytes with new bytes.
		self.0.copy_from_slice(&master_bytes);
		// Create new message key.
		Into::into(msg_bytes)
	}
}

impl From<[u8; 32]> for Master {
	#[inline]
	#[must_use]
	fn from(bytes: [u8; 32]) -> Self {
		Self::new(bytes)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_evolve() {
		let mut master_key = super::Master::new([5; 32]);
		let msg_key = master_key.evolve();
		assert_eq!(master_key.0, [
			209, 212, 226, 169, 74, 9, 90, 229, 50, 123, 154, 102, 0, 50, 194,
			34, 37, 174, 119, 16, 130, 63, 153, 124, 194, 80, 243, 30, 93, 58,
			235, 80
		]);
		assert_eq!(zerocopy::AsBytes::as_bytes(&msg_key), &[
			13, 139, 27, 91, 217, 40, 164, 207, 171, 103, 8, 182, 175, 111,
			225, 93, 93, 65, 179, 38, 142, 109, 216, 237, 156, 91, 14, 205,
			225, 12, 164, 162
		]);
	}
}
