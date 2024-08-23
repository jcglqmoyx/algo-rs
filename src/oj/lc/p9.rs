struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || x > 0 && x % 10 == 0 {
            return false;
        }
        let mut x = x;
        let mut r = 0;
        while x > r {
            r = r * 10 + x % 10;
            x /= 10;
        }
        r == x || r / 10 == x
    }
}