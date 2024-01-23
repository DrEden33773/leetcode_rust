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

impl Solution {
  pub fn official_remove_nth_from_end(
    head: Option<Box<ListNode>>,
    n: i32,
  ) -> Option<Box<ListNode>> {
    unsafe {
      let mut dummy = Box::new(ListNode { val: 0, next: head });
      let mut slow = &mut dummy as *mut Box<ListNode>;
      let mut fast = &mut dummy as *mut Box<ListNode>;
      for _ in 0..n {
        fast = (*fast).next.as_mut().unwrap();
      }
      while (*fast).next.is_some() {
        fast = (*fast).next.as_mut().unwrap();
        slow = (*slow).next.as_mut().unwrap();
      }
      (*slow).next = (*slow).next.take().unwrap().next;
      dummy.next
    }
  }
  pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    unsafe {
      let dummy = &mut ListNode { val: 0, next: head } as *mut ListNode;
      let mut slow = dummy;
      let mut fast = dummy;
      for _ in 0..n {
        fast = (*fast).next.as_mut().unwrap().as_mut();
      }
      while (*fast).next.is_some() {
        fast = (*fast).next.as_mut().unwrap().as_mut();
        slow = (*slow).next.as_mut().unwrap().as_mut();
      }
      (*slow).next = (*slow).next.take().unwrap().next;
      (*dummy).next.to_owned()
    }
  }
}
