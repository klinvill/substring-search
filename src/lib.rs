use std::collections::HashSet;

/// Given two strings, returns the first found common substring of length k or None if no such
/// substring exists.
///
/// This function uses a hashmap (as per the assignment guidelines).
pub fn substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    _naive_prereserve_substring(s1, s2, k)
}

/// Given two strings, returns if there is a common substring of length k.
pub fn has_substring(s1: &str, s2: &str, k: usize) -> bool {
    substring(s1, s2, k).is_some()
}

/// Naive implementation of substring search. Sticks all k-length substrings of first in a hashmap,
/// then checks all the k-length substrings in s2 to see if any are already in the hashmap. Runs
/// in ~O(n) time (n-k+1 insertions for substrings in s1 (where n is the length of s1), up to
/// n-k+1 queries for substrings in s2 (where n is the length of s2)).
#[deprecated = "The implementations shouldn't be called directly. Call substring() instead."]
pub fn _naive_substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    // Trivial to have matching substrings of length 0
    if k == 0 {
        // In this case we opt to return the empty string. Another implementation could return None
        // instead.
        return Some("");
    }

    // Since strings are essentially lists of UTF-8 bytes in Rust, we instead want to iterate over
    // the UTF-8 characters (unicode scalar values). We also keep track of the original indices in
    // s1 and s2 so we can more efficiently get substrings (without having to create new strings).
    let (cis1, cs1): (Vec<usize>, Vec<char>) = s1.char_indices().unzip();
    let (cis2, cs2): (Vec<usize>, Vec<char>) = s2.char_indices().unzip();

    // Impossible to have a substring longer than the original strings.
    if cs1.len() < k || cs2.len() < k {
        return None;
    }

    let mut substrings = HashSet::new();
    for i in 0..(cs1.len()-k+1) {
        let start = cis1[i];
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let end = *cis1.get(i+k).unwrap_or(&s1.len());
        let sub = &s1[start..end];
        substrings.insert(sub);
    }

    for i in 0..(cs2.len()-k+1) {
        let start = cis2[i];
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let end = *cis2.get(i+k).unwrap_or(&s2.len());
        let sub = &s2[start..end];
        if substrings.contains(sub) {
            // Substring found in both s1 and s2, can return early.
            return Some(sub);
        }
    }

    // No substring of length k in s2 is also in s1.
    return None;
}


/// Naive implementation of substring search. Sticks all k-length substrings of first in a hashmap,
/// then checks all the k-length substrings in s2 to see if any are already in the hashmap. Runs
/// in ~O(n) time (n-k+1 insertions for substrings in s1 (where n is the length of s1), up to
/// n-k+1 queries for substrings in s2 (where n is the length of s2)). This function pre-reserves
/// the needed size of the hash table up front so rehashing is not needed.
#[deprecated = "The implementations shouldn't be called directly. Call substring() instead."]
pub fn _naive_prereserve_substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    // Trivial to have matching substrings of length 0
    if k == 0 {
        // In this case we opt to return the empty string. Another implementation could return None
        // instead.
        return Some("");
    }

    // Since strings are essentially lists of UTF-8 bytes in Rust, we instead want to iterate over
    // the UTF-8 characters (unicode scalar values). We also keep track of the original indices in
    // s1 and s2 so we can more efficiently get substrings (without having to create new strings).
    let (cis1, cs1): (Vec<usize>, Vec<char>) = s1.char_indices().unzip();
    let (cis2, cs2): (Vec<usize>, Vec<char>) = s2.char_indices().unzip();

    // Impossible to have a substring longer than the original strings.
    if cs1.len() < k || cs2.len() < k {
        return None;
    }

    // Note: with_capacity() guarantees that the hash map can hold at least `capacity` elements
    // without reallocating.
    let mut substrings = HashSet::with_capacity(std::cmp::max(cs1.len(), cs2.len()));
    for i in 0..(cs1.len()-k+1) {
        let start = cis1[i];
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let end = *cis1.get(i+k).unwrap_or(&s1.len());
        let sub = &s1[start..end];
        substrings.insert(sub);
    }

    for i in 0..(cs2.len()-k+1) {
        let start = cis2[i];
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let end = *cis2.get(i+k).unwrap_or(&s2.len());
        let sub = &s2[start..end];
        if substrings.contains(sub) {
            // Substring found in both s1 and s2, can return early.
            return Some(sub);
        }
    }

    // No substring of length k in s2 is also in s1.
    return None;
}

#[cfg(test)]
mod tests {
    use std::ops::Range;
    use substring::Substring;
    use proptest::prelude::*;
    use crate::substring;

    // Reference implementation for substring to compare against for correctness
    fn substring_reference_impl<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
        // Trivial to have matching substrings of length 0
        if k == 0 {
            return Some("");
        }

        // Since strings are essentially lists of UTF-8 bytes in Rust, we instead want to iterate over
        // the UTF-8 characters (unicode scalar values).
        let s2_n_chars = s2.chars().count();

        // Impossible to have a substring longer than the original strings.
        if s2_n_chars < k {
            return None;
        }

        for i in 0..(s2_n_chars-k+1) {
            let sub = s2.substring(i,i+k);
            // In this reference implementation we return the first substring in s2 which appears in s1.
            if s1.contains(sub) {
                return Some(sub)
            }
        }

        return None
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
            let expected_substring = substring_reference_impl(&s1, &s2, k);
            let r = substring(&s1, &s2, k);
            assert_eq!(r, expected_substring);
        }
    }

}
