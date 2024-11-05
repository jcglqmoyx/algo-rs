struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut res = 0;
        let s = s.as_bytes();
        for i in (0..s.len()).step_by(2) {
            if s[i] != s[i + 1] {
                res += 1;
            }
        }
        res
    }
}