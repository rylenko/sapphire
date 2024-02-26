mod utils;

const ITERS: usize = 20;

const ALICE_BUFF: [u8; 52] =
	*b"09876543211234567890---------------MAC--------------";
const ALICE_AUTH: &[u8] = b"alice's auth";
const BOB_BUFF: [u8; 52] =
	*b"12345678900987654321---------------MAC--------------";
const BOB_AUTH: &[u8] = b"bob's auth";
const CIPHER_LEN: usize = ALICE_BUFF.len() - 32;

#[test]
fn test_decrypt_and_encrypt() {
	let (mut alice, mut bob) = utils::init();

	// Because Bob does not know about Alice
	let mut bob_buff = BOB_BUFF;
	let mut bob_encrypted_header_buff = dr::create_encrypted_header_buff();
	assert!(bob
		.encrypt(&mut bob_buff, BOB_AUTH, &mut bob_encrypted_header_buff)
		.is_err());

	for _ in 0..ITERS {
		// Create invalid buffers
		let mut invalid_buff =
			*b"invalid buff-------------------------------------";
		let mut invalid_header = *b"invalid header";

		// Encrypt from Alice's side
		let mut alice_buff = ALICE_BUFF;
		let mut alice_encrypted_header_buff =
			dr::create_encrypted_header_buff();
		alice
			.encrypt(
				&mut alice_buff,
				ALICE_AUTH,
				&mut alice_encrypted_header_buff,
			)
			.unwrap();
		assert!(bob
			.decrypt(&mut alice_buff, b"another auth", &mut invalid_header)
			.is_err());
		assert!(bob
			.decrypt(&mut alice_buff, ALICE_AUTH, &mut invalid_header)
			.is_err());
		assert!(bob
			.decrypt(
				&mut invalid_buff,
				b"another auth",
				&mut alice_encrypted_header_buff
			)
			.is_err());
		assert!(bob
			.decrypt(
				&mut invalid_buff,
				ALICE_AUTH,
				&mut alice_encrypted_header_buff
			)
			.is_err());
		assert!(bob
			.decrypt(
				&mut alice_buff,
				b"another auth",
				&mut alice_encrypted_header_buff
			)
			.is_err());
		assert!(bob
			.decrypt(
				&mut alice_buff,
				ALICE_AUTH,
				&mut alice_encrypted_header_buff
			)
			.is_ok());
		assert_eq!(alice_buff[..CIPHER_LEN], ALICE_BUFF[..CIPHER_LEN]);

		// Encrypt from Bob's side
		let mut bob_buff = BOB_BUFF;
		let mut bob_encrypted_header_buff = dr::create_encrypted_header_buff();
		bob.encrypt(&mut bob_buff, BOB_AUTH, &mut bob_encrypted_header_buff)
			.unwrap();
		assert!(alice
			.decrypt(&mut alice_buff, b"another auth", &mut invalid_header)
			.is_err());
		assert!(alice
			.decrypt(&mut alice_buff, BOB_AUTH, &mut invalid_header)
			.is_err());
		assert!(alice
			.decrypt(
				&mut invalid_buff,
				b"another auth",
				&mut bob_encrypted_header_buff
			)
			.is_err());
		assert!(alice
			.decrypt(
				&mut invalid_buff,
				BOB_AUTH,
				&mut bob_encrypted_header_buff
			)
			.is_err());
		assert!(alice
			.decrypt(
				&mut bob_buff,
				b"another auth",
				&mut bob_encrypted_header_buff
			)
			.is_err());
		assert!(alice
			.decrypt(&mut bob_buff, BOB_AUTH, &mut bob_encrypted_header_buff)
			.is_ok());
		assert_eq!(bob_buff[..CIPHER_LEN], BOB_BUFF[..CIPHER_LEN]);
	}
}

#[test]
fn test_double_sending() {
	let (mut alice, mut bob) = utils::init();

	for _ in 0..ITERS {
		// Encrypt from Alice's side
		let mut alice_buff = ALICE_BUFF;
		let mut alice_encrypted_header_buff =
			dr::create_encrypted_header_buff();
		alice
			.encrypt(
				&mut alice_buff,
				ALICE_AUTH,
				&mut alice_encrypted_header_buff,
			)
			.unwrap();
		assert!(bob
			.decrypt(
				&mut alice_buff,
				ALICE_AUTH,
				&mut alice_encrypted_header_buff
			)
			.is_ok());
		assert!(bob
			.decrypt(
				&mut alice_buff,
				ALICE_AUTH,
				&mut alice_encrypted_header_buff
			)
			.is_err());

		// Encrypt from Bob's side
		let mut bob_buff = BOB_BUFF;
		let mut bob_encrypted_header_buff = dr::create_encrypted_header_buff();
		bob.encrypt(&mut bob_buff, BOB_AUTH, &mut bob_encrypted_header_buff)
			.unwrap();
		assert!(alice
			.decrypt(&mut bob_buff, BOB_AUTH, &mut bob_encrypted_header_buff)
			.is_ok());
		assert!(alice
			.decrypt(&mut bob_buff, BOB_AUTH, &mut bob_encrypted_header_buff)
			.is_err());
	}
}
