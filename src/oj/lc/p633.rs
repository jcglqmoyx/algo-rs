struct Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut i = 0;
        while i * i <= c {
            let a = i * i;
            let b = c - i * i;
            if b < a {
                break;
            }
            let st = (b as f64).sqrt() as i32;
            if st * st == b {
                return true;
            }
            i += 1
        }
        false
    }
}