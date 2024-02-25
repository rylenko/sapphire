#[must_use]
pub fn init() -> (dr::State, dr::State) {
	const MAX_CNT: u32 = 5;
	const ALICE_RECV_HEADER_KEY: [u8; 32] = [3; 32];
	const ROOT_KEY: [u8; 32] = [1; 32];
	const ALICE_SEND_HEADER_KEY: [u8; 32] = [2; 32];

	let private_key = dr::key::Private::random();

	// Create Alice with Bob
	let bob = dr::State::new_bob(
		private_key.clone(),
		ROOT_KEY.into(),
		ALICE_RECV_HEADER_KEY.into(),
		ALICE_SEND_HEADER_KEY.into(),
		MAX_CNT,
	);
	let alice = dr::State::new_alice(
		dr::key::Public::from(&private_key),
		ROOT_KEY.into(),
		ALICE_SEND_HEADER_KEY.into(),
		ALICE_RECV_HEADER_KEY.into(),
		MAX_CNT,
	);
	(alice, bob)
}
