use std::collections::VecDeque;

crate::sln!();

impl Solution {
  pub fn final_string(s: String) -> String {
    let s = s.as_bytes();
    let mut is_reversed = false;
    let mut buffer = VecDeque::new();

    for byte in s {
      match byte {
        b'i' => is_reversed = !is_reversed,
        _ => {
          if is_reversed {
            buffer.push_front(*byte)
          } else {
            buffer.push_back(*byte)
          }
        }
      }
    }

    if !is_reversed {
      String::from_utf8_lossy(&buffer.into_iter().collect::<Vec<_>>()).to_string()
    } else {
      String::from_utf8_lossy(&buffer.into_iter().rev().collect::<Vec<_>>()).to_string()
    }
  }
}
