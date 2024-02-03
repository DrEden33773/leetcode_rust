crate::sln!();

impl Solution {
  fn brute_force(haystack: String, needle: String) -> i32 {
    let (haystack, needle) = (haystack.into_bytes(), needle.into_bytes());
    let (h, n) = (haystack.len(), needle.len());
    if h < n {
      return -1;
    }
    for i in 0..=h - n {
      let (mut i_h, mut i_n) = (i, 0);
      while i_n < n && haystack[i_h] == needle[i_n] {
        i_h += 1;
        i_n += 1;
      }
    }
    -1
  }

  fn kmp(haystack: String, needle: String) -> i32 {
    let (haystack, needle) = (haystack.into_bytes(), needle.into_bytes());
    let (h, n) = (haystack.len(), needle.len());
    if n == 0 {
      return 0;
    }
    if h < n {
      return -1;
    }
    let next = {
      let mut next = vec![0; n];
      let mut j = 0;
      for i in 1..n {
        while j > 0 && needle[i] != needle[j] {
          j = next[j - 1];
        }
        if needle[i] == needle[j] {
          j += 1;
        }
        next[i] = j;
      }
      next
    };
    let mut j = 0;
    for (i, &curr_h) in haystack.iter().enumerate().take(h) {
      while j > 0 && curr_h != needle[j] {
        j = next[j - 1];
      }
      if curr_h == needle[j] {
        j += 1;
      }
      if j == n {
        return (i - n + 1) as i32;
      }
    }
    -1
  }

  pub fn str_str(haystack: String, needle: String) -> i32 {
    // Self::brute_force(haystack, needle)
    Self::kmp(haystack, needle)
  }
}
