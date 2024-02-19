/// Header of new sent message.
#[derive(Debug, Eq, PartialEq)]
pub struct Header<P>
where
	P: crate::crypto::Provider,
{
	/// Local public key.
	public_key: <P::KeyPair as crate::crypto::KeyPair>::Public,

	/// Message number.
	msg_num: super::chain::Num,

	/// Messages count in previous sending chain.
	prev_send_chain_msgs_cnt: super::chain::Num,
}

impl<P> Header<P>
where
	P: crate::crypto::Provider,
{
	#[inline]
	#[must_use]
	pub(super) const fn new(
		public_key: <P::KeyPair as crate::crypto::KeyPair>::Public,
		msg_num: super::chain::Num,
		prev_send_chain_msgs_cnt: super::chain::Num,
	) -> Self {
		Self { public_key, msg_num, prev_send_chain_msgs_cnt }
	}
}

impl<P> bincode::Encode for Header<P>
where
	P: crate::crypto::Provider,
{
	fn encode<E>(&self, e: &mut E) -> Result<(), bincode::error::EncodeError>
	where
		E: bincode::enc::Encoder,
	{
		bincode::Encode::encode(&self.public_key, e)?;
		bincode::Encode::encode(&self.msg_num, e)?;
		bincode::Encode::encode(&self.prev_send_chain_msgs_cnt, e)
	}
}

impl<P> bincode::Decode for Header<P>
where
	P: crate::crypto::Provider,
{
	fn decode<D>(d: &mut D) -> Result<Self, bincode::error::DecodeError>
	where
		D: bincode::de::Decoder,
	{
		Ok(Self::new(
			bincode::Decode::decode(d)?,
			bincode::Decode::decode(d)?,
			bincode::Decode::decode(d)?,
		))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_encode_and_decode() {
		use crate::crypto::KeyPair as _;

		// Create valid test data
		let valid_key_pair = crate::default_crypto::KeyPair::rand();
		let valid_header =
			super::Header::<crate::default_crypto::Provider>::new(
				*valid_key_pair.public(),
				55,
				1111,
			);
		let valid_header_bytes: alloc::vec::Vec<u8> =
			[valid_key_pair.public().as_ref(), &[55], &[251, 87, 4]].concat();

		// Test encoding.
		//
		// TODO: Wait for `bincode`'s `core::error::Error` impl to return
		// `Result`
		let bytes =
			bincode::encode_to_vec(&valid_header, bincode::config::standard())
				.unwrap();
		assert_eq!(bytes, valid_header_bytes);
		assert!(bytes.len() < core::mem::size_of_val(&valid_header));

		// Test decoding.
		//
		// TODO: Wait for `bincode`'s `core::error::Error` impl to return
		// `Result`
		let header: super::Header<crate::default_crypto::Provider> =
			bincode::decode_from_slice(&bytes, bincode::config::standard())
				.unwrap()
				.0;
		assert_eq!(header, valid_header);
	}
}
