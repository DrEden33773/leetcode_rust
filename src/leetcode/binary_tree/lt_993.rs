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
  pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    let x = x as usize;
    let y = y as usize;

    #[derive(Default, Clone, Copy)]
    struct Attribute {
      parent: usize,
      depth: usize,
    }

    fn dfs(
      root: &Option<Rc<RefCell<TreeNode>>>,
      depth: usize,
      parent: usize,
      attr: &mut [Attribute],
    ) {
      if let Some(node) = root {
        let node = node.borrow();
        let node_val = node.val as usize;
        attr[node_val].depth = depth;
        attr[node_val].parent = parent;
        dfs(&node.left, depth + 1, node_val, attr);
        dfs(&node.right, depth + 1, node_val, attr);
      }
    }

    let mut attr = [Attribute::default(); 101];
    dfs(&root, 0, 0, &mut attr);
    attr[x].depth == attr[y].depth && attr[x].parent != attr[y].parent
  }
}
