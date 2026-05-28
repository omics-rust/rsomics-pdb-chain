use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_pdb_chain(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-pdb-chain");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pdb = manifest.join("tests/golden/small.pdb");
    c.bench_function("rsomics-pdb-chain golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args(["list", pdb.to_str().unwrap()])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_pdb_chain);
criterion_main!(benches);
