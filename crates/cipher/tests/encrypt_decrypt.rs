#[test]
fn test_encrypt_and_decrypt() -> Result<(), cipher::error::Decrypt> {
	// Encrypt
	let mut buf = [111; 111];
	let auth = cipher::encrypt(b"secret-key", &mut buf, &[b"a1", b"a2"]);
	assert_ne!(buf, [111; 111]);
	assert_eq!(auth, [
		167, 239, 52, 19, 236, 101, 217, 51, 178, 24, 192, 142, 154, 89, 38,
		149, 54, 106, 214, 192, 121, 33, 205, 110, 90, 208, 132, 116, 85, 105,
		15, 116
	]);

	// Decrypt with invalid associated data
	assert!(cipher::decrypt(b"secret-key", &mut buf, &[b"a1"], &auth).is_err());

	// Decrypt with invalid key
	assert!(
		cipher::decrypt(b"inval", &mut buf, &[b"a1", b"a2"], &auth).is_err()
	);

	// Decrypt with invalid authentication
	let inval_auth = [1; 32];
	assert!(cipher::decrypt(
		b"secret-key",
		&mut buf,
		&[b"a1", b"a2"],
		&inval_auth
	)
	.is_err());

	// Decrypt with valid key and associated data
	cipher::decrypt(b"secret-key", &mut buf, &[b"a1", b"a2"], &auth)?;
	assert_eq!(buf, [111; 111]);
	Ok(())
}
