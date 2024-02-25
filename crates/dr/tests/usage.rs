mod utils;

#[test]
fn test_decrypt_and_encrypt() {
	const ALICE_PLAIN: &[u8] = b"alice's plain";
	const ALICE_AUTH: &[u8] = b"alice's auth";
	const BOB_PLAIN: &[u8] = b"bob's plain";
	const BOB_AUTH: &[u8] = b"bob's auth";

	let (mut alice, mut bob) = utils::init();

	// Because Bob does not know about Alice
	assert!(bob.encrypt(BOB_PLAIN, BOB_AUTH).is_err());

	for _ in 0..20 {
		// Encrypt from Alice's side
		let (eh_1, c_1) = alice.encrypt(ALICE_PLAIN, ALICE_AUTH).unwrap();
		assert!(bob
			.decrypt(b"invalid header", &c_1, b"another auth")
			.is_err());
		assert!(bob.decrypt(b"invalid header", &c_1, ALICE_AUTH).is_err());
		assert!(bob
			.decrypt(&eh_1, b"another cipher", b"another auth")
			.is_err());
		assert!(bob.decrypt(&eh_1, b"another cipher", ALICE_AUTH).is_err());
		assert!(bob.decrypt(&eh_1, &c_1, b"another auth").is_err());
		assert_eq!(bob.decrypt(&eh_1, &c_1, ALICE_AUTH).unwrap(), ALICE_PLAIN);

		// Encrypt from Bob's side
		let (eh_2, c_2) = bob.encrypt(BOB_PLAIN, BOB_AUTH).unwrap();
		assert!(alice
			.decrypt(b"invalid header", &c_2, b"another auth")
			.is_err());
		assert!(alice.decrypt(b"invalid header", &c_2, BOB_AUTH).is_err());
		assert!(alice
			.decrypt(&eh_2, b"another cipher", b"another auth")
			.is_err());
		assert!(alice.decrypt(&eh_2, b"another cipher", BOB_AUTH).is_err());
		assert!(alice.decrypt(&eh_2, &c_2, b"another auth").is_err());
		assert_eq!(alice.decrypt(&eh_2, &c_2, BOB_AUTH).unwrap(), BOB_PLAIN);
	}
}

#[test]
fn test_double_sending() {}
