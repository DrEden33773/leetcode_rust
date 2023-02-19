#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn beautiful_bouquet(flowers: Vec<i32>, cnt: i32) -> i32 {
        use std::collections::HashMap;

        const MOD: i32 = 1000000007;
        let mut res = 0;

        for len in 1..=flowers.len() {
            for window in flowers.windows(len) {
                if len == 1 {
                    res += 1;
                    continue;
                }
                let mut species_num_map: HashMap<i32, i32> = HashMap::new();
                for species in window {
                    if let Some(num) = species_num_map.get_mut(species) {
                        *num += 1;
                    } else {
                        species_num_map.insert(*species, 1);
                    }
                }
                let if_satisfy = {
                    let mut if_satisfy = true;
                    for (_species, num) in species_num_map {
                        if num > cnt {
                            if_satisfy = false;
                            break;
                        }
                    }
                    if_satisfy
                };
                res += if if_satisfy { 1 } else { 0 }
            }
        }

        res % MOD
    }
}
