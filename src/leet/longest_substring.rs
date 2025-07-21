// Given a string s, find the length of the longest substring without duplicate characters.
pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut m_len:usize = 0;
        let mut pos : [usize; 128] = [0; 128];
        let mut start = 0;
        for (end, ch) in s.chars().enumerate(){
            start = start.max(pos[ch as usize]);
            m_len = m_len.max(end - start + 1);
            pos[ch as usize] = end +1;
        }
        m_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(3i32, Solution::length_of_longest_substring("abcabcbb".to_string()));
    }
}