/// Receiving chain of [`State`].
///
/// [`State`]: super::State
#[derive(Clone, Eq, PartialEq)]
pub(super) struct Recv {
	/// Is initially a shared secret. Later is the next header key.
	hdr_key: Option<super::key::Hdr>,
	/// Output chain key of KDF when receiving messages.
	key: Option<super::key::MsgChain>,
	/// Is initially a shared secret. Later is the output of KDF from root
	/// key and DH output.
	next_hdr_key: super::key::Hdr,
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
		next_hdr_key: super::key::Hdr,
		skipped_msg_keys_max_cnt: u32,
	) -> Self {
		Self {
			hdr_key: None,
			key: None,
			next_hdr_key,
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
	/// See [`DecryptHdr`].
	///
	/// [`DecryptHdr`]: super::error::DecryptHdr
	pub(super) fn decrypt_hdr(
		&self,
		encrypted_hdr_buf: &mut [u8],
	) -> Result<(super::hdr::Hdr, bool), super::error::DecryptHdr> {
		use zerocopy::FromBytes as _;

		// Try to decrypt with current header key
		if let Some(ref hdr_key) = self.hdr_key {
			if super::cipher::decrypt(
				hdr_key.as_bytes(),
				encrypted_hdr_buf,
				&[],
			)
			.is_ok()
			{
				let hdr = super::hdr::Hdr::ref_from(
					&encrypted_hdr_buf[..encrypted_hdr_buf.len() - 32],
				)
				.ok_or(super::error::DecryptHdr::FromBytes)?;
				return Ok((*hdr, false));
			}
		}

		// Try to decrypt with next header key
		if super::cipher::decrypt(
			self.next_hdr_key.as_bytes(),
			encrypted_hdr_buf,
			&[],
		)
		.is_ok()
		{
			let hdr = super::hdr::Hdr::ref_from(
				&encrypted_hdr_buf[..encrypted_hdr_buf.len() - 32],
			)
			.ok_or(super::error::DecryptHdr::FromBytes)?;
			return Ok((*hdr, true));
		}

		Err(super::error::DecryptHdr::KeysNotFit)
	}

	#[cfg(test)]
	#[inline]
	#[must_use]
	pub(super) const fn hdr_key(&self) -> Option<&super::key::Hdr> {
		self.hdr_key.as_ref()
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
	pub(super) const fn next_hdr_key(&self) -> &super::key::Hdr {
		&self.next_hdr_key
	}

	/// See [pop] for more.
	///
	/// [pop]: super::skipped_msg_keys::SkippedMsgKeys::pop
	#[inline]
	pub(super) fn pop_skipped_msg_key(
		&mut self,
		encrypted_hdr_buf: &mut [u8],
	) -> Result<Option<super::key::Msg>, super::error::PopSkippedMsgKey> {
		self.skipped_msg_keys.pop(encrypted_hdr_buf)
	}

	pub(super) fn skip_msg_keys(
		&mut self,
		until: u32,
	) -> Result<(), super::error::SkipMsgKeys> {
		use super::msg_chain::MsgChain as _;

		// Validate `until`
		if self.next_msg_num + self.skipped_msg_keys_max_cnt < until {
			return Err(super::error::SkipMsgKeys::TooMuch);
		}

		// KDF many times to remember skipped message keys
		while self.next_msg_num < until {
			// KDF and commit
			let (msg_chain_key, msg_key) = self.kdf()?;
			self.commit_kdf(msg_chain_key);

			// Insert new skipped message key
			match self.hdr_key {
				Some(ref hdr_key) => {
					self.skipped_msg_keys.insert(
						hdr_key.clone(),
						self.next_msg_num - 1,
						msg_key,
					);
				}
				None => return Err(super::error::SkipMsgKeys::NoHdrKey),
			}
		}

		Ok(())
	}
}

impl super::msg_chain::MsgChain for Recv {
	type KdfError = super::error::RecvKdf;
	type KdfOk<'a> = (super::key::MsgChain, super::key::Msg);

	fn commit_kdf(&mut self, key: super::key::MsgChain) {
		debug_assert_eq!(
			key,
			self.kdf().expect("Must be Ok if we use this.").0
		);
		self.next_msg_num += 1;
		self.key = Some(key);
	}

	fn kdf(&self) -> Result<Self::KdfOk<'_>, Self::KdfError> {
		self.key.as_ref().map(Self::kdf_inner).ok_or(Self::KdfError::NoKey)
	}

	fn upgrade(
		&mut self,
		new_key: super::key::MsgChain,
		new_next_hdr_key: super::key::Hdr,
	) {
		self.hdr_key =
			Some(core::mem::replace(&mut self.next_hdr_key, new_next_hdr_key));
		self.key = Some(new_key);
		self.next_msg_num = 0;
	}
}

#[cfg(test)]
mod tests {
	const SKIPPED_MSG_KEYS_MAX_CNT: u32 = 100;

	fn create_chain() -> super::Recv {
		super::Recv::new(
			super::super::key::Hdr::from([123; 32]),
			SKIPPED_MSG_KEYS_MAX_CNT,
		)
	}

	fn create_hdr(msg_num: u32) -> super::super::hdr::Hdr {
		super::super::hdr::Hdr::new(
			super::super::key::Public::from([1; 32]),
			msg_num,
			100,
		)
	}

	#[test]
	fn test_decrypt_hdr() {
		use zerocopy::AsBytes as _;

		// Create and upgrade chain
		let mut chain = create_chain();
		upgrade(&mut chain, [1; 32], [2; 32]);

		// Create header and it's encryption bufer
		let hdr = create_hdr(1);
		let mut hdr_buf_1 = [hdr.as_bytes(), &[0; 32]].concat();
		let mut hdr_buf_2 = [hdr.as_bytes(), &[0; 32]].concat();

		// Encrypt header bytes with current header key
		super::super::cipher::encrypt(
			// `Option::unwrap` because of upgrade
			chain.hdr_key.as_ref().unwrap().as_bytes(),
			&mut hdr_buf_1,
			&[],
		)
		.unwrap();
		// Encrypt header bytes with next header key
		super::super::cipher::encrypt(
			chain.next_hdr_key.as_bytes(),
			&mut hdr_buf_2,
			&[],
		)
		.unwrap();

		// Validate usage of keys
		assert_eq!(chain.decrypt_hdr(&mut hdr_buf_1).unwrap(), (hdr, false));
		assert_eq!(chain.decrypt_hdr(&mut hdr_buf_2).unwrap(), (hdr, true));
		let mut bufer = [0; 150];
		assert!(chain.decrypt_hdr(&mut bufer).is_err());
	}

	#[test]
	fn test_skip_msg_keys_and_pop_skipped_msg_key() {
		use {super::super::msg_chain::MsgChain as _, zerocopy::AsBytes as _};

		// Create chain and try skip too much
		let mut chain = create_chain();
		assert!(chain.skip_msg_keys(SKIPPED_MSG_KEYS_MAX_CNT).is_err());
		upgrade(&mut chain, [1; 32], [2; 32]);
		// Skip message keys
		chain.skip_msg_keys(2).unwrap();
		assert_eq!(chain.next_msg_num, 2);

		// Create header bufers
		let (mut hdr_1_buf, mut hdr_2_buf) = {
			let hdr_1 = create_hdr(0);
			let hdr_2 = create_hdr(1);
			(
				[hdr_1.as_bytes(), &[0; 32]].concat(),
				[hdr_2.as_bytes(), &[0; 32]].concat(),
			)
		};

		// Create copy of chain
		let mut chain_clone = create_chain();
		upgrade(&mut chain_clone, [1; 32], [2; 32]);
		// KDF cloned chain
		let (msg_chain_key_1, msg_key_1) = chain_clone.kdf().unwrap();
		chain_clone.commit_kdf(msg_chain_key_1);
		let (msg_chain_key_2, msg_key_2) = chain_clone.kdf().unwrap();
		chain_clone.commit_kdf(msg_chain_key_2);

		// Encrypt headers
		super::super::cipher::encrypt(
			chain_clone.hdr_key.as_ref().unwrap().as_bytes(),
			&mut hdr_1_buf,
			&[],
		)
		.unwrap();
		super::super::cipher::encrypt(
			chain_clone.hdr_key.as_ref().unwrap().as_bytes(),
			&mut hdr_2_buf,
			&[],
		)
		.unwrap();

		// Pop from original chain
		assert_eq!(
			chain.pop_skipped_msg_key(&mut hdr_1_buf).unwrap(),
			Some(msg_key_1),
		);
		assert_eq!(chain.pop_skipped_msg_key(&mut hdr_1_buf).unwrap(), None);
		assert_eq!(chain.skipped_msg_keys.inner().len(), 1);
		assert_eq!(
			chain.pop_skipped_msg_key(&mut hdr_2_buf).unwrap(),
			Some(msg_key_2),
		);
		assert_eq!(chain.pop_skipped_msg_key(&mut hdr_2_buf).unwrap(), None);
		assert!(chain.skipped_msg_keys.inner().is_empty());
	}

	#[test]
	fn test_upgrade_and_kdf() {
		use super::super::msg_chain::MsgChain as _;

		// Create chain
		let mut chain = create_chain();
		assert!(chain.kdf().is_err());
		let old_next_hdr_key = chain.next_hdr_key.clone();

		// Upgrade chain
		upgrade(&mut chain, [1; 32], [2; 32]);

		// Check upgrade is done
		assert_eq!(chain.hdr_key.as_ref(), Some(&old_next_hdr_key));
		assert_eq!(chain.key.as_ref().unwrap().as_bytes(), &[1; 32]);
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.next_hdr_key.as_bytes(), &[2; 32]);

		// Use KDF
		let (msg_chain_key_1, _) = chain.kdf().unwrap();
		chain.commit_kdf(msg_chain_key_1);
		let (msg_chain_key_2, _) = chain.kdf().unwrap();
		chain.commit_kdf(msg_chain_key_2);
		let (msg_chain_key_3, _) = chain.kdf().unwrap();
		chain.commit_kdf(msg_chain_key_3);

		// Check KDF is done
		assert_eq!(chain.hdr_key, Some(old_next_hdr_key));
		assert_ne!(chain.key.as_ref().unwrap().as_bytes(), &[1; 32]);
		assert_eq!(chain.next_msg_num, 3);
		assert_eq!(chain.next_hdr_key.as_bytes(), &[2; 32]);

		// Upgrade chain
		upgrade(&mut chain, [3; 32], [4; 32]);

		// Check upgrade is done
		assert_eq!(chain.hdr_key.as_ref().unwrap().as_bytes(), &[2; 32]);
		assert_eq!(chain.key.as_ref().unwrap().as_bytes(), &[3; 32]);
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.next_hdr_key.as_bytes(), &[4; 32]);
	}

	fn upgrade(chain: &mut super::Recv, key: [u8; 32], hdr_key: [u8; 32]) {
		super::super::msg_chain::MsgChain::upgrade(
			chain,
			super::super::key::MsgChain::from(key),
			super::super::key::Hdr::from(hdr_key),
		);
	}
}
