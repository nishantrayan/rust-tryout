use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max: i32 = 0;
        for i in 0..s.len() {
            let mut char_count: HashMap<char, bool> = HashMap::new();
            for j in i..s.len() {
                if char_count.contains_key(&s.chars().nth(j).unwrap()) {
                    let len = j - i;
                    max = max.max(len as i32);
                    break;
                }
                char_count.insert(s.chars().nth(j).unwrap(), true);
            }
        }
        max
    }
}

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(
        3,
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
}
