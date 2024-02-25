/// Sending chain of [`State`].
///
/// [`State`]: super::State
#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) struct Send {
	/// Is initially a shared key. Later is the next header key.
	header_key: Option<super::key::Header>,

	/// Sending chain key. Output chain key of KDF when sending messages.
	key: Option<super::key::MsgChain>,

	/// Number of the next message.
	next_msg_num: u32,

	/// Is initially a shared key. Later is the output of KDF from root key
	/// and Diffie-Hellman output.
	next_header_key: super::key::Header,

	/// Number of messages in previous sending chain.
	prev_msgs_cnt: u32,
}

impl Send {
	#[inline]
	#[must_use]
	pub(super) const fn new(
		key: Option<super::key::MsgChain>,
		header_key: Option<super::key::Header>,
		next_header_key: super::key::Header,
	) -> Self {
		Self {
			header_key,
			key,
			next_msg_num: 0,
			next_header_key,
			prev_msgs_cnt: 0,
		}
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
}

impl super::msg_chain::MsgChain for Send {
	type KdfError = super::error::SendKdf;
	type KdfOk<'a> = (
		super::key::MsgChain,
		super::key::Msg,
		u32,
		&'a super::key::Header,
		u32,
	);

	fn commit_kdf(&mut self, key: super::key::MsgChain) {
		debug_assert_eq!(
			key,
			self.kdf().expect("Must be Ok if we use this.").0
		);
		self.key = Some(key);
		self.next_msg_num += 1;
	}

	fn kdf(&self) -> Result<Self::KdfOk<'_>, Self::KdfError> {
		match self.key {
			Some(ref key) => match self.header_key {
				Some(ref header_key) => {
					// Use inner KDF to get new root and message keys
					let (new_key, msg_key) = Self::kdf_inner(key);
					Ok((
						new_key,
						msg_key,
						self.next_msg_num,
						header_key,
						self.prev_msgs_cnt,
					))
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
		self.prev_msgs_cnt = self.next_msg_num;
		self.next_msg_num = 0;
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_kdf_error() {
		use super::super::msg_chain::MsgChain as _;

		let chain = super::Send::new(
			None,
			None,
			super::super::key::Header::from([1; 32]),
		);
		// No key
		assert!(chain.kdf().is_err());
	}

	#[test]
	fn test_kdf_and_upgrade_ok() -> Result<(), super::super::error::SendKdf> {
		use super::super::msg_chain::MsgChain as _;

		// Create chain
		let mut chain = super::Send::new(
			Some(super::super::key::MsgChain::from([1; 32])),
			Some(super::super::key::Header::from([2; 32])),
			super::super::key::Header::from([3; 32]),
		);

		// Base asserts
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.prev_msgs_cnt, 0);

		// KDF
		let (msg_chain_key, ..) = chain.kdf()?;
		chain.commit_kdf(msg_chain_key);

		// Check KDF is done
		assert_eq!(chain.header_key.as_ref().unwrap().as_bytes(), &[2; 32]);
		assert_ne!(chain.key.as_ref().unwrap().as_bytes(), &[1; 32]);
		assert_eq!(chain.next_header_key.as_bytes(), &[3; 32]);
		assert_eq!(chain.next_msg_num, 1);
		assert_eq!(chain.prev_msgs_cnt, 0);

		// Upgrade chain
		chain.upgrade(
			super::super::key::MsgChain::from([4; 32]),
			super::super::key::Header::from([5; 32]),
		);

		// Check upgrade is done
		assert_eq!(chain.header_key.as_ref().unwrap().as_bytes(), &[3; 32]);
		assert_eq!(chain.key.as_ref().unwrap().as_bytes(), &[4; 32]);
		assert_eq!(chain.next_header_key.as_bytes(), &[5; 32]);
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.prev_msgs_cnt, 1);

		Ok(())
	}
}
