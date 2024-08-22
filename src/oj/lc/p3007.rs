struct Solution;

impl Solution {
    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        let x = x as usize;
        let mut l = 1;
        let mut r = k << x as usize;
        let check = |mut mid| {
            let mut cnt = 0;
            let mut v = vec![];
            while mid > 0 {
                v.push(mid & 1);
                mid >>= 1;
            }
            let n = v.len();
            for i in (x - 1..n).step_by(x) {
                let mut right: i64 = 0;
                for j in (i + 1..n).rev() {
                    right = right * 2 + v[j];
                }
                cnt += right * (1 << i);
                if v[i] == 1 {
                    let mut left = 0;
                    for j in (0..i).rev() {
                        left = left * 2 + v[j];
                    }
                    cnt += left + 1;
                }
            }
            return cnt <= k;
        };
        while l < r {
            let mid = (l + r + 1) >> 1;
            if check(mid) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        l
    }
}