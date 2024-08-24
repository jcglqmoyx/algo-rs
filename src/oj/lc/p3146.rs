struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut res = 0;
        let mut p = [0; 26];
        for (i, &c) in s.as_bytes().iter().enumerate() {
            p[c as usize - 'a' as usize] = i;
        }
        for (i, &c) in t.as_bytes().iter().enumerate() {
            res += ((i - p[c as usize - 'a' as usize]) as i32).abs();
        }
        res
    }
}