use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::PathBuf;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use itertools::Itertools;
use substring_search::substring;
use substring_search::implementations::{_naive_substring, _naive_prereserve_substring, _naive_prereserve_iter_substring, _naive_prereserve_iter_fx_substring, _naive_prereserve_iter_fx_shorter_substring, _alternate_prereserve_iter_fx_substring, _naive_prereserve_iter_rolling_adler_shorter_substring, build_fx_substring, build_rolling_substring, build_sip_substring, build_rolling_polynomial_substring, _naive_prereserve_iter_rolling_poly_shorter_substring};
use substring_search::helpers::preprocess_string;

#[derive(Clone)]
struct File {
    name: String,
    path: PathBuf,
}

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
            // Note: we only preprocess the strings in the benchmarks (rather than in the substring
            // functions themselves) to preserve generality.
            let s1 = preprocess_string(&std::fs::read_to_string(f1.path()).unwrap());
            let s2 = preprocess_string(&std::fs::read_to_string(f2.path()).unwrap());
            c.bench_function(&format!("{}_{}", f1_name.to_str().unwrap(), f2_name.to_str().unwrap()), |b| b.iter(|| {
                substring(black_box(&s1), black_box(&s2), black_box(20))
            }));
        })
    })
}

pub fn bench_substring_impls<'a>(c: &mut Criterion) {
    let mut group = c.benchmark_group("Substring");

    // TODO(klinvill): I'm running into trouble making this function range over the functions. Worth
    //  later investigation.
    // let functions: HashMap<_, _> = HashMap::from([
    //     ("naive_substring", _naive_substring as fn(&'a str, &'a str, usize) -> Option<&'a str>),
    //     ("naive_prereserve_substring", _naive_prereserve_substring as fn(&'a str, &'a str, usize) -> Option<&'a str>),
    // ]);

    // The test files are stored in the path format: data/<category>/<file>
    let test_categories = std::fs::read_dir("./data").unwrap()
        .filter(|d| {
            // Filter out hidden directories and files
            !d.as_ref().unwrap().file_name().to_str().unwrap().starts_with(".")
        });
    let test_file_pairs: Vec<(File, File)> = test_categories.flat_map(|cat| {
        let test_files: Vec<_> = std::fs::read_dir(cat.unwrap().path()).unwrap()
            .filter(|f| {
                // Filter out hidden directories and files
                !f.as_ref().unwrap().file_name().to_str().unwrap().starts_with(".")
            }).map(|f| {
                let _f = f.unwrap();
                File {
                    name: _f.file_name().into_string().unwrap(),
                    path: _f.path(),
                }
            }).collect();
        test_files.iter().cloned().tuple_combinations().collect::<Vec<(_,_)>>()
    }).collect();

    // Simple test
    group.bench_function(BenchmarkId::new("naive_substring", "simple"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        _naive_substring(black_box(s1), black_box(s2), black_box(5))
    }));
    group.bench_function(BenchmarkId::new("naive_prereserve_substring", "simple"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        _naive_prereserve_substring(black_box(s1), black_box(s2), black_box(5))
    }));
    group.bench_function(BenchmarkId::new("naive_prereserve_iter_substring", "simple"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        _naive_prereserve_iter_substring(black_box(s1), black_box(s2), black_box(5))
    }));
    group.bench_function(BenchmarkId::new("naive_prereserve_iter_fx_substring", "simple"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        _naive_prereserve_iter_fx_substring(black_box(s1), black_box(s2), black_box(5))
    }));
    group.bench_function(BenchmarkId::new("naive_prereserve_iter_fx_shorter_substring", "simple"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        _naive_prereserve_iter_fx_shorter_substring(black_box(s1), black_box(s2), black_box(5))
    }));
    group.bench_function(BenchmarkId::new("naive_prereserve_iter_rolling_adler_shorter_substring", "simple"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        _naive_prereserve_iter_rolling_adler_shorter_substring(black_box(s1), black_box(s2), black_box(5))
    }));
    group.bench_function(BenchmarkId::new("naive_prereserve_iter_rolling_poly_shorter_substring", "simple"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        _naive_prereserve_iter_rolling_poly_shorter_substring(black_box(s1), black_box(s2), black_box(5))
    }));
    group.bench_function(BenchmarkId::new("alternate_prereserve_iter_fx_substring", "simple"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        _alternate_prereserve_iter_fx_substring(black_box(s1), black_box(s2), black_box(5))
    }));

    for (f1, f2) in test_file_pairs {
        // Note: we only preprocess the strings in the benchmarks (rather than in the substring
        // functions themselves) to preserve generality.
        let s1 = preprocess_string(&std::fs::read_to_string(f1.path.clone()).unwrap());
        let s2 = preprocess_string(&std::fs::read_to_string(f2.path.clone()).unwrap());

        // Note: 320 characters should be enough to make sure the project gutenberg headers don't trivially match
        for k in [5, 10, 20, 40, 80, 160, 320] {
            group.bench_with_input(
                BenchmarkId::new("naive_substring", &format!("{}_{}_{}", f1.name, f2.name, k)),
                &(&s1, &s2),
                |b, (s_1, s_2)| b.iter(|| {
                    _naive_substring(black_box(s_1), black_box(s_2), black_box(k))
                })
            );
            group.bench_with_input(
                BenchmarkId::new("naive_prereserve_substring", &format!("{}_{}_{}", f1.name, f2.name, k)),
                &(&s1, &s2),
                |b, (s_1, s_2)| b.iter(|| {
                    _naive_prereserve_substring(black_box(s_1), black_box(s_2), black_box(k))
                })
            );
            group.bench_with_input(
                BenchmarkId::new("naive_prereserve_iter_substring", &format!("{}_{}_{}", f1.name, f2.name, k)),
                &(&s1, &s2),
                |b, (s_1, s_2)| b.iter(|| {
                    _naive_prereserve_iter_substring(black_box(s_1), black_box(s_2), black_box(k))
                })
            );
            group.bench_with_input(
                BenchmarkId::new("naive_prereserve_iter_fx_substring", &format!("{}_{}_{}", f1.name, f2.name, k)),
                &(&s1, &s2),
                |b, (s_1, s_2)| b.iter(|| {
                    _naive_prereserve_iter_fx_substring(black_box(s_1), black_box(s_2), black_box(k))
                })
            );
            group.bench_with_input(
                BenchmarkId::new("naive_prereserve_iter_fx_shorter_substring", &format!("{}_{}_{}", f1.name, f2.name, k)),
                &(&s1, &s2),
                |b, (s_1, s_2)| b.iter(|| {
                    _naive_prereserve_iter_fx_shorter_substring(black_box(s_1), black_box(s_2), black_box(k))
                })
            );
            group.bench_with_input(
                BenchmarkId::new("naive_prereserve_iter_rolling_adler_shorter_substring", &format!("{}_{}_{}", f1.name, f2.name, k)),
                &(&s1, &s2),
                |b, (s_1, s_2)| b.iter(|| {
                    _naive_prereserve_iter_rolling_adler_shorter_substring(black_box(s_1), black_box(s_2), black_box(k))
                })
            );
            group.bench_with_input(
                BenchmarkId::new("naive_prereserve_iter_rolling_poly_shorter_substring", &format!("{}_{}_{}", f1.name, f2.name, k)),
                &(&s1, &s2),
                |b, (s_1, s_2)| b.iter(|| {
                    _naive_prereserve_iter_rolling_poly_shorter_substring(black_box(s_1), black_box(s_2), black_box(k))
                })
            );
            group.bench_with_input(
                BenchmarkId::new("alternate_prereserve_iter_fx_substring", &format!("{}_{}_{}", f1.name, f2.name, k)),
                &(&s1, &s2),
                |b, (s_1, s_2)| b.iter(|| {
                    _alternate_prereserve_iter_fx_substring(black_box(s_1), black_box(s_2), black_box(k))
                })
            );
        }
    }
}

pub fn bench_hashes<'a>(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hashes");

    // The test files are stored in the path format: data/<category>/<file>
    let test_categories = std::fs::read_dir("./data").unwrap()
        .filter(|d| {
            // Filter out hidden directories and files
            !d.as_ref().unwrap().file_name().to_str().unwrap().starts_with(".")
        });
    let test_files: Vec<_> = test_categories.flat_map(|cat| {
        std::fs::read_dir(cat.unwrap().path()).unwrap()
    }).filter(|f| {
        // Filter out hidden directories and files
        !f.as_ref().unwrap().file_name().to_str().unwrap().starts_with(".")
    }).map(|f| {
        let _f = f.unwrap();
        File {
            name: _f.file_name().into_string().unwrap(),
            path: _f.path(),
        }
    }).collect();

    // Simple test
    group.bench_function(BenchmarkId::new("sip", "simple_5"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let k = 5;
        let mut sub_fn = build_sip_substring(black_box(s1), black_box(k));
        for _ in 0..s1.len() - k {
            sub_fn();
        }
    }));
    group.bench_function(BenchmarkId::new("fx", "simple_5"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let k = 5;
        let mut sub_fn = build_fx_substring(black_box(s1), black_box(k));
        for _ in 0..s1.len() - k {
            sub_fn();
        }
    }));
    group.bench_function(BenchmarkId::new("rolling_adler", "simple_5"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let k = 5;
        let mut sub_fn = build_rolling_substring(black_box(s1), black_box(k));
        for _ in 0..s1.len() - k {
            sub_fn();
        }
    }));
    group.bench_function(BenchmarkId::new("rolling_poly", "simple_5"), |b| b.iter(|| {
        let s1 = "This is a test string. - Normal Person";
        let k = 5;
        let mut sub_fn = build_rolling_polynomial_substring(black_box(s1), black_box(k));
        for _ in 0..s1.len() - k {
            sub_fn();
        }
    }));

    for f in test_files {
        // Note: we only preprocess the strings in the benchmarks (rather than in the substring
        // functions themselves) to preserve generality.
        let s = preprocess_string(&std::fs::read_to_string(f.path.clone()).unwrap());

        // Note: 320 characters should be enough to make sure the project gutenberg headers don't trivially match
        for k in [5, 10, 20, 40, 80, 160, 320] {
            group.bench_with_input(
                BenchmarkId::new("sip", &format!("{}_{}", f.name, k)),
                &s,
                |b, s| b.iter(|| {
                    let mut sub_fn = build_sip_substring(black_box(s), black_box(k));
                    for _ in 0..s.len() - k {
                        sub_fn();
                    }
                })
            );
            group.bench_with_input(
                BenchmarkId::new("fx", &format!("{}_{}", f.name, k)),
                &s,
                |b, s| b.iter(|| {
                    let mut sub_fn = build_fx_substring(black_box(s), black_box(k));
                    for _ in 0..s.len() - k {
                        sub_fn();
                    }
                })
            );
            group.bench_with_input(
                BenchmarkId::new("rolling_adler", &format!("{}_{}", f.name, k)),
                &s,
                |b, s| b.iter(|| {
                    let mut sub_fn = build_rolling_substring(black_box(s), black_box(k));
                    for _ in 0..s.len() - k {
                        sub_fn();
                    }
                })
            );
            group.bench_with_input(
                BenchmarkId::new("rolling_poly", &format!("{}_{}", f.name, k)),
                &s,
                |b, s| b.iter(|| {
                    let mut sub_fn = build_rolling_polynomial_substring(black_box(s), black_box(k));
                    for _ in 0..s.len() - k {
                        sub_fn();
                    }
                })
            );
        }
    }
}

criterion_group!(benches, bench_substring_impls, bench_hashes);
criterion_main!(benches);
