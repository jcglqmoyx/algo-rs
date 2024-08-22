struct Solution {}
impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::cmp::min;
        let n = nums.len();
        let m = queries.len();
        let mut tr = vec![vec![0; 2]; n * 31];
        let mut idx = 0;
        let mut mn = vec![1_000_000_000; n * 31];
        let mut insert = |x| {
            let mut p = 0;
            for i in (0..31).rev() {
                let u = x >> i & 1usize;
                if tr[p][u] == 0usize {
                    tr[p][u] = idx as usize + 1;
                    idx += 1;
                }
                p = tr[p][u];
                mn[p] = min(mn[p], x);
            }
        };
        let mut min_value = 1_000_000_000;
        for x in nums {
            insert(x as usize);
            min_value = min(min_value, x);
        }
        let mut res = vec![0; m];
        for it in 0..m {
            let xi = queries[it][0] as usize;
            let mi = queries[it][1];
            if mi < min_value {
                res[it] = -1;
            } else {
                let mut num = 0;
                let mut p = 0;
                for i in (0..31).rev() {
                    let u = xi >> i & 1;
                    let v = u ^ 1;
                    if tr[p][v] != 0 && mn[tr[p][v]] <= mi as usize {
                        p = tr[p][v];
                        num += (1 << i) * v;
                    } else {
                        p = tr[p][u];
                        num += (1 << i) * u;
                    }
                }
                res[it] = (num ^ xi) as i32;
            }
        }
        res
    }
}