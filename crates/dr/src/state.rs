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

	/// Decrypts `buf` and authenticates it with concatenation of `assoc_data`
	/// and encrypted header's bytes.
	///
	/// # Errors
	///
	/// See [`Decrypt`].
	///
	/// [`Decrypt`]: super::error::Decrypt
	pub fn decrypt(
		&mut self,
		buf: &mut [u8],
		assoc_data: &[u8],
		clue: &super::clue::Clue,
	) -> Result<(), super::error::Decrypt> {
		use {
			super::{draft::Draft as _, msg_chain::MsgChain as _},
			zerocopy::AsBytes as _,
		};

		// Get encrypted header to not copy every time
		let encrypted_hdr = clue.encrypted_hdr();

		// Trying to check whether the message was skipped
		if let Some(msg_key) = self.recv.pop_skipped_msg_key(&encrypted_hdr) {
			return super::cipher::decrypt(
				msg_key.as_bytes(),
				buf,
				&[assoc_data, encrypted_hdr.as_bytes()],
				clue.buf_tag(),
			)
			.map_err(super::error::Decrypt::SkippedMsg);
		}

		// Create draft to do not corrupt state. See trait implementation for
		// more
		let mut draft = self.create_draft();

		// Decrypt the header with the receiving chain
		let (hdr, need_dh_ratchet) = draft.recv.decrypt_hdr(&encrypted_hdr)?;
		if need_dh_ratchet {
			// Skip current chain message keys and upgrade chains using DH
			// ratchet
			draft
				.recv
				.skip_msg_keys(hdr.prev_send_msgs_cnt())
				.map_err(super::error::Decrypt::SkipOldChainMsgKeys)?;
			draft.dh_ratchet(hdr.public_key());
		}
		// Skip message keys to get current key
		draft
			.recv
			.skip_msg_keys(hdr.msg_num())
			.map_err(super::error::Decrypt::SkipCurrChainMsgKeys)?;

		// KDF to get decryption key and decrypt
		let msg_key = draft.recv.kdf()?;
		super::cipher::decrypt(
			msg_key.as_bytes(),
			buf,
			&[assoc_data, encrypted_hdr.as_bytes()],
			clue.buf_tag(),
		)
		.map_err(super::error::Decrypt::NewMsg)?;

		// Commit draft and return
		self.commit_draft(draft);
		Ok(())
	}

	/// Encrypts `buf` and authenticates it with concatenation of `assoc_data`
	/// and encrypted header bytes.
	///
	/// # Errors
	///
	/// See [`Encrypt`].
	///
	/// [`Encrypt`]: super::error::Encrypt
	pub fn encrypt(
		&mut self,
		buf: &mut [u8],
		assoc_data: &[u8],
	) -> Result<super::clue::Clue, super::error::Encrypt> {
		use {
			super::{draft::Draft as _, msg_chain::MsgChain as _},
			zerocopy::AsBytes as _,
		};

		// Create draft to do not corrupt state. See trait implementation for
		// more
		let mut draft = self.create_draft();
		// Move sending chain forward
		let (msg_key, msg_num, hdr_key, prev_msgs_cnt) = draft.send.kdf()?;

		// Create header and encrypt it
		let hdr = super::hdr::Hdr::new(
			super::key::Public::from(&draft.local_private_key),
			msg_num,
			prev_msgs_cnt,
		);
		let encrypted_hdr = hdr.encrypt(hdr_key);
		// Encrypt and authenticate plain buffer
		let buf_tag = super::cipher::encrypt(msg_key.as_bytes(), buf, &[
			assoc_data,
			encrypted_hdr.as_bytes(),
		]);

		// Commit draft and return a clue
		self.commit_draft(draft);
		Ok(super::clue::Clue::new(buf_tag, encrypted_hdr))
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

impl super::draft::Draft for State {
	fn commit_draft(&mut self, draft: Self) {
		self.local_private_key = draft.local_private_key;
		self.remote_public_key = draft.remote_public_key;
		self.recv.commit_draft(draft.recv);
		self.root = draft.root;
		self.send = draft.send;
	}

	fn create_draft(&self) -> Self {
		Self {
			local_private_key: self.local_private_key.clone(),
			remote_public_key: self.remote_public_key,
			recv: self.recv.create_draft(),
			root: self.root.clone(),
			send: self.send.clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	fn init() -> (super::State, super::State) {
		const ALICE_RECV_HEADER_KEY: [u8; 32] = [3; 32];
		const ROOT_KEY: [u8; 32] = [1; 32];
		const ALICE_SEND_HEADER_KEY: [u8; 32] = [2; 32];
		// Create Alice and Bob
		let bob = super::State::new_bob(
			crate::key::Private::random(),
			ROOT_KEY.into(),
			ALICE_RECV_HEADER_KEY.into(),
			ALICE_SEND_HEADER_KEY.into(),
			3,
		);
		let alice = super::State::new_alice(
			crate::key::Public::from(&bob.local_private_key),
			ROOT_KEY.into(),
			ALICE_SEND_HEADER_KEY.into(),
			ALICE_RECV_HEADER_KEY.into(),
			5,
		);
		(alice, bob)
	}

	#[test]
	fn test_dh_ratchet() {
		let (mut alice, mut bob) = init();

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
