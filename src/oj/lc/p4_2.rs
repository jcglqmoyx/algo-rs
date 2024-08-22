struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let m = nums1.len() as i32;
        let n = nums2.len() as i32;
        let mut left = 0;
        let mut right = m;
        let mut median1 = 0;
        let mut median2 = 0;

        while left <= right {
            let i = (left + right) / 2;
            let j = (m + n + 1) / 2 - i;

            let nums_im1 = if i == 0 { i32::MIN } else { nums1[(i - 1) as usize] };
            let nums_i = if i == m { i32::MAX } else { nums1[i as usize] };
            let nums_jm1 = if j == 0 { i32::MIN } else { nums2[(j - 1) as usize] };
            let nums_j = if j == n { i32::MAX } else { nums2[j as usize] };

            if nums_im1 <= nums_j {
                median1 = nums_im1.max(nums_jm1);
                median2 = nums_i.min(nums_j);
                left = i + 1;
            } else {
                right = i - 1;
            }
        }

        if (m + n) % 2 == 0 {
            (median1 + median2) as f64 / 2.0
        } else {
            median1 as f64
        }
    }
}