use criterion::{criterion_group, criterion_main, Criterion};

use rbx_dom_weak::{RbxInstanceProperties, RbxTree};

pub fn de_folders_100(c: &mut Criterion) {
    static BUFFER: &[u8] = include_bytes!("../bench-files/folders-100.rbxm");

    c.bench_function("Deserialize 100 Folders", |b| {
        b.iter(|| {
            deserialize_bench(BUFFER);
        });
    });
}

pub fn de_deep_folders_100(c: &mut Criterion) {
    static BUFFER: &[u8] = include_bytes!("../bench-files/deep-folders-100.rbxm");

    c.bench_function("Deserialize 100 deeply nested Folders", |b| {
        b.iter(|| {
            deserialize_bench(BUFFER);
        });
    });
}

pub fn de_modulescripts_100_lines_100(c: &mut Criterion) {
    static BUFFER: &[u8] = include_bytes!("../bench-files/modulescripts-100-lines-100.rbxm");

    c.bench_function("Deserialize 100 100-line ModuleScripts", |b| {
        b.iter(|| {
            deserialize_bench(BUFFER);
        });
    });
}

#[inline(always)]
fn deserialize_bench(buffer: &[u8]) {
    let mut tree = RbxTree::new(RbxInstanceProperties {
        name: String::new(),
        class_name: String::new(),
        properties: Default::default(),
    });
    let root_id = tree.get_root_id();

    rbx_binary::decode(&mut tree, root_id, buffer).unwrap();
}

criterion_group!(
    deserializer,
    de_folders_100,
    de_deep_folders_100,
    de_modulescripts_100_lines_100
);
criterion_main!(deserializer);
