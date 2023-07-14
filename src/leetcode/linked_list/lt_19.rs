crate::sln!();

// Definition for singly-linked list.
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

type NullableListNode = Option<Box<ListNode>>;

impl Solution {
    pub fn official_remove_nth_from_end(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        unsafe {
            let mut slow = &mut dummy as *mut Box<ListNode>;
            let mut fast = &mut dummy as *mut Box<ListNode>;
            // move fast n forward
            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap();
            }

            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap();
                slow = (*slow).next.as_mut().unwrap();
            }
            (*slow).next = (*slow).next.take().unwrap().next;
        }

        dummy.next
    }
}
