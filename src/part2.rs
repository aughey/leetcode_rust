use std::{cell::RefCell, rc::Rc};

pub fn solve_max_avg_subarray_643(values: &[i32], k: usize) -> f64 {
    let ranges = 0..=values.len() - k;
    let mut ranges = ranges.map(|start| start..start + k);
    // Unwrap is safe because our ranges are calculated correctly

    let mut prev_sum = {
        ranges
            .by_ref()
            .next()
            .map(|r| values.get(r).unwrap())
            .map(|v| v.iter().sum::<i32>())
            .unwrap_or(0)
    };
    let mut max = prev_sum as f64 / k as f64;

    for r in ranges {
        // just subtract the previous and add the next
        prev_sum = prev_sum - values[r.start - 1] + values[r.end - 1];
        let this_avg = prev_sum as f64 / k as f64;
        if this_avg > max {
            max = this_avg
        }
    }
    max
}

pub fn solve_max_avg_subarray_643_slower(values: &[i32], k: usize) -> f64 {
    let ranges = 0..=values.len() - k;
    let ranges = ranges.map(|start| start..start + k);
    // Unwrap is safe because our ranges are calculated correctly
    let values = ranges.map(|range| values.get(range).unwrap());
    let averages = values.map(|v| v.iter().sum::<i32>() as f64 / v.len() as f64);
    averages
        .max_by(|a, b| {
            if a > b {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        })
        .unwrap_or(0.0)
}

pub fn solve_is_subsequence_392_iterator(
    mut s: impl Iterator<Item = char>,
    mut t: impl Iterator<Item = char>,
) -> bool {
    loop {
        let nexts = if let Some(nexts) = s.next() {
            nexts
        } else {
            // Empty good
            return true;
        };

        // Find the position of the next character we're looking for
        if t.by_ref().position(|v| v == nexts).is_none() {
            // Doesn't exist, isn't a subsequence
            return false;
        };
    }
}

pub fn solve_is_subsequence_392(mut s: impl Iterator<Item = char>, mut t: &[char]) -> bool {
    loop {
        let nexts = if let Some(nexts) = s.next() {
            nexts
        } else {
            // Empty good
            return true;
        };

        // Find the position of the next character we're looking for
        let pos = if let Some(pos) = t.iter().position(|v| *v == nexts) {
            pos
        } else {
            // Doesn't exist, isn't a subsequence
            return false;
        };

        t = &t[pos..];
    }
}

pub fn solve_move_zeros_283(values: &mut [i32]) -> &[i32] {
    let values = Rc::new(RefCell::new(values));
    let mut next_value = {
        let mut index = 0;
        let values = values.clone();
        move || loop {
            let values = values.borrow();
            if let Some(v) = values.get(index) {
                index += 1;
                if *v != 0 {
                    return *v;
                }
            } else {
                return 0;
            }
        }
    };

    let end = values.borrow().len();
    for i in 0..end {
        let value = next_value();
        values.borrow_mut()[i] = value;
    }
    values.take()
}

pub fn solve_move_zeros_283_non_borrow(values: &mut [i32]) -> &[i32] {
    let mut read_index = 0;
    for i in 0..values.len() {
        // Get the next value from the read_index
        assert!(read_index >= i);
        let new_value = loop {
            if let Some(value) = values.get(read_index) {
                read_index += 1;
                if value != &0 {
                    break *value;
                }
            } else {
                break 0;
            }
        };
        values[i] = new_value;
    }
    values
}

pub fn solve_string_compression_443(mut input: &[char]) -> Vec<char> {
    let mut ret = Vec::new();
    while !input.is_empty() {
        let c = input[0];
        let count = input.iter().take_while(|v| **v == c).count();
        input = &input[count..];

        ret.push(c);
        if count > 1 {
            ret.extend(format!("{count}").chars());
        }
    }
    ret
}

pub fn solve_increasing_triplet_334_slow(values: &[i32]) -> bool {
    for i in 0..values.len() {
        for j in i..values.len() {
            for k in j..values.len() {
                if values[i] < values[j] && values[j] < values[k] {
                    return true;
                }
            }
        }
    }

    false
}

pub fn solve_reverse_words_151(s: &str) -> String {
    let words = s.trim().split_whitespace().rev();
    let words = words.flat_map(|word| [" ", word]);
    words.skip(1).collect()
}

pub fn solve_product_array_except_self_238(values: &[i32]) -> Vec<i32> {
    let mut post = Vec::new();
    let mut accum = 1;
    for value in values.iter().rev().copied() {
        accum *= value;
        post.push(accum);
    }

    let mut prefix = 1;
    let mut ret = Vec::new();
    let mut postfix = post.into_iter().rev().skip(1);
    for value in values.iter() {
        ret.push(prefix * postfix.next().unwrap_or(1));
        prefix *= value;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words_151() {
        assert_eq!(
            "blue is sky the",
            solve_reverse_words_151("the sky is blue").as_str()
        );
        assert_eq!(
            "example good a",
            solve_reverse_words_151("a good   example").as_str()
        );
    }

    #[test]
    fn test_product_array_except_self_238() {
        assert_eq!(
            vec![24, 12, 8, 6],
            solve_product_array_except_self_238(&[1, 2, 3, 4])
        );
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            solve_product_array_except_self_238(&[-1, 1, 0, -3, 3])
        );
    }

    #[test]
    fn test_increasing_triplet_334() {
        assert_eq!(true, solve_increasing_triplet_334_slow(&[1, 2, 3, 4, 5]));
        assert_eq!(false, solve_increasing_triplet_334_slow(&[5, 4, 3, 2, 1]));
        assert_eq!(true, solve_increasing_triplet_334_slow(&[2, 1, 5, 0, 4, 6]));
    }

    #[test]
    fn test_string_compression() {
        assert_eq!(
            vec!['a', '2', 'b', '2', 'c', '3'],
            solve_string_compression_443(&['a', 'a', 'b', 'b', 'c', 'c', 'c'])
        );
        assert_eq!(
            vec!['a', 'b', '1', '2'],
            solve_string_compression_443(&[
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
            ])
        );
    }

    #[test]
    fn test_move_zeros_283() {
        let mut input = vec![0, 1, 0, 3, 12];
        assert_eq!(
            &[1, 3, 12, 0, 0],
            solve_move_zeros_283(input.as_mut_slice())
        );

        let mut input = vec![0, 1, 0, 3, 12];
        assert_eq!(
            &[1, 3, 12, 0, 0],
            solve_move_zeros_283_non_borrow(input.as_mut_slice())
        );
    }

    #[test]
    fn test_is_subsequence_392() {
        assert_eq!(
            true,
            solve_is_subsequence_392_iterator("abc".chars(), "ahbgdc".chars())
        );
        assert_eq!(
            false,
            solve_is_subsequence_392_iterator("axc".chars(), "ahbgdc".chars())
        );
    }

    #[test]
    fn test_max_avg_subarray_643() {
        assert!(
            (12.75000 - solve_max_avg_subarray_643(&[1, 12, -5, -6, 50, 3], 4)).abs() < 0.00001
        );
        assert!(
            (5.0 - solve_max_avg_subarray_643(&[5], 1)).abs() < 0.00001,
            "{}",
            solve_max_avg_subarray_643(&[5], 1)
        );
    }
}
