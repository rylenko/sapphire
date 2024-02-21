/// Private and public key pair.
pub trait KeyPair: Clone + Eq + PartialEq + core::fmt::Debug {
	/// Private key of pair.
	type Private;

	/// Public key of pair.
	type Public: Clone
		+ Eq
		+ PartialEq
		+ core::fmt::Debug
		+ bincode::Decode
		+ bincode::Encode;

	/// Generates a new random key pair.
	#[must_use]
	fn rand() -> Self;

	/// Reference to private key of the pair.
	#[must_use]
	fn private(&self) -> &Self::Private;

	/// Reference to public key of the pair.
	#[must_use]
	fn public(&self) -> &Self::Public;
}
