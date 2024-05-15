/// `auth`enticates encrypted `buf`fer and `assoc`iated data and decrypts
/// `buf`fer using `key`.
///
/// Note that encryption and authentication occurs using keys derived from
/// `key`.
///
/// # Errors
///
/// [`Auth`] when passed `auth`entication code is not equal to real one.
pub fn decrypt(
	key: &[u8],
	buf: &mut [u8],
	assoc: &[&[u8]],
	auth: &[u8],
) -> Result<(), super::error::Decrypt> {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Derive new encryption key, authentication key and nonce.
	let mut deriver = super::kdf::Deriver::new();
	deriver.derive(key);

	// Authenticate using derived authentication key
	if super::mac::auth(deriver.auth_key(), buf, assoc) != auth {
		return Err(super::error::Decrypt::Auth);
	}

	// Decrypt buffer using derived encryption key and nonce.
	chacha20::XChaCha20::new(
		deriver.encrypt_key().into(),
		deriver.nonce().into(),
	)
	.apply_keystream(buf);
	Ok(())
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_decrypt() -> Result<(), super::super::error::Decrypt> {
		let mut buf = [
			180, 14, 21, 155, 162, 31, 103, 119, 216, 196, 122, 119, 232, 216,
			160, 103, 174, 171, 163, 194, 128, 109, 220, 19, 223, 51, 23, 22,
			58, 207, 144, 110,
		];
		let auth = [
			22, 192, 247, 139, 160, 169, 189, 126, 66, 185, 63, 25, 47, 100,
			200, 37, 98, 244, 240, 10, 147, 218, 226, 20, 212, 144, 51, 233,
			209, 241, 203, 230,
		];
		super::decrypt(b"key", &mut buf, &[b"assoc1", b"assoc2"], &auth)?;
		assert_eq!(buf, [10; 32]);
		Ok(())
	}
}
