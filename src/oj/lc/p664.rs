struct Solution {}

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        use std::cmp::min;
        let n = s.len();
        let s = s.as_bytes();
        let mut f = vec![vec![0x3f3f3f3f; n + 1]; n + 1];
        for i in (0..n).rev() {
            f[i][i] = 1;
            for j in i + 1..n {
                if s[i] == s[j] {
                    f[i][j] = f[i][j - 1];
                } else {
                    for k in i..j {
                        f[i][j] = min(f[i][j], f[i][k] + f[k + 1][j]);
                    }
                }
            }
        }
        f[0][n - 1]
    }
}