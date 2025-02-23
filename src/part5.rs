use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap},
};

use itertools::Itertools as _;

fn is_overlapping(r0: &[i32; 2], r1: &[i32; 2]) -> bool {
    if r0 == r1 {
        true
    } else if r1[0] > r0[0] && r1[0] < r0[1] {
        true
    } else if r0[0] > r1[0] && r0[0] < r1[1] {
        true
    } else if r0[1] > r1[0] && r0[1] < r1[1] {
        true
    } else if r1[1] > r0[0] && r1[1] < r0[1] {
        true
    } else {
        false
    }
}

fn are_ranges_overlapping(ranges: &[&R]) -> bool {
    ranges.iter().enumerate().any(|(i0, r0)| {
        ranges
            .iter()
            .enumerate()
            .filter(|(i1, _)| i0 != *i1)
            .any(|(_, r1)| is_overlapping(r0, r1))
    })
}

pub fn solve_nonoverlapping_intervals_435_adding(ranges: &[[i32; 2]]) -> usize {
    let max = ranges
        .iter()
        .powerset()
        .filter(|ps| !are_ranges_overlapping(ps.as_slice()))
        .max_by_key(|ps| ps.len());

    ranges.len() - max.unwrap().len()
}

pub fn solve_nonoverlapping_intervals_435(ranges: &mut Vec<[i32; 2]>) -> usize {
    ranges.len()
        - solve_nonoverlapping_intervals_435_max(
            ranges,
            &mut Default::default(),
            &mut Default::default(),
        )
}

type R = [i32; 2];

fn solve_nonoverlapping_intervals_435_max(
    ranges: &mut Vec<[i32; 2]>,
    removed: &mut BTreeSet<R>,
    cache: &mut HashMap<BTreeSet<R>, usize>,
) -> usize {
    if let Some(cached) = cache.get(removed) {
        return *cached;
    }
    let overlapping_possible = ranges
        .iter()
        .enumerate()
        .filter(|(i0, r0)| {
            ranges
                .iter()
                .enumerate()
                .filter(|(i1, _)| i0 != i1)
                .any(|(_, r1)| is_overlapping(r0, r1))
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    if overlapping_possible.is_empty() {
        cache.insert(removed.clone(), ranges.len());
        return ranges.len();
    }
    let lengths_when_removed = overlapping_possible.into_iter().map(|pos| {
        // Remove this element
        let was = ranges.remove(pos);
        removed.insert(was);
        let ret = solve_nonoverlapping_intervals_435_max(ranges, removed, cache);
        removed.remove(&was);
        ranges.insert(pos, was);
        ret
    });
    let ret = lengths_when_removed.max().unwrap();

    cache.insert(removed.clone(), ret);

    ret
}

#[derive(Default, Debug, Clone)]
pub struct Trie {
    term: bool,
    children: Vec<Option<Trie>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        self.insert_str(&word);
    }

    fn insert_str(&mut self, word: &str) {
        let mut t = self;
        for c in word.chars() {
            t = t.get_char(c);
        }
        t.term = true
    }

    fn get_char(&mut self, c: char) -> &mut Trie {
        let index = c as u32 - 'a' as u32;
        let index = index as usize;
        self.children
            .resize(self.children.len().max(index + 1), Default::default());
        let slot = &mut self.children[index];
        if slot.is_none() {
            slot.replace(Trie::default());
        }
        slot.as_mut().unwrap()
    }

    fn try_get_char(&self, c: char) -> Option<&Trie> {
        let index = c as u32 - 'a' as u32;
        let index = index as usize;
        self.children.get(index)?.as_ref()
    }

    fn search(&self, word: String) -> bool {
        let mut t = self;
        for c in word.chars() {
            t = if let Some(t) = t.try_get_char(c) {
                t
            } else {
                return false;
            };
        }
        t.term
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut t = self;
        for c in prefix.as_str().chars() {
            t = if let Some(t) = t.try_get_char(c) {
                t
            } else {
                return false;
            };
        }
        true
    }
}

pub fn solve_counting_bits_338(n: i32) -> Vec<i32> {
    let n = n as u32;

    (0..=n)
        .into_iter()
        .map(|n| {
            let bits = (0..32).map(|shift| 1 << shift);
            let set_as_one = bits.filter(|bit| n & bit != 0);
            set_as_one.count()
        })
        .map(|count| count as i32)
        .collect()
}

pub fn solve_unique_paths_62(
    cur_pos: (usize, usize),
    m: usize,
    n: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(cached) = cache.get(&cur_pos) {
        return *cached;
    }

    if cur_pos == (m - 1, n - 1) {
        return 1;
    }

    let directions = [(0, 1), (1, 0)];
    let cells = directions
        .into_iter()
        .map(|d| (cur_pos.0 + d.0, cur_pos.1 + d.1))
        .filter(|p| p.0 < m && p.1 < n);

    let count = cells
        .map(|pos| solve_unique_paths_62(pos, m, n, cache))
        .sum();
    cache.insert(cur_pos, count);

    count
}

pub fn solve_tribonacci_1137(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(ans) = cache.get(&n) {
        return *ans;
    }
    let ans = match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => {
            solve_tribonacci_1137(n - 1, cache)
                + solve_tribonacci_1137(n - 2, cache)
                + solve_tribonacci_1137(n - 3, cache)
        }
    };
    cache.insert(n, ans);
    ans
}

pub fn solve_letter_combinations_17_recurse(
    digits: &str,
    so_far: &mut Vec<char>,
    accum: &mut Vec<String>,
) {
    if digits.is_empty() {
        if !so_far.is_empty() {
            accum.push(so_far.iter().collect());
        }
        return;
    }

    let letters = match digits.chars().next().unwrap() {
        '2' => "abc",
        '3' => "def",
        '4' => "ghi",
        '5' => "jkl",
        '6' => "mno",
        '7' => "pqrs",
        '8' => "tuv",
        '9' => "wxyz",
        _ => "",
    };
    for l in letters.chars() {
        so_far.push(l);
        solve_letter_combinations_17_recurse(&digits[1..], so_far, accum);
        so_far.pop();
    }
}

pub fn solve_letter_combinations_17(digits: &str) -> Vec<String> {
    let mut ret = Vec::new();
    solve_letter_combinations_17_recurse(digits, &mut Vec::new(), &mut ret);
    ret
}

pub fn solve_guess_374(n: i32, guess: impl Fn(i32) -> i32) -> i32 {
    // Binary search.
    let mut min = 1;
    let mut max = n;
    loop {
        let mid = min + (max - min) / 2;
        match guess(mid) {
            -1 => max = mid - 1,
            1 => min = mid + 1,
            0 => return mid,
            _ => unreachable!(),
        }
    }
}

pub fn solve_kth_largest_215(numbers: &[i32], k: usize) -> Option<i32> {
    let mut heap = BinaryHeap::new();
    for n in numbers {
        heap.push(Reverse(n));
        while heap.len() > k {
            heap.pop();
        }
    }
    heap.into_iter().next().map(|r| r.0).copied()
}

#[cfg(test)]
mod tests {

    use std::cmp::Reverse;

    use super::*;

    #[test]
    fn test_kth_largest_215() {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(1));
        heap.push(Reverse(2));
        heap.push(Reverse(3));
        assert_eq!(heap.iter().last(), Some(&Reverse(3)));
        heap.pop();
        assert_eq!(heap.iter().last(), Some(&Reverse(3)));

        assert_eq!(Some(5), solve_kth_largest_215(&[3, 2, 1, 5, 6, 4], 2));
    }

    #[test]
    fn test_guess_number_374() {
        let guess = |answer| {
            move |n: i32| match n.cmp(&answer) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => -1,
            }
        };

        assert_eq!(6, solve_guess_374(10, guess(6)));
        assert_eq!(49, solve_guess_374(100, guess(49)));
        assert_eq!(1702766719, solve_guess_374(2126753390, guess(1702766719)));
    }

    #[test]
    fn test_letter_combinations_17() {
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            solve_letter_combinations_17("23")
        );
    }

    #[test]
    fn test_tribonacci_1137() {
        assert_eq!(4, solve_tribonacci_1137(4, &mut Default::default()));
    }

    #[test]
    fn test_unique_paths_62() {
        assert_eq!(
            28,
            solve_unique_paths_62((0, 0), 3, 7, &mut Default::default())
        );
        assert_eq!(
            4568648125690,
            solve_unique_paths_62((0, 0), 19, 30, &mut Default::default())
        );
    }

    #[test]
    fn test_counting_bits_338() {
        assert_eq!(vec![0, 1, 1, 2, 1, 2], solve_counting_bits_338(5));
    }

    #[test]
    fn test_tri_208() {
        let mut tri = Trie::new();

        tri.insert_str("app");
        tri.insert_str("apple");
        assert_eq!(true, tri.search("app".to_string()));
        assert_eq!(false, tri.search("appl".to_string()));
        assert_eq!(true, tri.search("apple".to_string()));
        tri.insert_str("beer");
        tri.insert_str("add");
        assert_eq!(true, tri.search("app".to_string()));
        tri.insert_str("jam");
        tri.insert_str("rental");

        println!("{tri:?}");

        assert_eq!(false, tri.search("apps".to_string()));
        assert_eq!(true, tri.search("app".to_string()));
    }

    #[test]
    fn test_nonoverlapping_intervals_435() {
        assert_eq!(true, is_overlapping(&[0, 2], &[1, 2]));
        assert_eq!(true, is_overlapping(&[1, 2], &[0, 2]));
        assert_eq!(true, is_overlapping(&[1, 2], &[1, 2]));
        assert_eq!(false, is_overlapping(&[1, 2], &[2, 3]));

        assert_eq!(true, is_overlapping(&[1, 2], &[1, 3]));

        assert_eq!(
            1,
            solve_nonoverlapping_intervals_435(&mut vec![[1, 2], [2, 3], [3, 4], [1, 3]])
        );
        assert_eq!(
            2,
            solve_nonoverlapping_intervals_435(&mut vec![[1, 2], [1, 2], [1, 2]])
        );
        assert_eq!(
            2,
            solve_nonoverlapping_intervals_435(&mut vec![[1, 2], [1, 3], [1, 4]])
        );
        assert_eq!(
            7,
            solve_nonoverlapping_intervals_435_adding(&[
                [-52, 31],
                [-73, -26],
                [82, 97],
                [-65, -11],
                [-62, -49],
                [95, 99],
                [58, 95],
                [-31, 49],
                [66, 98],
                [-63, 2],
                [30, 47],
                [-40, -26]
            ])
        );
        assert_eq!(
            19,
            solve_nonoverlapping_intervals_435_adding(&mut vec![
                [-25322, -4602],
                [-35630, -28832],
                [-33802, 29009],
                [13393, 24550],
                [-10655, 16361],
                [-2835, 10053],
                [-2290, 17156],
                [1236, 14847],
                [-45022, -1296],
                [-34574, -1993],
                [-14129, 15626],
                [3010, 14502],
                [42403, 45946],
                [-22117, 13380],
                [7337, 33635],
                [-38153, 27794],
                [47640, 49108],
                [40578, 46264],
                [-38497, -13790],
                [-7530, 4977],
                [-29009, 43543],
                [-49069, 32526],
                [21409, 43622],
                [-28569, 16493],
                [-28301, 34058]
            ])
        );
    }
}
