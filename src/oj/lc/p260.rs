struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut x_or_sum = 0;
        for &x in nums.iter() {
            x_or_sum ^= x;
        }
        let a = x_or_sum & -x_or_sum;
        let mut res = vec![0, 0];
        for &x in nums.iter() {
            if x & a != 0 {
                res[0] ^= x;
            }
        }
        res[1] = res[0] ^ x_or_sum;
        res
    }
}