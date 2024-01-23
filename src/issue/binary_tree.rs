use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TreeNode<T: Clone> {
  pub val: T,
  pub left: Option<Rc<RefCell<TreeNode<T>>>>,
  pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

#[allow(dead_code)]
impl<T: Clone> TreeNode<T> {
  pub fn new(val: T) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BinaryTree<T: Clone> {
  root: Option<Rc<RefCell<TreeNode<T>>>>,
  cnt: usize,
}

#[allow(dead_code)]
impl<T: Clone> BinaryTree<T> {
  pub fn new() -> Self {
    BinaryTree { root: None, cnt: 0 }
  }

  fn pre_order_builder(&mut self, seq: &Vec<Option<T>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
    if self.cnt >= seq.len() {
      return None;
    }
    if seq[self.cnt].is_none() {
      self.cnt += 1;
      return None;
    }
    let root = Some(Rc::new(RefCell::new(TreeNode::new(
      seq[self.cnt].as_ref().unwrap().to_owned(),
    ))));
    self.cnt += 1;
    root.as_ref().unwrap().borrow_mut().left = self.pre_order_builder(seq);
    root.as_ref().unwrap().borrow_mut().right = self.pre_order_builder(seq);
    root
  }
  fn level_order_builder(&mut self, seq: &Vec<Option<T>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
    if seq.is_empty() {
      return None;
    }
    use std::collections::VecDeque;
    let root = Some(Rc::new(RefCell::new(TreeNode::new(
      seq[0].as_ref().unwrap().to_owned(),
    ))));
    let mut queue = VecDeque::new();
    queue.push_back(root.as_ref().unwrap().clone());
    for children in seq[1..].chunks(2) {
      let parent = queue.pop_front().unwrap();
      if let Some(v) = children[0].as_ref() {
        parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v.to_owned()))));
        queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
      }
      if children.len() <= 1 {
        break;
      }
      if let Some(v) = children[1].as_ref() {
        parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v.to_owned()))));
        queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
      }
    }
    root
  }

  pub fn init_by_pre_order(&mut self, seq: Vec<Option<T>>) {
    self.root = self.pre_order_builder(&seq);
    self.cnt = 0;
  }
  pub fn init_by_level_order(&mut self, seq: Vec<Option<T>>) {
    self.root = self.level_order_builder(&seq);
  }

  pub fn from_pre_order(seq: Vec<Option<T>>) -> BinaryTree<T> {
    let mut binary_tree = BinaryTree::new();
    binary_tree.init_by_pre_order(seq);
    binary_tree
  }
  pub fn from_level_order(seq: Vec<Option<T>>) -> BinaryTree<T> {
    let mut binary_tree = BinaryTree::new();
    binary_tree.init_by_level_order(seq);
    binary_tree
  }

  pub fn to_pre_order_seq_iterative(&self) -> Vec<Option<T>> {
    use std::collections::VecDeque;
    let mut seq: Vec<Option<T>> = vec![];
    let mut stack: VecDeque<Option<Rc<RefCell<TreeNode<T>>>>> = VecDeque::new();
    let mut node = self.root.clone();
    while node.is_some() || !stack.is_empty() {
      // iterate
      while let Some(curr) = node.clone() {
        seq.push(Some(curr.borrow().val.clone()));
        stack.push_back(Some(curr.clone()));
        node = curr.borrow().left.clone();
      }
      // node == None
      seq.push(None);
      // trace back
      if let Some(curr) = stack.pop_back().unwrap() {
        node = curr.borrow().right.clone();
      }
    }
    // The last `node` of tree is always `None`, and will never being visited.
    // So we need to add it manually.
    seq.push(None);
    seq
  }
  pub fn to_pre_order_seq(&self) -> Vec<Option<T>> {
    fn recursive_func<T: Clone>(node: Option<Rc<RefCell<TreeNode<T>>>>, seq: &mut Vec<Option<T>>) {
      if let Some(curr) = node {
        seq.push(Some(curr.borrow().val.clone()));
        recursive_func(curr.borrow().left.clone(), seq);
        recursive_func(curr.borrow().right.clone(), seq);
      } else {
        // must add `else`, or non_null node will be treated as null
        // ( after being treated as non_null normally )
        seq.push(None);
      }
    }
    let mut seq: Vec<Option<T>> = vec![];
    recursive_func(self.root.clone(), &mut seq);
    seq
  }
  pub fn to_level_order_seq(&self) -> Vec<Option<T>> {
    use std::collections::VecDeque;
    let mut seq: Vec<Option<T>> = vec![];
    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode<T>>>>> = VecDeque::new();
    queue.push_back(self.root.clone());
    while !queue.is_empty() {
      if let Some(node) = queue.pop_front().unwrap() {
        seq.push(Some(node.borrow().val.clone()));
        queue.push_back(node.borrow().left.clone());
        queue.push_back(node.borrow().right.clone());
      } else {
        seq.push(None);
      }
    }
    seq
  }
}

#[allow(dead_code)]
impl<T: PartialEq + Clone> PartialEq for BinaryTree<T> {
  fn eq(&self, other: &Self) -> bool {
    self.to_level_order_seq() == other.to_level_order_seq()
  }
}

#[allow(dead_code)]
impl<T: Clone> BinaryTree<T> {
  pub fn get_height(&self) -> usize {
    if self.root.as_ref().is_none() {
      return 0;
    }
    let mut height = 0;
    use std::collections::VecDeque;
    let mut node = self.root.as_ref().unwrap().clone();
    let mut queue = VecDeque::new();
    queue.push_back(node.clone());
    while !queue.is_empty() {
      let len = queue.len();
      for _ in 0..len {
        node = queue.pop_front().unwrap();
        if let Some(left) = node.borrow().left.as_ref() {
          queue.push_back(left.to_owned());
        }
        if let Some(right) = node.borrow().right.as_ref() {
          queue.push_back(right.to_owned());
        }
      }
      height += 1;
    }
    height
  }
}

#[allow(dead_code)]
impl<T: Display + Clone> BinaryTree<T> {
  pub fn print_in_layer(&self) {
    if self.root.as_ref().is_none() {
      println!("Empty...");
      return;
    }
    use std::collections::VecDeque;
    let mut node = self.root.as_ref().unwrap().clone();
    let mut queue = VecDeque::new();
    queue.push_back(node.clone());
    while !queue.is_empty() {
      let len = queue.len();
      for _ in 0..len {
        node = queue.pop_front().unwrap();
        println!("{} ", node.borrow().val);
        if let Some(left) = node.borrow().left.as_ref() {
          queue.push_back(left.to_owned());
        }
        if let Some(right) = node.borrow().right.as_ref() {
          queue.push_back(right.to_owned());
        }
      }
      println!();
    }
  }
}

#[cfg(test)]
mod the_binary_tree {
  use super::*;

  #[test]
  fn pre_order_serialize_deserialize() {
    let seq = vec![
      Some(1),
      Some(2),
      Some(4),
      None,
      None,
      None,
      Some(3),
      Some(6),
      None,
      None,
      Some(7),
      None,
      None,
    ];
    let seq_b = vec![
      Some(1),
      Some(2),
      Some(4),
      None,
      None,
      None,
      Some(3),
      Some(6),
      None,
      None,
      Some(7),
    ];
    let tree = BinaryTree::from_pre_order(seq);
    let tree_b = BinaryTree::from_pre_order(seq_b);
    assert_eq!(tree, tree_b);
    let iteratively_serialized = tree.to_pre_order_seq_iterative();
    let deserialized_tree = BinaryTree::from_pre_order(iteratively_serialized);
    assert_eq!(tree, deserialized_tree);
  }

  #[test]
  fn level_order_serialize_deserialize() {
    let seq = vec![Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7)];
    let seq_b = vec![
      Some(1),
      Some(2),
      Some(3),
      Some(4),
      None,
      Some(6),
      Some(7),
      None,
      None,
      None,
      None,
    ];
    let tree = BinaryTree::from_level_order(seq);
    let tree_b = BinaryTree::from_level_order(seq_b);
    assert_eq!(tree, tree_b);
    let serialized = tree.to_level_order_seq();
    let deserialized_tree = BinaryTree::from_level_order(serialized);
    assert_eq!(tree, deserialized_tree);
  }

  #[test]
  fn get_height_test() {
    let seq = vec![Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7)];
    let tree = BinaryTree::from_level_order(seq);
    let expected_height = 3;
    let calculated_height = tree.get_height();
    assert_eq!(expected_height, calculated_height);
  }

  #[test]
  fn clone_tree_test() {
    let seq = vec![Some(1), Some(2), Some(3), Some(4), None, Some(6), Some(7)];
    let tree = BinaryTree::from_level_order(seq);
    assert_eq!(tree, tree.clone());
  }
}
