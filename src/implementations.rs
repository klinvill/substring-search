use std::collections::{HashSet, VecDeque};
use std::hash::Hasher;
use std::str::CharIndices;
use rustc_hash::{FxHasher, FxHashSet};
use hashbrown::raw::RawTable;
use adler32::RollingAdler32;
use crate::hashers::RollingPolynomial;

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

// Returns a function that, when called, returns the next substring of length k from `source` and
// its computed hash value.
pub fn build_rolling_substring<'b>(source: &'b str, k: usize) -> Box<dyn FnMut() -> (&'b str, u64) + 'b> {
    let mut cs = source.char_indices();

    // We use a Deque so we can quickly slide a window along the indices (using pop_front() and
    // push_back()). It stores tuples of (c, i) where c is the character ending at index i.
    let mut prev_chars: VecDeque<(&str, usize)> = VecDeque::with_capacity(k+1);
    let mut prev_i = 0;

    // Pre-loads the indices for the first substring
    for _ in 0..k {
        let (i, _) = cs.next().unwrap();
        prev_chars.push_back((&source[prev_i..i], i));
        prev_i = i;
    }

    let mut hash = RollingAdler32::from_buffer(source[..prev_i].as_bytes());

    Box::new(move || {
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let (i, _) = cs.next().unwrap_or((source.len(), 'a'));
        let next_char = &source[prev_i..i];
        let (old_char, old_offset) = prev_chars.pop_front().unwrap();
        prev_chars.push_back((next_char, i));
        old_char.bytes().enumerate().for_each(|(i, b)| {
            hash.remove(prev_i - old_offset + old_char.len() - i, b)
        });
        hash.update_buffer(next_char.as_bytes());
        prev_i = i;
        (&source[old_offset..i], hash.hash() as u64)
    })
}

// Returns a function that, when called, returns the next substring of length k from `source` and
// its computed hash value.
pub fn build_rolling_polynomial_substring<'b>(source: &'b str, k: usize) -> Box<dyn FnMut() -> (&'b str, u64) + 'b> {
    let mut cs = source.char_indices();

    // We use a Deque so we can quickly slide a window along the indices (using pop_front() and
    // push_back()). It stores tuples of (c, i) where c is the character ending at index i.
    let mut prev_chars: VecDeque<(&str, usize)> = VecDeque::with_capacity(k+1);
    let mut prev_i = 0;

    // Pre-loads the indices for the first substring
    for _ in 0..k {
        let (i, _) = cs.next().unwrap();
        prev_chars.push_back((&source[prev_i..i], i));
        prev_i = i;
    }

    let mut hash = RollingPolynomial::from_buffer(source[..prev_i].as_bytes());

    Box::new(move || {
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let (i, _) = cs.next().unwrap_or((source.len(), 'a'));
        let next_char = &source[prev_i..i];
        let (old_char, old_offset) = prev_chars.pop_front().unwrap();
        prev_chars.push_back((next_char, i));
        old_char.bytes().enumerate().for_each(|(i, b)| {
            hash.remove((prev_i - old_offset + old_char.len() - i) as u32, b)
        });
        hash.update_buffer(next_char.as_bytes());
        prev_i = i;
        (&source[old_offset..i], hash.hash() as u64)
    })
}

// Returns a function that, when called, returns the next substring of length k from `source` and
// its computed hash value.
pub fn build_fx_substring<'b>(source: &'b str, k: usize) -> Box<dyn FnMut() -> (&'b str, u64) + 'b> {
    let mut cs = source.char_indices();

    // We use a Deque so we can quickly slide a window along the indices (using pop_front() and
    // push_back()). It stores tuples of (c, i) where c is the character ending at index i.
    let mut prev_chars: VecDeque<(&str, usize)> = VecDeque::with_capacity(k+1);
    let mut prev_i = 0;

    // Pre-loads the indices for the first substring
    for _ in 0..k {
        let (i, _) = cs.next().unwrap();
        prev_chars.push_back((&source[prev_i..i], i));
        prev_i = i;
    }

    #[inline]
    fn hash(s: &str) -> u64 {
        let mut hasher = FxHasher::default();
        hasher.write(s.as_bytes());
        hasher.finish()
    }

    Box::new(move || {
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let (i, _) = cs.next().unwrap_or((source.len(), 'a'));
        let next_char = &source[prev_i..i];
        let (old_char, old_offset) = prev_chars.pop_front().unwrap();
        prev_chars.push_back((next_char, i));
        prev_i = i;
        let sub = &source[old_offset..i];
        (sub, hash(sub))
    })
}

// Returns a function that, when called, returns the next substring of length k from `source` and
// its computed hash value.
pub fn build_sip_substring<'b>(source: &'b str, k: usize) -> Box<dyn FnMut() -> (&'b str, u64) + 'b> {
    let mut cs = source.char_indices();

    // We use a Deque so we can quickly slide a window along the indices (using pop_front() and
    // push_back()). It stores tuples of (c, i) where c is the character ending at index i.
    let mut prev_chars: VecDeque<(&str, usize)> = VecDeque::with_capacity(k+1);
    let mut prev_i = 0;

    // Pre-loads the indices for the first substring
    for _ in 0..k {
        let (i, _) = cs.next().unwrap();
        prev_chars.push_back((&source[prev_i..i], i));
        prev_i = i;
    }

    #[inline]
    fn hash(s: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        hasher.write(s.as_bytes());
        hasher.finish()
    }

    Box::new(move || {
        // Normally we want to read the bytes up until the start of the next character, but when
        // we've reached past the end of the string, it suffices to just read the rest of the string.
        let (i, _) = cs.next().unwrap_or((source.len(), 'a'));
        let next_char = &source[prev_i..i];
        let (old_char, old_offset) = prev_chars.pop_front().unwrap();
        prev_chars.push_back((next_char, i));
        prev_i = i;
        let sub = &source[old_offset..i];
        (sub, hash(sub))
    })
}

/// Naive implementation of substring search. Sticks all k-length substrings of the shortest string
/// in a hashmap, then checks all the k-length substrings in the other string to see if any are
/// already in the hashmap. Runs in ~O(n) time (n-k+1 insertions for substrings in the shorter
/// string (where n is the length of the string), up to n-k+1 queries for substrings in the other
/// string (where n is the length of the string)). This function pre-reserves the needed size of
/// the hash table up front so rehashing is not needed. It also uses the char_indices() iterator
/// directly instead of copying it to a vec for better performance. This function also uses the
/// rolling adler32 hashing algorithm to attempt to improve hashing performance for sliding windows.
#[deprecated = "The implementations shouldn't be called directly. Call substring() instead."]
pub fn _naive_prereserve_iter_rolling_adler_shorter_substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
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

    // Note: we reserve space to guarantee that the hash map can hold at least `capacity` elements
    // without reallocating.
    // We need to use a `RawTable` to ensure that we can re-use our previously computed rolling
    // hash. The standard hash table will just recompute the rolling hash from scratch (I think).
    let mut substrings = RawTable::with_capacity(cs_short_len);

    let mut short_sub_fn = build_rolling_substring(shorter, k);
    let mut long_sub_fn = build_rolling_substring(longer, k);

    /// Ensures that a single closure type across uses of this which, in turn prevents multiple
    /// instances of any functions like RawTable::reserve from being generated. Taken from hashbrown.
    #[inline]
    fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
        where
            K: core::borrow::Borrow<Q>,
            Q: ?Sized + Eq,
    {
        move |x| k.eq(x.0.borrow())
    }

    for _ in k..cs_short_len+1 {
        let (sub, hash) = short_sub_fn();
        // We want to use a hash set, so we only insert into the table if it's not already in there.
        if substrings.find(hash, equivalent_key(sub)).is_none() {
            substrings.try_insert_no_grow(hash, (sub, ())).unwrap();
        }
    }
    // Sanity check to make sure we've read all the characters
    assert!(short_sub_fn().0.chars().count() < k);

    for _ in k..cs_long_len+1 {
        let (sub, hash) = long_sub_fn();
        if substrings.find(hash, equivalent_key(&sub)).is_some() {
            // Substring found in both s1 and s2, can return early.
            return Some(sub);
        }
    }
    // Sanity check to make sure we've read all the characters
    assert!(long_sub_fn().0.chars().count() < k);

    // No substring of length k in s2 is also in s1.
    return None;
}

/// Naive implementation of substring search. Sticks all k-length substrings of the shortest string
/// in a hashmap, then checks all the k-length substrings in the other string to see if any are
/// already in the hashmap. Runs in ~O(n) time (n-k+1 insertions for substrings in the shorter
/// string (where n is the length of the string), up to n-k+1 queries for substrings in the other
/// string (where n is the length of the string)). This function pre-reserves the needed size of
/// the hash table up front so rehashing is not needed. It also uses the char_indices() iterator
/// directly instead of copying it to a vec for better performance. This function also uses a
/// rolling polynomial hash to attempt to improve hashing performance for sliding windows.
#[deprecated = "The implementations shouldn't be called directly. Call substring() instead."]
pub fn _naive_prereserve_iter_rolling_poly_shorter_substring<'a>(s1: &'a str, s2: &'a str, k: usize) -> Option<&'a str> {
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

    // Note: we reserve space to guarantee that the hash map can hold at least `capacity` elements
    // without reallocating.
    // We need to use a `RawTable` to ensure that we can re-use our previously computed rolling
    // hash. The standard hash table will just recompute the rolling hash from scratch (I think).
    let mut substrings = RawTable::with_capacity(cs_short_len);

    let mut short_sub_fn = build_rolling_polynomial_substring(shorter, k);
    let mut long_sub_fn = build_rolling_polynomial_substring(longer, k);

    /// Ensures that a single closure type across uses of this which, in turn prevents multiple
    /// instances of any functions like RawTable::reserve from being generated. Taken from hashbrown.
    #[inline]
    fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
        where
            K: core::borrow::Borrow<Q>,
            Q: ?Sized + Eq,
    {
        move |x| k.eq(x.0.borrow())
    }

    for _ in k..cs_short_len+1 {
        let (sub, hash) = short_sub_fn();
        // We want to use a hash set, so we only insert into the table if it's not already in there.
        if substrings.find(hash, equivalent_key(sub)).is_none() {
            substrings.try_insert_no_grow(hash, (sub, ())).unwrap();
        }
    }
    // Sanity check to make sure we've read all the characters
    assert!(short_sub_fn().0.chars().count() < k);

    for _ in k..cs_long_len+1 {
        let (sub, hash) = long_sub_fn();
        if substrings.find(hash, equivalent_key(&sub)).is_some() {
            // Substring found in both s1 and s2, can return early.
            return Some(sub);
        }
    }
    // Sanity check to make sure we've read all the characters
    assert!(long_sub_fn().0.chars().count() < k);

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
    use adler32::RollingAdler32;
    use crate::implementations::build_rolling_substring;

    #[test]
    // Sanity check to make sure the rolling adler hash works as I expect. That is, removing the
    // first element and adding another element should produce the same hash as if you hashed those
    // same elements (in the same order) to begin with.
    fn test_adler32_rolling() {
        let s = "This is a test string. - Normal Person";
        // Testing with window size of 5
        let mut hash = RollingAdler32::from_buffer(s[0..5].as_bytes());
        hash.remove(5, s[0..1].as_bytes()[0]);
        hash.update(s[5..6].as_bytes()[0]);
        assert_eq!(hash.hash(), RollingAdler32::from_buffer(s[1..6].as_bytes()).hash());
    }

    #[test]
    fn test_rolling_next_substring() {
        // Test string that includes multi-byte characters.
        let s = "›It costs €10 for this item…";
        let k = 5;
        let mut next_substring = build_rolling_substring(s, k);

        for i in 0..s.len() - k {
            let expected_sub = s.chars().skip(i).take(k).collect::<String>();
            assert_eq!(
                next_substring(),
                (expected_sub.as_str(), RollingAdler32::from_buffer(expected_sub.as_bytes()).hash() as u64)
            );
        }
    }
}
