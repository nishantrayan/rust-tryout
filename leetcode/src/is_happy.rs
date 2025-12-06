struct Solution;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = vec![n as u32];
        let mut n = n as u32;
        loop {
            let sum_sq_dig = n
                .to_string()
                .chars()
                .fold(0, |acc, digit| acc + digit.to_digit(10).unwrap().pow(2));

            match sum_sq_dig {
                1 => return true,
                seen_n if seen.contains(&seen_n) => return false,
                any_other_n => {
                    seen.push(any_other_n);
                    n = any_other_n;
                }
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(7), true);
        assert_eq!(Solution::is_happy(44), true);
    }
    #[test]
    fn test_is_not_happy() {
        assert_eq!(Solution::is_happy(2), false);
        assert_eq!(Solution::is_happy(3), false);
        assert_eq!(Solution::is_happy(41), false);
        assert_eq!(Solution::is_happy(43), false);
    }
}
