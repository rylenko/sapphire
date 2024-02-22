/// Encodes into [`Vec`] of bytes.
///
/// [`Vec`]: alloc::vec::Vec
pub trait Encode {
	#[must_use]
	fn encode(&self) -> alloc::vec::Vec<u8>;
}

/// Decodes from slice.
pub trait Decode: Sized {
	type Error: core::error::Error + 'static;

	/// # Return
	///
	/// `Self` and count of bytes readed.
	fn decode(slice: &[u8]) -> Result<(Self, usize), Self::Error>;
}
