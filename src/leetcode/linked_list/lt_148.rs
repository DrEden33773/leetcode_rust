crate::sln!();

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
  #[inline]
  fn o_n_space(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut buf = vec![];
    // 1. load into buf
    let mut head = head;
    while let Some(node) = head {
      buf.push(node.val);
      head = node.next;
    }
    // 2. sort data in buf
    buf.sort_unstable();
    // 3. build list from buf
    // (head-insertion requires a reverse iteration of buf)
    let mut head = None;
    for val in buf.into_iter().rev() {
      let mut node = ListNode::new(val);
      node.next = head;
      head = Some(Box::new(node));
    }
    head
  }

  #[inline]
  pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Self::o_n_space(head)
  }
}
