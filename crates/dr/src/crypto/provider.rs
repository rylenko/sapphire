/// Cryptography provider for Double Ratchet algorithm.
pub trait Provider: Eq + PartialEq + core::fmt::Debug {
	/// Error for encryption.
	type EncryptError: core::error::Error + 'static;

	/// Error for encryption.
	type EncryptHeaderBytesError: core::error::Error + 'static;

	/// Key for header encryption.
	type HeaderKey: Eq + core::hash::Hash;

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

	/// Diffie hellman calculation.
	#[must_use]
	fn dh(
		pair: &Self::KeyPair,
		public: &<Self::KeyPair as super::key_pair::KeyPair>::Public,
	) -> Self::SharedSecret;

	/// Encrypts `plain` with authenticated data `auth` using `key`.
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

	/// Encrypts header `bytes` using `key`.
	///
	/// # Errors
	///
	/// See [`EncryptHeaderBytesError`].
	///
	/// [`EncryptHeaderBytesError`]: Self::EncryptHeaderBytesError
	fn encrypt_header_bytes(
		key: &Self::HeaderKey,
		bytes: &[u8],
	) -> Result<alloc::vec::Vec<u8>, Self::EncryptHeaderBytesError>;

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
