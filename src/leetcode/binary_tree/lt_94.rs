use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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

pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;

        while node.is_some() || stack.len() > 0 {
            while let Some(curr) = node {
                node = curr.borrow_mut().left.take();
                stack.push(curr);
            }
            if let Some(curr) = stack.pop() {
                ans.push(curr.borrow().val);
                node = curr.borrow_mut().right.take();
            }
        }
        ans
    }
}

// https://leetcode.cn/problems/binary-tree-inorder-traversal/solution/er-cha-shu-by-custerfun-zxm3/
#[allow(dead_code)]
pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;

    if vec.is_empty() {
        return None;
    }

    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

// https://leetcode.cn/problems/binary-tree-inorder-traversal/solution/er-cha-shu-by-custerfun-zxm3/
#[allow(unused)]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}

// https://leetcode.cn/problems/binary-tree-inorder-traversal/solution/er-cha-shu-by-custerfun-zxm3/
#[cfg(test)]
mod inorder_traversal {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::inorder_traversal(tree![1, null, 2, 3]),
            vec![1, 3, 2]
        );
        assert_eq!(Solution::inorder_traversal(tree![]), vec![]);
        assert_eq!(Solution::inorder_traversal(tree![1]), vec![1]);
    }
}
