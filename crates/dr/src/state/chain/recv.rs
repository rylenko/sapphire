/// Receiving chain for Alice and Bob.
pub(in crate::state) struct Recv<P>
where
	P: crate::crypto::Provider,
{
	/// Is initially a shared secret. Later is the next header key.
	header_key: Option<P::HeaderKey>,

	/// Output chain key of KDF when receiving messages.
	key: Option<P::MsgChainKey>,

	/// Is initially a shared secret. Later is the output of KDF from root
	/// key and Diffie-Hellman output.
	next_header_key: P::HeaderKey,

	/// Number of the next message.
	next_msg_num: super::num::Num,

	/// Skipped message keys, which can be adopted in the future.
	skipped_msg_keys: super::skipped_msg_keys::SkippedMsgKeys<P>,

	/// Max count of skipped messages.
	skipped_msg_keys_max_cnt: super::num::Num,
}

impl<P> Recv<P>
where
	P: crate::crypto::Provider,
{
	/// Creates new receiving chain.
	#[inline]
	#[must_use]
	pub(in crate::state) fn new(
		next_header_key: P::HeaderKey,
		skipped_msg_keys_max_cnt: super::num::Num,
	) -> Self {
		Self {
			header_key: None,
			key: None,
			next_header_key,
			next_msg_num: 0,
			skipped_msg_keys: super::skipped_msg_keys::SkippedMsgKeys::new(),
			skipped_msg_keys_max_cnt,
		}
	}

	/// Trying to decrypt the header.
	///
	/// # Return
	///
	/// If the following header key is suitable for decryption, then the second
	/// returned element is `true` and you will need to [upgrade] the chain.
	///
	/// [upgrade]: Self::upgrade
	///
	/// # Errors
	///
	/// See [`DecryptHeader`].
	///
	/// [`DecryptHeader`]: super::error::DecryptHeader
	pub(in crate::state) fn decrypt_header(
		&self,
		encrypted_header: &[u8],
	) -> Result<(super::header::Header<P>, bool), super::error::DecryptHeader>
	{
		let mut need_to_upgrade = false;

		// Decrypt bytes
		let bytes = match self.header_key {
			Some(ref header_key) => {
				// Try to decrypt with current header key
				if let Ok(b) = P::decrypt_header(header_key, encrypted_header)
				{
					b
				} else {
					// Try to decrypt with next header key
					let bytes = P::decrypt_header(
						&self.next_header_key,
						encrypted_header,
					)
					.map_err(|e| {
						super::error::DecryptHeader::Decrypt(e.into())
					})?;
					need_to_upgrade = true;
					bytes
				}
			}
			None => return Err(super::error::DecryptHeader::NoKey),
		};

		// Decode bytes
		let header =
			bincode::decode_from_slice(&bytes, bincode::config::standard())?.0;
		Ok((header, need_to_upgrade))
	}

	/// See [pop] for more.
	///
	/// [pop]: super::skipped_msg_keys::SkippedMsgKeys::pop
	#[inline]
	pub(in crate::state) fn pop_skipped_msg_key(
		&mut self,
		encrypted_header: &[u8],
	) -> Result<Option<P::MsgKey>, super::error::PopSkippedMsgKey> {
		self.skipped_msg_keys.pop(encrypted_header)
	}

	pub(in crate::state) fn skip_msg_keys(
		&mut self,
		until: super::num::Num,
	) -> Result<(), super::error::SkipMsgKeys> {
		use {super::Chain as _, alloc::borrow::ToOwned as _};

		// Validate `until`
		if self.next_msg_num + self.skipped_msg_keys_max_cnt < until {
			return Err(super::error::SkipMsgKeys::TooMuch);
		}

		// KDF many times to remember skipped message keys
		while self.next_msg_num < until {
			let (msg_key, header_key) = {
				// To use mutable reference later to insert new skipped message
				// key
				let (msg_key, header_key) = self.kdf()?;
				(msg_key, header_key.to_owned())
			};
			self.skipped_msg_keys.insert(
				header_key.to_owned(),
				self.next_msg_num - 1,
				msg_key,
			);
		}

		Ok(())
	}
}

impl<P> super::Chain<P> for Recv<P>
where
	P: crate::crypto::Provider,
{
	type KdfError = super::error::RecvKdf;

	fn kdf(&mut self) -> Result<(P::MsgKey, &P::HeaderKey), Self::KdfError> {
		match self.key {
			Some(ref key) => match self.header_key {
				Some(ref header_key) => {
					let (new_key, msg_key) = P::kdf_msg_chain(key);
					self.key = Some(new_key);
					self.next_msg_num += 1;
					Ok((msg_key, header_key))
				}
				None => Err(Self::KdfError::NoHeaderKey),
			},
			None => Err(Self::KdfError::NoKey),
		}
	}

	fn upgrade(
		&mut self,
		new_key: P::MsgChainKey,
		new_next_header_key: P::HeaderKey,
	) {
		self.header_key = Some(core::mem::replace(
			&mut self.next_header_key,
			new_next_header_key,
		));
		self.key = Some(new_key);
		self.next_msg_num = 0;
	}
}

#[cfg(test)]
mod tests {
	const SKIPPED_MSG_KEYS_MAX_CNT: super::super::num::Num = 100;

	fn create_chain() -> super::Recv<crate::default_crypto::Provider> {
		super::Recv::<crate::default_crypto::Provider>::new(
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from([123; 32]),
			SKIPPED_MSG_KEYS_MAX_CNT,
		)
	}

	fn create_header(
		msg_num: super::super::num::Num,
	) -> (
		super::super::Header<crate::default_crypto::Provider>,
		alloc::vec::Vec<u8>,
	) {
		// Create header
		let header = super::super::Header::<crate::default_crypto::Provider>::new(
			<crate::default_crypto::KeyPair as crate::crypto::KeyPair>
				::Public::from([1; 32]),
			msg_num,
			100,
		);

		// Encode header
		let header_bytes =
			bincode::encode_to_vec(&header, bincode::config::standard())
				.unwrap();

		(header, header_bytes)
	}

	#[test]
	fn test_decrypt_header() {
		use crate::crypto::Provider as _;

		// Create and upgrade chain
		let mut chain = create_chain();
		upgrade(&mut chain, [1; 32], [2; 32]);

		let (header, header_bytes) = create_header(1);

		// Encrypt header bytes with current header key
		let encrypted_header =
			crate::default_crypto::Provider::encrypt_header(
				// `Option::unwrap` because of upgrade
				chain.header_key.as_ref().unwrap(),
				&header_bytes,
			)
			.unwrap();

		// Encrypt header bytes with next header key
		let next_encrypted_header =
			crate::default_crypto::Provider::encrypt_header(
				&chain.next_header_key,
				&header_bytes,
			)
			.unwrap();

		// Validate usage of keys
		assert_eq!(
			chain.decrypt_header(&encrypted_header).unwrap(),
			(header.clone(), false)
		);
		assert_eq!(
			chain.decrypt_header(&next_encrypted_header).unwrap(),
			(header, true)
		);
		assert!(chain.decrypt_header(&[0; 150]).is_err());
	}

	#[test]
	fn test_kdf_error() {
		use super::super::Chain as _;

		let mut chain = create_chain();
		assert!(matches!(
			chain.kdf(),
			Err(super::super::error::RecvKdf::NoKey)
		));
	}

	#[test]
	fn test_skip_msg_keys_and_pop_skipped_msg_key() {
		use {super::super::Chain as _, crate::crypto::Provider as _};

		// Create chain and try skip too much
		let mut chain = create_chain();
		assert!(chain.skip_msg_keys(SKIPPED_MSG_KEYS_MAX_CNT).is_err());

		// Update chain to set key
		upgrade(&mut chain, [1; 32], [2; 32]);

		// Skip message keys
		chain.skip_msg_keys(2).unwrap();
		assert_eq!(chain.next_msg_num, 2);

		// Create headers
		let (_header_1, header_bytes_1) = create_header(0);
		let (_header_2, header_bytes_2) = create_header(1);

		// Create copy of chain
		let mut chain_clone = create_chain();
		upgrade(&mut chain_clone, [1; 32], [2; 32]);
		let (msg_key_1, _) = chain_clone.kdf().unwrap();
		let (msg_key_2, header_key) = chain_clone.kdf().unwrap();

		// Encrypt headers
		let encrypted_header_1 =
			crate::default_crypto::Provider::encrypt_header(
				header_key,
				&header_bytes_1,
			)
			.unwrap();
		let encrypted_header_2 =
			crate::default_crypto::Provider::encrypt_header(
				header_key,
				&header_bytes_2,
			)
			.unwrap();

		// Pop from original chain
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_header_1).unwrap().as_deref(),
			Some(&*msg_key_1),
		);
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_header_1).unwrap(),
			None
		);
		assert_eq!(chain.skipped_msg_keys.inner().len(), 1);
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_header_2).unwrap().as_deref(),
			Some(&*msg_key_2),
		);
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_header_2).unwrap(),
			None
		);
		assert!(chain.skipped_msg_keys.inner().is_empty());
	}

	#[test]
	fn test_upgrade_and_kdf() {
		use super::super::Chain as _;

		// Create chain
		let mut chain = create_chain();
		let old_next_header_key = chain.next_header_key.clone();

		// Upgrade chain
		upgrade(&mut chain, [1; 32], [2; 32]);

		// Check upgrade is done
		assert_eq!(chain.header_key.as_ref(), Some(&old_next_header_key));
		assert_eq!(chain.key.as_deref(), Some(&[1; 32]));
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(&*chain.next_header_key, &[2; 32]);

		// Use KDF
		chain.kdf().unwrap();
		chain.kdf().unwrap();
		chain.kdf().unwrap();

		// Check KDF is done
		assert_eq!(chain.header_key, Some(old_next_header_key));
		assert_ne!(chain.key.as_deref(), Some(&[1; 32]));
		assert_eq!(chain.next_msg_num, 3);
		assert_eq!(*chain.next_header_key, [2; 32]);

		// Upgrade chain
		upgrade(&mut chain, [3; 32], [4; 32]);

		// Check upgrade is done
		assert_eq!(chain.header_key.as_deref(), Some(&[2; 32]));
		assert_eq!(chain.key.as_deref(), Some(&[3; 32]));
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(*chain.next_header_key, [4; 32]);
	}

	fn upgrade(
		chain: &mut super::Recv<crate::default_crypto::Provider>,
		key: [u8; 32],
		header_key: [u8; 32],
	) {
		use super::super::Chain as _;
		chain.upgrade(
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::MsgChainKey::from(key),
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from(header_key),
		);
	}
}
