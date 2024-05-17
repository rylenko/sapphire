#![allow(missing_docs)]

fn bench_encrypt_decrypt(c: &mut criterion::Criterion) {
	let mut buf = [123; 4096];

	c.bench_function("encrypt", |b| {
		b.iter(|| {
			let auth = cipher::encrypt(&[1; 128], &mut buf, &[
				&[0; 128],
				&[100; 512],
			]);
			cipher::decrypt(
				&[1; 128],
				&mut buf,
				&[&[0; 128], &[100; 512]],
				&auth,
			)
		});
	});
}

criterion::criterion_group!(benches, bench_encrypt_decrypt);
criterion::criterion_main!(benches);
