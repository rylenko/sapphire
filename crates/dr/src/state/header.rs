/// Header of new sent message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Header<P>
where
	P: crate::crypto::Provider,
{
	/// Local public key.
	public_key: <P::KeyPair as crate::crypto::KeyPair>::Public,

	/// Message number.
	msg_num: super::num::Num,

	/// Messages count in previous sending chain.
	prev_send_msgs_cnt: super::num::Num,
}

impl<P> Header<P>
where
	P: crate::crypto::Provider,
{
	#[inline]
	#[must_use]
	pub(super) const fn new(
		public_key: <P::KeyPair as crate::crypto::KeyPair>::Public,
		msg_num: super::num::Num,
		prev_send_msgs_cnt: super::num::Num,
	) -> Self {
		Self { public_key, msg_num, prev_send_msgs_cnt }
	}

	#[inline]
	#[must_use]
	pub(super) const fn msg_num(&self) -> super::num::Num {
		self.msg_num
	}

	#[inline]
	#[must_use]
	pub(super) const fn prev_send_msgs_cnt(&self) -> super::num::Num {
		self.prev_send_msgs_cnt
	}

	#[inline]
	#[must_use]
	pub(super) const fn public_key(
		&self,
	) -> &<P::KeyPair as crate::crypto::KeyPair>::Public {
		&self.public_key
	}
}

impl<P> crate::code::Encode for Header<P>
where
	P: crate::crypto::Provider,
{
	fn encode(&self) -> alloc::vec::Vec<u8> {
		// TODO: set right capacity?
		let mut ret = self.public_key.encode();
		ret.extend(self.msg_num.to_le_bytes());
		ret.extend(self.prev_send_msgs_cnt.to_le_bytes());
		ret
	}
}

impl<P> crate::code::Decode for Header<P>
where
	P: crate::crypto::Provider,
{
	type Error = super::error::HeaderDecode;

	fn decode(slice: &[u8]) -> Result<(Self, usize), Self::Error> {
		// Decode data
		let (public_key, public_key_size) =
			<P::KeyPair as crate::crypto::KeyPair>::Public::decode(slice)
				.map_err(|e| Self::Error::PublicKey(e.into()))?;
		let (msg_num, msg_num_size) =
			super::num::Num::decode(&slice[public_key_size..])
				.map_err(Self::Error::MsgNum)?;
		let (prev_send_msgs_cnt, prev_send_msgs_cnt_size) =
			super::num::Num::decode(&slice[public_key_size + msg_num_size..])
				.map_err(Self::Error::PrevSendMsgsCnt)?;

		// Compute length and return
		Ok((
			Self::new(public_key, msg_num, prev_send_msgs_cnt),
			public_key_size + msg_num_size + prev_send_msgs_cnt_size,
		))
	}
}

#[cfg(test)]
mod tests {
	const MSG_NUM: super::super::num::Num = 55;
	const PREV_SEND_MSGS_CNT: super::super::num::Num = 1111;

	#[test]
	fn test_encode_and_decode() {
		use crate::{
			code::{Decode as _, Encode as _},
			crypto::KeyPair as _,
		};

		// Create valid test data
		let valid_key_pair = crate::default_crypto::KeyPair::rand();
		let valid_header =
			super::Header::<crate::default_crypto::Provider>::new(
				*valid_key_pair.public(),
				MSG_NUM,
				PREV_SEND_MSGS_CNT,
			);
		// TODO: untie [0, 0, 4, 87] from u32
		let valid_header_bytes: alloc::vec::Vec<u8> = [
			valid_key_pair.public().as_ref(),
			&MSG_NUM.to_le_bytes(),
			&PREV_SEND_MSGS_CNT.to_le_bytes(),
		]
		.concat();

		// Test encoding.
		let bytes = valid_header.encode();
		assert_eq!(bytes, valid_header_bytes);
		assert!(bytes.len() <= core::mem::size_of_val(&valid_header));

		// Test decoding.
		assert_eq!(super::Header::decode(&bytes).unwrap().0, valid_header);
	}
}
