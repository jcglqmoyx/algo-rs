use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let n = robot.len();
        let m = factory.len();
        let mut robot = robot;
        let mut factory = factory;
        robot.sort();
        factory.sort();
        let mut f = vec![i64::MAX / 2; n + 1];
        f[0] = 0;
        for i in 0..m {
            let mut g = vec![i64::MAX / 2; n + 1];
            g[0] = 0;
            for j in 1..=n {
                let mut dist = 0;
                g[j] = f[j];
                let mut k = 1;
                while k <= min(j, factory[i][1] as usize) {
                    dist += (factory[i][0] - robot[j - k]).abs() as i64;
                    g[j] = min(g[j], f[j - k] + dist);
                    k += 1;
                }
            }
            f = g;
        }
        f[n]
    }
}