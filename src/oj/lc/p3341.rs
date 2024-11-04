use std::cmp::max;
use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        let n = move_time.len();
        let m = move_time[0].len();
        let mut f = vec![vec![i32::MAX / 2; m]; n];
        let mut q = VecDeque::new();
        f[0][0] = 0;
        q.push_back((0, 0));
        while !q.is_empty() {
            match q.pop_front() {
                Some((x, y)) => {
                    for i in 0..4 {
                        let a = x as i32 + dx[i];
                        let b = y as i32 + dy[i];
                        if a < 0 || a == n as i32 || b < 0 || b == m as i32 || f[a as usize][b as usize] <= f[x][y] + 1 {
                            continue;
                        }
                        f[a as usize][b as usize] = max(f[x][y], move_time[a as usize][b as usize]) + 1;

                        q.push_back((a as usize, b as usize));
                    }
                }
                None => {
                    break;
                }
            }
        }
        f[n - 1][m - 1]
    }
}