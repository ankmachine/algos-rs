pub struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut zigzag_num: Vec<_> = (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .collect();
        zigzag_num.sort_by_key(|&(key, _)| key);
        zigzag_num.into_iter().map(|(_, char)| char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(
            "PINALSIGYAHRPI".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );
    }
}
