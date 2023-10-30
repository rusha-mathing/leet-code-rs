pub struct Solution();

pub struct Wrapper(Vec<i32>);
impl Wrapper {
    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return i32::MIN;
        }
        let index = index as usize;
        *self.0.get(index).unwrap_or(&i32::MAX)
    }

    fn len(&self) -> i32 {
        self.0.len() as i32
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            (nums1, nums2) = (nums2, nums1);
        }
        let (nums1, nums2) = (Wrapper(nums1), Wrapper(nums2));
        let (mut left, mut right) = (0, nums1.len());
        while left <= right {
            let mid_a = (left + right) / 2;
            let mid_b = (nums1.len() + nums2.len() + 1) / 2 - mid_a;
            let (a_left, a_right) = (nums1.get(mid_a - 1), nums1.get(mid_a));
            let (b_left, b_right) = (nums2.get(mid_b - 1), nums2.get(mid_b));
            if b_left > a_right {
                left = mid_a + 1;
            } else if a_left > b_right {
                right = mid_a - 1;
            } else {
                return if (nums1.len() + nums2.len()) % 2 == 0 {
                    (a_left.max(b_left) + a_right.min(b_right)) as f64 / 2.0
                } else {
                    a_left.max(b_left) as f64
                };
            }
        }
        return 0.0;
    }
}
