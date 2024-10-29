use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let dx = [-1, 0, 1];
        let n = grid.len();
        let m = grid[0].len();
        let mut f = vec![vec![0; m]; n];
        let mut q = VecDeque::new();
        for i in 0..n {
            f[i][0] = 0;
            q.push_back((i, 0));
        }
        let mut res = 0;
        while !q.is_empty() {
            let (x, y) = q.front().unwrap();
            let x = *x;
            let y = *y;
            q.pop_front();
            for i in 0..3 {
                let a = x as i32 + dx[i];
                let b = y + 1;
                if a >= 0 && a < n as i32 && b < m && grid[a as usize][b] > grid[x][y] && f[a as usize][b] <= f[x][y] {
                    q.push_back((a as usize, b));
                    f[a as usize][b] = f[x][y] + 1;
                    res = res.max(f[a as usize][b]);
                }
            }
        }
        res
    }
}