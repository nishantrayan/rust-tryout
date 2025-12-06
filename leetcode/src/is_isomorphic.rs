use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let t_chars = t.chars().collect::<Vec<char>>();
        let mut char_map: HashMap<char, char> = HashMap::new();
        let mut taken = [false; 128];
        for (s_char, index) in s.chars().zip(0..s.len()) {
            let t_char = t_chars[index];
            if let Some(char_map_val) = char_map.get(&s_char) {
                if *char_map_val != t_char {
                    return false;
                }
            } else {
                let char_index = t_char as usize;
                if taken[char_index] {
                    return false;
                }
                taken[char_index] = true;
                char_map.insert(s_char, t_char);
            }
        }
        true
    }
}
mod tests {
    use super::*;
    #[test]
    fn test_is_isomorphic_single_char() {
        assert_eq!(
            Solution::is_isomorphic("e".to_string(), "a".to_string()),
            true
        );
    }

    #[test]
    fn test_is_isomorphic_simple() {
        assert_eq!(
            Solution::is_isomorphic("add".to_string(), "egg".to_string()),
            true
        );
    }

    #[test]
    fn test_is_isomorphic_not_match() {
        assert_eq!(
            Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
            false
        );
    }

    #[test]
    fn test_is_isomorphic_long_string() {
        assert_eq!(
            Solution::is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
    }

    #[test]
    fn test_is_isomorphic_failed_leetcode_test() {
        assert_eq!(
            Solution::is_isomorphic("badc".to_string(), "baba".to_string()),
            false
        );
    }

    #[test]
    fn test_is_isomorphic_numbers() {
        assert_eq!(
            Solution::is_isomorphic("13".to_string(), "42".to_string()),
            true
        );
    }

    #[test]
    fn test_is_isomorphic_long_strings() {
        assert_eq!(
            Solution::is_isomorphic("qwertyuiop[]asdfghjkl;'\\zxcvbnm,./".to_string(), "',.pyfgcrl/=aoeuidhtns-\\;qjkxbmwvz".to_string()),
            true
        );
    }
}
