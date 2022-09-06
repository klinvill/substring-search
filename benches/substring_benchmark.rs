use std::fs::DirEntry;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use substring_search::substring;

pub fn bench_substring(c: &mut Criterion) {
    // Simple test
    c.bench_function("Simple substring", |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        substring(black_box(s1), black_box(s2), black_box(5))
    }));

    // The test files are stored in the path format: data/<category>/<file>
    let test_categories = std::fs::read_dir("./data").unwrap()
        .filter(|d| {
            // Filter out hidden directories and files
            !d.as_ref().unwrap().file_name().to_str().unwrap().starts_with(".")
        });
    test_categories.for_each(|cat| {
        let test_files: Vec<DirEntry> = std::fs::read_dir(cat.unwrap().path()).unwrap()
            .filter(|f| {
                // Filter out hidden directories and files
                !f.as_ref().unwrap().file_name().to_str().unwrap().starts_with(".")
            }).collect::<Result<Vec<_>,_>>().unwrap();
        test_files.iter().tuple_combinations().for_each(|(f1, f2)| {
            let f1_name = f1.file_name();
            let f2_name = f2.file_name();
            let s1 = std::fs::read_to_string(f1.path()).unwrap();
            let s2 = std::fs::read_to_string(f2.path()).unwrap();
            c.bench_function(&format!("{}_{}", f1_name.to_str().unwrap(), f2_name.to_str().unwrap()), |b| b.iter(|| {
                substring(black_box(&s1), black_box(&s2), black_box(20))
            }));
        })
    })

}

criterion_group!(benches, bench_substring);
criterion_main!(benches);
