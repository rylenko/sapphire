pub(in crate::state) mod error;
mod header;
mod num;
mod recv;
mod root;
mod send;
mod skipped_msg_keys;

pub use num::Num;
pub(in crate::state) use {
	header::Header, recv::Recv, root::Root, send::Send,
};

/// Common trait for receiving and sending chains.
///
/// They are should provide the same API for new message keys creation and
/// upgrading after Diffie-Hellman ratchet.
pub(in crate::state) trait Chain<P>
where
	P: crate::crypto::Provider,
{
	type KdfError: core::error::Error;

	/// Moves chain forward, updates current key and return new message and
	/// header keys.
	fn kdf(&mut self) -> Result<(P::MsgKey, &P::HeaderKey), Self::KdfError>;

	/// Upgrades the chain after Diffie-Hellman ratchet.
	fn upgrade(
		&mut self,
		new_key: P::MsgChainKey,
		new_next_header_key: P::HeaderKey,
	);
}
