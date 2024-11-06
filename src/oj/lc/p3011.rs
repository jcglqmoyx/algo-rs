struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let n = nums.len();
        let mut i = 0;
        while i < n {
            let bits = nums[i].count_ones();
            let mut j = i + 1;
            while j < n && nums[j].count_ones() == bits {
                j += 1;
            }
            nums[i..j].sort_unstable();
            i = j;
        }
        for i in 0..n - 1 {
            if nums[i] > nums[i + 1] {
                return false;
            }
        }
        true
    }
}