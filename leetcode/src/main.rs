fn main() {
    println!("Run tests with: cargo test");
}

// Helper function to convert ListNode to Vec for testing
fn listnode_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = vec![];
    while let Some(node) = list {
        v.push(node.val);
        list = node.next;
    }
    v
}

// ============================================================================
// LeetCode Solutions
// ============================================================================
pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack_chars: Vec<char> = haystack.chars().collect();
    for (index, w) in haystack_chars.windows(needle.len()).enumerate() {
        let word = String::from_iter(w);
        if word == needle {
            return index as i32;
        }
    }
    -1
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    let mut rev_digits = digits.iter().rev();
    let mut final_digits = Vec::new();
    while let Some(digit) = rev_digits.next() {
        let mut add_digit = carry + digit;
        match add_digit {
            10 => {
                carry = 1;
                add_digit = 0;
            }
            _ => {
                carry = 0;
            }
        }
        final_digits.insert(0, add_digit);
    }
    if carry == 1 {
        final_digits.insert(0, 1);
    }
    final_digits
}

pub fn add_binary(a: String, b: String) -> String {
    let mut result = Vec::<char>::new();
    let mut carry = 0;
    let max_len = a.len().max(b.len());
    let (mut a_padded, mut b_padded) = (
        a.chars().collect::<Vec<char>>(),
        b.chars().collect::<Vec<char>>(),
    );
    for _ in 0..(max_len - a.len()) {
        a_padded.insert(0, '0')
    }
    for _ in 0..(max_len - b.len()) {
        b_padded.insert(0, '0')
    }
    let (mut a_rev, mut b_rev) = (a_padded.iter().rev(), b_padded.iter().rev());
    for (a_digit, b_digit) in a_rev.zip(b_rev) {
        let mut sum = (a_digit.to_string().parse::<i32>()).unwrap()
            + (b_digit.to_string().parse::<i32>()).unwrap()
            + carry;
        result.insert(0, char::from_digit((sum % 2) as u32, 10).unwrap());
        carry = sum / 2;
    }
    if carry == 1 {
        result.insert(0, '1');
    }
    result.iter().collect::<String>()
}
pub fn length_of_last_word(s: String) -> i32 {
    let mut c_count = 0;
    let mut chars = s.chars().rev();
    while let Some(c) = chars.next() {
        if c == ' ' {
            if c_count == 0 {
                continue;
            } else {
                return c_count;
            }
        } else {
            c_count += 1;
        }
    }
    c_count
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let (mut left, mut right) = (0, nums.len() - 1);

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] < target {
            left = mid + 1;
        } else {
            if mid == 0 {
                return 0;
            }
            right = mid - 1;
        }
    }

    left as i32
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
        let all_chars: Vec<_> = strs.iter().map(|s| s.chars().nth(index)).collect();

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

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }
    stack.is_empty()
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut new_list = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut new_list;
    while list1.is_some() && list2.is_some() {
        let (l1, l2) = (list1.as_mut().unwrap(), list2.as_mut().unwrap());
        if l1.val <= l2.val {
            let mut node = list1.take().unwrap();
            list1 = node.next.take();
            tail.next = Some(node);
        } else {
            let mut node = list2.take().unwrap();
            list2 = node.next.take();
            tail.next = Some(node);
        }
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = if list1.is_some() { list1 } else { list2 };
    new_list.next
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut idx = 0;
    let mut i = 1;
    while i < nums.len() {
        if nums[i] != nums[idx] {
            idx += 1;
            nums[idx] = nums[i];
        }
        i += 1;
    }
    (idx + 1) as i32
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;

    struct EmptySlotList(Vec<usize>);
    impl EmptySlotList {
        fn new() -> Self {
            EmptySlotList(Vec::new())
        }

        fn push(&mut self, index: usize) {
            self.0.insert(0, index);
        }

        fn pop(&mut self) -> Option<usize> {
            self.0.pop()
        }
    }

    let mut empty_slots = EmptySlotList::new();
    let mut count: usize = 0;
    while i < nums.len() {
        if nums[i] == val {
            empty_slots.push(i);
        } else {
            count = count + 1;
            if let Some(empty_index) = empty_slots.pop() {
                nums[empty_index] = nums[i];
                empty_slots.push(i);
            }
        }
        i = i + 1;
    }
    count as i32
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary_overflow() {
        assert_eq!(
            add_binary("10".to_string(), "10".to_string()),
            "100".to_string()
        );
    }

    #[test]
    fn test_add_binary_basic() {
        assert_eq!(
            add_binary("1".to_string(), "0".to_string()),
            "1".to_string()
        );
    }

    #[test]
    fn test_add_binary_carry_follow() {
        assert_eq!(
            add_binary("111111".to_string(), "1".to_string()),
            "1000000".to_string()
        );
    }

    #[test]
    fn test_plus_one_same_len() {
        assert_eq!(plus_one(vec![1, 0]), vec![1, 1]);
    }

    #[test]
    fn test_plus_one_increases_len() {
        assert_eq!(plus_one(vec![9, 9]), vec![1, 0, 0]);
    }

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(length_of_last_word("abc d efg".to_string()), 3);
        assert_eq!(length_of_last_word("abcd".to_string()), 4);
        assert_eq!(length_of_last_word(" a b ".to_string()), 1);
    }
    #[test]
    fn test_search_insert_not_present() {
        let nums = vec![1, 3, 5];
        assert_eq!(search_insert(nums, 2), 1);
    }

    #[test]
    fn test_search_insert_present_last() {
        let nums = vec![1, 2, 3, 5];
        assert_eq!(search_insert(nums, 6), 4);
    }

    #[test]
    fn test_search_insert_present_first() {
        let nums = vec![1, 2, 3, 5];
        assert_eq!(search_insert(nums, 0), 0);
    }

    #[test]
    fn test_search_insert_present_present() {
        let nums = vec![1, 2, 3, 5];
        assert_eq!(search_insert(nums, 5), 3);
    }
    #[test]
    fn test_str_str_contains() {
        let needle = "abcd";
        let haystack = "abcdabcdabcd";
        assert_eq!(str_str(haystack.to_string(), needle.to_string()), 0);
    }

    #[test]
    fn test_str_str_not_contains() {
        let needle = "abcd";
        let haystack = "efghabc";
        assert_eq!(str_str(haystack.to_string(), needle.to_string()), -1);
    }

    // Two Sum Tests
    #[test]
    fn test_two_sum_basic() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    // Is Palindrome Tests
    #[test]
    fn test_is_palindrome_true() {
        assert_eq!(true, is_palindrome(121));
    }

    #[test]
    fn test_is_palindrome_false() {
        assert_eq!(false, is_palindrome(12));
    }

    // Roman to Int Tests
    #[test]
    fn test_roman_to_int() {
        assert_eq!(1994, roman_to_int("MCMXCIV".to_string()));
    }

    // Longest Common Prefix Tests
    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            String::from("abcd"),
            longest_common_prefix(vec![String::from("abcd"), String::from("abcde")])
        );
    }

    // Is Valid Parentheses Tests
    #[test]
    fn test_is_valid_simple() {
        assert_eq!(true, is_valid("()".to_string()));
    }

    #[test]
    fn test_is_valid_multiple() {
        assert_eq!(true, is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_is_valid_invalid_pair() {
        assert_eq!(false, is_valid("(]".to_string()));
    }

    #[test]
    fn test_is_valid_wrong_order() {
        assert_eq!(false, is_valid("([)]".to_string()));
    }

    #[test]
    fn test_is_valid_nested() {
        assert_eq!(true, is_valid("{[]}".to_string()));
    }

    #[test]
    fn test_is_valid_unclosed() {
        assert_eq!(false, is_valid("([".to_string()));
    }

    #[test]
    fn test_is_valid_closing_only() {
        assert_eq!(false, is_valid("]".to_string()));
    }

    // Merge Two Lists Tests
    #[test]
    fn test_merge_two_lists_basic() {
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let result = merge_two_lists(l1, l2);
        assert_eq!(listnode_to_vec(result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_two_lists_one_empty() {
        let l1 = None;
        let l2 = Some(Box::new(ListNode { val: 0, next: None }));
        let result = merge_two_lists(l1, l2);
        assert_eq!(listnode_to_vec(result), vec![0]);
    }

    #[test]
    fn test_merge_two_lists_both_empty() {
        let l1 = None;
        let l2 = None;
        let result = merge_two_lists(l1, l2);
        assert_eq!(listnode_to_vec(result), vec![]);
    }

    // Remove Element Tests
    #[test]
    fn test_remove_element_basic() {
        let mut nums = vec![3, 2, 2, 3];
        let len = remove_element(&mut nums, 3);
        assert_eq!(len, 2);
        assert_eq!(&nums[0..len as usize], &[2, 2]);
    }

    #[test]
    fn test_remove_element_multiple() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let len = remove_element(&mut nums, 2);
        assert_eq!(len, 5);
        let mut result = nums[0..len as usize].to_vec();
        result.sort();
        assert_eq!(result, vec![0, 0, 1, 3, 4]);
    }

    #[test]
    fn test_remove_element_all_equal() {
        let mut nums = vec![2, 2, 2, 2];
        let len = remove_element(&mut nums, 2);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_remove_element_none_equal() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let len = remove_element(&mut nums, 6);
        assert_eq!(len, 5);
        assert_eq!(&nums[0..len as usize], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_element_empty() {
        let mut nums: Vec<i32> = vec![];
        let len = remove_element(&mut nums, 1);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_remove_element_single_match() {
        let mut nums = vec![1];
        let len = remove_element(&mut nums, 1);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_remove_element_single_no_match() {
        let mut nums = vec![1];
        let len = remove_element(&mut nums, 2);
        assert_eq!(len, 1);
        assert_eq!(&nums[0..len as usize], &[1]);
    }

    #[test]
    fn test_remove_element_multiple_positions() {
        let mut nums = vec![1, 1, 1, 2, 3, 1, 4, 1];
        let len = remove_element(&mut nums, 1);
        assert_eq!(len, 3);
        let mut result = nums[0..len as usize].to_vec();
        result.sort();
        assert_eq!(result, vec![2, 3, 4]);
    }
}
