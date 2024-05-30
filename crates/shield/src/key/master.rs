type MacImpl = hmac::Hmac<sha2::Sha256>;

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
	pub(crate) const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}

	/// Creates new master and [message] keys based on the current master key.
	/// The old master key is replaced with the new one and the [message] key
	/// is returned.
	///
	/// [message]: super::message::Message
	#[must_use]
	pub(crate) fn evolve(&mut self) -> super::message::Message {
		// Create message authenticator based on the current key.
		let mut mac: MacImpl =
			hmac::Mac::new_from_slice(&self.0).expect("Any size is good.");

		// Update authenticator with magic master byte specified by the
		// protocol.
		hmac::Mac::update(&mut mac, &[0x2]);
		// Finalize new master key bytes into an array. Do not forget to reset
		// authenticator to derive new message key later.
		let master_bytes: [u8; 32] =
			Into::into(hmac::Mac::finalize_reset(&mut mac).into_bytes());

		// Update authenticator with magic message byte specified by the
		// protocol.
		hmac::Mac::update(&mut mac, &[0x1]);
		// Finalize new message key bytes into an array.
		let message_bytes: [u8; 32] =
			Into::into(hmac::Mac::finalize(mac).into_bytes());

		// Replace old master key bytes with new bytes.
		self.0 = master_bytes;
		// Return evolved new message key.
		super::message::Message::new(message_bytes)
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
		let message_key = master_key.evolve();
		assert_eq!(master_key.0, [
			209, 212, 226, 169, 74, 9, 90, 229, 50, 123, 154, 102, 0, 50, 194,
			34, 37, 174, 119, 16, 130, 63, 153, 124, 194, 80, 243, 30, 93, 58,
			235, 80
		]);
		assert_eq!(zerocopy::AsBytes::as_bytes(&message_key), &[
			13, 139, 27, 91, 217, 40, 164, 207, 171, 103, 8, 182, 175, 111,
			225, 93, 93, 65, 179, 38, 142, 109, 216, 237, 156, 91, 14, 205,
			225, 12, 164, 162
		]);
	}
}
