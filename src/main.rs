use std::collections::{HashMap, HashSet};
use substring_search::has_substring;
use substring_search::helpers::preprocess_string;

use substring_search::implementations::{build_fx_substring, build_rolling_polynomial_substring, build_rolling_adler_substring, build_sip_substring};

/// This function is used to test for frequency of collisions for hash functions. Unit tests are
/// found in the rust files themselves.
fn check_collisions<'b>() {
    let files = ["./data/Tolstoy/war_and_peace_tolstoy.txt", "./data/genomes/monkeypox-genome.txt"];
    for file in files {
        let s = preprocess_string(&std::fs::read_to_string(file).unwrap());
        let k = 20;
        let builder_fns: HashMap<_, _> = HashMap::from([
            ("sip", build_sip_substring(&s, k)),
            ("fx", build_fx_substring(&s, k)),
            ("rolling_adler", build_rolling_adler_substring(&s, k)),
            ("rolling_poly", build_rolling_polynomial_substring(&s, k)),
        ]);

        let mut results = HashMap::new();

        for (name, mut sub_fn) in builder_fns {
            let mut hashes = HashSet::new();
            // let mut sub_fn = f(&s, k);
            for _ in 0..s.len() - k {
                let (_, hash) = sub_fn();
                hashes.insert(hash);
            }
            results.insert(name, hashes.len());
        }

        println!("Unique hashes in {}: {:?}", file, results);
    }
}




fn main() {
    let s1 = std::fs::read_to_string("./data/Tolstoy/war_and_peace_tolstoy.txt").unwrap();
    let s2 = std::fs::read_to_string("./data/Tolstoy/anna_karenina_tolstoy.txt").unwrap();
    // let s1 = std::fs::read_to_string("./data/genomes/bacterial_genome_1.txt").unwrap();
    // let s2 = std::fs::read_to_string("./data/genomes/monkeypox-genome.txt").unwrap();
    println!("Common substring?: {}", has_substring(&s1, &s2, 20));

    // check_collisions()
}
