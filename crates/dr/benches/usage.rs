#![allow(missing_docs)]

const BUF: [u8; 4096] = [111; 4096];
const ASSOC_DATA: &[u8] = &[222; 256];

pub fn bench_key_creation(c: &mut criterion::Criterion) {
	c.bench_function("key creation", |b| {
		b.iter(|| {
			#[allow(clippy::unit_arg)]
			criterion::black_box(dr::key::Private::random());
		});
	});
}

#[allow(clippy::missing_panics_doc)]
pub fn bench_encryption(c: &mut criterion::Criterion) {
	// Prepare data
	let mut state = dr::State::new_alice(
		dr::key::Public::from(&dr::key::Private::random()),
		[1; 32].into(),
		[2; 32].into(),
		[3; 32].into(),
		5,
	);
	let mut buf = BUF;

	c.bench_function("encryption", |b| {
		b.iter(|| {
			#[allow(clippy::unit_arg)]
			criterion::black_box(state.encrypt(&mut buf, ASSOC_DATA).unwrap());
		});
	});
}

criterion::criterion_group!(benches, bench_key_creation, bench_encryption);
criterion::criterion_main!(benches);
