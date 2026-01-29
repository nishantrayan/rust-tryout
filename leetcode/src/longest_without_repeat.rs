struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut max: i32 = (s.len() as i32).min(1);
        for i in 0..s.len() {
            let mut char_count: HashMap<char, bool> = HashMap::new();
            for j in i..s.len() {
                if char_count.contains_key(&s.chars().nth(j).unwrap()) {
                    let len = j - i;
                    max = max.max(len as i32);
                    break;
                } else {
                    let len = j - i + 1;
                    max = max.max(len as i32);
                    char_count.insert(s.chars().nth(j).unwrap(), true);
                }
            }
        }
        max
    }
}

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(2, Solution::length_of_longest_substring("au".to_string()));
}
