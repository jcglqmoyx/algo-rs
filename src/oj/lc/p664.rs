struct Solution {}

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        use std::cmp::min;
        let n = s.len();
        let s = s.as_bytes();
        let mut f = vec![vec![0; n + 1]; n + 1];
        for i in (0..n).rev() {
            f[i][i] = 1;
            for j in i + 1..n {
                if s[i] == s[j] {
                    f[i][j] = f[i][j - 1];
                } else {
                    let mut mn = i32::MAX;
                    for k in i..j {
                        mn = min(mn, f[i][k] + f[k + 1][j]);
                    }
                    f[i][j] = mn;
                }
            }
        }
        f[0][n - 1]
    }
}