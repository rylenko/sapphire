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
				match P::decrypt_header(header_key, encrypted_header) {
					Ok(b) => b,
					Err(_) => {
						// Try to decrypt with next header key
						match P::decrypt_header(
							&self.next_header_key,
							encrypted_header,
						) {
							Ok(b) => {
								need_to_upgrade = true;
								b
							}
							Err(e) => {
								return Err(
									super::error::DecryptHeader::Decrypt(
										e.into(),
									),
								)
							}
						}
					}
				}
			}
			None => return Err(super::error::DecryptHeader::NoKey),
		};

		// Decode bytes
		let header =
			bincode::decode_from_slice(&bytes, bincode::config::standard())?.0;
		Ok((header, need_to_upgrade))
	}

	/// Moves chain forward, updates current key and return new message key and
	/// header key.
	pub(in crate::state) fn kdf(
		&mut self,
	) -> Result<(P::MsgKey, &P::HeaderKey), super::error::RecvKdf> {
		match self.key {
			Some(ref key) => match self.header_key {
				Some(ref header_key) => {
					let (new_key, msg_key) = P::kdf_msg_chain(key);
					self.key = Some(new_key);
					self.next_msg_num += 1;
					Ok((msg_key, header_key))
				}
				None => Err(super::error::RecvKdf::NoHeaderKey),
			},
			None => Err(super::error::RecvKdf::NoKey),
		}
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
		use alloc::borrow::ToOwned as _;

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

	/// Updates the chain as if it were the next chain.
	pub(in crate::state) fn upgrade(
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
	#[test]
	fn test_upgrade_and_kdf() -> Result<(), super::super::error::RecvKdf> {
		// Create chain
		let mut chain = super::Recv::<crate::default_crypto::Provider>::new(
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from([123; 32]),
			100,
		);
		let old_next_header_key = chain.next_header_key.clone();

		// Upgrade chain
		let key = <crate::default_crypto::Provider as crate::crypto::Provider>
			::MsgChainKey::from([234; 32]);
		let next_header_key =
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from([120; 32]);
		chain.upgrade(key.clone(), next_header_key.clone());

		// Check upgrade is done
		assert_eq!(chain.header_key.as_ref(), Some(&old_next_header_key));
		assert_eq!(chain.key.as_ref(), Some(&key));
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.next_header_key, next_header_key);

		// Use KDF
		chain.kdf()?;
		chain.kdf()?;
		chain.kdf()?;

		// Check KDF is done
		assert_eq!(chain.header_key, Some(old_next_header_key));
		assert_ne!(chain.key, Some(key));
		assert_eq!(chain.next_msg_num, 3);
		assert_eq!(chain.next_header_key, next_header_key);

		// Upgrade chain
		let new_key = <crate::default_crypto::Provider as crate::crypto::Provider>
			::MsgChainKey::from([200; 32]);
		let new_next_header_key =
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from([120; 32]);
		chain.upgrade(new_key.clone(), new_next_header_key.clone());

		// Check upgrade is done
		assert_eq!(chain.header_key, Some(next_header_key));
		assert_eq!(chain.key, Some(new_key));
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.next_header_key, new_next_header_key);

		Ok(())
	}
}
