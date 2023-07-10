use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
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
struct BSTIterator {
    index: usize,
    ordered: Vec<i32>,
}

impl BSTIterator {
    fn collect(root: Option<Rc<RefCell<TreeNode>>>, into: &mut Vec<i32>) {
        if let Some(root) = root {
            Self::collect(root.borrow_mut().left.take(), into);
            into.push(root.borrow().val);
            Self::collect(root.borrow_mut().right.take(), into);
        }
    }

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut ordered = vec![];
        Self::collect(root, &mut ordered);
        Self { index: 0, ordered }
    }

    fn next(&mut self) -> i32 {
        if self.index < self.ordered.len() {
            let res = self.ordered[self.index];
            self.index += 1;
            res
        } else {
            self.ordered.last().unwrap().to_owned()
        }
    }

    fn has_next(&self) -> bool {
        self.index < self.ordered.len()
    }
}
