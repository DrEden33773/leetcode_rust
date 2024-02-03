crate::sln!();

const VALUE_SYMBOLS: [(i32, &str); 13] = [
  (1000, "M"),
  (900, "CM"),
  (500, "D"),
  (400, "CD"),
  (100, "C"),
  (90, "XC"),
  (50, "L"),
  (40, "XL"),
  (10, "X"),
  (9, "IX"),
  (5, "V"),
  (4, "IV"),
  (1, "I"),
];

const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];

const HUNDREDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];

const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];

const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

impl Solution {
  pub fn int_to_roman_simulation(mut num: i32) -> String {
    let mut roman = "".to_string();
    for (value, symbol) in VALUE_SYMBOLS.into_iter() {
      while num >= value {
        num -= value;
        roman.push_str(symbol);
      }
      if num == 0 {
        break;
      }
    }
    roman
  }

  pub fn int_to_roman_hardcode(num: i32) -> String {
    format!(
      "{}{}{}{}",
      THOUSANDS[(num / 1000) as usize],
      HUNDREDS[((num % 1000) / 100) as usize],
      TENS[((num % 100) / 10) as usize],
      ONES[(num % 10) as usize]
    )
  }

  pub fn int_to_roman(num: i32) -> String {
    Self::int_to_roman_hardcode(num)
  }
}
