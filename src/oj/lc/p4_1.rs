struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        use std::cmp::min;
        fn find(nums1: &Vec<i32>, i: usize, nums2: &Vec<i32>, j: usize, k: usize) -> i32 {
            if nums1.len() - i > nums2.len() - j {
                return find(nums2, j, nums1, i, k);
            }
            if i == nums1.len() {
                return nums2[j + k - 1];
            }
            if k == 1 {
                return min(nums1[i], nums2[j]);
            }
            let ni = min(i + k / 2 - 1, nums1.len() - 1);
            let nj = j + (k - k / 2) - 1;
            if nums1[ni] < nums2[nj] {
                find(nums1, ni + 1, nums2, j, k - (ni - i + 1))
            } else {
                find(nums1, i, nums2, nj + 1, k / 2)
            }
        }
        let tot = nums1.len() + nums2.len();
        if tot & 1 == 1 {
            find(&nums1, 0, &nums2, 0, tot / 2 + 1) as f64
        } else {
            (find(&nums1, 0, &nums2, 0, tot / 2) + find(&nums1, 0, &nums2, 0, tot / 2 + 1)) as f64 * 0.5
        }
    }
}