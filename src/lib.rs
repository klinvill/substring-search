pub mod helpers;
pub mod implementations;
mod hashers;

/// Given two strings, returns the first found common substring of length k or None if no such
/// substring exists.
///
/// This function uses a hashmap (as per the assignment guidelines).
pub fn substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    // The `_naive_prereserve_iter_fx_shorter_substring` function showed the best performance on
    // smaller length substrings. For that reason we rely on it as default implementation.
    implementations::_naive_prereserve_iter_fx_shorter_substring(s1, s2, k)
}

/// Given two strings, returns the a common substring of length k or None if no such substring
/// exists. If there are multiple common substrings, this function does not guarantee which will be
/// returned.
///
/// This function runs much faster than `substring()` if a common substring is early on in both
/// strings, but otherwise tends to run slower.
///
/// This function uses a hashmap (as per the assignment guidelines).
pub fn unordered_substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    implementations::_alternate_prereserve_iter_fx_substring(s1, s2, k)
}

/// Given two strings, returns if there is a common substring of length k.
pub fn has_substring(s1: &str, s2: &str, k: usize) -> bool {
    substring(s1, s2, k).is_some()
}


#[cfg(test)]
mod tests {
    use std::ops::Range;
    use substring::Substring;
    use proptest::prelude::*;
    use crate::{substring, unordered_substring};

    // Current implementation uses the shortest first (to insert into the hash table)
    const SHORTEST_FIRST: bool = true;

    // Reference implementation for substring to compare against for correctness
    fn substring_reference_impl<'a>(s1: &'a str, s2: &'a str, k: usize, shortest_first: bool) -> Option<&'a str> {
        // Trivial to have matching substrings of length 0
        if k == 0 {
            return Some("");
        }

        // If we use the variants that hash the shortest string into the table, we need to make sure
        // our reference implementation does the same since it can affect the order in which
        // substrings are returned.
        let (_s1, _s2) = if shortest_first {
            if s1.chars().count() <= s2.chars().count() { (s1, s2) }
            else { (s2, s1) }
        } else {
            (s1, s2)
        };

        // Since strings are essentially lists of UTF-8 bytes in Rust, we instead want to iterate over
        // the UTF-8 characters (unicode scalar values).
        let s2_n_chars = _s2.chars().count();

        // Impossible to have a substring longer than the original strings.
        if s2_n_chars < k {
            return None;
        }

        for i in 0..(s2_n_chars-k+1) {
            let sub = _s2.substring(i,i+k);
            // In this reference implementation we return the first substring in s2 which appears in s1.
            if _s1.contains(sub) {
                return Some(sub)
            }
        }

        return None
    }

    // Checks to see if the provided string `sub` is of length `k` and is a substring of `s1` and
    // `s2`
    fn unordered_substring_correct(sub: &str, s1: &str, s2: &str, k: usize) -> bool {
        sub.chars().count() == k && s1.contains(sub) && s2.contains(sub)
    }

    // Proptest strategy to generate a random strings with length within range
    fn string_in_range(range: Range<usize>) -> impl Strategy<Value=String> {
        range.prop_flat_map(|l| {
            prop::string::string_regex(&format!(".{{{}}}", l)).unwrap()
        })
    }

    // Proptest strategy to generate strings where at least one is shorter than k. k must be greater
    // than 0 since it is impossible to generate a string with negative length
    fn strings_one_shorter_than_k<'a>(k_range: Option<Range<usize>>, string_range: Option<Range<usize>>) -> impl Strategy<Value=(String, String, usize)> {
        // Defaults
        let _k_range = match k_range {
            // k must be greater than 0
            None => 1..usize::MAX,
            Some(range) => {
                assert!(range.start > 0, "k must be greater than 0");
                range
            },
        };
        let _string_range = match string_range {
            None => usize::MIN..usize::MAX,
            Some(range) => range,
        };

        _k_range.prop_flat_map(move |k| {
            // Chooses a random test string with length less than k
            let short_string = string_in_range(0..k);
            // Chooses a random test string that could be longer than k
            let other_string = string_in_range(_string_range.clone());
            (any::<bool>(), short_string, other_string).prop_map(move |(is_left, short_string, other_string)| {
                // Randomly choose whether the shorter string is string 1 or string 2
                if is_left {
                    (short_string, other_string, k)
                } else {
                    (other_string, short_string, k)
                }
            })
        })
    }

    #[test]
    fn test_substring() {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        let k = 5;
        // We expect only the first substring will be returned.
        let expected_substring = Some(" test");

        let r = substring(s1, s2, k);
        assert_eq!(r, expected_substring);
    }

    #[test]
    fn test_no_substring() {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Who lives in a pineapple under the sea? - Patchy";
        let k = 5;
        let expected_substring = None;

        let r = substring(s1, s2, k);
        assert_eq!(r, expected_substring);
    }

    #[test]
    fn test_no_string() {
        let s1 = "";
        let s2 = "Who lives in a pineapple under the sea? - Patchy";
        let k = 5;
        let expected_substring = None;

        let r = substring(s1, s2, k);
        assert_eq!(r, expected_substring);
    }

    #[test]
    fn test_no_k() {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        let k = 0;
        let expected_substring = Some("");

        let r = substring(s1, s2, k);
        assert_eq!(r, expected_substring);
    }

    #[test]
    fn test_string_of_length_k() {
        let s1 = "Test";
        let s2 = "Test";
        let s3 = "Uhoh";
        let k = 4;

        assert_eq!(substring(s1, s2, k), Some(s1));
        assert_eq!(substring(s1, s3, k), None);
    }

    #[test]
    fn test_case_sensitive() {
        let s1 = "Test";
        let s2 = "test";
        let k = 4;
        let expected_substring = None;

        let r = substring(s1, s2, k);
        assert_eq!(r, expected_substring);
    }

    // These tests use proptest to do some light fuzzing
    proptest! {
        #[test]
        // Tests behavior when at least one string is shorter than k
        fn test_shorter_strings((s1, s2, k) in strings_one_shorter_than_k(Some(1..10), Some(0..20))) {
            let expected_substring = None;

            let r = substring(&s1, &s2, k);
            assert_eq!(r, expected_substring);
        }

        #[test]
        // Tests behavior against a reference implementation
        fn test_against_reference(
            s1 in string_in_range(0..20),
            s2 in string_in_range(0..20),
            k in 1..10usize,
        ) {
            let expected_substring = substring_reference_impl(&s1, &s2, k, SHORTEST_FIRST);
            let r = substring(&s1, &s2, k);
            assert_eq!(r, expected_substring);
        }
    }

    #[test]
    fn test_substring_unordered() {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        let k = 5;

        let r = unordered_substring(s1, s2, k);
        assert_eq!(r.is_some(), true);
        assert_eq!(unordered_substring_correct(r.unwrap(), s1, s2, k), true);
    }

    #[test]
    fn test_no_substring_unordered() {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Who lives in a pineapple under the sea? - Patchy";
        let k = 5;
        let expected_substring = None;

        let r = unordered_substring(s1, s2, k);
        assert_eq!(r, expected_substring);
    }

    #[test]
    fn test_no_string_unordered() {
        let s1 = "";
        let s2 = "Who lives in a pineapple under the sea? - Patchy";
        let k = 5;
        let expected_substring = None;

        let r = unordered_substring(s1, s2, k);
        assert_eq!(r, expected_substring);
    }

    #[test]
    fn test_no_k_unordered() {
        let s1 = "This is a test string. - Normal Person";
        let s2 = "Here be another test string. Yaargh. - Pirate";
        let k = 0;
        let expected_substring = Some("");

        let r = unordered_substring(s1, s2, k);
        assert_eq!(r, expected_substring);
    }

    #[test]
    fn test_string_of_length_k_unordered() {
        let s1 = "Test";
        let s2 = "Test";
        let s3 = "Uhoh";
        let k = 4;

        assert_eq!(unordered_substring(s1, s2, k), Some(s1));
        assert_eq!(unordered_substring(s1, s3, k), None);
    }

    #[test]
    fn test_case_sensitive_unordered() {
        let s1 = "Test";
        let s2 = "test";
        let k = 4;
        let expected_substring = None;

        let r = unordered_substring(s1, s2, k);
        assert_eq!(r, expected_substring);
    }

    // These tests use proptest to do some light fuzzing
    proptest! {
        #[test]
        // Tests behavior when at least one string is shorter than k
        fn test_shorter_strings_unordered((s1, s2, k) in strings_one_shorter_than_k(Some(1..10), Some(0..20))) {
            let expected_substring = None;

            let r = unordered_substring(&s1, &s2, k);
            assert_eq!(r, expected_substring);
        }

        #[test]
        // Tests behavior against correctness oracle
        fn test_against_oracle_unordered(
            s1 in string_in_range(0..20),
            s2 in string_in_range(0..20),
            k in 1..10usize,
        ) {
            let r = substring(&s1, &s2, k);
            match r {
                None => assert_eq!(r, substring_reference_impl(&s1, &s2, k, SHORTEST_FIRST)),
                Some(sub) => assert_eq!(unordered_substring_correct(sub, &s1, &s2, k), true),
            };
        }
    }
}
