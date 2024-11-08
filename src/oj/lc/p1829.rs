struct Solution;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut s = 0;
        for x in nums.iter() {
            s ^= x;
        }
        let mut res = vec![0; n];
        let mx = (1 << maximum_bit) - 1;
        for i in 0..n {
            res[i] = s ^ mx;
            s ^= nums[n - i - 1];
        }
        res
    }
}