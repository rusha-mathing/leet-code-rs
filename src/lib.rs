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

    fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums1.len() + nums2.len());
        let mut nums1 = nums1.into_iter().peekable();
        let mut nums2 = nums2.into_iter().peekable();
        while let (Some(&i), Some(&j)) = (nums1.peek(), nums2.peek()) {
            result.push(if i < j {
                nums1.next();
                i
            } else {
                nums2.next();
                j
            })
        }
        result.extend(nums1);
        result.extend(nums2);
        result
    }
}
