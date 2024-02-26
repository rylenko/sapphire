pub(crate) const ENCRYPTED_HEADER_BUFF_LEN: usize =
	core::mem::size_of::<super::header::Header>() + 32;

/// Creates empty buffer for encrypted header.
#[inline]
#[must_use]
pub const fn create_encrypted_header_buff() -> [u8; ENCRYPTED_HEADER_BUFF_LEN]
{
	[0; ENCRYPTED_HEADER_BUFF_LEN]
}
