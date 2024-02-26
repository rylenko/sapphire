#![allow(missing_docs)]

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
	let mut state = dr::State::new_alice(
		dr::key::Public::from(&dr::key::Private::random()),
		[1; 32].into(),
		[2; 32].into(),
		[3; 32].into(),
		5,
	);
	let mut plain = [5; 1024];
	let auth = [1; 128];
	let mut encrypted_hdr_buf = dr::encrypted_hdr_buf::create();

	c.bench_function("encryption", |b| {
		b.iter(|| {
			#[allow(clippy::unit_arg)]
			criterion::black_box(
				state
					.encrypt(&mut plain, &auth, &mut encrypted_hdr_buf)
					.unwrap(),
			);
		});
	});
}

criterion::criterion_group!(benches, bench_key_creation, bench_encryption);
criterion::criterion_main!(benches);
