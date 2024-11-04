struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut o = 0;
        let mut e = 0;
        let s = num.as_bytes();
        for i in 0..s.len() {
            if (i & 1) == 1 {
                o += (s[i] - b'0') as i16;
            } else {
                e += (s[i] - b'0') as i16;
            }
        }
        o == e
    }
}