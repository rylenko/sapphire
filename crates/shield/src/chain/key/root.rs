type EvolveKdf = hkdf::Hkdf<sha2::Sha256>;

/// Root key on which the root chain of Double Ratchet based.
///
/// Is initially an agreed shared key.
#[derive(Clone, Debug, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub struct Root([u8; 32]);

impl Root {
	#[inline]
	#[must_use]
	pub const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}

	/// Creates new root, [master] and [header] keys based on the current root
	/// key and shared secret. The old root key is replaced with new one and
	/// [master] and [header] keys are returned.
	///
	/// [master]: super::master::Master
	/// [header]: super::header::Header
	pub(in crate::chain) fn evolve(
		&mut self,
		input: &x25519_dalek::SharedSecret,
	) -> (super::master::Master, super::header::Header) {
		// Derivation output, which contains root, master and header keys.
		let mut kdf_out = zeroize::Zeroizing::new([0; 96]);
		// Derive new root, master and header keys.
		EvolveKdf::new(Some(&self.0), input.as_bytes())
			.expand(b"root_evolve", kdf_out.as_mut())
			.expect("Any size is good.");

		// Replace current root key with a new one.
		self.0.copy_from_slice(&kdf_out[..32]);

		// Create a new master key using derived output bytes.
		let master_key = {
			let mut bytes = [0; 32];
			bytes.copy_from_slice(&kdf_out[32..64]);
			super::master::Master::new(bytes)
		};

		// Create a new header key using derived output bytes.
		let header_key = {
			let mut bytes = [0; 32];
			bytes.copy_from_slice(&kdf_out[64..96]);
			super::header::Header::new(bytes)
		};
		(master_key, header_key)
	}
}

impl From<[u8; 32]> for Root {
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
		// Get shared secret to use in evolving function as input material.
		let local_private_key = x25519_dalek::StaticSecret::from([0; 32]);
		let remote_public_key = x25519_dalek::PublicKey::from([1; 32]);
		let shared_secret =
			local_private_key.diffie_hellman(&remote_public_key);

		// Create a new root key and evolve it.
		let mut root_key = super::Root::new([2; 32]);
		let (master_key, header_key) = root_key.evolve(&shared_secret);

		assert_eq!(root_key.0, [
			122, 0, 73, 244, 151, 167, 77, 2, 73, 209, 48, 48, 180, 68, 122,
			248, 248, 93, 178, 175, 182, 143, 112, 149, 37, 86, 121, 129, 149,
			124, 179, 192
		]);
		assert_eq!(zerocopy::AsBytes::as_bytes(&master_key), [
			120, 204, 58, 134, 123, 152, 181, 11, 48, 190, 71, 206, 54, 192,
			155, 64, 25, 171, 211, 244, 236, 86, 153, 125, 137, 216, 160, 17,
			20, 141, 228, 12
		]);
		assert_eq!(zerocopy::AsBytes::as_bytes(&header_key), [
			195, 191, 253, 157, 70, 89, 78, 13, 16, 137, 91, 178, 228, 133,
			66, 141, 212, 53, 213, 118, 103, 217, 13, 165, 207, 39, 72, 34,
			163, 232, 149, 134
		]);
	}
}
