

pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // we implement binary search method
        let(mut nums1, mut nums2) = if nums1.len() > nums2.len() {(nums2, nums1)} else {(nums1, nums2)};
        // m and n will be used to find the mid y point
        // m will also be used for controlling while loop
        let (m, n) = (nums1.len(), nums2.len());
        let(mut low, mut high)=(0, m);
        while low <= high {
            let mid_x = (high + low) / 2;
            let mid_y = (m + n + 1) / 2 - mid_x;
            let max_x = if mid_x == 0 {i32::MIN} else {nums1[mid_x - 1]};
            let min_x = if mid_x == m {i32::MAX} else {nums1[mid_x ]};
            let max_y = if mid_y == 0 {i32::MIN} else {nums2[mid_y - 1]};
            let min_y = if mid_y == n {i32::MAX} else {nums2[mid_y ]};
            if max_x <= min_y && max_y <= min_x {
                if (m + n) % 2 == 0 {return (max_x.max(max_y)+ min_x.min(min_y) )as f64/2.0;}
                else{return max_x.max(max_y) as f64};
            }else if max_x > min_y{high = mid_x-1}
            else{low = mid_x+1};
        }

        0.0
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let nums1: Vec<i32> = vec![1, 3];
        let nums2: Vec<i32> = vec![2];
        let output = 2.0;
        assert_eq!(output, Solution::find_median_sorted_arrays(nums1, nums2));
    }
}