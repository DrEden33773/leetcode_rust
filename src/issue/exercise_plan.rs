use rand::Rng;

#[allow(dead_code)]
pub fn better_solution(a: i64, b: i64, mut n: i64) -> i64 {
  let mut res: i64 = 0;
  let t = a * 5 + b * 2;
  let sub_t_a = a * 5;

  // deal with the biggest T
  res += {
    let num_of_t = n / t;
    n %= t;
    num_of_t * 7
  };

  // n <= sub_T_a => won't exceed sub_T_a
  if n <= sub_t_a {
    res += n / a;
    n %= a;
    // compensate if num_of_left_day in (0, a)
    if n != 0 {
      res += 1;
    }
    return res;
  }
  // n > sub_T_a => finally goes into sub_T_b

  // go through sub_T_a
  res += 5;
  n %= sub_t_a;
  // into sub_T_b
  res += n / b;
  n %= b;
  // compensate if num_of_left_day in (0, b)
  if n != 0 {
    res += 1
  }

  // return
  res
}

#[allow(dead_code)]
pub fn general_model_solution(
  amount_per_day_in_period: &Vec<i64>,
  days_per_period: &Vec<i64>,
  mut amount: i64,
) -> i64 {
  let apd_in_period = amount_per_day_in_period;
  assert_eq!(apd_in_period.len(), days_per_period.len());
  let len = apd_in_period.len();
  let mut need_days: i64 = 0;

  // deal with the sum_period
  let days_of_sum_period: i64 = days_per_period.iter().sum();
  let amount_in_sum_period: i64 = {
    let mut the_amount: i64 = 0;
    for i in 0..len {
      the_amount += apd_in_period[i] * days_per_period[i];
    }
    the_amount
  };
  need_days += {
    let num_of_sum_period = amount / amount_in_sum_period;
    amount %= amount_in_sum_period;
    num_of_sum_period * days_of_sum_period
  };

  // deal with each sub_period, from `left` to `right`
  for i in 0..len {
    // case 1 => won't exceed current `sub_period`
    if amount <= apd_in_period[i] * days_per_period[i] {
      need_days += amount / apd_in_period[i];
      amount %= apd_in_period[i];
      if amount != 0 {
        // compensate if remained amount in (0, apd_in_period[i])
        need_days += 1;
      }
      return need_days;
    }
    // case 2 => will exceed current `sub_period`
    need_days += days_per_period[i];
    let curr_period_amount = apd_in_period[i] * days_per_period[i];
    amount %= curr_period_amount;
  }

  need_days
}

#[allow(dead_code)]
pub fn test() {
  println!("exercise_plan:\n");
  for _test in 0..4 {
    let (a, b, n) = {
      let v = (0..3)
        .map(|index| {
          let upper_lim = if index == 2 { 1e18 as i64 } else { 1e3 as i64 };
          rand::thread_rng().gen_range(1..=upper_lim)
        })
        .collect::<Vec<_>>();
      (v[0], v[1], v[2])
    };
    println!("a = {}, b = {}, n = {}", a, b, n);
    println!("answer => {}", better_solution(a, b, n));
    println!(
      "general answer => {}",
      general_model_solution(&vec![a, b], &vec![5, 2], n)
    );
    println!();
  }
}
