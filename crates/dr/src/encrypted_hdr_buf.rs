/*!
Helpers to simplify encrypted header buffers usage.
*/

pub(super) const LEN: usize = LEN_WITHOUT_MAC + 32;
pub(super) const LEN_WITHOUT_MAC: usize =
	core::mem::size_of::<super::hdr::Hdr>();

/// Helper to create array for encrypted header buffer.
#[inline]
#[must_use]
pub const fn create() -> [u8; LEN] {
	[0; LEN]
}
