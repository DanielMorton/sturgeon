use crate::bwt::bucket::{find_bucket_heads, find_bucket_tails};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;

pub const L: u8 = b'L';
pub const S: u8 = b'S';

pub(crate) fn build_type_map<T:Ord>(data: &[T]) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(if data.is_empty() {
        vec![S]
    } else {
        let n = data.len();
        let mut type_map = vec![0; n + 1];
        type_map[n] = S;
        type_map[n - 1] = L;
        for i in (0..=n - 2).rev() {
            if data[i] > data[i + 1] {
                type_map[i] = L;
            } else if data[i] < data[i + 1] {
                type_map[i] = S;
            } else {
                type_map[i] = type_map[i+1];
            }
        }
        type_map
    })
}

pub(crate) fn is_lms_char(type_map: &[u8], offset: usize) -> Result<bool, Box<dyn Error>> {
    Ok(if offset == 0 {
        false
    } else if type_map[offset] == S && type_map[offset - 1] == L {
        true
    } else {
        false
    })
}

pub(crate) fn lms_substrings_are_equal<T: PartialEq>(
    text_bytes: &[T],
    type_map: &[u8],
    offset_a: usize,
    offset_b: usize,
) -> Result<bool, Box<dyn Error>> {
    let n = text_bytes.len();
    if offset_a == n || offset_b == n {
        return Ok(false);
    }

    let mut i = 0;
    loop {
        // Avoid bounds checking in hot loop by pre-calculating max positions
        let pos_a = offset_a + i;
        let pos_b = offset_b + i;

        // Protect against out-of-bounds access
        if pos_a >= n || pos_b >= n {
            return Ok(false);
        }

        let a_is_lms = is_lms_char(type_map, pos_a)?;
        let b_is_lms = is_lms_char(type_map, pos_b)?;

        // If we've found the start of the next LMS substrings...
        if i > 0 && a_is_lms && b_is_lms {
            // We made it through original LMS substrings without difference
            return Ok(true);
        }

        if a_is_lms != b_is_lms {
            // End of one LMS substring before the other
            return Ok(false);
        }

        if text_bytes[pos_a] != text_bytes[pos_b] {
            // Character difference found
            return Ok(false);
        }

        i += 1;
    }
}

pub(crate) fn guess_lms_sort<T: Copy + Eq + Hash>(
    text_bytes: &[T],
    char_map: &HashMap<T, usize>,
    bucket_sizes: &[usize],
    type_map: &[u8],
) -> Result<Vec<Option<usize>>, Box<dyn Error>> {
    let n = text_bytes.len();
    let mut guessed_suffix_array = vec![None; n + 1];

    let mut bucket_tails = find_bucket_tails(bucket_sizes)?;
    for i in 0..n {
        if is_lms_char(type_map, i)? {
            let bucket_index = *char_map.get(&text_bytes[i]).unwrap();
            guessed_suffix_array[bucket_tails[bucket_index]] = Some(i);
            bucket_tails[bucket_index] -= 1;
        }
    }
    guessed_suffix_array[0] = Some(n);

    Ok(guessed_suffix_array)
}

pub(crate) fn induce_sort_l<T: Copy + Display + Eq + Hash>(
    guessed_suffix_array: &mut [Option<usize>],
    text_bytes: &[T],
    char_map: &HashMap<T, usize>,
    bucket_sizes: &[usize],
    type_map: &[u8],
) -> Result<(), Box<dyn Error>> {
    let mut bucket_heads = find_bucket_heads(bucket_sizes)?;
    let n = text_bytes.len();
    for i in 0..=n {
        let Some(j) = guessed_suffix_array[i] else { continue };

        if j == 0 { continue };

        let prev = j - 1;
        if type_map[prev] == L {
            // Use direct array indexing with unwrap_or_else for better performance
            let bucket_index = *char_map.get(&text_bytes[prev])
                .expect(&format!("Character {} should exist in char_map", &text_bytes[prev]));

            guessed_suffix_array[bucket_heads[bucket_index]] = Some(prev);
            bucket_heads[bucket_index] += 1;
        }
    }
    Ok(())
}

pub(crate) fn induce_sort_s<T: Copy + Display + Eq + Hash>(
    guessed_suffix_array: &mut [Option<usize>],
    text_bytes: &[T],
    char_map: &HashMap<T, usize>,
    bucket_sizes: &[usize],
    type_map: &[u8],
) -> Result<(), Box<dyn Error>> {
    let mut bucket_tails = find_bucket_tails(bucket_sizes)?;
    let n = text_bytes.len();

    for i in (0..=n).rev() {
        let Some(j) = guessed_suffix_array[i] else { continue };

        if j == 0 { continue };

        let prev = j - 1;
        if type_map[prev] == S {
            let bucket_index = *char_map.get(&text_bytes[prev])
                .expect(&format!("Character {} should exist in char_map", &text_bytes[prev]));
            guessed_suffix_array[bucket_tails[bucket_index]] = Some(prev);
            bucket_tails[bucket_index] -= 1;
        }
    }
    Ok(())
}

pub(crate) fn accurate_lms_sort<T: Copy + Eq + Hash>(
    text_bytes: &[T],
    char_map: &HashMap<T, usize>,
    bucket_sizes: &[usize],
    summary_suffix_array: &[usize],
    summary_suffix_offsets: &[usize],
) -> Result<Vec<Option<usize>>, Box<dyn Error>> {
    let n = text_bytes.len();
    // A suffix for every character, plus the empty suffix
    let mut suffix_offsets = vec![None; n + 1];

    // Find bucket tails for placing suffixes
    let mut bucket_tails = find_bucket_tails(bucket_sizes)?;

    // Iterate through summary suffix array in reverse order
    for &suffix_index in summary_suffix_array.iter().skip(2).rev() {
        let string_index = summary_suffix_offsets[suffix_index];

        // Which bucket does this suffix go into?
        let bucket_index = *char_map.get(&text_bytes[string_index]).unwrap();

        // Add the suffix at the tail of the bucket
        suffix_offsets[bucket_tails[bucket_index]] = Some(string_index);

        // Move the tail pointer down
        bucket_tails[bucket_index] -= 1;
    }

    // Always include the empty suffix at the beginning
    suffix_offsets[0] = Some(n);

    Ok(suffix_offsets)
}

#[cfg(test)]
mod tests {
    use crate::bwt::bucket::char_buckets;
    use crate::bwt::lms::{
        accurate_lms_sort, build_type_map, guess_lms_sort, induce_sort_l, induce_sort_s, L, S,
    };
    use crate::bwt::suffix_array::make_summary_suffix_array;
    use crate::bwt::summary::summarize_suffix_array;
    use std::collections::HashMap;
    use std::error::Error;

    #[test]
    fn test_build_type_map1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            build_type_map("cabbage".as_bytes())?,
            vec![L, S, L, L, S, L, L, S]
        );
        Ok(())
    }

    #[test]
    fn test_guess_lms_sort1() -> Result<(), Box<dyn Error>> {
        let cabbage = "cabbage".as_bytes();
        let char_map = (0..7)
            .map(|n| (b'a' + n, n as usize))
            .collect::<HashMap<_, _>>();
        let cabbage_bucket = char_buckets(cabbage, &char_map)?;
        let cabbage_types = build_type_map(cabbage)?;
        let guessed_suffix_array = guess_lms_sort(cabbage, &char_map, &cabbage_bucket, &cabbage_types)?;
        let gsa = guessed_suffix_array.iter().map(|g| g.map(|s| s as i32).unwrap_or(-1)).collect::<Vec<_>>();
        assert_eq!(gsa, vec![7, 4, 1, -1, -1, -1, -1, -1]);
        Ok(())
    }

    #[test]
    fn test_induce_sort_l1() -> Result<(), Box<dyn Error>> {
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
        let gsa = guessed_suffix_array.iter().map(|g| g.map(|s| s as i32).unwrap_or(-1)).collect::<Vec<_>>();
        assert_eq!(gsa, vec![7, 4, 1, 3, 2, 0, 6, 5]);
        Ok(())
    }

    #[test]
    fn test_induce_sort_l2() -> Result<(), Box<dyn Error>> {
        let cabbage = "baabaabac".as_bytes();
        let char_map = (0..3)
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
        let gsa = guessed_suffix_array.iter().map(|g| g.map(|s| s as i32).unwrap_or(-1)).collect::<Vec<_>>();
        assert_eq!(gsa, vec![9, -1, -1, 7, 4, 1, 6, 3, 0, 8]);
        Ok(())
    }

    #[test]
    fn test_induce_sort_s1() -> Result<(), Box<dyn Error>> {
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
        let gsa = guessed_suffix_array.iter().map(|g| g.map(|s| s as i32).unwrap_or(-1)).collect::<Vec<_>>();
        assert_eq!(gsa, vec![7, 1, 4, 3, 2, 0, 6, 5]);
        Ok(())
    }

    #[test]
    fn test_induce_sort_s2() -> Result<(), Box<dyn Error>> {
        let cabbage = "baabaabac".as_bytes();
        let char_map = (0..3)
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
        let gsa = guessed_suffix_array.iter().map(|g| g.map(|s| s as i32).unwrap_or(-1)).collect::<Vec<_>>();
        assert_eq!(gsa, vec![9, 4, 1, 5, 2, 7, 6, 3, 0, 8]);
        Ok(())
    }

    #[test]
    fn test_accurate_lms_sort1() -> Result<(), Box<dyn Error>> {
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
            .into_iter().flatten()
            .collect::<Vec<_>>();
        let (summary_string, summary_alphabet_size, summary_suffix_offsets) =
            summarize_suffix_array(cabbage, &guessed_suffix_array, &cabbage_types)?;
        let cabbage_summary_suffix_array =
            make_summary_suffix_array(&summary_string, summary_alphabet_size)?;
        let suffix_array = accurate_lms_sort(
            cabbage,
            &char_map,
            &cabbage_bucket,
            &cabbage_summary_suffix_array,
            &summary_suffix_offsets,
        )?;
        let sa = suffix_array.iter().map(|g| g.map(|s| s as i32).unwrap_or(-1)).collect::<Vec<_>>();
        assert_eq!(sa, vec![7, 1, 4, -1, -1, -1, -1, -1]);
        Ok(())
    }
}
