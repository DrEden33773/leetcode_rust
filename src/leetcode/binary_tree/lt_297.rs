use std::cell::RefCell;
use std::rc::Rc;

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

#[allow(unused)]
struct Codec;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    #[allow(dead_code)]
    fn new() -> Self {
        Codec
    }

    #[allow(dead_code)]
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        use std::collections::VecDeque;

        if root.is_none() {
            return vec![];
        }

        let mut ans: Vec<Option<i32>> = vec![];
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if let Some(unwrapped) = node {
                ans.push(Some(unwrapped.borrow().val));
                queue.push_back(unwrapped.borrow().left.clone());
                queue.push_back(unwrapped.borrow().right.clone());
            } else {
                ans.push(None);
            }
        }
        ans
    }

    #[allow(dead_code)]
    fn deserialize(&self, data: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;

        if data.is_empty() {
            return None;
        }

        let root = Some(Rc::new(RefCell::new(TreeNode::new(data[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(root.as_ref().unwrap().clone());

        for children in data[1..].chunks(2) {
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
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data = obj.serialize(strs);
 * let ans = obj.deserialize(data);
 */

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

#[cfg(test)]
mod serialize_deserialize {
    use super::*;

    fn case1() {
        let codec = Codec::new();
        let tree_1 = tree![1, 2, 3, null, null, 4, 5];
        let tree_1_serialized = codec.serialize(tree_1);
        let tree_1_deserialized = codec.deserialize(tree_1_serialized.clone());
        let tree_1_deserialized_serialized = codec.serialize(tree_1_deserialized);
        assert_eq!(tree_1_serialized, tree_1_deserialized_serialized)
    }

    #[test]
    fn it_works() {
        case1();
    }
}
