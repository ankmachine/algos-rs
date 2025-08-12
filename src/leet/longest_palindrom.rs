pub struct Solution;
impl Solution {
    pub fn longest_palindrome(input: String) -> String {
        let input_bytes = input.as_bytes();
        let mut position = (0, 0);
        for i in 0..input_bytes.len() * 2 {
            let (mut left, mut right) = (i / 2, i / 2 + (i % 2 != 0) as usize);
            while left <= right && left < input_bytes.len() && right < input_bytes.len() {
                if input_bytes.get(left).unwrap() == input_bytes.get(right).unwrap() {
                    if right - left > position.1 - position.0 {
                        position = (left, right);
                    }
                } else {
                    break;
                }
                if left > 0 {
                    left -= 1;
                } else {
                    break;
                }
                right += 1;
            }
        }
        input.get(position.0..=position.1).unwrap().to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!("bab".to_string(), Solution::longest_palindrome("babad".to_string()));
    }
}