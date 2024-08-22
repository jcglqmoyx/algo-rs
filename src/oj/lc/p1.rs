struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if let Some(p) = map.get(&(target - nums[i])) {
                return vec![*p, i as i32];
            }
            map.insert(nums[i], i as i32);
        }
        unreachable!()
    }
}
