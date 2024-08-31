struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let n = s.len();
        let m = p.len();
        let mut f = vec![vec![false; m + 1]; n + 1];
        f[0][0] = true;
        for i in 0..n + 1 {
            for j in 1..m + 1 {
                if p[j - 1] == '*' as u8 {
                    f[i][j] |= f[i][j - 2];
                    if i > 0 {
                        for k in (1..i + 1).rev() {
                            if p[j - 2] != '.' as u8 && s[k - 1] != p[j - 2] {
                                break;
                            }
                            f[i][j] |= f[k - 1][j - 2];
                        }
                    }
                } else if i > 0 && (s[i - 1] == p[j - 1] || p[j - 1] == '.' as u8) {
                    f[i][j] |= f[i - 1][j - 1];
                }
            }
        }
        f[n][m]
    }
}