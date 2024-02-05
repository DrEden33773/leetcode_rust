use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
  pub val: i32,
  pub next: Option<Rc<RefCell<Node>>>,
  pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
  #[inline]
  pub fn new(val: i32) -> Node {
    Node {
      val,
      next: None,
      random: None,
    }
  }
}

#[derive(Default)]
pub struct Solution {
  cached_node: HashMap<*mut Node, Rc<RefCell<Node>>>,
}

impl Solution {
  pub fn copy_random_list(
    &mut self,
    head: Option<&Rc<RefCell<Node>>>,
  ) -> Option<Rc<RefCell<Node>>> {
    match head {
      None => None,
      Some(old_node) => {
        if let Some(node) = self.cached_node.get(&(old_node.as_ptr())) {
          Some(node.clone())
        } else {
          let node = Rc::new(RefCell::new(Node::new(old_node.borrow().val)));
          self.cached_node.insert(old_node.as_ptr(), node.clone());
          node.borrow_mut().next = self.copy_random_list(old_node.borrow().next.as_ref());
          node.borrow_mut().random = self.copy_random_list(old_node.borrow().random.as_ref());
          Some(node)
        }
      }
    }
  }
}
