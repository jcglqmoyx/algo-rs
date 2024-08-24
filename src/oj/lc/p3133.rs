struct Solution;

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let n = (n - 1) as i64;
        let mut ans = x as i64;
        let mut i = 0;
        let mut j = 0;
        while n >> j > 0 {
            if (ans >> i & 1) == 0 {
                ans |= (n >> j & 1) << i;
                j = j + 1;
            }
            i = i + 1;
        }
        ans
    }
}