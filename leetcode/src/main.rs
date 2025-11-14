fn main() {
    let result = two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(result, vec![0, 1]);

    assert_eq!(true, is_palindrome(121));
    assert_eq!(false, is_palindrome(12));

    assert_eq!(1994, roman_to_int("MCMXCIV".to_string()));

    assert_eq!(
        String::from("abcd"),
        longest_common_prefix(vec![String::from("abcd"), String::from("abcde")])
    )
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let (mut n, mut rev) = (x, 0);
    while n != 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev == x
}

pub fn roman_to_int(s: String) -> i32 {
    let s = String::from(s + " ");
    let chars: Vec<char> = s.chars().collect::<Vec<char>>();
    let mut windows = chars.windows(2);
    let mut num = 0;
    while let Some(window) = windows.next() {
        let (first, second) = window.split_at(1);
        // first and second are &[char], so destructure by indexing
        let add_num = match (first[0], second[0]) {
            ('I', 'V') | ('I', 'X') => -1,
            ('I', _) => 1,
            ('X', 'L') | ('X', 'C') => -10,
            ('X', _) => 10,
            ('C', 'D') | ('C', 'M') => -100,
            ('C', _) => 100,
            ('V', _) => 5,
            ('L', _) => 50,
            ('D', _) => 500,
            ('M', _) => 1000,
            _ => 0,
        };
        num += add_num;
    }
    num
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut index: usize = 0;
    loop {
        let all_chars: Vec<_> = strs
            .iter()
            .map(|s| s.chars().nth(index))
            .collect();

        if all_chars.iter().any(|c| c.is_none()) {
            break;
        }

        if let Some(first) = all_chars.first() {
            let all_match = all_chars.iter().all(|c| c == first);
            if all_match {
                index += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    strs.first()
        .unwrap()
        .chars()
        .take(index)
        .collect::<String>()
}
