struct Solution;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut stk = Vec::with_capacity(n);
        let mut l = vec![0; n];
        let mut res = 0;

        for i in 0..n {
            while let Some(&last) = stk.last() {
                if arr[last] >= arr[i] {
                    stk.pop();
                } else {
                    break;
                }
            }
            l[i] = if let Some(&last) = stk.last() {
                last + 1
            } else {
                0
            };
            stk.push(i);
        }

        stk.clear();

        for i in (0..n).rev() {
            while let Some(&last) = stk.last() {
                if arr[last] > arr[i] {
                    stk.pop();
                } else {
                    break;
                }
            }
            let ri = if let Some(&last) = stk.last() {
                last - 1
            } else {
                n - 1
            };
            stk.push(i);
            res = (res + arr[i] as usize * (i - l[i] + 1) * (ri - i + 1)) % 1_000_000_007;
        }

        res as i32
    }
}