/// State of Alice and Bob.
#[derive(Clone)]
pub struct State {
	/// Local private key.
	local_private_key: super::key::Private,
	/// Diffie-Hellman remote public key.
	remote_public_key: Option<super::key::Public>,
	/// Receiving chain.
	recv: super::recv::Recv,
	/// Root chain.
	root: super::root::Root,
	/// Sending chain.
	send: super::send::Send,
}

impl State {
	/// Creates Alice's state.
	#[must_use]
	pub fn new_alice(
		bob_public_key: super::key::Public,
		root_key: super::key::Root,
		send_header_key: super::key::Header,
		recv_next_header_key: super::key::Header,
		skipped_msg_keys_max_cnt: u32,
	) -> Self {
		// Generate new private key
		let local_private_key = super::key::Private::random();

		// Create root chain
		let mut root = super::root::Root::new(root_key);
		// Use KDF in root chain for sending chain
		let (send_key, send_next_header_key) =
			root.kdf(&local_private_key.dh(bob_public_key));

		Self {
			local_private_key,
			remote_public_key: Some(bob_public_key),
			recv: super::recv::Recv::new(
				recv_next_header_key,
				skipped_msg_keys_max_cnt,
			),
			root,
			send: super::send::Send::new(
				Some(send_key),
				Some(send_header_key),
				send_next_header_key,
			),
		}
	}

	/// Creates Bob's state.
	#[inline]
	#[must_use]
	pub fn new_bob(
		private_key: super::key::Private,
		root_key: super::key::Root,
		send_next_header_key: super::key::Header,
		recv_next_header_key: super::key::Header,
		skipped_msg_keys_max_cnt: u32,
	) -> Self {
		Self {
			local_private_key: private_key,
			remote_public_key: None,
			recv: super::recv::Recv::new(
				recv_next_header_key,
				skipped_msg_keys_max_cnt,
			),
			root: super::root::Root::new(root_key),
			send: super::send::Send::new(None, None, send_next_header_key),
		}
	}

	/// Decrypts `cipher` and authenticates it with concatenation of `auth` and
	/// `encrypted_header`.
	///
	/// # Errors
	///
	/// See [`Decrypt`].
	///
	/// [`Decrypt`]: super::error::Decrypt
	pub fn decrypt(
		&mut self,
		encrypted_header: &[u8],
		cipher: &[u8],
		auth: &[u8],
	) -> Result<alloc::vec::Vec<u8>, super::error::Decrypt> {
		use super::msg_chain::MsgChain as _;

		// Trying to check whether the message was skipped
		if let Some(msg_key) =
			self.recv.pop_skipped_msg_key(encrypted_header)?
		{
			return super::cipher::decrypt_auth(
				msg_key.as_bytes(),
				cipher,
				&[auth, encrypted_header],
			)
			.map_err(super::error::Decrypt::SkippedMsg);
		}

		// Trying to decrypt the header with the receiving chain
		let (header, need_dh_ratchet) =
			self.recv.decrypt_header(encrypted_header)?;
		if need_dh_ratchet {
			// Skip current chain message keys and upgrade chains using DH
			// ratchet
			self.recv
				.skip_msg_keys(header.prev_send_msgs_cnt())
				.map_err(super::error::Decrypt::SkipOldChainMsgKeys)?;
			self.dh_ratchet(header.public_key());
		}

		// Skip message keys to get current key
		self.recv
			.skip_msg_keys(header.msg_num())
			.map_err(super::error::Decrypt::SkipCurrChainMsgKeys)?;

		// KDF receiving chain to get message key
		let (msg_chain_key, msg_key) = self.recv.kdf()?;

		// Decrypt
		let plain =
			super::cipher::decrypt_auth(msg_key.as_bytes(), cipher, &[
				auth,
				encrypted_header,
			])
			.map_err(super::error::Decrypt::NewMsg)?;

		// Commit changes in receiving chain
		self.recv.commit_kdf(msg_chain_key);
		Ok(plain)
	}

	/// Encrypts `plain` text and authenticates it with concatenation of `auth`
	/// and encrypted header.
	///
	/// # Errors
	///
	/// See [`Encrypt`].
	///
	/// [`Encrypt`]: super::error::Encrypt
	pub fn encrypt(
		&mut self,
		plain: &[u8],
		auth: &[u8],
	) -> Result<
		(alloc::vec::Vec<u8>, alloc::vec::Vec<u8>),
		super::error::Encrypt,
	> {
		use {super::msg_chain::MsgChain as _, zerocopy::AsBytes as _};

		// Move sending chain forward
		let (msg_chain_key, msg_key, msg_num, header_key, prev_msgs_cnt) =
			self.send.kdf()?;

		// Create header and encode it to bytes
		let header = super::header::Header::new(
			super::key::Public::from(&self.local_private_key),
			msg_num,
			prev_msgs_cnt,
		);

		// Encrypt header's bytes
		let encrypted_header =
			super::cipher::encrypt(header_key.as_bytes(), header.as_bytes())?;

		// Encrypt plain data with encrypted header authentication
		let cipher =
			super::cipher::encrypt_auth(msg_key.as_bytes(), plain, &[
				auth,
				&encrypted_header,
			])?;

		// Commit new KDF key
		self.send.commit_kdf(msg_chain_key);
		Ok((encrypted_header, cipher))
	}

	/// Diffie-Hellman ratchet of the state with new data from `header`.
	fn dh_ratchet(&mut self, new_remote_public_key: super::key::Public) {
		use super::msg_chain::MsgChain as _;

		// Update key
		self.remote_public_key = Some(new_remote_public_key);

		// Use KDF in root chain for receiving chain
		let (chain_key, next_header_key) =
			self.root.kdf(&self.local_private_key.dh(new_remote_public_key));
		self.recv.upgrade(chain_key, next_header_key);

		// Generate new key pair
		self.local_private_key = super::key::Private::random();

		// Use KDF in root chain for sending chain
		let (chain_key, next_header_key) =
			self.root.kdf(&self.local_private_key.dh(new_remote_public_key));
		self.send.upgrade(chain_key, next_header_key);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_dh_ratchet() {
		const MAX_CNT: u32 = 5;
		const ALICE_RECV_HEADER_KEY: [u8; 32] = [3; 32];
		const ROOT_KEY: [u8; 32] = [1; 32];
		const ALICE_SEND_HEADER_KEY: [u8; 32] = [2; 32];

		// Create Alice and Bob
		let mut bob = super::State::new_bob(
			super::super::key::Private::random(),
			ROOT_KEY.into(),
			ALICE_RECV_HEADER_KEY.into(),
			ALICE_SEND_HEADER_KEY.into(),
			MAX_CNT,
		);
		let mut alice = super::State::new_alice(
			super::super::key::Public::from(&bob.local_private_key),
			ROOT_KEY.into(),
			ALICE_SEND_HEADER_KEY.into(),
			ALICE_RECV_HEADER_KEY.into(),
			MAX_CNT,
		);

		// Bob's Diffie-Hellman ratchet
		bob.dh_ratchet((&alice.local_private_key).into());

		// Compare root chains. They are not equal because of Bob's root chain
		// double KDF
		assert_ne!(bob.root.key(), alice.root.key());

		// Compare Bob's receiving and Alice's sending chains
		assert_eq!(bob.recv.key(), alice.send.key());
		assert_eq!(bob.recv.header_key(), alice.send.header_key());
		assert_eq!(bob.recv.next_header_key(), alice.send.next_header_key());

		// Compare Bob's sending and Alice's receiving chains. They are not
		// equal because of Bob's root chain double KDF
		assert_ne!(bob.send.key(), alice.recv.key());
		assert_ne!(bob.send.header_key(), alice.recv.header_key());
		assert_ne!(bob.send.next_header_key(), alice.recv.next_header_key());

		// Alice's Diffie-Hellman ratchet
		alice.dh_ratchet((&bob.local_private_key).into());

		// Compare root chains. They are not equal because of Alice's root
		// chain double KDF
		assert_ne!(bob.root.key(), alice.root.key());

		// Compare Bob's sending and Alice's receiving chains after Alice's
		// ratchet
		assert_eq!(bob.send.key(), alice.recv.key());
		assert_eq!(bob.send.header_key(), alice.recv.header_key());
		assert_eq!(bob.send.next_header_key(), alice.recv.next_header_key());

		// Compare Bob's receiving and Alice's sending chains.
		// Compare root chains. They are not equal because of Alice's root
		// chain double KDF
		assert_ne!(bob.recv.key(), alice.send.key());
		assert_ne!(bob.recv.header_key(), alice.send.header_key());
		assert_ne!(bob.recv.next_header_key(), alice.send.next_header_key());
	}
}
