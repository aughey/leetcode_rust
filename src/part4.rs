use std::{cell::RefCell, collections::HashSet, rc::Rc};

fn on_edge(maze_height: usize, maze_width: usize, y: usize, x: usize) -> bool {
    x == 0 || x == maze_width - 1 || y == 0 || y == maze_height - 1
}

pub fn solve_nearest_exit(
    maze: &[impl AsRef<[char]>],
    [y, x]: [usize; 2],
    visited: &mut HashSet<(usize, usize)>,
    min_found: &mut Option<i32>,
) -> Option<i32> {
    if visited.contains(&(y, x)) {
        return None;
    }
    // Early break if we're greater than the min depth already.
    if let Some(min_found) = min_found {
        if visited.len() >= *min_found as usize {
            return None;
        }
    }
    //println!("visiting: {y} {x}");
    if !visited.is_empty() && on_edge(maze.len(), maze[y].as_ref().len(), y, x) {
        //   println!("Found exit: {y} {x}");
        return Some(0);
    }
    visited.insert((y, x));
    let directions: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
    let valid_locations = directions.into_iter().filter_map(|d| {
        let newy = y.checked_add_signed(d.0)?;
        let newx = x.checked_add_signed(d.1)?;
        let newy = usize::try_from(newy).ok()?;
        let newx = usize::try_from(newx).ok()?;
        let cell = maze.get(newy)?.as_ref().get(newx)?;
        if cell == &'.' {
            Some((newy, newx))
        } else {
            None
        }
    });
    let distances = valid_locations.filter_map(|(y, x)| {
        solve_nearest_exit(maze, [y, x], visited, min_found).map(|count| count + 1)
    });
    let min = distances.min();
    match (min, min_found.as_ref().copied()) {
        (Some(min), None) => {
            min_found.replace(min);
        }
        (Some(min), Some(mf)) => {
            if min < mf {
                min_found.replace(min);
            }
        }
        _ => {}
    }
    // Unvisit
    visited.remove(&(y, x));
    min
}

pub fn solve_can_visit_all_rooms_841(rooms: &[impl AsRef<[i32]>]) -> bool {
    let mut keys = rooms[0].as_ref().to_vec();
    let mut visited = HashSet::new();
    visited.insert(0);
    while let Some(key) = keys.pop() {
        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);
        keys.extend_from_slice(rooms[key as usize].as_ref());
    }
    visited.len() == rooms.len()
}

pub fn solve_search_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root) = root.as_ref() {
        match root.borrow().val.cmp(&val) {
            std::cmp::Ordering::Equal => Some(root.clone()),
            std::cmp::Ordering::Less => solve_search_bst(root.borrow().right.clone(), val),
            std::cmp::Ordering::Greater => solve_search_bst(root.borrow().left.clone(), val),
        }
    } else {
        None
    }
}

pub fn solve_right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }
    let mut res = Vec::new();

    let mut breadth = vec![root.unwrap()];

    while !breadth.is_empty() {
        res.push(breadth.last().unwrap().borrow().val);

        let mut next_breadth = Vec::new();

        for node in breadth {
            if let Some(l) = node.borrow().left.as_ref() {
                next_breadth.push(l.clone());
            }
            if let Some(r) = node.borrow().right.as_ref() {
                next_breadth.push(r.clone());
            }
        }
        breadth = next_breadth;
    }
    res
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root) = root {
        let mut root = root.borrow_mut();
        let l = max_depth(root.left.take());
        let r = max_depth(root.right.take());
        1 + l.max(r)
    } else {
        0
    }
}

// Definition for a binary tree node.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn o(tree: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(tree)))
    }

    #[test]
    fn test_max_depth_104() {
        let tree = TreeNode {
            val: 3,
            left: o(TreeNode {
                val: 9,
                ..Default::default()
            }),
            right: o(TreeNode {
                val: 20,
                left: o(TreeNode {
                    val: 15,
                    ..Default::default()
                }),
                right: o(TreeNode {
                    val: 7,
                    ..Default::default()
                }),
            }),
        };
        let tree = o(tree);
        assert_eq!(3, max_depth(tree));
    }

    #[test]
    fn test_right_side_199() {
        let tree = TreeNode {
            val: 1,
            left: o(TreeNode {
                val: 2,
                ..Default::default()
            }),
            right: o(TreeNode {
                val: 3,
                left: None,
                right: o(TreeNode {
                    val: 4,
                    ..Default::default()
                }),
            }),
        };
        let tree = o(tree);
        assert_eq!(vec![1, 3, 4], solve_right_side_view(tree));

        let tree = TreeNode {
            val: 1,
            left: o(TreeNode {
                val: 2,
                left: o(TreeNode {
                    val: 4,
                    left: o(TreeNode {
                        val: 5,
                        ..Default::default()
                    }),
                    right: None,
                }),
                right: None,
            }),
            right: o(TreeNode {
                val: 3,
                left: None,
                right: None,
            }),
        };
        let tree = o(tree);
        assert_eq!(vec![1, 3, 4, 5], solve_right_side_view(tree));
    }

    #[test]
    fn test_solve_search_700() {
        let tree = TreeNode {
            val: 4,
            left: o(TreeNode {
                val: 2,
                left: o(TreeNode {
                    val: 1,
                    ..Default::default()
                }),
                right: o(TreeNode {
                    val: 3,
                    ..Default::default()
                }),
            }),
            right: o(TreeNode {
                val: 7,
                ..Default::default()
            }),
        };
        let tree = o(tree);
        let res = solve_search_bst(tree, 2);
        assert_eq!(2, res.unwrap().borrow().val);
    }

    #[test]
    fn test_keys_and_rooms_841() {
        assert_eq!(
            true,
            solve_can_visit_all_rooms_841(&vec![vec![1], vec![2], vec![3], vec![]])
        );
    }

    #[test]
    fn test_nearest_exit_1926() {
        let maze = [
            ['+', '+', '.', '+'],
            ['.', '.', '.', '+'],
            ['+', '+', '+', '.'],
        ];
        let entrance = [1, 2];

        assert_eq!(
            Some(1),
            solve_nearest_exit(
                &maze,
                [entrance[0], entrance[1]],
                &mut Default::default(),
                &mut None
            )
        );

        let maze = [['+', '+', '+'], ['.', '.', '.'], ['+', '+', '+']];
        let entrance = [1, 0];

        assert_eq!(
            Some(2),
            solve_nearest_exit(
                &maze,
                [entrance[0], entrance[1]],
                &mut Default::default(),
                &mut None
            )
        );

        let maze = [
            ['+', '.', '+', '+', '+', '+', '+'],
            ['+', '.', '+', '.', '.', '.', '+'],
            ['+', '.', '+', '.', '+', '.', '+'],
            ['+', '.', '.', '.', '.', '.', '+'],
            ['+', '+', '+', '+', '.', '+', '.'],
        ];
        let entrance = [0, 1];
        println!("**");

        assert_eq!(
            Some(7),
            solve_nearest_exit(
                &maze,
                [entrance[0], entrance[1]],
                &mut Default::default(),
                &mut None
            )
        );
    }
}
