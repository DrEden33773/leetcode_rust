crate::sln!();

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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut mv = 0;

    fn dfs(mv: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      match root {
        Some(root) => {
          let mv_left = dfs(mv, root.borrow().left.to_owned());
          let mv_right = dfs(mv, root.borrow().right.to_owned());
          *mv += mv_left.abs() + mv_right.abs();
          mv_left + mv_right + root.borrow().val - 1
        }
        None => 0,
      }
    }

    dfs(&mut mv, root);
    mv
  }
}
