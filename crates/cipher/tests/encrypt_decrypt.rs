#[test]
fn test_encrypt_decrypt() -> Result<(), cipher::decrypt::Error> {
	// Encrypt
	let mut buf = [111; 111];
	let tag = cipher::encrypt(b"secret-key", &mut buf, &[b"a1", b"a2"]);
	assert_ne!(buf, [111; 111]);

	// Decrypt with invalid associated data
	assert!(cipher::decrypt(b"secret-key", &mut buf, &[b"a1"], tag).is_err());

	// Decrypt with invalid key
	assert!(cipher::decrypt(b"inval", &mut buf, &[b"a1", b"a2"], tag).is_err());

	// Decrypt with invalid authentication
	let mut inval_auth = tag;
	zerocopy::AsBytes::as_bytes_mut(&mut inval_auth)[0] += 1;
	assert!(cipher::decrypt(
		b"secret-key",
		&mut buf,
		&[b"a1", b"a2"],
		inval_auth
	)
	.is_err());

	// Decrypt with valid key and associated data
	cipher::decrypt(b"secret-key", &mut buf, &[b"a1", b"a2"], tag)?;
	assert_eq!(buf, [111; 111]);
	Ok(())
}
