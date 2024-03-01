mod utils;

const ITERS: usize = 20;

const BUF: [u8; 10] = *b"0987654321";
const ASSOC_DATA: &[u8] = b"adadadadadadadadadadadadad";

#[test]
fn test_simple_decrypt_and_encrypt() {
	use zerocopy::AsBytes as _;

	let (mut alice, mut bob) = utils::init();

	// Because Bob does not know about Alice
	let mut buf = BUF;
	assert!(bob.encrypt(&mut buf, ASSOC_DATA).is_err());

	for _ in 0..ITERS {
		let mut bad_buf = [0; 100];

		// Encrypt from Alice's side
		let mut buf = BUF;
		let mut clue = alice.encrypt(&mut buf, ASSOC_DATA).unwrap();
		// Test errors
		assert!(bob.decrypt(&mut buf, b"bad auth", &clue).is_err());
		assert!(bob.decrypt(&mut bad_buf, ASSOC_DATA, &clue).is_err());
		clue.as_bytes_mut()[50] ^= u8::MAX;
		assert!(bob.decrypt(&mut buf, ASSOC_DATA, &clue).is_err());
		clue.as_bytes_mut()[50] ^= u8::MAX;
		// Test success
		assert_ne!(buf, BUF);
		bob.decrypt(&mut buf, ASSOC_DATA, &clue).unwrap();
		assert_eq!(buf, BUF);

		// Encrypt from Bob's side
		let mut clue = bob.encrypt(&mut buf, ASSOC_DATA).unwrap();
		// Test errors
		assert!(alice.decrypt(&mut buf, b"bad auth", &clue).is_err());
		assert!(alice.decrypt(&mut bad_buf, ASSOC_DATA, &clue).is_err());
		// Clue replacement
		clue.as_bytes_mut()[50] ^= u8::MAX;
		assert!(alice.decrypt(&mut buf, ASSOC_DATA, &clue).is_err());
		clue.as_bytes_mut()[50] ^= u8::MAX;
		// Test success
		assert_ne!(buf, BUF);
		alice.decrypt(&mut buf, ASSOC_DATA, &clue).unwrap();
		assert_eq!(buf, BUF);
	}
}

#[test]
fn test_double_send() {
	let (mut alice, mut bob) = utils::init();

	for _ in 0..ITERS {
		// Encrypt from Alice's side
		let mut buf = BUF;
		let clue = alice.encrypt(&mut buf, ASSOC_DATA).unwrap();
		{
			let mut buf_copy = buf;
			bob.decrypt(&mut buf_copy, ASSOC_DATA, &clue).unwrap();
		}
		assert!(bob.decrypt(&mut buf, ASSOC_DATA, &clue).is_err());

		// Encrypt from Bob's side
		let mut buf = BUF;
		let clue = bob.encrypt(&mut buf, ASSOC_DATA).unwrap();
		{
			let mut buf_copy = buf;
			alice.decrypt(&mut buf_copy, ASSOC_DATA, &clue).unwrap();
		}
		assert!(alice.decrypt(&mut buf, ASSOC_DATA, &clue).is_err());
	}
}

#[test]
fn test_large_skip() {
	let (mut alice, mut bob) = utils::init();

	// First encryption
	let mut buf_1 = BUF;
	let clue_1 = alice.encrypt(&mut buf_1, ASSOC_DATA).unwrap();
	// Skip
	{
		let mut skip_buf = BUF;
		for _ in 0..=utils::MAX_SKIPPED_MSG_KEYS_CNT {
			alice.encrypt(&mut skip_buf, ASSOC_DATA).unwrap();
		}
	}
	// Encrypt after skip
	let mut buf_2 = BUF;
	let clue_2 = alice.encrypt(&mut buf_2, ASSOC_DATA).unwrap();

	// From current chain
	bob.decrypt(&mut buf_1, ASSOC_DATA, &clue_1).unwrap();
	assert!(bob.decrypt(&mut buf_2, ASSOC_DATA, &clue_2).is_err());
	assert_eq!(buf_1, BUF);

	// Try to encrypt from Bob's side
	let mut buf_3 = BUF;
	let clue_3 = bob.encrypt(&mut buf_3, ASSOC_DATA).unwrap();
	// Try to decrypt Bob's buffer
	alice.decrypt(&mut buf_3, ASSOC_DATA, &clue_3).unwrap();

	// Encrypt after Bob's message
	let mut buf_4 = BUF;
	let clue_4 = alice.encrypt(&mut buf_4, ASSOC_DATA).unwrap();
	// Failed to skip previous chain keys
	assert!(bob.decrypt(&mut buf_4, ASSOC_DATA, &clue_4).is_err());
	assert!(bob.decrypt(&mut buf_2, ASSOC_DATA, &clue_2).is_err());
}
