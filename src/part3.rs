use std::collections::{HashSet, VecDeque};

fn ll_len(head: &ListNode) -> usize {
    let mut len = 1;
    let mut next = &head.next;
    while let Some(n) = next {
        len += 1;
        next = &n.next;
    }
    len
}

pub fn solve_remove_ll_2095(mut head: Box<ListNode>) -> Option<Box<ListNode>> {
    let len = ll_len(&head);
    if len == 1 {
        return None;
    }
    let len = len / 2 - 1;

    let mut next = Some(&mut head);
    for _ in 0..len {
        next = next.unwrap().next.as_mut();
    }

    next.map(|n| n.next = n.next.as_mut().and_then(|n| n.next.take()));

    Some(head)
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn append(&mut self, val: i32) {
        let mut next = &mut self.next;
        while let Some(n) = next {
            next = &mut n.next
        }

        let new_node = ListNode::new(val);
        next.replace(Box::new(new_node));
    }
    pub fn extend(&mut self, values: impl IntoIterator<Item = i32>) {
        for v in values {
            self.append(v);
        }
    }
    pub fn collect_vec(&self) -> Vec<i32> {
        let mut ret = Vec::new();
        ret.push(self.val);
        let mut next = &self.next;
        while let Some(n) = next {
            ret.push(n.val);
            next = &n.next
        }
        ret
    }
}

pub struct RecentCounter {
    times: VecDeque<i32>,
}
impl RecentCounter {
    pub fn new() -> Self {
        RecentCounter {
            times: Default::default(),
        }
    }
    pub fn ping(&mut self, t: i32) -> i32 {
        self.times.push_back(t);
        let time_ago = t - 3000;
        while self.times.front().filter(|&&v| v < time_ago).is_some() {
            self.times.pop_front();
        }
        self.times.len() as i32
    }
}

pub fn solve_remove_stars_2390(s: &str) -> String {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == '*' {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.iter().collect()
}

pub fn solve_diff_two_arrays_2215(nums1: &[i32], nums2: &[i32]) -> Vec<Vec<i32>> {
    let in_1 = nums1.iter().copied().collect::<HashSet<_>>();
    let in_2 = nums2.iter().copied().collect::<HashSet<_>>();

    let one_not_in_2 = in_1.iter().filter(|v| !in_2.contains(v)).copied();
    let two_not_in_1 = in_2.iter().filter(|v| !in_1.contains(v)).copied();

    vec![one_not_in_2.collect(), two_not_in_1.collect()]
}

pub fn solve_highest_altitude_1732(values: &[i32]) -> i32 {
    let mut accum = 0;
    let altitudes = values.iter().map(|v| {
        accum = accum + v;
        println!("{accum}");
        accum
    });
    [0].into_iter().chain(altitudes).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_altitude_1732() {
        assert_eq!(1, solve_highest_altitude_1732(&[-5, 1, 5, 0, -7]));
        assert_eq!(0, solve_highest_altitude_1732(&[-4, -3, -2, -1, 4, 3, 2]));
    }

    #[test]
    fn test_diff_two_arrays_2215() {
        // let nums1 = [1, 2, 3];
        // let nums2 = [2, 4, 6];
        // assert_eq!(
        //     vec![vec![1, 3].sort(), vec![4, 6].],
        //     solve_diff_two_arrays_2215(&nums1, &nums2)
        // );
    }

    #[test]
    fn test_remove_stars_2390() {
        assert_eq!("lecoe", solve_remove_stars_2390("leet**cod*e"));
        assert_eq!("", solve_remove_stars_2390("erase*****"));
    }

    #[test]
    fn test_recent_counter_933() {
        let mut rc = RecentCounter::new();
        assert_eq!(1, rc.ping(1));
        assert_eq!(2, rc.ping(100));
        assert_eq!(3, rc.ping(3001));
        assert_eq!(3, rc.ping(3002));
    }

    #[test]
    fn test_ll_2095() {
        let orig = [1, 3, 4, 7, 1, 2, 6];
        let mut ll = orig.into_iter();
        let mut head = ListNode::new(ll.next().unwrap());
        head.extend(ll);

        assert_eq!(orig.to_vec(), head.collect_vec());

        assert_eq!(7, ll_len(&head));

        let res = solve_remove_ll_2095(Box::new(head));
        assert_eq!(vec![1, 3, 4, 1, 2, 6], res.unwrap().collect_vec());

        let mut head = ListNode::new(1);
        head.append(2);
        head.append(3);
        head.append(4);

        assert_eq!(vec![1, 2, 3, 4], head.collect_vec());

        let head = Some(Box::new(head));

        let res = solve_remove_ll_2095(head.unwrap());

        assert_eq!(vec![1, 2, 4], res.unwrap().collect_vec());
    }
}
