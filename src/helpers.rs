use itertools::Itertools;

/// Strips out newlines and carriage returns (\n and \r) and strips spaces down to a single space
/// character. This was advised in the assignment guidelines to match the instructor's results.
pub fn preprocess_string(string: &str) -> String {
    if string.len() == 0 {
        return String::new();
    }
    let single_stripped = string.bytes().filter(|byte| {
        *byte != b'\r' && *byte != b'\n'
    });
    // The sliding window approach we take below to look at pairs of bytes has an off-by-one
    // problem. To fix this, we just manually add the first byte to the front of the iterator.
    let try_first_byte = single_stripped.clone().next();
    if try_first_byte.is_none() {
        // No characters left in string
        return String::new();
    }
    let first_iter = std::iter::once((0u8, try_first_byte.unwrap()));
    // We strip out neighboring spaces after stripping out single bytes (like `\r`) to properly
    // handle the case when we have spaces like: ` \r `.
    let new_string = first_iter.chain(single_stripped.tuple_windows())
        .filter_map(|(b1, b2)| {
        if b1 != b' ' || b2 != b' ' {
            Some(b2)
        } else {
            None
        }
    }).collect();
    String::from_utf8(new_string).unwrap()
}


mod tests {
    use crate::helpers::preprocess_string;

    #[test]
    fn test_preprocess_substring() {
        let s = "\rThis is a \r\n\r test string.   - Normal Person\n";
        let expected = "This is a test string. - Normal Person";

        assert_eq!(preprocess_string(s), expected);
    }

    #[test]
    fn test_preprocess_newlines_only() {
        let s = "\n\r\r\n\r";
        let expected = String::new();

        assert_eq!(preprocess_string(s), expected);
    }

    #[test]
    fn test_preprocess_spaces_only() {
        let s = "       ";
        let expected = " ";

        assert_eq!(preprocess_string(s), expected);
    }
}
