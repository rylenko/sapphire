mod decrypt;
mod encrypt;
pub(crate) mod error;
mod kdf_out;
mod mac;
mod tag;

pub(crate) use {decrypt::decrypt, encrypt::encrypt, tag::Tag};

#[cfg(test)]
mod tests {
	const KEY: &[u8] = b"key to encrypt plain text";
	const BUF: [u8; 10] = *b"1234567890";
	const ASSOC_DATA: &[&[u8]] = &[b"encrypted-header", b"user-auth-data"];

	#[test]
	fn test_decrypt_and_encrypt() {
		// Clone and encrypt the buffer
		let mut buf = BUF;
		let tag = super::encrypt(KEY, &mut buf, ASSOC_DATA);

		assert_ne!(buf, BUF);
		assert!(
			super::decrypt(b"another key", &mut buf, ASSOC_DATA, tag).is_err()
		);
		assert_ne!(buf, BUF);
		assert!(super::decrypt(KEY, &mut buf, &[b"invalid-assoc-data"], tag)
			.is_err());
		assert_ne!(buf, BUF);
		assert!(super::decrypt(
			KEY,
			&mut buf,
			ASSOC_DATA,
			super::Tag::from([0; 32])
		)
		.is_err());
		assert_ne!(buf, BUF);
		super::decrypt(KEY, &mut buf, ASSOC_DATA, tag).unwrap();
		assert_eq!(buf, BUF);
	}
}
