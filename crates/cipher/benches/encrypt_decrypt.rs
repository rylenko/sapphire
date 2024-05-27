#![allow(missing_docs)]

fn bench_encrypt_decrypt(c: &mut criterion::Criterion) {
	let mut buf = [123; 4096];
	let mut cipher = cipher::Cipher::new(&[1; 128]);

	c.bench_function("encrypt_decrypt", |b| {
		b.iter(|| {
			let tag = cipher.encrypt(&mut buf, &[&[0; 128], &[100; 512]]);
			cipher.decrypt(&mut buf, &[&[0; 128], &[100; 512]], tag)
		});
	});
}

criterion::criterion_group!(benches, bench_encrypt_decrypt);
criterion::criterion_main!(benches);
