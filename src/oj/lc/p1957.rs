struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut res = String::from("");
        for c in s.chars() {
            if res.len() < 2 || res.as_bytes()[res.len() - 1] != res.as_bytes()[res.len() - 2] || res.as_bytes()[res.len() - 1] != c as u8 {
                res.push(c);
            }
        }
        res
    }
}