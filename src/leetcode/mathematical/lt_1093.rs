#![allow(dead_code)]

fn gen_vt_vec(count: Vec<i32>) -> Vec<(i32, i32)> {
    count
        .iter()
        .enumerate()
        .filter(|(_, &times)| times > 0)
        .map(|(value, &times)| (value as i32, times))
        .collect::<Vec<_>>()
}

pub struct Solution;

use std::ops::Add;

impl Solution {
    pub fn stable_sample_stats(count: Vec<i32>) -> Vec<f64> {
        let vt_vec /* [(value, times)] */ = count
            .iter()
            .enumerate()
            .filter(|(_, &times)| times > 0)
            .map(|(value, &times)| (value as i32, times))
            .collect::<Vec<_>>();
        /// Get `nth(start from 0)` element in `vt_vec`
        fn get_nth(vt_vec: &Vec<(i32, i32)>, nth: i64) -> i32 {
            let (mut curr_v, mut curr_t) = vt_vec[0];
            let mut i = 0;
            let mut n = 0;
            while n < nth {
                curr_t -= 1;
                n += 1;
                if curr_t == 0 {
                    i += 1;
                    curr_v = vt_vec[i].0;
                    curr_t = vt_vec[i].1;
                }
            }
            curr_v
        }
        let num: i64 = vt_vec
            .iter()
            .map(|(_, times)| times.to_owned() as i64)
            .sum();
        let sum: i64 = vt_vec
            .iter()
            .map(|(value, times)| value.to_owned() as i64 * times.to_owned() as i64)
            .sum();
        let minimum = vt_vec[0].0 as f64;
        let maximum = vt_vec[vt_vec.len() - 1].0 as f64;
        let mean = sum as f64 / num as f64;
        let mode = vt_vec.iter().max_by_key(|(_, times)| times).unwrap().0 as f64;
        let median = if num % 2 == 0 {
            get_nth(&vt_vec, num / 2 - 1).add(get_nth(&vt_vec, num / 2)) as f64 / 2 as f64
        } else {
            get_nth(&vt_vec, num / 2) as f64
        };
        vec![minimum, maximum, mean, median, mode]
    }
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let minimum = count
            .iter()
            .position(|&times| times > 0)
            .unwrap_or_default() as f64;
        let maximum = count
            .iter()
            .rposition(|&times| times > 0)
            .unwrap_or_default() as f64;
        let sum = count
            .iter()
            .enumerate()
            .map(|(v, &t)| v as i64 * t as i64)
            .sum::<i64>();
        let num = count
            .iter()
            .enumerate()
            .map(|(_, &t)| t as i64)
            .sum::<i64>();
        let mean = sum as f64 / num as f64;
        let mode = count
            .iter()
            .enumerate()
            .max_by_key(|(_, &t)| t)
            .map(|(v, _)| v)
            .unwrap_or_default() as f64;
        let left = {
            let mut steps = (num + 1) / 2; // this could take
            count
                .iter()
                .position(|&t| {
                    steps -= t as i64;
                    steps <= 0
                })
                .unwrap_or_default() as f64
        };
        let right = {
            let mut steps = (num + 1) / 2;
            count
                .iter()
                .rposition(|&t| {
                    steps -= t as i64;
                    steps <= 0
                })
                .unwrap_or_default() as f64
        };
        let median = (left + right) / 2 as f64;
        vec![minimum, maximum, mean, median, mode]
    }
}

#[cfg(test)]
mod sample_stats {
    use super::*;

    #[test]
    fn it_works() {
        let count = vec![
            2725123, 2529890, 2612115, 3807943, 3002363, 3107290, 2767526, 981092, 896521, 2576757,
            2808163, 3315813, 2004022, 2516900, 607052, 1203189, 2907162, 1849193, 1486120, 743035,
            3621726, 3366475, 639843, 3836904, 462733, 2614577, 1881392, 85099, 709390, 3534613,
            360309, 404975, 715871, 2258745, 1682843, 3725079, 564127, 1893839, 2793387, 2236577,
            522108, 1183512, 859756, 3431566, 907265, 1272267, 2261055, 2234764, 1901434, 3023329,
            863353, 2140290, 2221702, 623198, 955635, 304443, 282157, 3133971, 1985993, 1113476,
            2092502, 2896781, 1245030, 2681380, 2286852, 3423914, 3549428, 2720176, 2832468,
            3608887, 174642, 1437770, 1545228, 650920, 2357584, 3037465, 3674038, 2450617, 578392,
            622803, 3206006, 3685232, 2687252, 1001246, 3865843, 2755767, 184888, 2543886, 2567950,
            1755006, 249516, 3241670, 1422728, 809805, 955992, 415481, 26094, 2757283, 995334,
            3713918, 2772540, 2719728, 1204666, 1590541, 2962447, 779517, 1322374, 1675147,
            3146304, 2412486, 902468, 259007, 3161334, 1735554, 2623893, 1863961, 520352, 167827,
            3654335, 3492218, 1449347, 1460253, 983079, 1135, 208617, 969433, 2669769, 284741,
            1002734, 3694338, 2567646, 3042965, 3186843, 906766, 2755956, 2075889, 1241484,
            3790012, 2037406, 2776032, 1123633, 2537866, 3028339, 3375304, 1621954, 2299012,
            1518828, 1380554, 2083623, 3521053, 1291275, 180303, 1344232, 2122185, 2519290, 832389,
            1711223, 2828198, 2747583, 789884, 2116590, 2294299, 1038729, 1996529, 600580, 184130,
            3044375, 261274, 3041086, 3473202, 2318793, 2967147, 2506188, 127448, 290011, 3868450,
            1659949, 3662189, 1720152, 25266, 1126602, 1015878, 2635566, 619797, 2898869, 3470795,
            2226675, 2348104, 2914940, 1907109, 604482, 2574752, 1841777, 880254, 616721, 3786049,
            2278898, 3797514, 1328854, 1881493, 1802018, 3034791, 3615171, 400080, 2277949, 221689,
            1021253, 544372, 3101480, 1155691, 3730276, 1827138, 3621214, 2348383, 2305429, 313820,
            36481, 2581470, 2794393, 902504, 2589859, 740480, 2387513, 2716342, 1914543, 3219912,
            1865333, 2388350, 3525289, 3758988, 961406, 1539328, 448809, 1326527, 1339048, 2924378,
            2715811, 376047, 3642811, 2973602, 389167, 1026011, 3633833, 2848596, 3353421, 1426817,
            219995, 1503946, 2311246, 2618861, 1497325, 3758762, 2115273, 3238053, 2419849,
            2545790,
        ];
        let original = Solution::stable_sample_stats(count.clone());
        let improved = Solution::sample_stats(count);
        assert_eq!(original, improved);
        eprintln!("original => {:?}", original);
        eprintln!("improved => {:?}", improved);
    }
}
