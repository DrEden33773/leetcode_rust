crate::sln!();

impl Solution {
  pub fn add_binary_old(longer: String, shorter: String) -> String {
    if longer.len() < shorter.len() {
      return Self::add_binary(shorter, longer);
    }
    let (l, s) = (longer.len(), shorter.len());
    let lhs = longer.chars().rev().collect::<String>();
    let rhs = shorter.chars().rev().collect::<String>() + "0".repeat(l - s).as_ref();
    let mut ans = String::new();
    let mut carry = 0;
    for (l, r) in lhs
      .bytes()
      .zip(rhs.bytes())
      .map(|(l, r)| (l - b'0', r - b'0'))
    {
      let sum = l + r + carry;
      carry = sum / 2;
      ans.push((sum % 2 + b'0') as char);
    }
    if carry != 0 {
      ans.push('1');
    }
    ans.chars().rev().collect()
  }

  pub fn add_binary(a: String, b: String) -> String {
    let mut carry = 0u8;
    let mut res = String::new();
    let mut a = a.bytes().rev();
    let mut b = b.bytes().rev();

    loop {
      let (aop, bop) = (a.next(), b.next());
      match (aop, bop) {
        (Some(av), Some(bv)) => {
          let val = av - b'0' + bv - b'0' + carry;
          carry = val / 2;
          res.push(char::from(val % 2 + b'0'));
        }
        (None, Some(v)) | (Some(v), None) => {
          let val = v - b'0' + carry;
          carry = val / 2;
          res.push(char::from(val % 2 + b'0'));
        }
        (None, None) => {
          if carry == 1 {
            res.push('1');
          }
          break;
        }
      }
    }

    res.chars().rev().collect()
  }
}
