/// Root chain of [State](crate::state::State).
#[repr(transparent)]
pub(in crate::state) struct Root<P>
where
	P: crate::crypto::Provider,
{
	/// Is initially a shared secret. Later is the output root key of KDF.
	key: P::RootChainKey,
}

impl<P> Root<P>
where
	P: crate::crypto::Provider,
{
	#[inline]
	#[must_use]
	pub(in crate::state) const fn new(key: P::RootChainKey) -> Self {
		Self { key }
	}

	/// Key derivation function.
	///
	/// # Return
	///
	/// New chain and header keys.
	pub(in crate::state) fn kdf(
		&mut self,
		input: &P::SharedSecret,
	) -> (P::MsgChainKey, P::HeaderKey) {
		let (root_key, chain_key, header_key) =
			P::kdf_root_chain(&self.key, input);
		self.key = root_key;
		(chain_key, header_key)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_kdf() {
		use crate::crypto::{KeyPair as _, Provider as _};

		// Create chain
		let key = <crate::default_crypto::Provider as crate::crypto::Provider>
			::RootChainKey::from([100; 32]);
		let mut chain =
			super::Root::<crate::default_crypto::Provider>::new(key.clone());

		// Create local and remote keys
		let remote_public_key = <crate::default_crypto::KeyPair as crate::crypto::KeyPair>
			::Public::from([222; 32]);
		let local_key_pair = crate::default_crypto::KeyPair::rand();

		// Calculate Diffie-Hellman input and use KDF
		let input = crate::default_crypto::Provider::dh(
			&local_key_pair,
			&remote_public_key,
		);
		chain.kdf(&input);

		// TODO: find new key value with third-party resource
		// TODO: more asserts?
		assert_ne!(chain.key, key);
	}
}
