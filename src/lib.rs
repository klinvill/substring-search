pub mod helpers;

use std::collections::{HashSet, VecDeque};
use std::str::CharIndices;
use rustc_hash::FxHashSet;

/// Given two strings, returns the first found common substring of length k or None if no such
/// substring exists.
///
/// This function uses a hashmap (as per the assignment guidelines).
pub fn substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    _naive_prereserve_iter_fx_substring(s1, s2, k)
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
    _alternate_prereserve_iter_fx_substring(s1, s2, k)
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
    let mut substrings = HashSet::with_capacity(cs1.len());
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
/// the needed size of the hash table up front so rehashing is not needed. It also uses the
/// char_indices() iterator directly instead of copying it to a vec for better performance.
#[deprecated = "The implementations shouldn't be called directly. Call substring() instead."]
pub fn _naive_prereserve_iter_substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    // Trivial to have matching substrings of length 0
    if k == 0 {
        // In this case we opt to return the empty string. Another implementation could return None
        // instead.
        return Some("");
    }

    // Since strings are essentially lists of UTF-8 bytes in Rust, we instead want to iterate over
    // the UTF-8 characters (unicode scalar values). We also keep track of the original indices in
    // s1 and s2 so we can more efficiently get substrings (without having to create new strings).
    // We use the iterators directly instead of extracting to a vec for better performance.
    let cs1_len = s1.chars().count();
    let cs2_len = s2.chars().count();
    let mut cs1 = s1.char_indices();
    let mut cs2 = s2.char_indices();

    // Impossible to have a substring longer than the original strings.
    if cs1_len < k || cs2_len < k {
        return None;
    }

    // Note: with_capacity() guarantees that the hash map can hold at least `capacity` elements
    // without reallocating.
    let mut substrings = HashSet::with_capacity(cs1_len);
    // We use a Deque so we can quickly slide a window along the indices (using pop_front() and
    // push_back()).
    let mut sub_cs1_ind: VecDeque<usize> = VecDeque::with_capacity(k+1);
    let mut sub_cs2_ind: VecDeque<usize> = VecDeque::with_capacity(k+1);
    // Pre-loads the indices for the first substring
    for _ in 0..k {
        let (i1, _) = cs1.next().unwrap();
        sub_cs1_ind.push_back(i1);
        let (i2, _) = cs2.next().unwrap();
        sub_cs2_ind.push_back(i2);
    }

    // Helper function to fetch the next substring using a stateful sliding window (sub_indices) of
    // character indices.
    fn next_substring<'b>(cs: &mut CharIndices, sub_indices: &mut VecDeque<usize>, source: &'b str) -> &'b str {
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let (i, _) = cs.next().unwrap_or((source.len(), 'a'));
        sub_indices.push_back(i);
        let start = sub_indices.pop_front().unwrap();
        let end = *sub_indices.back().unwrap();
        let sub = &source[start..end];
        sub
    }

    for _ in k..cs1_len+1 {
        let sub = next_substring(&mut cs1, &mut sub_cs1_ind, s1);
        substrings.insert(sub);
    }
    // Sanity check to make sure we've read all the characters
    assert!(cs1.next().is_none());

    for _ in k..cs2_len+1 {
        let sub = next_substring(&mut cs2, &mut sub_cs2_ind, s2);
        if substrings.contains(sub) {
            // Substring found in both s1 and s2, can return early.
            return Some(sub);
        }
    }
    // Sanity check to make sure we've read all the characters
    assert!(cs2.next().is_none());

    // No substring of length k in s2 is also in s1.
    return None;
}

/// Naive implementation of substring search. Sticks all k-length substrings of first in a hashmap,
/// then checks all the k-length substrings in s2 to see if any are already in the hashmap. Runs
/// in ~O(n) time (n-k+1 insertions for substrings in s1 (where n is the length of s1), up to
/// n-k+1 queries for substrings in s2 (where n is the length of s2)). This function pre-reserves
/// the needed size of the hash table up front so rehashing is not needed. It also uses the
/// char_indices() iterator directly instead of copying it to a vec for better performance. This
/// function also uses the firefox hashing algorithm which is faster than Rust's default SIP
/// hashing algorithm at the cost of reduced resilience against an adversarial user.
#[deprecated = "The implementations shouldn't be called directly. Call substring() instead."]
pub fn _naive_prereserve_iter_fx_substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    // Trivial to have matching substrings of length 0
    if k == 0 {
        // In this case we opt to return the empty string. Another implementation could return None
        // instead.
        return Some("");
    }

    // Since strings are essentially lists of UTF-8 bytes in Rust, we instead want to iterate over
    // the UTF-8 characters (unicode scalar values). We also keep track of the original indices in
    // s1 and s2 so we can more efficiently get substrings (without having to create new strings).
    // We use the iterators directly instead of extracting to a vec for better performance.
    let cs1_len = s1.chars().count();
    let cs2_len = s2.chars().count();
    let mut cs1 = s1.char_indices();
    let mut cs2 = s2.char_indices();

    // Impossible to have a substring longer than the original strings.
    if cs1_len < k || cs2_len < k {
        return None;
    }

    // Note: we reserve space to guarantee that the hash map can hold at least `capacity` elements
    // without reallocating.
    let mut substrings = FxHashSet::default();
    substrings.reserve(cs1_len);
    // We use a Deque so we can quickly slide a window along the indices (using pop_front() and
    // push_back()).
    let mut sub_cs1_ind: VecDeque<usize> = VecDeque::with_capacity(k+1);
    let mut sub_cs2_ind: VecDeque<usize> = VecDeque::with_capacity(k+1);
    // Pre-loads the indices for the first substring
    for _ in 0..k {
        let (i1, _) = cs1.next().unwrap();
        sub_cs1_ind.push_back(i1);
        let (i2, _) = cs2.next().unwrap();
        sub_cs2_ind.push_back(i2);
    }

    // Helper function to fetch the next substring using a stateful sliding window (sub_indices) of
    // character indices.
    fn next_substring<'b>(cs: &mut CharIndices, sub_indices: &mut VecDeque<usize>, source: &'b str) -> &'b str {
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let (i, _) = cs.next().unwrap_or((source.len(), 'a'));
        sub_indices.push_back(i);
        let start = sub_indices.pop_front().unwrap();
        let end = *sub_indices.back().unwrap();
        let sub = &source[start..end];
        sub
    }

    for _ in k..cs1_len+1 {
        let sub = next_substring(&mut cs1, &mut sub_cs1_ind, s1);
        substrings.insert(sub);
    }
    // Sanity check to make sure we've read all the characters
    assert!(cs1.next().is_none());

    for _ in k..cs2_len+1 {
        let sub = next_substring(&mut cs2, &mut sub_cs2_ind, s2);
        if substrings.contains(sub) {
            // Substring found in both s1 and s2, can return early.
            return Some(sub);
        }
    }
    // Sanity check to make sure we've read all the characters
    assert!(cs2.next().is_none());

    // No substring of length k in s2 is also in s1.
    return None;
}

/// Naive implementation of substring search. Sticks all k-length substrings of the shortest string
/// in a hashmap, then checks all the k-length substrings in the other string to see if any are
/// already in the hashmap. Runs in ~O(n) time (n-k+1 insertions for substrings in the shorter
/// string (where n is the length of the string), up to n-k+1 queries for substrings in the other
/// string (where n is the length of the string)). This function pre-reserves the needed size of
/// the hash table up front so rehashing is not needed. It also uses the char_indices() iterator
/// directly instead of copying it to a vec for better performance. This function also uses the
/// firefox hashing algorithm which is faster than Rust's default SIP hashing algorithm at the cost
/// of reduced resilience against an adversarial user.
#[deprecated = "The implementations shouldn't be called directly. Call substring() instead."]
pub fn _naive_prereserve_iter_fx_shorter_substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    // Trivial to have matching substrings of length 0
    if k == 0 {
        // In this case we opt to return the empty string. Another implementation could return None
        // instead.
        return Some("");
    }

    // Since strings are essentially lists of UTF-8 bytes in Rust, we instead want to iterate over
    // the UTF-8 characters (unicode scalar values). We also keep track of the original indices in
    // s1 and s2 so we can more efficiently get substrings (without having to create new strings).
    // We use the iterators directly instead of extracting to a vec for better performance.
    let cs1_len = s1.chars().count();
    let cs2_len = s2.chars().count();

    // Impossible to have a substring longer than the original strings.
    if cs1_len < k || cs2_len < k {
        return None;
    }

    // Choose shorter string to be the one we store in the hash table
    let (shorter, longer) = if cs1_len <= cs2_len {(s1, s2)} else {(s2, s1)};
    let (cs_short_len, cs_long_len) = if cs1_len <= cs2_len {(cs1_len, cs2_len)} else {(cs2_len, cs1_len)};

    let mut cs_short = shorter.char_indices();
    let mut cs_long = longer.char_indices();

    // Note: we reserve space to guarantee that the hash map can hold at least `capacity` elements
    // without reallocating.
    let mut substrings = FxHashSet::default();
    substrings.reserve(cs_short_len);
    // We use a Deque so we can quickly slide a window along the indices (using pop_front() and
    // push_back()).
    let mut sub_cs_short_ind: VecDeque<usize> = VecDeque::with_capacity(k+1);
    let mut sub_cs_long_ind: VecDeque<usize> = VecDeque::with_capacity(k+1);
    // Pre-loads the indices for the first substring
    for _ in 0..k {
        let (i1, _) = cs_short.next().unwrap();
        sub_cs_short_ind.push_back(i1);
        let (i2, _) = cs_long.next().unwrap();
        sub_cs_long_ind.push_back(i2);
    }

    // Helper function to fetch the next substring using a stateful sliding window (sub_indices) of
    // character indices.
    fn next_substring<'b>(cs: &mut CharIndices, sub_indices: &mut VecDeque<usize>, source: &'b str) -> &'b str {
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let (i, _) = cs.next().unwrap_or((source.len(), 'a'));
        sub_indices.push_back(i);
        let start = sub_indices.pop_front().unwrap();
        let end = *sub_indices.back().unwrap();
        let sub = &source[start..end];
        sub
    }

    for _ in k..cs_short_len+1 {
        let sub = next_substring(&mut cs_short, &mut sub_cs_short_ind, shorter);
        substrings.insert(sub);
    }
    // Sanity check to make sure we've read all the characters
    assert!(cs_short.next().is_none());

    for _ in k..cs_long_len+1 {
        let sub = next_substring(&mut cs_long, &mut sub_cs_long_ind, longer);
        if substrings.contains(sub) {
            // Substring found in both s1 and s2, can return early.
            return Some(sub);
        }
    }
    // Sanity check to make sure we've read all the characters
    assert!(cs_long.next().is_none());

    // No substring of length k in s2 is also in s1.
    return None;
}

/// Implementation of substring search that uses two hash tables to store seen substrings. It
/// alternates between insertion and checking for each of the two strings with the idea that
/// matching substrings may often be found early in very long strings. As a result, fewer insertions
/// (which currently dominate the running time) should be needed on average. Runs in ~O(n) time
/// (up to n-k+1 insertions and queries for each string (where n is the length of the string)).
///
/// Unlike previous implementations, this implementation does not guarantee that the first
/// substring found is the first one in the second string that occurs anywhere in the first string.
///
/// The algorithm is correct because only three cases can occur:
///     1) A substring appears later in the first string than in the second string
///     2) A substring appears later in the second string than in the first string
///     3) A substring appears at the same index in the first and second strings
/// In the first case, the substring will already have been inserted into the second hash table so
/// when it is reached in the first string, a lookup in the second hash table will show it is a
/// matching substring. The second case works the same as the first, but using the first hash table.
/// In the third case, since we ensure that insertions happen before lookups, the substring will
/// already be inserted into both hash tables. A lookup into either suffices to show it is a
/// matching substring.
///
/// This function pre-reserves the needed size of the hash table up front so rehashing is not
/// needed. It also uses the char_indices() iterator directly instead of copying it to a vec for
/// better performance. This function also uses the firefox hashing algorithm which is faster than
/// Rust's  default SIP hashing algorithm at the cost of reduced resilience against an adversarial
/// user.
#[deprecated = "The implementations shouldn't be called directly. Call substring() instead."]
pub fn _alternate_prereserve_iter_fx_substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
    // Trivial to have matching substrings of length 0
    if k == 0 {
        // In this case we opt to return the empty string. Another implementation could return None
        // instead.
        return Some("");
    }

    // Since strings are essentially lists of UTF-8 bytes in Rust, we instead want to iterate over
    // the UTF-8 characters (unicode scalar values). We also keep track of the original indices in
    // s1 and s2 so we can more efficiently get substrings (without having to create new strings).
    // We use the iterators directly instead of extracting to a vec for better performance.
    let cs1_len = s1.chars().count();
    let cs2_len = s2.chars().count();
    let mut cs1 = s1.char_indices();
    let mut cs2 = s2.char_indices();

    // Impossible to have a substring longer than the original strings.
    if cs1_len < k || cs2_len < k {
        return None;
    }

    // Note: we reserve space to guarantee that the hash map can hold at least `capacity` elements
    // without reallocating.
    let mut substrings1 = FxHashSet::default();
    substrings1.reserve(cs1_len);
    let mut substrings2 = FxHashSet::default();
    substrings2.reserve(cs2_len);

    // We use a Deque so we can quickly slide a window along the indices (using pop_front() and
    // push_back()).
    let mut sub_cs1_ind: VecDeque<usize> = VecDeque::with_capacity(k+1);
    let mut sub_cs2_ind: VecDeque<usize> = VecDeque::with_capacity(k+1);
    // Pre-loads the indices for the first substring
    for _ in 0..k {
        let (i1, _) = cs1.next().unwrap();
        sub_cs1_ind.push_back(i1);
        let (i2, _) = cs2.next().unwrap();
        sub_cs2_ind.push_back(i2);
    }

    // Helper function to fetch the next substring using a stateful sliding window (sub_indices) of
    // character indices.
    fn next_substring<'b>(cs: &mut CharIndices, sub_indices: &mut VecDeque<usize>, source: &'b str) -> &'b str {
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let (i, _) = cs.next().unwrap_or((source.len(), 'a'));
        sub_indices.push_back(i);
        let start = sub_indices.pop_front().unwrap();
        let end = *sub_indices.back().unwrap();
        let sub = &source[start..end];
        sub
    }

    for _ in k..std::cmp::min(cs1_len, cs2_len)+1 {
        // Insert next substring
        let sub1 = next_substring(&mut cs1, &mut sub_cs1_ind, s1);
        substrings1.insert(sub1);
        let sub2 = next_substring(&mut cs2, &mut sub_cs2_ind, s2);
        substrings2.insert(sub2);

        // Then check to see if that substring has been before in the other string. We need to
        // insert before checking to make sure we don't end up in a case where a matching substring
        // is deemed not to be matching because both strings insert it simultaneously after
        // determining that it is not in the other yet, and may never be checked again.
        if substrings2.contains(sub1) {
            // Substring found in both s1 and s2, can return early.
            return Some(sub1);
        } else if substrings1.contains(sub2) {
            // Substring found in both s1 and s2, can return early.
            return Some(sub2);
        }
    }

    // One of the strings has been fully inserted, this means we only need to check substrings of
    // the other to see if their in the fully inserted string.
    let (longer, longer_ind, longer_s, shorter_table) = if cs1_len <= cs2_len {(&mut cs2, &mut sub_cs2_ind, s2, &substrings1)} else {(&mut cs1, &mut sub_cs1_ind, s1, &substrings2)};
    for _ in std::cmp::min(cs1_len, cs2_len)+1..std::cmp::max(cs1_len, cs2_len)+1 {
        let sub = next_substring(longer, longer_ind, longer_s);
        if shorter_table.contains(sub) {
            // Substring found in both s1 and s2, can return early.
            return Some(sub);
        }
    }

    // Sanity check to make sure we've read all the characters
    assert!(cs1.next().is_none());
    assert!(cs2.next().is_none());

    // No substring of length k in s2 is also in s1.
    return None;
}

#[cfg(test)]
mod tests {
    use std::ops::Range;
    use substring::Substring;
    use proptest::prelude::*;
    use crate::{substring, unordered_substring};

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
            let expected_substring = substring_reference_impl(&s1, &s2, k);
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
                None => assert_eq!(r, substring_reference_impl(&s1, &s2, k)),
                Some(sub) => assert_eq!(unordered_substring_correct(sub, &s1, &s2, k), true),
            };
        }
    }

}
