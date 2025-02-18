use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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
}
