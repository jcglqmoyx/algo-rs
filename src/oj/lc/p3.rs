struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut res = 0;
        let s = s.as_bytes();
        let mut cnt = [0; 128];
        let mut i = 0;
        let mut j = 0;
        while j < s.len() {
            let u = s[j] as usize;
            cnt[u] += 1;
            while cnt[u] > 1 {
                cnt[s[i] as usize] -= 1;
                i += 1
            }
            res = max(res, j - i + 1);
            j += 1;
        }
        res as i32
    }
}