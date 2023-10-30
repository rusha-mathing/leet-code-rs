pub struct Solution();

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums = Self::merge(nums1, nums2);
        let mid = nums.len() / 2;
        if nums.len() % 2 == 1 {
            nums[mid] as f64
        } else {
            (nums[mid - 1] + nums[mid]) as f64 / 2.0
        }
    }

    fn merge(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1.extend(nums2);
        nums1.sort();
        nums1
    }
}
