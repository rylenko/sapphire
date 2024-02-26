/// State of Alice and Bob.
#[derive(Clone)]
pub struct State {
	/// Local private key.
	local_private_key: super::key::Private,
	/// DH remote public key.
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
		send_hdr_key: super::key::Hdr,
		recv_next_hdr_key: super::key::Hdr,
		skipped_msg_keys_max_cnt: u32,
	) -> Self {
		// Generate new private key
		let local_private_key = super::key::Private::random();

		// Create root chain
		let mut root = super::root::Root::new(root_key);
		// Use KDF in root chain for sending chain
		let (send_key, send_next_hdr_key) =
			root.kdf(&local_private_key.dh(bob_public_key));

		Self {
			local_private_key,
			remote_public_key: Some(bob_public_key),
			recv: super::recv::Recv::new(
				recv_next_hdr_key,
				skipped_msg_keys_max_cnt,
			),
			root,
			send: super::send::Send::new(
				Some(send_key),
				Some(send_hdr_key),
				send_next_hdr_key,
			),
		}
	}

	/// Creates Bob's state.
	#[inline]
	#[must_use]
	pub fn new_bob(
		private_key: super::key::Private,
		root_key: super::key::Root,
		send_next_hdr_key: super::key::Hdr,
		recv_next_hdr_key: super::key::Hdr,
		skipped_msg_keys_max_cnt: u32,
	) -> Self {
		Self {
			local_private_key: private_key,
			remote_public_key: None,
			recv: super::recv::Recv::new(
				recv_next_hdr_key,
				skipped_msg_keys_max_cnt,
			),
			root: super::root::Root::new(root_key),
			send: super::send::Send::new(None, None, send_next_hdr_key),
		}
	}

	/// Decrypts `cipher` and authenticates it with concatenation of `auth` and
	/// `encrypted_hdr`.
	///
	/// # Errors
	///
	/// May corrupt `encrypted_hdr_buf` on error.
	///
	/// See [`Decrypt`].
	///
	/// [`Decrypt`]: super::error::Decrypt
	pub fn decrypt(
		&mut self,
		buf: &mut [u8],
		auth: &[u8],
		encrypted_hdr_buf: &mut [u8; super::encrypted_hdr_buf::LEN],
	) -> Result<(), super::error::Decrypt> {
		use super::msg_chain::MsgChain as _;

		// Trying to check whether the message was skipped
		if let Some(msg_key) =
			self.recv.pop_skipped_msg_key(encrypted_hdr_buf)?
		{
			return super::cipher::decrypt(msg_key.as_bytes(), buf, &[
				auth,
				encrypted_hdr_buf,
			])
			.map_err(super::error::Decrypt::SkippedMsg);
		}

		// TODO: Try to escape this.
		//
		// We create a copy of the header because `decrypt_hdr` will decrypt
		// buffer and we will not be able to authenticate him.
		let encrypted_hdr_buf_copy = *encrypted_hdr_buf;

		// Trying to decrypt the header with the receiving chain
		let (hdr, need_dh_ratchet) =
			self.recv.decrypt_hdr(encrypted_hdr_buf)?;
		if need_dh_ratchet {
			// Skip current chain message keys and upgrade chains using DH
			// ratchet
			self.recv
				.skip_msg_keys(hdr.prev_send_msgs_cnt())
				.map_err(super::error::Decrypt::SkipOldChainMsgKeys)?;
			self.dh_ratchet(hdr.public_key());
		}
		// Skip message keys to get current key
		self.recv
			.skip_msg_keys(hdr.msg_num())
			.map_err(super::error::Decrypt::SkipCurrChainMsgKeys)?;

		// KDF to get decryption key, decrypt and commit new chain key
		let (msg_chain_key, msg_key) = self.recv.kdf()?;
		super::cipher::decrypt(msg_key.as_bytes(), buf, &[
			auth,
			&encrypted_hdr_buf_copy,
		])
		.map_err(super::error::Decrypt::NewMsg)?;
		self.recv.commit_kdf(msg_chain_key);
		Ok(())
	}

	/// Encrypts `buf` and authenticates it with concatenation of `auth` and
	/// `encrypted_hdr_buf`. Before that encrypts header and fills the
	/// `encrypted_hdr_buf` with encrypted bytes.
	///
	/// Encrypts everything except the last 32 bytes. The last 32 bytes are
	/// occupied by MAC.
	///
	/// # Errors
	///
	/// May corrupt `encrypted_hdr_buf` on error.
	///
	/// See [`Encrypt`].
	///
	/// [`Encrypt`]: super::error::Encrypt
	pub fn encrypt(
		&mut self,
		buf: &mut [u8],
		auth: &[u8],
		encrypted_hdr_buf: &mut [u8; super::encrypted_hdr_buf::LEN],
	) -> Result<(), super::error::Encrypt> {
		use {super::msg_chain::MsgChain as _, zerocopy::AsBytes as _};

		// Move sending chain forward
		let (msg_chain_key, msg_key, msg_num, hdr_key, prev_msgs_cnt) =
			self.send.kdf()?;

		// Create header and get it's bytes
		let hdr = super::hdr::Hdr::new(
			super::key::Public::from(&self.local_private_key),
			msg_num,
			prev_msgs_cnt,
		);
		let hdr_bytes = hdr.as_bytes();

		// Copy header bytes to encrypted header buffer
		debug_assert_eq!(
			hdr_bytes.len(),
			super::encrypted_hdr_buf::LEN_WITHOUT_MAC
		);
		encrypted_hdr_buf[..hdr_bytes.len()].copy_from_slice(hdr_bytes);
		// Encrypt header bytes buffer
		super::cipher::encrypt(hdr_key.as_bytes(), encrypted_hdr_buf, &[])
			.map_err(super::error::Encrypt::Hdr)?;

		// Encrypt plain data with encrypted header authentication
		super::cipher::encrypt(msg_key.as_bytes(), buf, &[
			auth,
			encrypted_hdr_buf,
		])
		.map_err(super::error::Encrypt::Buf)?;
		// Commit new KDF key because of successful encryption
		self.send.commit_kdf(msg_chain_key);

		Ok(())
	}

	/// DH ratchet of the state with nere remote public key.
	fn dh_ratchet(&mut self, new_remote_public_key: super::key::Public) {
		use super::msg_chain::MsgChain as _;

		// Update key
		self.remote_public_key = Some(new_remote_public_key);

		// Use KDF in root chain for receiving chain
		let (chain_key, next_hdr_key) =
			self.root.kdf(&self.local_private_key.dh(new_remote_public_key));
		self.recv.upgrade(chain_key, next_hdr_key);

		// Generate new key pair
		self.local_private_key = super::key::Private::random();

		// Use KDF in root chain for sending chain
		let (chain_key, next_hdr_key) =
			self.root.kdf(&self.local_private_key.dh(new_remote_public_key));
		self.send.upgrade(chain_key, next_hdr_key);
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

		// Bob's DH ratchet
		bob.dh_ratchet((&alice.local_private_key).into());

		// Compare root chains. They are not equal because of Bob's root chain
		// double KDF
		assert_ne!(bob.root.key(), alice.root.key());

		// Compare Bob's receiving and Alice's sending chains
		assert_eq!(bob.recv.key(), alice.send.key());
		assert_eq!(bob.recv.hdr_key(), alice.send.hdr_key());
		assert_eq!(bob.recv.next_hdr_key(), alice.send.next_hdr_key());

		// Compare Bob's sending and Alice's receiving chains. They are not
		// equal because of Bob's root chain double KDF
		assert_ne!(bob.send.key(), alice.recv.key());
		assert_ne!(bob.send.hdr_key(), alice.recv.hdr_key());
		assert_ne!(bob.send.next_hdr_key(), alice.recv.next_hdr_key());

		// Alice's DH ratchet
		alice.dh_ratchet((&bob.local_private_key).into());

		// Compare root chains. They are not equal because of Alice's root
		// chain double KDF
		assert_ne!(bob.root.key(), alice.root.key());

		// Compare Bob's sending and Alice's receiving chains after Alice's
		// ratchet
		assert_eq!(bob.send.key(), alice.recv.key());
		assert_eq!(bob.send.hdr_key(), alice.recv.hdr_key());
		assert_eq!(bob.send.next_hdr_key(), alice.recv.next_hdr_key());

		// Compare Bob's receiving and Alice's sending chains.
		// Compare root chains. They are not equal because of Alice's root
		// chain double KDF
		assert_ne!(bob.recv.key(), alice.send.key());
		assert_ne!(bob.recv.hdr_key(), alice.send.hdr_key());
		assert_ne!(bob.recv.next_hdr_key(), alice.send.next_hdr_key());
	}
}
