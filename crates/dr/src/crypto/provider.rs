/// Cryptography provider for Double Ratchet algorithm.
pub trait Provider: Eq + PartialEq + core::fmt::Debug {
	/// Decryption error.
	type DecryptError: core::error::Error + 'static;

	/// Header decryption error.
	type DecryptHeaderError: core::error::Error + 'static;

	/// Encryption error.
	type EncryptError: core::error::Error + 'static;

	/// Header encryption error.
	type EncryptHeaderError: core::error::Error + 'static;

	/// Key for header encryption.
	type HeaderKey: Eq
		+ alloc::borrow::ToOwned<Owned = Self::HeaderKey>
		+ core::hash::Hash;

	/// Private and public keys pair.
	type KeyPair: super::key_pair::KeyPair;

	/// Key for sending and receiving chains.
	type MsgChainKey;

	/// Key for messages encryption.
	type MsgKey;

	/// Key for root chain.
	type RootChainKey;

	/// The result of Diffie-Hellman calculations.
	type SharedSecret;

	/// Decrypts `cipher` using `key` and authentication data `auth`.
	///
	/// # Errors
	///
	/// See [`DecryptError`].
	///
	/// [`DecryptError`]: Self::DecryptError
	fn decrypt(
		key: &Self::MsgKey,
		cipher: &[u8],
		auth: &[&[u8]],
	) -> Result<alloc::vec::Vec<u8>, Self::DecryptError>;

	/// Decrypts header `cipher` using `key`.
	///
	/// # Errors
	///
	/// See [`DecryptHeaderError`].
	///
	/// [`DecryptHeaderError`]: Self::DecryptHeaderError
	fn decrypt_header(
		key: &Self::HeaderKey,
		cipher: &[u8],
	) -> Result<alloc::vec::Vec<u8>, Self::DecryptHeaderError>;

	/// Diffie-Hellman calculation.
	#[must_use]
	fn dh(
		pair: &Self::KeyPair,
		public: &<Self::KeyPair as super::key_pair::KeyPair>::Public,
	) -> Self::SharedSecret;

	/// Encrypts `plain` using `key` and authentication data `auth`.
	///
	/// # Errors
	///
	/// See [`EncryptError`].
	///
	/// [`EncryptError`]: Self::EncryptError
	fn encrypt(
		key: &Self::MsgKey,
		plain: &[u8],
		auth: &[&[u8]],
	) -> Result<alloc::vec::Vec<u8>, Self::EncryptError>;

	/// Encrypts header `plain` using `key`.
	///
	/// # Errors
	///
	/// See [`EncryptHeaderError`].
	///
	/// [`EncryptHeaderError`]: Self::EncryptHeaderError
	fn encrypt_header(
		key: &Self::HeaderKey,
		plain: &[u8],
	) -> Result<alloc::vec::Vec<u8>, Self::EncryptHeaderError>;

	/// Generates new root chain key, sending chain key and sending chain next
	/// header key.
	#[must_use]
	fn kdf_root_chain(
		key: &Self::RootChainKey,
		input: &Self::SharedSecret,
	) -> (Self::RootChainKey, Self::MsgChainKey, Self::HeaderKey);

	/// Generates new chain key and message key for sending and receive chains.
	#[must_use]
	fn kdf_msg_chain(
		key: &Self::MsgChainKey,
	) -> (Self::MsgChainKey, Self::MsgKey);
}
