use crate::bwt::lms::{is_lms_char, lms_substrings_are_equal};
use std::error::Error;

pub(crate) fn summarize_suffix_array<T: PartialEq>(
    text_bytes: &[T],
    guessed_suffix_array: &[usize],
    type_map: &[u8],
) -> Result<(Vec<usize>, usize, Vec<usize>), Box<dyn Error>> {
    // We will use this array to store the names of LMS substrings
    let mut lms_names = vec![-1; text_bytes.len() + 1];

    // Keep track of what names we've allocated
    let mut current_name = 0;

    // Where in the original string was the last LMS suffix we checked?
    let mut last_lms_suffix_offset = guessed_suffix_array[0];

    // We know that the first LMS-substring will be the empty suffix at position 0
    lms_names[last_lms_suffix_offset] = current_name;

    // For each suffix in the suffix array...
    for &suffix_offset in guessed_suffix_array.iter().skip(1) {
        // We only care about LMS suffixes
        if !is_lms_char(type_map, suffix_offset)? {
            continue;
        }

        // If this LMS suffix starts with a different LMS substring
        // from the last suffix we looked at...
        if !lms_substrings_are_equal(text_bytes, type_map, last_lms_suffix_offset, suffix_offset)? {
            // ...then it gets a new name
            current_name += 1;
        }

        // Record the last LMS suffix we looked at
        last_lms_suffix_offset = suffix_offset;

        // Store the name of this LMS suffix
        lms_names[suffix_offset] = current_name;
    }

    // Allocate vectors with estimated capacity to avoid reallocations
    let mut summary_suffix_offsets = Vec::new();
    let mut summary_string = Vec::new();

    // Extract only the valid LMS names and their positions
    for (index, &name) in lms_names.iter().enumerate() {
        if name != -1 {
            summary_suffix_offsets.push(index);
            summary_string.push(name as usize);
        }
    }

    // The summary alphabet size is one larger than the largest numbered character
    let summary_alphabet_size = (current_name + 1) as usize;

    Ok((
        summary_string,
        summary_alphabet_size,
        summary_suffix_offsets,
    ))
}

#[cfg(test)]
mod tests {
    use crate::bwt::bucket::char_buckets;
    use crate::bwt::lms::{build_type_map, guess_lms_sort, induce_sort_l, induce_sort_s};
    use crate::bwt::summary::summarize_suffix_array;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_summarize_suffix_array1() -> Result<(), Box<dyn Error>> {
        let cabbage = "cabbage".as_bytes();
        let char_map = (0..7)
            .map(|n| (b'a' + n, n as usize))
            .collect::<HashMap<_, _>>();
        let cabbage_bucket = char_buckets(cabbage, &char_map)?;
        let cabbage_types = build_type_map(cabbage)?;
        let mut guessed_suffix_array =
            guess_lms_sort(cabbage, &char_map, &cabbage_bucket, &cabbage_types)?;
        induce_sort_l(
            &mut guessed_suffix_array,
            cabbage,
            &char_map,
            &cabbage_bucket,
            &cabbage_types,
        )?;
        induce_sort_s(
            &mut guessed_suffix_array,
            cabbage,
            &char_map,
            &cabbage_bucket,
            &cabbage_types,
        )?;
        let guessed_suffix_array = guessed_suffix_array
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let (summary_string, summary_alphabet_size, summary_suffix_offsets) =
            summarize_suffix_array(cabbage, &guessed_suffix_array, &cabbage_types)?;
        assert_eq!(summary_string, vec![1, 2, 0]);
        assert_eq!(summary_alphabet_size, 3);
        assert_eq!(summary_suffix_offsets, vec![1, 4, 7]);
        Ok(())
    }

    #[test]
    fn test_summarize_suffix_array2() -> Result<(), Box<dyn Error>> {
        let cabbage = "baabaabac".as_bytes();
        let char_map = (0..7)
            .map(|n| (b'a' + n, n as usize))
            .collect::<HashMap<_, _>>();
        let cabbage_bucket = char_buckets(cabbage, &char_map)?;
        let cabbage_types = build_type_map(cabbage)?;
        let mut guessed_suffix_array =
            guess_lms_sort(cabbage, &char_map, &cabbage_bucket, &cabbage_types)?;
        induce_sort_l(
            &mut guessed_suffix_array,
            cabbage,
            &char_map,
            &cabbage_bucket,
            &cabbage_types,
        )?;
        induce_sort_s(
            &mut guessed_suffix_array,
            cabbage,
            &char_map,
            &cabbage_bucket,
            &cabbage_types,
        )?;
        let guessed_suffix_array = guessed_suffix_array
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let (summary_string, summary_alphabet_size, summary_suffix_offsets) =
            summarize_suffix_array(cabbage, &guessed_suffix_array, &cabbage_types)?;
        assert_eq!(summary_string, vec![1, 1, 2, 0]);
        assert_eq!(summary_alphabet_size, 3);
        assert_eq!(summary_suffix_offsets, vec![1, 4, 7, 9]);
        Ok(())
    }
}
