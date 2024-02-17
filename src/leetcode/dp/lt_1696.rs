crate::sln!();

impl Solution {
  /// `dp[i] = nums[..=i].score.max = nums[..i].score.max + nums[i]`
  ///
  /// Therefore, `dp[i] = dp[max(0, i - k)..i].max`
  ///
  /// Specially, `dp[0] = nums[0]`
  pub fn normal_dp(nums: Vec<i32>, k: i32) -> i32 {
    let mut dp = nums.clone();
    let k = k as usize;
    let len = dp.len();
    for i in 1..len {
      let start = if i > k { i - k } else { 0 };
      dp[i] = *dp[start..i].iter().max().unwrap() + nums[i];
    }
    dp[dp.len() - 1]
  }

  pub fn heap_optimized(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut heap = std::collections::BinaryHeap::new();
    for (i, x) in nums.into_iter().enumerate() {
      let i = i as i32;
      let mut cur = 0;
      while let Some(&(t, idx)) = heap.peek() {
        if i - idx > k {
          heap.pop();
        } else {
          cur = t;
          break;
        }
      }
      heap.push((cur + x, i));
    }
    while let Some((t, idx)) = heap.pop() {
      if idx == n as i32 - 1 {
        return t;
      }
    }
    0
  }

  pub fn sliding_window_queue(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    for i in 1..n {
      // 1. move out
      if *q.front().unwrap() + k < i {
        q.pop_front();
      }
      // 2. transfer
      nums[i] += nums[*q.front().unwrap()];
      // 3. move in
      while !q.is_empty() && nums[i] >= nums[*q.back().unwrap()] {
        q.pop_back();
      }
      q.push_back(i);
    }
    nums[n - 1]
  }

  pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    Self::sliding_window_queue(nums, k)
  }
}
