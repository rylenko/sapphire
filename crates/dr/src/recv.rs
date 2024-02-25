/// Receiving chain of [`State`].
///
/// [`State`]: super::State
#[derive(Clone, Eq, PartialEq)]
pub(super) struct Recv {
	/// Is initially a shared secret. Later is the next header key.
	header_key: Option<super::key::Header>,
	/// Output chain key of KDF when receiving messages.
	key: Option<super::key::MsgChain>,
	/// Is initially a shared secret. Later is the output of KDF from root
	/// key and Diffie-Hellman output.
	next_header_key: super::key::Header,
	/// Number of the next message.
	next_msg_num: u32,
	/// Skipped message keys, which can be adopted in the future.
	skipped_msg_keys: super::skipped_msg_keys::SkippedMsgKeys,
	/// Max count of skipped messages.
	skipped_msg_keys_max_cnt: u32,
}

impl Recv {
	/// Creates new receiving chain.
	#[inline]
	#[must_use]
	pub(super) fn new(
		next_header_key: super::key::Header,
		skipped_msg_keys_max_cnt: u32,
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
	pub(super) fn decrypt_header(
		&self,
		encrypted_header: &[u8],
	) -> Result<(super::header::Header, bool), super::error::DecryptHeader> {
		use {alloc::borrow::ToOwned as _, zerocopy::FromBytes as _};

		// Try to decrypt with current header key
		if let Some(ref header_key) = self.header_key {
			if let Ok(bytes) =
				super::cipher::decrypt(header_key.as_bytes(), encrypted_header)
			{
				let header = super::header::Header::ref_from(&bytes)
					.ok_or(super::error::DecryptHeader::FromBytes)?;
				return Ok((header.to_owned(), false));
			}
		}

		// Try to decrypt with next header key
		if let Ok(bytes) = super::cipher::decrypt(
			self.next_header_key.as_bytes(),
			encrypted_header,
		) {
			let header = super::header::Header::ref_from(&bytes)
				.ok_or(super::error::DecryptHeader::FromBytes)?;
			return Ok((header.to_owned(), true));
		}

		Err(super::error::DecryptHeader::KeysNotFit)
	}

	#[cfg(test)]
	#[inline]
	#[must_use]
	pub(super) const fn header_key(&self) -> Option<&super::key::Header> {
		self.header_key.as_ref()
	}

	#[cfg(test)]
	#[inline]
	#[must_use]
	pub(super) const fn key(&self) -> Option<&super::key::MsgChain> {
		self.key.as_ref()
	}

	#[cfg(test)]
	#[inline]
	#[must_use]
	pub(super) const fn next_header_key(&self) -> &super::key::Header {
		&self.next_header_key
	}

	/// See [pop] for more.
	///
	/// [pop]: super::skipped_msg_keys::SkippedMsgKeys::pop
	#[inline]
	pub(super) fn pop_skipped_msg_key(
		&mut self,
		encrypted_header: &[u8],
	) -> Result<Option<super::key::Msg>, super::error::PopSkippedMsgKey> {
		self.skipped_msg_keys.pop(encrypted_header)
	}

	pub(super) fn skip_msg_keys(
		&mut self,
		until: u32,
	) -> Result<(), super::error::SkipMsgKeys> {
		use {super::msg_chain::MsgChain as _, alloc::borrow::ToOwned as _};

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
				header_key,
				self.next_msg_num - 1,
				msg_key,
			);
		}

		Ok(())
	}
}

impl super::msg_chain::MsgChain for Recv {
	type KdfError = super::error::RecvKdf;
	type KdfOk<'a> = (super::key::Msg, &'a super::key::Header);

	fn kdf(&mut self) -> Result<Self::KdfOk<'_>, Self::KdfError> {
		match self.key {
			Some(ref key) => match self.header_key {
				Some(ref header_key) => {
					self.next_msg_num += 1;

					let (new_key, msg_key) = Self::kdf_inner(key);
					self.key = Some(new_key);
					Ok((msg_key, header_key))
				}
				None => Err(Self::KdfError::NoHeaderKey),
			},
			None => Err(Self::KdfError::NoKey),
		}
	}

	fn upgrade(
		&mut self,
		new_key: super::key::MsgChain,
		new_next_header_key: super::key::Header,
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
	const SKIPPED_MSG_KEYS_MAX_CNT: u32 = 100;

	fn create_chain() -> super::Recv {
		super::Recv::new(
			super::super::key::Header::from([123; 32]),
			SKIPPED_MSG_KEYS_MAX_CNT,
		)
	}

	fn create_header(msg_num: u32) -> super::super::header::Header {
		super::super::header::Header::new(
			super::super::key::Public::from([1; 32]),
			msg_num,
			100,
		)
	}

	#[test]
	fn test_decrypt_header() {
		use zerocopy::AsBytes as _;

		// Create and upgrade chain
		let mut chain = create_chain();
		upgrade(&mut chain, [1; 32], [2; 32]);

		// Create header
		let header = create_header(1);

		// Encrypt header bytes with current header key
		let encrypted_header = super::super::cipher::encrypt(
			// `Option::unwrap` because of upgrade
			chain.header_key.as_ref().unwrap().as_bytes(),
			header.as_bytes(),
		)
		.unwrap();

		// Encrypt header bytes with next header key
		let next_encrypted_header = super::super::cipher::encrypt(
			chain.next_header_key.as_bytes(),
			header.as_bytes(),
		)
		.unwrap();

		// Validate usage of keys
		assert_eq!(
			chain.decrypt_header(&encrypted_header).unwrap(),
			(header, false)
		);
		assert_eq!(
			chain.decrypt_header(&next_encrypted_header).unwrap(),
			(header, true)
		);
		assert!(chain.decrypt_header(&[0; 150]).is_err());
	}

	#[test]
	fn test_skip_msg_keys_and_pop_skipped_msg_key() {
		use {super::super::msg_chain::MsgChain as _, zerocopy::AsBytes as _};

		// Create chain and try skip too much
		let mut chain = create_chain();
		assert!(chain.skip_msg_keys(SKIPPED_MSG_KEYS_MAX_CNT).is_err());

		// Update chain to set key
		upgrade(&mut chain, [1; 32], [2; 32]);

		// Skip message keys
		chain.skip_msg_keys(2).unwrap();
		assert_eq!(chain.next_msg_num, 2);

		// Create headers
		let header_1 = create_header(0);
		let header_2 = create_header(1);

		// Create copy of chain
		let mut chain_clone = create_chain();
		upgrade(&mut chain_clone, [1; 32], [2; 32]);
		let (msg_key_1, _) = chain_clone.kdf().unwrap();
		let (msg_key_2, header_key) = chain_clone.kdf().unwrap();

		// Encrypt headers
		let encrypted_header_1 = super::super::cipher::encrypt(
			header_key.as_bytes(),
			header_1.as_bytes(),
		)
		.unwrap();
		let encrypted_header_2 = super::super::cipher::encrypt(
			header_key.as_bytes(),
			header_2.as_bytes(),
		)
		.unwrap();

		// Pop from original chain
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_header_1).unwrap(),
			Some(msg_key_1),
		);
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_header_1).unwrap(),
			None
		);
		assert_eq!(chain.skipped_msg_keys.inner().len(), 1);
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_header_2).unwrap(),
			Some(msg_key_2),
		);
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_header_2).unwrap(),
			None
		);
		assert!(chain.skipped_msg_keys.inner().is_empty());
	}

	#[test]
	fn test_upgrade_and_kdf() {
		use super::super::msg_chain::MsgChain as _;

		// Create chain
		let mut chain = create_chain();
		assert!(chain.kdf().is_err());
		let old_next_header_key = chain.next_header_key.clone();

		// Upgrade chain
		upgrade(&mut chain, [1; 32], [2; 32]);

		// Check upgrade is done
		assert_eq!(chain.header_key.as_ref(), Some(&old_next_header_key));
		assert_eq!(chain.key.as_ref().unwrap().as_bytes(), &[1; 32]);
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.next_header_key.as_bytes(), &[2; 32]);

		// Use KDF
		chain.kdf().unwrap();
		chain.kdf().unwrap();
		chain.kdf().unwrap();

		// Check KDF is done
		assert_eq!(chain.header_key, Some(old_next_header_key));
		assert_ne!(chain.key.as_ref().unwrap().as_bytes(), &[1; 32]);
		assert_eq!(chain.next_msg_num, 3);
		assert_eq!(chain.next_header_key.as_bytes(), &[2; 32]);

		// Upgrade chain
		upgrade(&mut chain, [3; 32], [4; 32]);

		// Check upgrade is done
		assert_eq!(chain.header_key.as_ref().unwrap().as_bytes(), &[2; 32]);
		assert_eq!(chain.key.as_ref().unwrap().as_bytes(), &[3; 32]);
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.next_header_key.as_bytes(), &[4; 32]);
	}

	fn upgrade(chain: &mut super::Recv, key: [u8; 32], header_key: [u8; 32]) {
		super::super::msg_chain::MsgChain::upgrade(
			chain,
			super::super::key::MsgChain::from(key),
			super::super::key::Header::from(header_key),
		);
	}
}
