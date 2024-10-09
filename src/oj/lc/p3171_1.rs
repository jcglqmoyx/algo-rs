use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut res = i32::MAX;
        for i in 1..nums.len() + 1 {
            res = min(res, (nums[i - 1] - k).abs());
            for j in (1..i).rev() {
                if (nums[j - 1] | nums[i - 1]) == nums[j - 1] {
                    break;
                }
                nums[j - 1] |= nums[i - 1];
                res = min(res, (nums[j - 1] - k).abs());
            }
        }
        res
    }
}