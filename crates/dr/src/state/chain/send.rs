/// Sending chain.
pub(in crate::state) struct Send<P>
where
	P: crate::crypto::Provider,
{
	/// Is initially a shared key. Later is the next header key.
	header_key: Option<P::HeaderKey>,

	/// Sending chain key. Output chain key of KDF when sending messages.
	key: Option<P::MsgChainKey>,

	/// Number of the next message.
	next_msg_num: super::num::Num,

	/// Is initially a shared key. Later is the output of KDF from root key
	/// and Diffie-Hellman output.
	next_header_key: P::HeaderKey,

	/// Number of messages in previous sending chain.
	prev_msgs_cnt: super::num::Num,
}

impl<P> Send<P>
where
	P: crate::crypto::Provider,
{
	#[inline]
	#[must_use]
	pub(in crate::state) const fn new(
		key: Option<P::MsgChainKey>,
		header_key: Option<P::HeaderKey>,
		next_header_key: P::HeaderKey,
	) -> Self {
		Self {
			header_key,
			key,
			next_msg_num: 0,
			next_header_key,
			prev_msgs_cnt: 0,
		}
	}

	#[inline]
	#[must_use]
	pub(in crate::state) const fn next_msg_num(&self) -> super::num::Num {
		self.next_msg_num
	}

	#[inline]
	#[must_use]
	pub(in crate::state) const fn prev_msgs_cnt(&self) -> super::num::Num {
		self.prev_msgs_cnt
	}
}

impl<P> super::Chain<P> for Send<P>
where
	P: crate::crypto::Provider,
{
	type KdfError = super::error::SendKdf;

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
		self.prev_msgs_cnt = self.next_msg_num;
		self.next_msg_num = 0;
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_kdf_error() {
		use super::super::Chain as _;

		let mut chain = super::Send::<crate::default_crypto::Provider>::new(
			None,
			None,
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from([1; 32]),
		);
		// No key
		assert!(chain.kdf().is_err());
	}

	#[test]
	fn test_kdf_and_upgrade_ok() -> Result<(), super::super::error::SendKdf> {
		use super::super::Chain as _;

		// Create chain
		let mut chain = super::Send::<crate::default_crypto::Provider>::new(
			Some(
				<crate::default_crypto::Provider as crate::crypto::Provider>
					::MsgChainKey::from([1; 32]),
			),
			Some(
				<crate::default_crypto::Provider as crate::crypto::Provider>
					::HeaderKey::from([2; 32]),
			),
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from([3; 32]),
		);

		// Base asserts
		assert_eq!(chain.next_msg_num(), 0);
		assert_eq!(chain.prev_msgs_cnt(), 0);

		// KDF
		chain.kdf()?;

		// Check KDF is done
		assert_eq!(chain.header_key.as_deref(), Some(&[2; 32]));
		assert_ne!(chain.key.as_deref(), Some(&[1; 32]));
		assert_eq!(*chain.next_header_key, [3; 32]);
		assert_eq!(chain.next_msg_num(), 1);
		assert_eq!(chain.prev_msgs_cnt(), 0);

		// Upgrade chain
		chain.upgrade(
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::MsgChainKey::new([4; 32]),
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from([5; 32]),
		);

		// Check upgrade is done
		assert_eq!(chain.header_key.as_deref(), Some(&[3; 32]));
		assert_eq!(chain.key.as_deref(), Some(&[4; 32]));
		assert_eq!(*chain.next_header_key, [5; 32]);
		assert_eq!(chain.next_msg_num(), 0);
		assert_eq!(chain.prev_msgs_cnt(), 1);

		Ok(())
	}
}
