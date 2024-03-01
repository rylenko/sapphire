/*! Clue for encrypted buffer, which contains encrypted header and MAC. */

/// Clue with encryped buffer tag and encrypted header for decryption.
///
/// TODO: Currently 64 bytes. Need to find out whether it should be a [`Copy`].
#[derive(
	Copy,
	Clone,
	Debug,
	Eq,
	Hash,
	PartialEq,
	zerocopy::AsBytes,
	zerocopy::FromBytes,
	zerocopy::FromZeroes,
)]
#[repr(packed)]
pub struct Clue {
	buf_tag: super::cipher::Tag,
	encrypted_hdr: super::hdr::Encrypted,
}

impl Clue {
	#[inline]
	#[must_use]
	pub(super) fn new(
		buf_tag: super::cipher::Tag,
		encrypted_hdr: super::hdr::Encrypted,
	) -> Self {
		Self { buf_tag, encrypted_hdr }
	}

	#[inline]
	#[must_use]
	pub(super) fn buf_tag(&self) -> super::cipher::Tag {
		self.buf_tag
	}

	#[inline]
	#[must_use]
	pub(super) fn encrypted_hdr(&self) -> super::hdr::Encrypted {
		self.encrypted_hdr
	}
}
