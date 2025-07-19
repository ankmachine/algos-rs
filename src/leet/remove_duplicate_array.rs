/*
Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.

Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

Return k after placing the final result in the first k slots of nums.
 */
pub struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut j =1;
        for i in 1..nums.len() {
            if j ==1 || nums[j-2] != nums[i]{
                nums[j] = nums[i];
                j+=1;
                } 
            }
        j as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let mut input: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
        let output = 5;
        assert_eq!(output, Solution::remove_duplicates(&mut input));
    }
}