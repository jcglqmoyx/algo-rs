struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let bits = 32 - num.leading_zeros();
        !num & ((1 << bits) - 1)
    }
}