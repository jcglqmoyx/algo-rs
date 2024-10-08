struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut rev = 0;
        while x != 0 {
            if rev < i32::MIN / 10 || rev > i32::MAX / 10 {
                return 0;
            }
            let digit = x % 10;
            x /= 10;
            rev = rev * 10 + digit;
        }
        rev
    }
}