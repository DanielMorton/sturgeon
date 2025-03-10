use crate::bwt::bucket::{find_bucket_heads, find_bucket_tails};
use std::error::Error;

pub const L: u8 = b'L';
pub const S: u8 = b'S';

pub(crate) fn build_type_map<T: Eq + Ord>(data: &[T]) -> Result<Vec<u8>, Box<dyn Error>> {
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
            } else if data[i] == data[i + 1] && type_map[i + 1] == L {
                type_map[i] = L;
            } else {
                type_map[i] = S;
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

pub(crate) fn guess_lms_sort<T: Copy + Into<usize>>(
    text_bytes: &[T],
    bucket_sizes: &[usize],
    type_map: &[u8],
) -> Result<Vec<i32>, Box<dyn Error>> {
    let n = text_bytes.len();
    let mut guessed_suffix_array = vec![-1; n + 1];

    let mut bucket_tails = find_bucket_tails(bucket_sizes)?;
    for i in 0..n {
        if is_lms_char(type_map, i)? {
            let bucket_index = text_bytes[i].into();
            guessed_suffix_array[bucket_tails[bucket_index]] = i as i32;
            bucket_tails[bucket_index] -= 1;
        }
    }
    guessed_suffix_array[0] = n as i32;

    Ok(guessed_suffix_array)
}

pub(crate) fn induce_sort_l<T: Copy + Into<usize>>(
    guessed_suffix_array: &mut [i32],
    text_bytes: &[T],
    bucket_sizes: &[usize],
    type_map: &[u8],
) -> Result<(), Box<dyn Error>> {
    let mut bucket_heads = find_bucket_heads(bucket_sizes)?;
    let n = text_bytes.len();
    for i in 0..=n {
        if guessed_suffix_array[i] >= 0 {
            let j = guessed_suffix_array[i] - 1;
            if j >= 0 && type_map[j as usize] == L {
                let bucket_index = text_bytes[j as usize].into();
                guessed_suffix_array[bucket_heads[bucket_index]] = j;
                bucket_heads[bucket_index] += 1
            }
        }
    }
    Ok(())
}

pub(crate) fn induce_sort_s<T: Copy + Into<usize>>(
    guessed_suffix_array: &mut [i32],
    text_bytes: &[T],
    bucket_sizes: &[usize],
    type_map: &[u8],
) -> Result<(), Box<dyn Error>> {
    let mut bucket_tails = find_bucket_tails(bucket_sizes)?;
    let n = text_bytes.len();

    for i in (0..=n).rev() {
        let j = guessed_suffix_array[i] - 1;
        if j >= 0 && type_map[j as usize] == S {
            let bucket_index = text_bytes[j as usize].into();
            guessed_suffix_array[bucket_tails[bucket_index]] = j;
            bucket_tails[bucket_index] -= 1;
        }
    }
    Ok(())
}

pub(crate) fn accurate_lms_sort<T: Copy + Into<usize>>(
    text_bytes: &[T],
    bucket_sizes: &[usize],
    summary_suffix_array: &[usize],
    summary_suffix_offsets: &[usize],
) -> Result<Vec<usize>, Box<dyn Error>> {
    let n = text_bytes.len();
    // A suffix for every character, plus the empty suffix
    let mut suffix_offsets = vec![usize::MAX; n + 1];

    // Find bucket tails for placing suffixes
    let mut bucket_tails = find_bucket_tails(bucket_sizes)?;

    // Iterate through summary suffix array in reverse order
    for &suffix_index in summary_suffix_array.iter().rev().skip(1) {
        if suffix_index > 1 {
            let string_index = summary_suffix_offsets[suffix_index];

            // Which bucket does this suffix go into?
            let bucket_index = text_bytes[string_index].into();

            // Add the suffix at the tail of the bucket
            suffix_offsets[bucket_tails[bucket_index]] = string_index;

            // Move the tail pointer down
            bucket_tails[bucket_index] -= 1;
        }
    }

    // Always include the empty suffix at the beginning
    suffix_offsets[0] = n;

    Ok(suffix_offsets)
}


#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::bwt::lms::{build_type_map, L, S};

    #[test]
    fn test_build_type_map1() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            build_type_map(
                "cabbage".as_bytes()
            )?,
            vec![L, S, L, L,S, L,L, S]
        );
        Ok(())
    }
}