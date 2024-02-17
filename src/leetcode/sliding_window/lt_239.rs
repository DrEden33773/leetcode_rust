crate::sln!();

impl Solution {
  fn binary_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::BinaryHeap;

    #[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct NumIdxPair((i32, usize));

    let k = k as usize;
    let mut heap = BinaryHeap::new();

    // 1. use `nums[..k]` to build the heap
    for (idx, &num) in nums[..k].iter().enumerate() {
      heap.push(NumIdxPair((num, idx)));
    }

    // 2. initialize `ans`
    let mut ans = vec![heap.peek().unwrap().0 .0];

    // 3. start sliding the window
    for (idx, &num) in nums.iter().enumerate().skip(k) {
      let left = idx - k + 1;
      // new element comes in
      heap.push(NumIdxPair((num, idx)));
      // specify `max` value (iff. max.idx >= left)
      while let Some(&NumIdxPair((_, idx))) = heap.peek() {
        if idx < left {
          heap.pop();
        } else {
          break;
        }
      }
      // get `actual_max` value
      ans.push(heap.peek().unwrap().0 .0)
    }

    ans
  }

  fn monotonic_queue(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;

    let k = k as usize;
    let len = nums.len();
    let mut mq = VecDeque::with_capacity(k + 1);

    for i in 0..k {
      while let Some(&idx) = mq.back() {
        if nums[i] >= nums[idx] {
          mq.pop_back();
        } else {
          break;
        }
      }
      mq.push_back(i)
    }

    let mut ans = vec![nums[*mq.front().unwrap()]];

    for i in (0..len).skip(k) {
      while let Some(&idx) = mq.back() {
        if nums[i] >= nums[idx] {
          mq.pop_back();
        } else {
          break;
        }
      }
      mq.push_back(i);

      while let Some(&idx) = mq.front() {
        if idx <= i - k {
          mq.pop_front();
        } else {
          break;
        }
      }
      ans.push(nums[*mq.front().unwrap()])
    }

    ans
  }

  fn faster_monotonic_queue(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;

    let k = k as usize;
    let n = nums.len();
    let mut dq = VecDeque::with_capacity(k + 1);
    let mut res = Vec::with_capacity(n - k + 1);

    for (i, x) in nums.into_iter().enumerate() {
      while let Some(&(_, y)) = dq.back() {
        if y >= x {
          break;
        }
        dq.pop_back();
      }
      dq.push_back((i, x));

      if i + 1 >= k {
        while let Some(&(j, _)) = dq.front() {
          if j + k > i {
            break;
          }
          dq.pop_front();
        }
        res.push(dq.front().unwrap().1);
      }
    }

    res
  }

  pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    Solution::faster_monotonic_queue(nums, k)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn binary_heap() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let output = Solution::binary_heap(nums, k);
    assert_eq!(output, vec![3, 3, 5, 5, 6, 7])
  }

  #[test]
  fn monotonic_queue() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let output = Solution::monotonic_queue(nums, k);
    assert_eq!(output, vec![3, 3, 5, 5, 6, 7])
  }

  #[test]
  fn faster_monotonic_queue() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let output = Solution::faster_monotonic_queue(nums, k);
    assert_eq!(output, vec![3, 3, 5, 5, 6, 7])
  }
}
