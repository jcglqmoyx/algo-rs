use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut res = i32::MAX;
        for i in 0..nums.len() {
            res = min(res, (nums[i] - k).abs());
            for j in (0..i).rev() {
                if (nums[j] | nums[i]) == nums[j] {
                    break;
                }
                nums[j] |= nums[i];
                res = min(res, (nums[j] - k).abs());
            }
        }
        res
    }
}