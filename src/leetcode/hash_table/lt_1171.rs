#![allow(dead_code)]

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = vec![];
        let mut cur = head;
        while let Some(node) = cur {
            vec.push(node.val);
            cur = &node.next;
        }
        vec
    }
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let vec = Solution::to_vec(&head);
        let mut sum = 0;
        let mut map = std::collections::HashMap::new();
        for (i, &e) in vec.iter().enumerate() {
            sum += e;
            if sum == 0 {
                return Solution::remove_zero_sum_sublists(head.unwrap().next);
            }
            if map.contains_key(&sum) {
                let mut head = head;
                let mut cur = head.as_mut().unwrap();
                let mut j = map[&sum] + 1;
                while j <= i {
                    cur = cur.next.as_mut().unwrap();
                    j += 1;
                }
                cur.next = cur.next.as_mut().unwrap().next.take();
                return Solution::remove_zero_sum_sublists(head);
            }
            map.insert(sum, i);
        }
        unimplemented!()
    }
}
