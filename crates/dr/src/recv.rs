/// Receiving chain of [`State`].
///
/// [`State`]: super::state::State
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
		encrypted_hdr: &super::hdr::Encrypted,
	) -> Result<(super::hdr::Hdr, bool), super::error::DecryptHdr> {
		// Try to decrypt with current header key
		if let Some(ref hdr_key) = self.hdr_key {
			if let Ok(hdr) = encrypted_hdr.decrypt(hdr_key) {
				return Ok((hdr, false));
			}
		}
		// Try to decrypt with next header key
		if let Ok(hdr) = encrypted_hdr.decrypt(&self.next_hdr_key) {
			return Ok((hdr, true));
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
		encrypted_hdr: &super::hdr::Encrypted,
	) -> Option<super::key::Msg> {
		self.skipped_msg_keys.pop(encrypted_hdr)
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
			let msg_key = self.kdf()?;
			// Insert new skipped message key
			match self.hdr_key {
				Some(ref hdr_key) => {
					self.skipped_msg_keys.insert(
						hdr_key.clone(),
						// KDF increases next message number by 1 so it's ok
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
	type KdfOk<'a> = super::key::Msg;

	fn kdf(&mut self) -> Result<Self::KdfOk<'_>, Self::KdfError> {
		match self.key {
			Some(ref key) => {
				let (new_key, msg_key) = Self::kdf_inner(key);
				self.key = Some(new_key);
				self.next_msg_num += 1;
				Ok(msg_key)
			}
			None => Err(Self::KdfError::NoKey),
		}
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

impl super::draft::Draft for Recv {
	fn commit_draft(&mut self, draft: Self) {
		self.hdr_key = draft.hdr_key;
		self.key = draft.key;
		self.next_hdr_key = draft.next_hdr_key;
		self.next_msg_num = draft.next_msg_num;
		self.skipped_msg_keys.extend(draft.skipped_msg_keys);
		self.skipped_msg_keys_max_cnt = draft.skipped_msg_keys_max_cnt;
	}

	/// The draft does not include the skipped message keys. Therefore, any
	/// actions that rely on skipped message keys should be did before working
	/// with the draft.
	#[inline]
	fn create_draft(&self) -> Self {
		Self {
			hdr_key: self.hdr_key.clone(),
			key: self.key.clone(),
			next_hdr_key: self.next_hdr_key.clone(),
			next_msg_num: self.next_msg_num,
			skipped_msg_keys: super::skipped_msg_keys::SkippedMsgKeys::new(),
			skipped_msg_keys_max_cnt: self.skipped_msg_keys_max_cnt,
		}
	}
}

#[cfg(test)]
mod tests {
	const SKIPPED_MSG_KEYS_MAX_CNT: u32 = 100;

	fn create_chain() -> super::Recv {
		super::Recv::new(
			crate::key::Hdr::from([123; 32]),
			SKIPPED_MSG_KEYS_MAX_CNT,
		)
	}

	fn create_hdr(msg_num: u32) -> crate::hdr::Hdr {
		crate::hdr::Hdr::new(crate::key::Public::from([1; 32]), msg_num, 100)
	}

	#[test]
	fn test_decrypt_hdr() {
		// Create and upgrade chain
		let mut chain = create_chain();
		upgrade(&mut chain, [1; 32], [2; 32]);

		// Create header and it's encryption buffer
		let hdr = create_hdr(1);
		let encrypted_1_hdr = hdr.encrypt(chain.hdr_key.as_ref().unwrap());
		let encrypted_2_hdr = hdr.encrypt(&chain.next_hdr_key);

		// Validate usage of keys
		assert_eq!(chain.decrypt_hdr(&encrypted_1_hdr).unwrap(), (hdr, false));
		assert_eq!(chain.decrypt_hdr(&encrypted_2_hdr).unwrap(), (hdr, true));

		// Test bad header
		let encrypted_3_hdr = hdr.encrypt(&[0; 32].into());
		assert!(chain.decrypt_hdr(&encrypted_3_hdr).is_err());
	}

	#[test]
	fn test_draft() {
		use crate::draft::Draft as _;

		// Create original with skipped message keys
		let mut orig = create_chain();
		orig.skipped_msg_keys.insert([1; 32].into(), 0, [2; 32].into());
		let old_orig_next_hdr_key = orig.next_hdr_key.clone();

		// Get draft and check that there are no skipped message keys
		let mut draft = orig.create_draft();
		assert!(draft.skipped_msg_keys.inner().is_empty());
		draft.skipped_msg_keys.insert([2; 32].into(), 1, [3; 32].into());

		// Upgrade and commit draft
		draft.skipped_msg_keys.insert([1; 32].into(), 1, [3; 32].into());
		upgrade(&mut draft, [1; 32], [2; 32]);
		orig.commit_draft(draft);

		assert_eq!(orig.key, Some([1; 32].into()));
		assert_eq!(orig.hdr_key, Some(old_orig_next_hdr_key));
		assert_eq!(orig.next_hdr_key.as_bytes(), &[2; 32]);
		assert_eq!(orig.skipped_msg_keys.inner().len(), 2);
	}

	#[test]
	fn test_skip_msg_keys_and_pop_skipped_msg_key() {
		use crate::msg_chain::MsgChain as _;

		// Create chain and try skip too much
		let mut chain = create_chain();
		assert!(chain.skip_msg_keys(SKIPPED_MSG_KEYS_MAX_CNT).is_err());
		upgrade(&mut chain, [1; 32], [2; 32]);
		// Skip message keys
		chain.skip_msg_keys(2).unwrap();
		assert_eq!(chain.next_msg_num, 2);

		// Create copy of chain
		let mut chain_clone = create_chain();
		upgrade(&mut chain_clone, [1; 32], [2; 32]);
		// KDF cloned chain
		let msg_key_1 = chain_clone.kdf().unwrap();
		let msg_key_2 = chain_clone.kdf().unwrap();

		// Create encrypted headers
		let hdr_key = chain_clone.hdr_key.as_ref().unwrap();
		let encrypted_hdr_1 = create_hdr(0).encrypt(hdr_key);
		let encrypted_hdr_2 = create_hdr(1).encrypt(hdr_key);

		// Pop from original chain
		assert_eq!(chain.skipped_msg_keys.inner().len(), 1);
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_hdr_1),
			Some(msg_key_1),
		);
		assert_eq!(chain.pop_skipped_msg_key(&encrypted_hdr_1), None);
		assert_eq!(chain.skipped_msg_keys.inner().len(), 1);
		assert_eq!(
			chain.pop_skipped_msg_key(&encrypted_hdr_2),
			Some(msg_key_2),
		);
		assert_eq!(chain.pop_skipped_msg_key(&encrypted_hdr_2), None);
		assert!(chain.skipped_msg_keys.inner().is_empty());
	}

	#[test]
	fn test_upgrade_and_kdf() {
		use crate::msg_chain::MsgChain as _;

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
		chain.kdf().unwrap();
		chain.kdf().unwrap();
		chain.kdf().unwrap();

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
		crate::msg_chain::MsgChain::upgrade(
			chain,
			crate::key::MsgChain::from(key),
			crate::key::Hdr::from(hdr_key),
		);
	}
}
