struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut cur = 0;
        loop {
            let ne = nums[cur];
            if ne == -1 {
                return cur as i32;
            }
            nums[cur] = -1;
            cur = ne as usize;
        }
    }
}