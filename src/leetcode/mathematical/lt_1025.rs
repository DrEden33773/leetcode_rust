#![allow(dead_code)]

pub struct Solution;

impl Solution {
    /**
        ### Divisor Game

        First, we could list a few basic cases:

        For current person, (n, if_has_chance_to_win):

        (1, false), (2, true), (3, false), (4, true), ...

        **Assume**: res = True if `n is even` else False

        **Prove**:

        Assume is *correct* for `n = 1 or 2`

        If Assume is *correct* for `n <= k`, *when* `k = n + 1`:

        1. k is `even` => k's factor could be `even` or `odd`,
        if it's `odd`, `k := even - odd = odd`, `Bob lose`, `Alice win`

        2. k is `odd` => k's factor could only be `odd`,
        `k := odd - odd = even`, `Bob win`, `Alice lose`

        **QED**
    */
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }

    pub fn dp_divisor_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false; n + 1];
        (dp[1], dp[2]) = (false, true);
        for curr in 3..=n {
            for factor in 1..n {
                if curr % factor == 0 && !dp[curr - factor] {
                    dp[curr] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}
