use std::{cell::RefCell, rc::Rc};

fn o(tree: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(tree)))
}

pub fn solve_right_side_view(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
}
