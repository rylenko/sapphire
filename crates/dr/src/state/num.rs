/// This type should be the number of messages in the previous sending chain,
/// the number of the current message, etc.
pub type Num = u32; // TODO: achieve an easy change to `u64`, `u16` and `u8`

impl crate::code::Encode for Num {
	#[inline]
	fn encode(&self) -> alloc::vec::Vec<u8> {
		self.to_le_bytes().to_vec()
	}
}

impl crate::code::Decode for Num {
	type Error = super::error::NumDecode;

	fn decode(slice: &[u8]) -> Result<(Self, usize), Self::Error> {
		const SIZE: usize = core::mem::size_of::<Num>();
		if slice.len() < SIZE {
			Err(Self::Error::InvalidLen)
		} else {
			let mut array = [0; SIZE];
			array.copy_from_slice(&slice[..SIZE]);
			Ok((Self::from_le_bytes(array), SIZE))
		}
	}
}
