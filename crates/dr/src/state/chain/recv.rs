/// Receiving chain for Alice and Bob.
pub(in crate::state) struct Recv<P>
where
	P: crate::crypto::Provider,
{
	/// Is initially a shared secret. Later is the next header key.
	header_key: Option<P::HeaderKey>,

	/// Output chain key of KDF when receiving messages.
	key: Option<P::MsgChainKey>,

	/// Number of the next message.
	msg_num: super::num::Num,

	/// Is initially a shared secret. Later is the output of KDF from root
	/// key and Diffie-Hellman output.
	next_header_key: P::HeaderKey,

	/// Storage for skipped message keys.
	skipped_msg_keys: super::skipped_msg_keys::SkippedMsgKeys<P>,
}

impl<P> Recv<P>
where
	P: crate::crypto::Provider,
{
	/// Creates new receiving chain.
	#[inline]
	#[must_use]
	pub(in crate::state) fn new(next_header_key: P::HeaderKey) -> Self {
		Self {
			header_key: None,
			key: None,
			msg_num: 0,
			next_header_key,
			skipped_msg_keys: super::skipped_msg_keys::SkippedMsgKeys::new(),
		}
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
		self.msg_num = 0;
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_upgrade() {
		// Create chain
		let mut chain = super::Recv::<crate::default_crypto::Provider>::new(
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from([123; 32]),
		);
		let old_next_header_key = chain.next_header_key.clone();

		// TODO: KDF test

		// Upgrade chain
		let new_key = <crate::default_crypto::Provider as crate::crypto::Provider>
			::MsgChainKey::from([234; 32]);
		let new_next_header_key =
			<crate::default_crypto::Provider as crate::crypto::Provider>
				::HeaderKey::from([120; 32]);
		chain.upgrade(new_key.clone(), new_next_header_key.clone());

		// Check upgrade is done
		assert_eq!(chain.header_key, Some(old_next_header_key));
		assert_eq!(chain.key, Some(new_key));
		assert_eq!(chain.msg_num, 0);
		assert_eq!(chain.next_header_key, new_next_header_key);
	}
}
