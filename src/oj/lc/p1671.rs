struct Solution;

use std::cmp::min;

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut l = vec![0; n];
        let mut v = vec![];
        let mut idx = 0;
        for i in 0..n {
            if idx == 0 || nums[i] > v[idx - 1] {
                v.push(nums[i]);
                idx += 1;
                l[i] = idx as i32;
            } else {
                let p = v.binary_search(&nums[i]).unwrap_or_else(|p| p);
                v[p] = nums[i];
                l[i] = (p + 1) as i32;
            }
        }
        idx = 0;
        v.clear();
        let mut res = n as i32;
        for i in (0..n).rev() {
            let mut ri = 0;
            if idx == 0 || nums[i] > v[idx - 1] {
                v.push(nums[i]);
                idx += 1;
                ri = idx as i32;
            } else {
                let p = v.binary_search(&nums[i]).unwrap_or_else(|p| p);
                v[p] = nums[i];
                ri = (p + 1) as i32;
            }
            if l[i] > 1 && ri > 1 {
                res = min(res, n as i32 - l[i] - ri + 1);
            }
        }
        res
    }
}