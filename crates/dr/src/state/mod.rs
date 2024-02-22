/*!
Home of state for Alice and Bob, chains and header.
*/

mod chain;
mod error;
mod header;
mod num;
mod recv;
mod root;
mod send;
mod skipped_msg_keys;

pub use num::Num;

/// State of Alice and Bob.
#[rustfmt::skip]
pub struct State<P: crate::crypto::Provider> {
	/// Diffie-Hellman local private and public keys pair.
	local_key_pair: P::KeyPair,

	/// Diffie-Hellman remote public key.
	remote_public_key: Option<<P::KeyPair as crate::crypto::KeyPair>::Public>,

	/// Receiving chain.
	recv_chain: recv::Recv<P>,

	/// Root chain.
	root_chain: root::Root<P>,

	/// Sending chain.
	send_chain: send::Send<P>,
}

impl<P> State<P>
where
	P: crate::crypto::Provider,
{
	/// Creates Alice's state.
	#[must_use]
	pub fn new_alice(
		bob_public_key: <P::KeyPair as crate::crypto::KeyPair>::Public,
		shared_secret: P::RootChainKey,
		shared_send_chain_header_key: P::HeaderKey,
		shared_recv_chain_next_header_key: P::HeaderKey,
		skipped_msg_keys_max_cnt: num::Num,
	) -> Self {
		use crate::crypto::KeyPair as _;

		// Generate new key pair
		let local_key_pair = P::KeyPair::rand();

		// Create root chain
		let mut root_chain = root::Root::new(shared_secret);
		// Use KDF in root chain for sending chain
		let (send_chain_key, send_chain_next_header_key) =
			root_chain.kdf(&P::dh(&local_key_pair, &bob_public_key));

		Self {
			local_key_pair,
			remote_public_key: Some(bob_public_key),
			recv_chain: recv::Recv::new(
				shared_recv_chain_next_header_key,
				skipped_msg_keys_max_cnt,
			),
			root_chain,
			send_chain: send::Send::new(
				Some(send_chain_key),
				Some(shared_send_chain_header_key),
				send_chain_next_header_key,
			),
		}
	}

	/// Creates Bob's state.
	#[inline]
	#[must_use]
	pub fn new_bob(
		key_pair: P::KeyPair,
		shared_secret: P::RootChainKey,
		shared_send_chain_next_header_key: P::HeaderKey,
		shared_recv_chain_next_header_key: P::HeaderKey,
		skipped_msg_keys_max_cnt: num::Num,
	) -> Self {
		Self {
			local_key_pair: key_pair,
			remote_public_key: None,
			recv_chain: recv::Recv::new(
				shared_recv_chain_next_header_key,
				skipped_msg_keys_max_cnt,
			),
			root_chain: root::Root::new(shared_secret),
			send_chain: send::Send::new(
				None,
				None,
				shared_send_chain_next_header_key,
			),
		}
	}

	/// Decrypts `cipher` and authenticates it with concatenation of `auth` and
	/// `encrypted_header`.
	///
	/// # Errors
	///
	/// See [`Decrypt`].
	///
	/// [`Decrypt`]: error::Decrypt
	pub fn decrypt(
		&mut self,
		encrypted_header: &[u8],
		cipher: &[u8],
		auth: &[u8],
	) -> Result<alloc::vec::Vec<u8>, error::Decrypt> {
		use chain::Chain as _;

		// Trying to check whether the message was skipped
		if let Some(msg_key) =
			self.recv_chain.pop_skipped_msg_key(encrypted_header)?
		{
			let plain =
				P::decrypt(&msg_key, cipher, &[auth, encrypted_header])
					.map_err(|e| error::Decrypt::SkippedMsg(e.into()))?;
			return Ok(plain);
		}

		// Trying to decrypt the header with the receiving chain
		let (header, dh_ratchet_needed) =
			self.recv_chain.decrypt_header(encrypted_header)?;
		if dh_ratchet_needed {
			// Skip current chain message keys and upgrade chains using DH
			// ratchet
			self.recv_chain
				.skip_msg_keys(header.prev_send_chain_msgs_cnt())
				.map_err(error::Decrypt::SkipOldChainMsgKeys)?;
			self.dh_ratchet(header.public_key().clone());
		}

		// Skip message keys to get current key
		self.recv_chain
			.skip_msg_keys(header.msg_num())
			.map_err(error::Decrypt::SkipCurrChainMsgKeys)?;

		// Get key and decrypt
		let (msg_key, _header_key) = self.recv_chain.kdf()?;
		let plain = P::decrypt(&msg_key, cipher, &[auth, encrypted_header])
			.map_err(|e| error::Decrypt::NewMsg(e.into()))?;
		Ok(plain)
	}

	/// Encrypts `plain` text and authenticates it with concatenation of `auth`
	/// and encrypted header.
	///
	/// # Errors
	///
	/// See [`Encrypt`].
	///
	/// [`Encrypt`]: error::Encrypt
	pub fn encrypt(
		&mut self,
		plain: &[u8],
		auth: &[u8],
	) -> Result<(alloc::vec::Vec<u8>, alloc::vec::Vec<u8>), error::Encrypt> {
		use {
			crate::crypto::KeyPair as _, alloc::borrow::ToOwned as _,
			chain::Chain as _,
		};

		// Create header and encode it to bytes
		let header_bytes = bincode::encode_to_vec(
			header::Header::<P>::new(
				self.local_key_pair.public().to_owned(),
				self.send_chain.next_msg_num(),
				self.send_chain.prev_msgs_cnt(),
			),
			bincode::config::standard(),
		)?;

		// Move sending chain forward
		let (msg_key, header_key) = self.send_chain.kdf()?;

		// Encrypt header's bytes
		let encrypted_header_bytes =
			P::encrypt_header(header_key, &header_bytes)
				.map_err(|e| error::Encrypt::HeaderBytes(e.into()))?;

		// Encrypt plain data using message key and concatenation of user's
		// auth data and encrypted header bytes as auth data
		let cipher =
			P::encrypt(&msg_key, plain, &[auth, &encrypted_header_bytes])
				.map_err(|e| error::Encrypt::Plain(e.into()))?;

		Ok((encrypted_header_bytes, cipher))
	}

	/// Diffie-Hellman ratchet of the state with new data from `header`.
	fn dh_ratchet(
		&mut self,
		new_remote_public_key: <P::KeyPair as crate::crypto::KeyPair>::Public,
	) {
		use {crate::crypto::KeyPair as _, chain::Chain as _};

		// Extract remote public key from the header
		let remote_public_key_ref = {
			self.remote_public_key = Some(new_remote_public_key);
			// SAFETY: we set `Some` on the previous line
			self.remote_public_key.as_ref().unwrap()
		};

		// Use KDF in root chain for receiving chain
		let (chain_key, next_header_key) = self
			.root_chain
			.kdf(&P::dh(&self.local_key_pair, remote_public_key_ref));
		self.recv_chain.upgrade(chain_key, next_header_key);

		// Generate new key pair
		self.local_key_pair = P::KeyPair::rand();

		// Use KDF in root chain for sending chain
		let (chain_key, next_header_key) = self
			.root_chain
			.kdf(&P::dh(&self.local_key_pair, remote_public_key_ref));
		self.send_chain.upgrade(chain_key, next_header_key);
	}
}
