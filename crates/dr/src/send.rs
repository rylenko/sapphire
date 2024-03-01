/// Sending chain of [`State`].
///
/// [`State`]: super::state:State
#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) struct Send {
	/// Is initially a shared key. Later is the next header key.
	hdr_key: Option<super::key::Hdr>,
	/// Sending chain key. Output chain key of KDF when sending messages.
	key: Option<super::key::MsgChain>,
	/// Number of the next message.
	next_msg_num: u32,
	/// Is initially a shared key. Later is the output of KDF from root key
	/// and DH output.
	next_hdr_key: super::key::Hdr,
	/// Number of messages in previous sending chain.
	prev_msgs_cnt: u32,
}

impl Send {
	#[inline]
	#[must_use]
	pub(super) const fn new(
		key: Option<super::key::MsgChain>,
		hdr_key: Option<super::key::Hdr>,
		next_hdr_key: super::key::Hdr,
	) -> Self {
		Self { hdr_key, key, next_msg_num: 0, next_hdr_key, prev_msgs_cnt: 0 }
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
}

impl super::msg_chain::MsgChain for Send {
	type KdfError = super::error::SendKdf;
	type KdfOk<'a> = (super::key::Msg, u32, &'a super::key::Hdr, u32);

	fn kdf(&mut self) -> Result<Self::KdfOk<'_>, Self::KdfError> {
		match self.key {
			Some(ref key) => match self.hdr_key {
				Some(ref hdr_key) => {
					// Use inner KDF to get new root and message keys
					let (new_key, msg_key) = Self::kdf_inner(key);
					self.key = Some(new_key);

					let ret = (
						msg_key,
						self.next_msg_num,
						hdr_key,
						self.prev_msgs_cnt,
					);
					self.next_msg_num += 1;
					Ok(ret)
				}
				None => Err(Self::KdfError::NoHdrKey),
			},
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
		self.prev_msgs_cnt = self.next_msg_num;
		self.next_msg_num = 0;
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_kdf_error() {
		use crate::msg_chain::MsgChain as _;

		let mut chain =
			super::Send::new(None, None, crate::key::Hdr::from([1; 32]));
		// No key
		assert!(chain.kdf().is_err());
	}

	#[test]
	fn test_kdf_and_upgrade_ok() {
		use crate::msg_chain::MsgChain as _;

		// Create chain
		let mut chain = super::Send::new(
			Some(crate::key::MsgChain::from([1; 32])),
			Some(crate::key::Hdr::from([2; 32])),
			crate::key::Hdr::from([3; 32]),
		);

		// Base asserts
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.prev_msgs_cnt, 0);

		// Check KDF is done
		chain.kdf().unwrap();
		assert_eq!(chain.hdr_key.as_ref().unwrap().as_bytes(), &[2; 32]);
		assert_ne!(chain.key.as_ref().unwrap().as_bytes(), &[1; 32]);
		assert_eq!(chain.next_hdr_key.as_bytes(), &[3; 32]);
		assert_eq!(chain.next_msg_num, 1);
		assert_eq!(chain.prev_msgs_cnt, 0);

		// Upgrade chain
		chain.upgrade(
			crate::key::MsgChain::from([4; 32]),
			crate::key::Hdr::from([5; 32]),
		);

		// Check upgrade is done
		assert_eq!(chain.hdr_key.as_ref().unwrap().as_bytes(), &[3; 32]);
		assert_eq!(chain.key.as_ref().unwrap().as_bytes(), &[4; 32]);
		assert_eq!(chain.next_hdr_key.as_bytes(), &[5; 32]);
		assert_eq!(chain.next_msg_num, 0);
		assert_eq!(chain.prev_msgs_cnt, 1);
	}
}
