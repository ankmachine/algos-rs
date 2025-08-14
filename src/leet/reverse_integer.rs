pub struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut rev: i32 = 0;
        let mut curr: i32 = x;
        while (curr != 0) {
            match rev.checked_mul(10) {
                None => return 0,
                Some(t) => match t.checked_add(curr % 10) {
                    None => return 0,
                    Some(r) => rev = r,
                },
            }
            curr = curr / 10;
        }
        rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(321, Solution::reverse(123));
    }
}
