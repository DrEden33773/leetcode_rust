crate::sln!();

impl Solution {
  pub fn my_atoi(s: String) -> i32 {
    /* 1. ignore any leading whitespace */
    let mut s = s.trim().bytes().peekable();
    let mut res = 0u64;
    let mut is_negative = false;
    /* 2. receive `+ / -` */
    match s.peek() {
      Some(b'-') => {
        is_negative = true;
        s.next();
      }
      Some(b'+') => {
        s.next();
      }
      _ => {}
    }
    /* 3. read until reach the end / find first non-numerical */
    for curr in s {
      if !curr.is_ascii_digit() {
        break;
      }
      /* attention: `res` itself may overflow in [`u64`]'s range */
      let new_res = res.wrapping_mul(10).wrapping_add((curr - b'0') as u64);
      res = if new_res < res {
        break;
      } else {
        new_res
      };
    }
    /* 4. clamp if need */
    let limit = if is_negative {
      -(i32::MIN as i64) as u64
    } else {
      i32::MAX as u64
    };
    let res = if res > limit { limit } else { res } as i32;
    if is_negative {
      -res
    } else {
      res
    }
  }
}

#[cfg(test)]
mod atoi_test {
  use super::*;

  #[test]
  fn u64_overflow() {
    let res = Solution::my_atoi("18446744073709551617".to_string());
    assert_eq!(res, i32::MAX);
  }
}
