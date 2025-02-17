use std::{cell::RefCell, iter::from_fn};

/// Is any character in this iterator of "guarenteed" lower case characters a vowel
fn is_vowel_lowercase(c: impl IntoIterator<Item = char>) -> bool {
    const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

    c.into_iter().any(|c| VOWELS.iter().any(move |v| *v == c))
}

/// Returns true if this character is a vowel for all unicode implementations.
fn is_vowel(c: char) -> bool {
    is_vowel_lowercase(c.to_lowercase())
}

pub fn solve_reverse_vowels_345(s: &str) -> String {
    let reverse_str = s.chars().rev();
    let mut reverse_vowels = reverse_str.filter(|c| is_vowel(*c));

    let forward_str = s.chars();
    forward_str
        .map(|c| {
            if is_vowel(c) {
                // Safety: the number of reverse vowels is the same as the forward
                reverse_vowels.next().unwrap()
            } else {
                c
            }
        })
        .collect()
}

pub trait FlowerBedSpace {
    fn occupied(&self) -> bool;
}

impl FlowerBedSpace for &i32 {
    fn occupied(&self) -> bool {
        self == &&1
    }
}

impl FlowerBedSpace for i32 {
    fn occupied(&self) -> bool {
        self == &1
    }
}

impl FlowerBedSpace for bool {
    fn occupied(&self) -> bool {
        *self
    }
}

// Can we plant in this slice if all spaces are unoccupied
fn can_plant_slice<'a>(
    flowerbed: impl IntoIterator<Item = &'a (impl FlowerBedSpace + 'a)>,
) -> bool {
    flowerbed.into_iter().all(|space| !space.occupied())
}

pub fn solve_can_place_flowers(flowerbed: &[impl FlowerBedSpace], n: usize) -> bool {
    let possible_spaces = 0..flowerbed.len();
    let possible_ranges = possible_spaces.map(|space| {
        space.saturating_add_signed(-1)..=space.saturating_add_signed(1).min(flowerbed.len() - 1)
    });
    // Safety: Unwrap safe because our ranges are all between 0 and flowerbed.len()
    let mut possible_slices = possible_ranges.map(|range| flowerbed.get(range).unwrap());

    let mut available_seeds = n;
    while available_seeds > 0 {
        loop {
            if let Some(next_slice) = possible_slices.next() {
                if can_plant_slice(next_slice) {
                    available_seeds -= 1;
                    // Eat the next slice because we cannot plant in this next one
                    _ = possible_slices.next();
                    break;
                }
            } else {
                return false;
            }
        }
    }
    true
}

pub fn solve_kids_candies_1431(candies: &[u32], extra_candies: u32) -> Vec<bool> {
    let previous_greatest = candies.iter().max().copied().unwrap_or(0);

    candies
        .iter()
        .map(|c| (c + extra_candies) >= previous_greatest)
        .collect()
}

fn str_divids(t_in: &str, s: &str) -> bool {
    if t_in.is_empty() {
        return true;
    }

    // Turn t_in into a fn() to get the next cycle of characters
    let t_in = move || t_in.chars();

    let mut t = t_in();

    for c in s.chars() {
        let next_t = if let Some(next_t) = t.next() {
            next_t
        } else {
            t = t_in();
            // Safety: t always has chars, this is safe.
            t.next().unwrap()
        };
        if c != next_t {
            return false;
        }
    }

    t.next().is_none()
}

/// Returns permutations of s where s goes from length 1 to length s.len().
/// Example: s = "abcd", return "a", "ab", "abc", "abcd"
fn grow_string(s: &str) -> impl Iterator<Item = &str> {
    let ranges = 1..=s.len();
    let ranges = ranges.map(|max| 0..max);
    ranges.filter_map(|r| s.get(r))
}

/// Returns permutations of s where s goes from length 1 to length s.len().
/// Example: s = "abcd", return "abcd", "abc", "ab", "a"
fn grow_string_rev(s: &str) -> impl Iterator<Item = &str> {
    let ranges = 1..=s.len();
    let ranges = ranges.rev();
    let ranges = ranges.map(|max| 0..max);
    ranges.filter_map(|r| s.get(r))
}

pub fn solve_gcd_strings_1070<'a>(str1: &'a str, str2: &str) -> Option<&'a str> {
    let mut possible = grow_string(str1);
    let mut biggest = None;
    loop {
        let test = possible.next();
        // If there's another string to test
        if let Some(test) = test {
            // If it passes both, keep track of the biggest.
            if str_divids(test, str1) && str_divids(test, str2) {
                biggest = Some(test);
            }
        } else {
            break;
        }
    }
    biggest
}

pub fn solve_gcd_strings_1070_rev<'a>(str1: &'a str, str2: &'a str) -> Option<&'a str> {
    let shortest = if str1.len() < str2.len() { str1 } else { str2 };
    let mut possible = grow_string_rev(shortest);
    loop {
        let test = possible.next();
        // If there's another string to test
        if let Some(test) = test {
            // If it passes both, keep track of the biggest.
            if str_divids(test, str1) && str_divids(test, str2) {
                return Some(test);
            }
        } else {
            break;
        }
    }
    None
}

pub fn solve_merge_strings_1768(word1: &str, word2: &str) -> String {
    let word1 = word1.chars();
    let word2 = word2.chars();
    let words = [RefCell::new(word1), RefCell::new(word2)];
    let mut words = words.iter().cycle();
    // Safety: unwrap will always succeed because of the cycle.
    let mut words = move || words.next().unwrap().borrow_mut();

    let interleaved = from_fn(move || {
        // Get the next character from the next word.
        let value = words().next().clone();
        // If there isn't a character from that word
        if let Some(value) = value {
            Some(value)
        } else {
            // Get the next character from the other word.
            words().next().clone()
        }
    });

    interleaved.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!("Hello, world!".to_string().as_str(), "Hello, world!");
    }

    // Example 1:

    // Input: word1 = "abc", word2 = "pqr"
    // Output: "apbqcr"
    // Explanation: The merged string will be merged as so:
    // word1:  a   b   c
    // word2:    p   q   r
    // merged: a p b q c r
    // Example 2:

    // Input: word1 = "ab", word2 = "pqrs"
    // Output: "apbqrs"
    // Explanation: Notice that as word2 is longer, "rs" is appended to the end.
    // word1:  a   b
    // word2:    p   q   r   s
    // merged: a p b q   r   s
    // Example 3:

    // Input: word1 = "abcd", word2 = "pq"
    // Output: "apbqcd"
    // Explanation: Notice that as word1 is longer, "cd" is appended to the end.
    // word1:  a   b   c   d
    // word2:    p   q
    // merged: a p b q c   d
    #[test]
    fn test_merge_strings_1768() {
        let word1 = "abc";
        let word2 = "pqr";
        let expected = "apbqcr";

        assert_eq!(expected, solve_merge_strings_1768(word1, word2).as_str());
        assert_eq!("apbqrs", solve_merge_strings_1768("ab", "pqrs"));
        assert_eq!("apbqcd", solve_merge_strings_1768("abcd", "pq"));
    }

    #[test]
    fn test_gcd_strings_1071() {
        assert!(str_divids("ABC", "ABCABC"));
        assert!(!str_divids("ABC", "ABCABCA"));
        assert!(!str_divids("ABCA", "ABCABC"));

        assert_eq!(
            vec!["a", "ab", "abc", "abcd"],
            grow_string("abcd").collect::<Vec<_>>(),
        );
        assert_eq!(
            vec!["abcd", "abc", "ab", "a"],
            grow_string_rev("abcd").collect::<Vec<_>>(),
        );

        assert_eq!(Some("ABC"), solve_gcd_strings_1070("ABCABC", "ABC"));
        assert_eq!(Some("AB"), solve_gcd_strings_1070("ABABAB", "ABAB"));
        assert_eq!(None, solve_gcd_strings_1070("LEET", "CODE"));

        assert_eq!(Some("ABC"), solve_gcd_strings_1070_rev("ABCABC", "ABC"));
        assert_eq!(Some("AB"), solve_gcd_strings_1070_rev("ABABAB", "ABAB"));
        assert_eq!(None, solve_gcd_strings_1070_rev("LEET", "CODE"));
    }

    #[test]
    fn test_kids_candies_1431() {
        assert_eq!(
            vec![true, true, true, false, true],
            solve_kids_candies_1431(&[2, 3, 5, 1, 3], 3)
        );
        assert_eq!(
            vec![true, false, false, false, false],
            solve_kids_candies_1431(&[4, 2, 1, 1, 2], 1)
        );
        assert_eq!(
            vec![true, false, true],
            solve_kids_candies_1431(&[12, 1, 12], 10)
        );
    }

    #[test]
    fn test_can_place_flowers_605() {
        assert!(!can_plant_slice(&[1, 0, 0]));
        assert!(!can_plant_slice(&[0, 1, 0]));
        assert!(!can_plant_slice(&[0, 0, 1]));
        assert!(can_plant_slice(&[0, 0, 0]));
        assert!(can_plant_slice(&[0, 0]));
        assert!(can_plant_slice(&[0]));

        assert_eq!(true, solve_can_place_flowers(&[1, 0, 0, 0, 1], 1));
        assert_eq!(false, solve_can_place_flowers(&[1, 0, 0, 0, 1], 2));
    }

    #[test]
    fn test_reverse_vowels_345() {
        assert!(is_vowel('a'));
        assert!(is_vowel('e'));
        assert!(is_vowel('i'));
        assert!(is_vowel('o'));
        assert!(is_vowel('u'));
        assert!(is_vowel('U'));
        assert!(!is_vowel('j'));

        assert_eq!("AceCreIm", solve_reverse_vowels_345("IceCreAm").as_str());
        assert_eq!("leotcede", solve_reverse_vowels_345("leetcode").as_str());
    }
}
