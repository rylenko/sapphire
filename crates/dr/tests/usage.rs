mod utils;

const ITERS: usize = 20;

const ALICE_BUF: [u8; 42] = *b"0987654321---------------MAC--------------";
const ALICE_AUTH: &[u8] = b"alice's auth";
const BOB_BUF: [u8; 42] = *b"1234567890---------------MAC--------------";
const BOB_AUTH: &[u8] = b"bob's auth";
const CIPHER_LEN: usize = ALICE_BUF.len() - 32;

#[test]
fn test_decrypt_and_encrypt() {
	let (mut alice, mut bob) = utils::init();

	// Because Bob does not know about Alice
	let mut bob_buf = BOB_BUF;
	let mut bob_hdr_buf = dr::encrypted_hdr_buf::create();
	assert!(bob.encrypt(&mut bob_buf, BOB_AUTH, &mut bob_hdr_buf).is_err());

	for _ in 0..ITERS {
		// Create bad buffers
		let mut bad_buf = *b"bad buf-------------------------------------";
		let mut bad_hdr = dr::encrypted_hdr_buf::create();

		// Encrypt from Alice's side
		let mut alice_buf = ALICE_BUF;
		let mut alice_hdr_buf = dr::encrypted_hdr_buf::create();
		alice.encrypt(&mut alice_buf, ALICE_AUTH, &mut alice_hdr_buf).unwrap();

		// Test errors
		{
			let mut alice_hdr_buf_copy = alice_hdr_buf;
			assert!(bob
				.decrypt(&mut alice_buf, b"bad auth", &mut alice_hdr_buf_copy)
				.is_err());
		}
		{
			let mut alice_hdr_buf_copy = alice_hdr_buf;
			assert!(bob
				.decrypt(&mut bad_buf, b"bad auth", &mut alice_hdr_buf_copy)
				.is_err());
		}
		{
			let mut alice_hdr_buf_copy = alice_hdr_buf;
			assert!(bob
				.decrypt(&mut bad_buf, ALICE_AUTH, &mut alice_hdr_buf_copy)
				.is_err());
		}
		assert!(bob
			.decrypt(&mut alice_buf, b"bad auth", &mut bad_hdr)
			.is_err());
		assert!(bob
			.decrypt(&mut alice_buf, ALICE_AUTH, &mut bad_hdr)
			.is_err());

		// Test success
		assert!(bob
			.decrypt(&mut alice_buf, ALICE_AUTH, &mut alice_hdr_buf)
			.is_ok());
		assert_eq!(alice_buf[..CIPHER_LEN], ALICE_BUF[..CIPHER_LEN]);

		// Encrypt from Bob's side
		let mut bob_buf = BOB_BUF;
		let mut bob_hdr_buf = dr::encrypted_hdr_buf::create();
		bob.encrypt(&mut bob_buf, BOB_AUTH, &mut bob_hdr_buf).unwrap();

		// Test errors
		{
			let mut bob_hdr_buf_copy = bob_hdr_buf;
			assert!(alice
				.decrypt(&mut bad_buf, b"bad auth", &mut bob_hdr_buf_copy)
				.is_err());
		}
		{
			let mut bob_hdr_buf_copy = bob_hdr_buf;
			assert!(alice
				.decrypt(&mut bad_buf, BOB_AUTH, &mut bob_hdr_buf_copy)
				.is_err());
		}
		{
			let mut bob_hdr_buf_copy = bob_hdr_buf;
			assert!(alice
				.decrypt(&mut bob_buf, b"bad auth", &mut bob_hdr_buf_copy)
				.is_err());
		}
		assert!(alice
			.decrypt(&mut bob_buf, b"bad auth", &mut bad_hdr)
			.is_err());
		assert!(alice.decrypt(&mut bob_buf, BOB_AUTH, &mut bad_hdr).is_err());

		// Test successs
		assert!(alice
			.decrypt(&mut bob_buf, BOB_AUTH, &mut bob_hdr_buf)
			.is_ok());
		assert_eq!(bob_buf[..CIPHER_LEN], BOB_BUF[..CIPHER_LEN]);
	}
}

#[test]
fn test_double_sending() {
	let (mut alice, mut bob) = utils::init();

	for _ in 0..ITERS {
		// Encrypt from Alice's side
		let mut alice_buf = ALICE_BUF;
		let mut alice_hdr_buf = dr::encrypted_hdr_buf::create();
		alice.encrypt(&mut alice_buf, ALICE_AUTH, &mut alice_hdr_buf).unwrap();
		assert!(bob
			.decrypt(&mut alice_buf, ALICE_AUTH, &mut alice_hdr_buf)
			.is_ok());
		assert!(bob
			.decrypt(&mut alice_buf, ALICE_AUTH, &mut alice_hdr_buf)
			.is_err());

		// Encrypt from Bob's side
		let mut bob_buf = BOB_BUF;
		let mut bob_hdr_buf = dr::encrypted_hdr_buf::create();
		bob.encrypt(&mut bob_buf, BOB_AUTH, &mut bob_hdr_buf).unwrap();
		assert!(alice
			.decrypt(&mut bob_buf, BOB_AUTH, &mut bob_hdr_buf)
			.is_ok());
		assert!(alice
			.decrypt(&mut bob_buf, BOB_AUTH, &mut bob_hdr_buf)
			.is_err());
	}
}
