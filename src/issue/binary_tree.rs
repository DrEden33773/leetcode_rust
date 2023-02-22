use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub struct TreeNode<T: Copy> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

#[allow(dead_code)]
impl<T: Copy> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct BinaryTree<T: Copy> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
    cnt: usize,
}

#[allow(dead_code)]
impl<T: Copy> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None, cnt: 0 }
    }

    fn pre_order_builder(&mut self, seq: &Vec<Option<T>>) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if self.cnt > seq.len() {
            return None;
        }
        if let None = seq[self.cnt] {
            self.cnt += 1;
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(seq[self.cnt].unwrap()))));
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

        let root = Some(Rc::new(RefCell::new(TreeNode::new(seq[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(root.as_ref().unwrap().clone());

        for children in seq[1..].chunks(2) {
            let parent = queue.pop_front().unwrap();
            if let Some(v) = children[0] {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if children.len() <= 1 {
                break;
            }
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }

        root
    }

    pub fn init_by_pre_order(&mut self, seq: Vec<Option<T>>) {
        self.root = self.pre_order_builder(&seq);
    }
    pub fn init_by_level_order(&mut self, seq: Vec<Option<T>>) {
        self.root = self.level_order_builder(&seq);
    }

    pub fn from_pre_order(seq: Vec<Option<T>>) -> BinaryTree<T> {
        let mut binary_tree = BinaryTree::<T>::new();
        binary_tree.init_by_pre_order(seq);
        binary_tree
    }
    pub fn from_level_order(seq: Vec<Option<T>>) -> BinaryTree<T> {
        let mut binary_tree = BinaryTree::<T>::new();
        binary_tree.init_by_level_order(seq);
        binary_tree
    }
}
