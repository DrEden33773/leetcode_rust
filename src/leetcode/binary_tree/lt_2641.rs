crate::sln!();

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
  pub fn dfs_twice(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn first_dfs(root: &Option<Rc<RefCell<TreeNode>>>, chunk: &mut Vec<i32>, depth: usize) {
      if let Some(root) = root {
        if chunk.len() <= depth {
          chunk.push(0);
        }
        chunk[depth] += root.borrow().val;
        first_dfs(&root.borrow().left, chunk, depth + 1);
        first_dfs(&root.borrow().right, chunk, depth + 1);
      }
    }

    fn second_dfs(root: &Option<Rc<RefCell<TreeNode>>>, chunk: &[i32], depth: usize) {
      if let Some(root) = root {
        let mut sub = 0;

        if let Some(left) = root.borrow().left.clone() {
          sub += left.borrow().val;
        }
        if let Some(right) = root.borrow().right.clone() {
          sub += right.borrow().val;
        }

        let next_depth = depth + 1;

        if let Some(left) = root.borrow().left.clone() {
          left.borrow_mut().val = chunk[next_depth] - sub;
          second_dfs(&root.borrow().left, chunk, next_depth);
        }
        if let Some(right) = root.borrow().right.clone() {
          right.borrow_mut().val = chunk[next_depth] - sub;
          second_dfs(&root.borrow().right, chunk, next_depth);
        }
      }
    }

    let mut chunk = vec![];
    first_dfs(&root, &mut chunk, 0);
    if let Some(root) = &root {
      root.borrow_mut().val = 0;
    }
    second_dfs(&root, &chunk, 0);
    root
  }

  pub fn bfs_twice(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut queue = std::collections::VecDeque::new();

    if let Some(node) = root.clone() {
      node.borrow_mut().val = 0;
      queue.push_back(node.clone());
    }

    while !queue.is_empty() {
      // temp: curr layer's nodes
      //
      // (do not try to store next_layer's to temp then overwrite queue with it)
      // (it's impossible to pass the borrow checker)

      let temp = queue.clone();
      queue.clear();

      let mut sum = 0;

      for node in temp.iter() {
        if let Some(left) = node.borrow().left.clone() {
          sum += left.borrow().val;
          queue.push_back(left);
        }
        if let Some(right) = node.borrow().right.clone() {
          sum += right.borrow().val;
          queue.push_back(right);
        }
      }

      for node in temp {
        let mut sub = 0;

        if let Some(left) = node.borrow().left.clone() {
          sub += left.borrow().val;
        }
        if let Some(right) = node.borrow().right.clone() {
          sub += right.borrow().val;
        }

        if let Some(left) = node.borrow().left.clone() {
          left.borrow_mut().val = sum - sub;
        }
        if let Some(right) = node.borrow().right.clone() {
          right.borrow_mut().val = sum - sub;
        }
      }
    }

    root
  }

  pub fn replace_value_in_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    Solution::dfs_twice(root)
  }
}
