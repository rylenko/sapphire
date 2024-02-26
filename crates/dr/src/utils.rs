pub(crate) const ENCRYPTED_HDR_BUF_LEN: usize =
	core::mem::size_of::<super::hdr::Hdr>() + 32;

/// Creates empty bufer for encrypted header.
#[inline]
#[must_use]
pub const fn create_encrypted_hdr_buf() -> [u8; ENCRYPTED_HDR_BUF_LEN] {
	[0; ENCRYPTED_HDR_BUF_LEN]
}
