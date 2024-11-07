use std::cmp::max;

struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut res = 0;
        let &m = candidates.iter().max().unwrap_or(&0);
        let m = (m as f64).log2().floor() as i32 + 1;
        for i in 0..m {
            let mut cnt = 0;
            for &x in &candidates {
                if (x >> i & 1) == 1 {
                    cnt += 1;
                }
            }
            res = max(res, cnt);
        }
        res
    }
}