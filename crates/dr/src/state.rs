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

		// Get key and decrypt
		let (msg_key, _header_key) = self.recv.kdf()?;
		super::cipher::decrypt_auth(msg_key.as_bytes(), cipher, &[
			auth,
			encrypted_header,
		])
		.map_err(super::error::Decrypt::NewMsg)
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
		let (msg_key, msg_num, header_key, prev_msgs_cnt) = self.send.kdf()?;

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
