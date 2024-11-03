struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let n = s.len();
        if n != goal.len() {
            return false;
        }
        let s = s.as_bytes();
        let goal = goal.as_bytes();
        let mut ne = vec![-1; n];
        let mut j = -1;
        for i in 1..n {
            while j != -1 && s[i] != s[(j + 1) as usize] {
                j = ne[j as usize];
            }
            if s[i] == s[(j + 1) as usize] {
                j += 1;
            }
            ne[i] = j;
        }
        j = -1;
        for i in 0..(n * 2 - 1) {
            while j != -1 && goal[i % n] != s[(j + 1) as usize] {
                j = ne[j as usize];
            }
            if goal[i % n] == s[(j + 1) as usize % n] {
                j += 1;
            }
            if j as usize == n - 1 {
                return true;
            }
        }
        false
    }
}