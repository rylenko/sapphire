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
pub(crate) struct Encrypted {
	bytes: [u8; super::Hdr::SIZE],
	tag: crate::cipher::Tag,
}

impl Encrypted {
	#[must_use]
	pub(super) fn new(hdr: &super::Hdr, key: &crate::key::Hdr) -> Self {
		use zerocopy::AsBytes as _;

		// Copy and encrypt bytes
		let mut bytes = [0; super::Hdr::SIZE];
		bytes.copy_from_slice(hdr.as_bytes());
		let tag = crate::cipher::encrypt(key.as_bytes(), &mut bytes, &[]);
		Self { bytes, tag }
	}

	pub(crate) fn decrypt(
		&self,
		key: &crate::key::Hdr,
	) -> Result<super::Hdr, super::error::Decrypt> {
		use zerocopy::FromBytes as _;

		// Decrypt
		let mut bytes = self.bytes;
		crate::cipher::decrypt(key.as_bytes(), &mut bytes, &[], self.tag)?;

		// Decrypted bytes to struct
		let hdr = super::Hdr::ref_from(&bytes)
			.ok_or(super::error::Decrypt::FromBytes)?;
		Ok(*hdr)
	}
}
